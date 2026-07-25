//> using scala 2.13
//> using dep com.daml:daml-lf-archive-reader_2.13:3.4.11
//> using dep com.lihaoyi::ujson:4.4.3
//> using dep org.slf4j:slf4j-nop:2.0.16

// Conformance oracle: read a DAR with the OFFICIAL `daml-lf-archive` reader
// (the same decoder Canton itself uses) and print its type-signature surface
// as canonical JSON. `crates/canton-lf/tests/oracle.rs` renders the identical
// document from the SDK's native decoder and asserts the two are equal.
//
// Dev-only tooling: this script is never part of the published SDK. Run via
//   scala-cli run tools/lf-oracle/LfOracle.scala -- /path/to/x.dar

import com.digitalasset.daml.lf.archive.DarDecoder
import com.digitalasset.daml.lf.data.Ref
import com.digitalasset.daml.lf.language.Ast

object LfOracle {

  /** `pkgid:Module.Path:Name` — the reference spelling shared with the Rust side. */
  private def renderRef(id: Ref.Identifier): String =
    s"${id.packageId}:${id.qualifiedName.module.dottedName}:${id.qualifiedName.name.dottedName}"

  private def builtinName(bt: Ast.BuiltinType): String = bt match {
    case Ast.BTUnit            => "Unit"
    case Ast.BTBool            => "Bool"
    case Ast.BTInt64           => "Int64"
    case Ast.BTDate            => "Date"
    case Ast.BTTimestamp       => "Timestamp"
    case Ast.BTNumeric         => "Numeric"
    case Ast.BTParty           => "Party"
    case Ast.BTText            => "Text"
    case Ast.BTContractId      => "ContractId"
    case Ast.BTOptional        => "Optional"
    case Ast.BTList            => "List"
    case Ast.BTGenMap          => "GenMap"
    case Ast.BTTextMap         => "TextMap"
    case Ast.BTAny             => "Any"
    case Ast.BTAnyException    => "AnyException"
    case Ast.BTTypeRep         => "TypeRep"
    case Ast.BTArrow           => "Arrow"
    case Ast.BTUpdate          => "Update"
    case Ast.BTFailureCategory => "FailureCategory"
    case Ast.BTBigNumeric      => "BigNumeric"
    case Ast.BTRoundingMode    => "RoundingMode"
    case other                 => sys.error(s"unmapped builtin type: $other")
  }

  /** Render a type in the shared grammar; the curried `TApp` spine is flattened
    * so `TApp(TApp(List, a), b)` and a flat application render identically.
    */
  private def renderType(ty: Ast.Type): String = {
    def spine(t: Ast.Type, args: List[Ast.Type]): (String, List[String]) = t match {
      case Ast.TApp(fun, arg) => spine(fun, arg :: args)
      case Ast.TVar(name)     => (name, args.map(renderType))
      case Ast.TNat(n)        => (n.toString, args.map(renderType))
      case Ast.TBuiltin(bt)   => (builtinName(bt), args.map(renderType))
      case Ast.TTyCon(id)     => (renderRef(id), args.map(renderType))
      case Ast.TSynApp(id, synArgs) =>
        (s"%syn ${renderRef(id)}", (synArgs.toList ++ args).map(renderType))
      case Ast.TForall((v, _), body) =>
        // Collapse nested foralls into one binder list, like the proto encoding.
        def collect(t: Ast.Type, vars: List[String]): (List[String], Ast.Type) = t match {
          case Ast.TForall((v2, _), b2) => collect(b2, vars :+ v2)
          case other                    => (vars, other)
        }
        val (vars, innermost) = collect(body, List(v))
        (s"%forall (${vars.mkString(" ")}) ${renderType(innermost)}", args.map(renderType))
      case Ast.TStruct(fields) =>
        val rendered = fields.iterator
          .map { case (name, fieldType) => s"($name ${renderType(fieldType)})" }
          .mkString(" ")
        (s"%struct $rendered", args.map(renderType))
    }
    spine(ty, Nil) match {
      case (head, Nil) if !head.contains(' ') => head
      case (head, rendered)                   => s"(${(head :: rendered).mkString(" ")})".replace(" )", ")")
    }
  }

  private def dataTypeJson(name: Ref.DottedName, dt: Ast.DDataType): Option[ujson.Value] = {
    if (!dt.serializable) return None
    val params = dt.params.iterator.map { case (v, _) => ujson.Str(v) }.toList
    val base = ujson.Obj(
      "name" -> name.dottedName,
      "params" -> ujson.Arr.from(params),
    )
    dt.cons match {
      case Ast.DataRecord(fields) =>
        base("kind") = "record"
        base("fields") = ujson.Arr.from(fields.iterator.map { case (label, fieldType) =>
          ujson.Arr(ujson.Str(label), ujson.Str(renderType(fieldType)))
        })
      case Ast.DataVariant(variants) =>
        base("kind") = "variant"
        base("constructors") = ujson.Arr.from(variants.iterator.map { case (ctor, payload) =>
          ujson.Arr(ujson.Str(ctor), ujson.Str(renderType(payload)))
        })
      case Ast.DataEnum(constructors) =>
        base("kind") = "enum"
        base("constructors") = ujson.Arr.from(constructors.iterator.map(ujson.Str(_)))
      case Ast.DataInterface => return None // marker; never serializable
    }
    Some(base)
  }

  private def choicesJson(choices: Iterable[Ast.GenTemplateChoice[_]]): ujson.Value =
    ujson.Arr.from(choices.map { choice =>
      ujson.Obj(
        "name" -> choice.name.toString,
        "consuming" -> choice.consuming,
        "arg" -> renderType(choice.argBinder._2),
        "ret" -> renderType(choice.returnType),
      )
    })

  private def templateJson(name: Ref.DottedName, template: Ast.GenTemplate[_]): ujson.Value =
    ujson.Obj(
      "name" -> name.dottedName,
      "key" -> template.key.fold[ujson.Value](ujson.Null)(k => ujson.Str(renderType(k.typ))),
      "implements" -> ujson.Arr.from(
        template.implements.keys.map(renderRef).toList.sorted.map(ujson.Str(_))
      ),
      "choices" -> choicesJson(template.choices.values),
    )

  private def interfaceJson(name: Ref.DottedName, interface: Ast.GenDefInterface[_]): ujson.Value =
    ujson.Obj(
      "name" -> name.dottedName,
      "view" -> ujson.Str(renderType(interface.view)),
      "choices" -> choicesJson(interface.choices.values),
    )

  private def packageJson(id: Ref.PackageId, pkg: Ast.GenPackage[_]): ujson.Value = {
    val modules = pkg.modules.toList.flatMap { case (moduleName, module) =>
      val dataTypes = module.definitions.toList.flatMap {
        case (name, dt: Ast.DDataType) => dataTypeJson(name, dt)
        case _                         => None
      }
      val templates = module.templates.toList.map { case (name, template) =>
        templateJson(name, template)
      }
      val interfaces = module.interfaces.toList.map { case (name, interface) =>
        interfaceJson(name, interface)
      }
      if (dataTypes.isEmpty && templates.isEmpty && interfaces.isEmpty) None
      else
        Some(
          ujson.Obj(
            "name" -> moduleName.dottedName,
            "dataTypes" -> ujson.Arr.from(dataTypes),
            "templates" -> ujson.Arr.from(templates),
            "interfaces" -> ujson.Arr.from(interfaces),
          )
        )
    }
    ujson.Obj(
      "id" -> id.toString,
      "name" -> pkg.metadata.name.toString,
      "version" -> pkg.metadata.version.toString,
      "modules" -> ujson.Arr.from(modules),
    )
  }

  def main(args: Array[String]): Unit = {
    require(args.length == 1, "usage: LfOracle <path-to-dar>")
    val dar = DarDecoder.assertReadArchiveFromFile(new java.io.File(args(0)))
    val packages = dar.all.map { case (id, pkg) => packageJson(id, pkg) }
    println(ujson.write(ujson.Obj("packages" -> ujson.Arr.from(packages))))
  }
}
