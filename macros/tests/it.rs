#[cfg(test)]
mod test {
    #[test]
    fn test_call_generation() {
        #[macros::create_stored_proc]
        fn create_stored() {
            let test = StoredProcedure::new("test2").add_param("id", PropType::Int32);
            let test2 = StoredProcedure::new("test3").add_param("id", PropType::Int32);
        }
    }

    // Mock
    struct StoredProcedure;
    impl StoredProcedure {
        pub fn new(name: &str) -> Self {
            Self
        }

        pub fn add_param(self, _name: &str, _ptype: PropType) -> Self {
            self
        }
    }

    enum PropType {
        Int32,
        VarChar,
        Float,
    }
}
