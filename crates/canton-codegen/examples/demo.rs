//! Print the Rust generated for a sample Daml record.
//!
//! `cargo run -p canton-codegen --example demo`

use canton_codegen::generate_record;
use canton_codegen::ir::{DamlType, Field, Record, TypeRef};

fn main() {
    let record = Record {
        name: "AppInstallRequest".to_string(),
        type_params: Vec::new(),
        fields: vec![
            Field {
                label: "provider".to_string(),
                ty: DamlType::Party,
            },
            Field {
                label: "installId".to_string(),
                ty: DamlType::Text,
            },
            Field {
                label: "amount".to_string(),
                ty: DamlType::Numeric(10),
            },
            Field {
                label: "wallet".to_string(),
                ty: DamlType::ContractId(Box::new(DamlType::Ref(TypeRef {
                    name: "Wallet".to_string(),
                    args: Vec::new(),
                }))),
            },
            Field {
                label: "tags".to_string(),
                ty: DamlType::List(Box::new(DamlType::Text)),
            },
            Field {
                label: "note".to_string(),
                ty: DamlType::Optional(Box::new(DamlType::Text)),
            },
        ],
    };

    match generate_record(&record) {
        Ok(src) => print!("{src}"),
        Err(e) => eprintln!("codegen error: {e}"),
    }
}
