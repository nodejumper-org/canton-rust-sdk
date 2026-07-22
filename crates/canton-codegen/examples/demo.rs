//! Print the Rust generated for a sample Daml template, variant, and enum.
//!
//! `cargo run -p canton-codegen --example demo`

use canton_codegen::ir::{
    Choice, DamlType, Enum, Field, Template, TypeRef, Variant, VariantConstructor,
};
use canton_codegen::{generate_data_type, generate_template};

fn field(label: &str, ty: DamlType) -> Field {
    Field {
        label: label.to_string(),
        ty,
    }
}

fn reference(name: &str) -> DamlType {
    DamlType::Ref(TypeRef::local(name, Vec::new()))
}

fn main() {
    // A template with a payload and one consuming choice.
    let template = Template {
        name: "AppInstall".to_string(),
        module_name: "Licensing.AppInstall".to_string(),
        package_id: "example".to_string(),
        fields: vec![
            field("provider", DamlType::Party),
            field("user", DamlType::Party),
            field("amount", DamlType::Numeric(10)),
            field("tags", DamlType::List(Box::new(DamlType::Text))),
        ],
        choices: vec![Choice {
            name: "Accept".to_string(),
            consuming: true,
            argument: reference("AppInstall_Accept"),
            returns: DamlType::ContractId(Box::new(reference("AppInstalled"))),
        }],
        key: None,
    };

    // A variant (sum) type.
    let variant = canton_codegen::ir::DataType::Variant(Variant {
        name: "Shape".to_string(),
        type_params: Vec::new(),
        constructors: vec![
            VariantConstructor {
                name: "Circle".to_string(),
                payload: Some(DamlType::Numeric(10)),
            },
            VariantConstructor {
                name: "Rectangle".to_string(),
                payload: Some(reference("Dimensions")),
            },
            VariantConstructor {
                name: "Point".to_string(),
                payload: None,
            },
        ],
    });

    // A plain enum.
    let enumeration = canton_codegen::ir::DataType::Enum(Enum {
        name: "DayOfWeek".to_string(),
        constructors: vec![
            "Monday".to_string(),
            "Tuesday".to_string(),
            "Wednesday".to_string(),
        ],
    });

    println!("// ── template ──");
    emit(generate_template(&template));
    println!("\n// ── variant ──");
    emit(generate_data_type(&variant));
    println!("\n// ── enum ──");
    emit(generate_data_type(&enumeration));
}

fn emit(result: Result<String, syn::Error>) {
    match result {
        Ok(src) => print!("{src}"),
        Err(e) => eprintln!("codegen error: {e}"),
    }
}
