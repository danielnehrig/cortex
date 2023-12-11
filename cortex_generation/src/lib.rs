pub mod stored_proc_extractor;
pub mod table_extractor;

pub fn create_data() {}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use syn::visit::Visit;

    use crate::{stored_proc_extractor::StoredProcExtractor, table_extractor::TableExtractor};

    #[test]
    fn test_table_extractor() {
        let mut extractor2 = TableExtractor {
            data: HashMap::new(),
        };
        extractor2.visit_file(
            &syn::parse_file(
                r#"
        fn main() {
            let mut sp = Table::new("table_name")
            .add_prop("prop1", PropType::i32)
            .add_prop("prop2", PropType::i32);
        }
    "#,
            )
            .expect("Failed to parse file"),
        );

        assert_eq!(
            extractor2.data.get("table_name").unwrap(),
            &vec![
                ("prop2".to_string(), "i32".to_string()),
                ("prop1".to_string(), "i32".to_string()),
            ]
        );
    }

    #[test]
    fn test_stored_proc_extractor() {
        let mut extractor = StoredProcExtractor {
            data: std::collections::HashMap::new(),
        };
        extractor.visit_file(
            &syn::parse_file(
                r#"
        fn main() {
            let mut sp = StoredProcedure::new("sp_name")
            .add_param("param1", PropType::i32)
            .add_param("param2", PropType::i32)
            .add_return(PropType::i32);
        }
    "#,
            )
            .expect("Failed to parse file"),
        );

        assert_eq!(
            extractor.data.get("sp_name").unwrap().0,
            vec![
                ("param2".to_string(), "i32".to_string()),
                ("param1".to_string(), "i32".to_string()),
            ]
        );

        assert_eq!(extractor.data.get("sp_name").unwrap().1, "i32".to_string());
    }
}
