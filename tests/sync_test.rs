#[cfg(test)]
mod tests {
    use jvm::func::sync::sync_data;
    use jvm::func::config::Config;
    use jvm::core::datatype::DataFile;

    #[tokio::test]
    async fn test_sync_data() {
        // Load the configuration to get the data directory
        let cfg = Config::load().expect("Failed to load config");
        
        // Run the sync_data function
        sync_data(&cfg).await;
        
        // Check that the expected files were created
        let distributions_file = cfg.data_dir.join(DataFile::Distributions.as_ref());
        let versions_file = cfg.data_dir.join(DataFile::Versions.as_ref());
        let packages_file = cfg.data_dir.join(DataFile::Packages.as_ref());
        
        assert!(distributions_file.exists(), "distributions.json should exist");
        assert!(versions_file.exists(), "versions.json should exist");
        assert!(packages_file.exists(), "packages.json should exist");
    }
}