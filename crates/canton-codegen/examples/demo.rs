//! Print the Rust generated for a sample Daml template, variant, and enum.
//!
//! `cargo run -p canton-codegen --example demo`

use canton_codegen::ir::{
    Choice, DamlType, Enum, Field, Template, TypeRef, Variant, VariantConstructor,
};
use canton_codegen::{generate_data_type, generate_template};

fn field(label: &str, ty: DamlType) -> Field {
    Field::new(label, ty)
}

fn reference(name: &str) -> DamlType {
    DamlType::Ref(TypeRef::local(name, Vec::new()))
}

fn main() {
    // A template with a payload and one consuming choice.
    let mut template = Template::new(
        "AppInstall",
        "Licensing.AppInstall",
        "example",
        "quickstart-licensing",
    );
    template.fields = vec![
        field("provider", DamlType::Party),
        field("user", DamlType::Party),
        field("amount", DamlType::Numeric(10)),
        field("tags", DamlType::List(Box::new(DamlType::Text))),
    ];
    let mut accept = Choice::new(
        "Accept",
        reference("AppInstall_Accept"),
        DamlType::ContractId(Box::new(reference("AppInstalled"))),
    );
    accept.consuming = true;
    template.choices = vec![accept];

    // A variant (sum) type.
    let mut shape = Variant::new("Shape");
    shape.constructors = vec![
        VariantConstructor::with_payload("Circle", DamlType::Numeric(10)),
        VariantConstructor::with_payload("Rectangle", reference("Dimensions")),
        VariantConstructor::new("Point"),
    ];
    let variant = canton_codegen::ir::DataType::Variant(shape);

    // A plain enum.
    let enumeration = canton_codegen::ir::DataType::Enum(Enum::new(
        "DayOfWeek",
        vec![
            "Monday".to_string(),
            "Tuesday".to_string(),
            "Wednesday".to_string(),
        ],
    ));

    println!("// ── template ──");
    emit(generate_template(&template));
    println!("\n// ── variant ──");
    emit(generate_data_type(&variant));
    println!("\n// ── enum ──");
    emit(generate_data_type(&enumeration));
}

fn emit(result: Result<String, canton_codegen::CodegenError>) {
    match result {
        Ok(src) => print!("{src}"),
        Err(e) => eprintln!("codegen error: {e}"),
    }
}
