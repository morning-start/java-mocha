#[cfg(test)]
mod tests {
    use jvm::func::query::{query_info, query_info_version, query_info_term};
    use jvm::core::datatype::SupportTerm;
    use jvm::func::config::Config;

    #[test]
    fn test_query_info() {
        // This is a basic test structure
        // In a real scenario, you would need to set up test data
        // and verify the function works correctly
        let cfg = Config::load().expect("Failed to load config");
        let data_dir = &cfg.data_dir;
        let publisher = "oracle";
        
        // This would fail without proper test data setup
        let result = query_info(data_dir, publisher);
        assert!(result.is_ok());
    }

    #[test]
    fn test_query_info_version() {
        // This is a basic test structure
        // In a real scenario, you would need to set up test data
        // and verify the function works correctly
        let cfg = Config::load().expect("Failed to load config");
        let data_dir = &cfg.data_dir;
        let publisher = "oracle";
        let major_version = 11;
        
        // This would fail without proper test data setup
        let result = query_info_version(data_dir, publisher, major_version);
        assert!(result.is_ok());
    }

    #[test]
    fn test_query_info_term() {
        // This is a basic test structure
        // In a real scenario, you would need to set up test data
        // and verify the function works correctly
        let cfg = Config::load().expect("Failed to load config");
        let data_dir = &cfg.data_dir;
        let publisher = "oracle";
        let term_of_support = SupportTerm::LTS;
        
        // This would fail without proper test data setup
        let result = query_info_term(data_dir, publisher, term_of_support);
        assert!(result.is_ok());
    }
}