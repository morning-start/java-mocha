use clap::{Parser, Subcommand};
// use core::style::{show_table, show_tree};
use jvm::core::datatype::SupportTerm;
use jvm::core::style::{show_table, show_tree};
use jvm::func::{
    config::{Config, init_config},
    install::full_install_process,
    list::{list_local_jdk, list_publish_version, list_publisher, list_version},
    query::{query_info, query_info_term, query_info_version},
    switch::switch_jdk,
    sync::sync_data,
    uninstall::uninstall_jdk,
};
use std::path::PathBuf;

#[derive(Parser)]
#[clap(
    name = "jvm",
    version = "1.0.0",
    about = "Java Mocha is a Java version management tool developed based on the Foojay API.",
    long_about = "Java Mocha is a Java version management tool developed based on the Foojay API.\nIt can be used for version management via the command-line interface or integrated through the API.",
    after_help = "Before using, \n1. please first initialize the configuration with `jvm config`,\n2. then sync the data with `jvm sync`. \n3. Use `--help` to view specific command usage."
)]
struct Cli {
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Configure the JDK directory, and cache directory for Java Mocha.
    #[clap(name = "config", long_flag = "cfg")]
    Config {
        /// JDK directory, default is the `jdk` directory under the JVM root directory.
        #[clap(
            long,
            env = "JDK_HOME",
            help = "JDK directory, default is the `jdk` directory under the JVM root directory."
        )]
        jdk_home: Option<String>,
        /// Cache directory, default is the `cache` directory under the JVM root directory.
        #[clap(long)]
        cache_home: Option<String>,
        /// JAVA_HOME env, the source of symlink.
        #[clap(long, env = "JAVA_HOME")]
        java_home: Option<String>,
        /// Proxy server, default is the `http_proxy` environment variable.
        #[clap(long)]
        proxy: Option<String>,
    },
    /// Sync the Foojay JDK data to local JSON files.
    #[clap(name = "sync")]
    Sync {},
    /// List infos for local jdk, all publisher, all version.
    #[clap(name = "list", long_flag = "ls")]
    List {
        /// Publisher name.
        #[clap(short, long)]
        publisher: bool,
        /// Version flag.
        #[clap(short, long)]
        version: bool,
    },
    /// Query available JDKs.
    #[clap(name = "query", long_flag = "q")]
    Query {
        /// The publisher name.
        publisher: String,
        /// Detailed major version information.
        #[clap(short = 'v', long)]
        major_version: Option<i32>,
        /// Term of support.
        #[clap(short, long)]
        term_of_support: Option<SupportTerm>,
    },
    /// Install JDKs.
    #[clap(name = "install", long_flag = "i")]
    Install {
        /// The JDK version format as publisher@version
        /// e.g. oracle@23, oracle@23.0.2, oracle@latest, oracle@lts
        jdk: String,
        /// Force install.
        #[clap(short, long)]
        force: bool,
        /// Skip checksum verification.
        #[clap(short = 's', long)]
        skip_check: bool,
    },
    /// Switch java version.
    #[clap(name = "switch", long_flag = "sw")]
    Switch {
        /// The JDK version format as publisher@version e.g. oracle@11
        jdk: String,
    },
    /// Uninstall JDKs.
    #[clap(name = "uninstall", long_flag = "rm")]
    Uninstall {
        /// The JDK version format as publisher@version e.g. oracle@11
        jdk: String,
    },
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Config {
            jdk_home,
            cache_home,
            java_home,
            proxy,
        } => {
            let jvm_root = Config::load_jvm();
            let jdk_home_path = jdk_home.map(PathBuf::from);
            let java_home_path = java_home.map(PathBuf::from);
            let cache_home_path = cache_home.map(PathBuf::from);

            match init_config(
                jvm_root,
                jdk_home_path,
                java_home_path,
                cache_home_path,
                proxy,
            ) {
                Ok(cfg) => {
                    println!("Config saved successfully.");
                    let env_java_home = std::env::var("JAVA_HOME").unwrap();
                    let env_java_home = PathBuf::from(env_java_home);
                    if cfg.java_home.eq(&env_java_home) {
                        println!("JAVA_HOME is set to {}", cfg.java_home.display());
                    } else {
                        println!("Please set JAVA_HOME to {} manually.", cfg.java_home.display());
                    }
                }
                Err(e) => eprintln!("Error: {}", e),
            }
        }
        Commands::Sync {} => match Config::load() {
            Ok(cfg) => {
                let rt = tokio::runtime::Runtime::new().unwrap();
                rt.block_on(sync_data(&cfg));
            }
            Err(e) => eprintln!("Error: {}", e),
        },
        Commands::List { publisher, version } => match Config::load() {
            Ok(cfg) => {
                if !publisher && !version {
                    let jdks = list_local_jdk(&cfg.jdk_home);
                    if jdks.is_empty() {
                        println!("No JDK found, please install first.");
                    } else {
                        let tree =
                            show_tree(&jdks, &cfg.jdk_version, "Installed jdk (* marks in use)");
                        println!("{}", tree);
                    }
                } else if publisher && version {
                    let data = list_publish_version(&cfg.data_dir);
                    let table = show_table(&data, "Each publisher available major versions");
                    println!("{}", table);
                } else if version {
                    let version_data = list_version(&cfg.data_dir);
                    let table = show_table(&version_data, "major versions infos");
                    println!("{}", table);
                } else if publisher {
                    let publisher_data = list_publisher(&cfg.data_dir);
                    let table = show_table(&publisher_data, "publishers infos");
                    println!("{}", table);
                }
            }
            Err(e) => eprintln!("Error: {}", e),
        },
        Commands::Query {
            publisher,
            major_version,
            term_of_support,
        } => match Config::load() {
            Ok(cfg) => {
                if term_of_support.is_some() && major_version.is_some() {
                    eprintln!("term_of_support and major_version can not be used at the same time");
                } else if let Some(major_version) = major_version {
                    match query_info_version(&cfg.data_dir, &publisher, major_version) {
                        Ok(data) => {
                            let table = show_table(
                                &data,
                                &format!("JDKs for {} version {}", publisher, major_version),
                            );
                            println!("{}", table);
                        }
                        Err(e) => eprintln!("Query failed: {}", e),
                    }
                } else if let Some(term_of_support) = term_of_support {
                    match query_info_term(&cfg.data_dir, &publisher, term_of_support) {
                        Ok(data) => {
                            let table = show_table(
                                &data,
                                &format!("Latest JDKs for {} on {}", publisher, term_of_support),
                            );
                            println!("{}", table);
                        }
                        Err(e) => eprintln!("Query failed: {}", e),
                    }
                } else {
                    match query_info(&cfg.data_dir, &publisher) {
                        Ok(data) => {
                            let table = show_table(
                                &data,
                                &format!("Latest JDKs for publisher {}", publisher),
                            );
                            println!("{}", table);
                        }
                        Err(e) => eprintln!("Query failed: {}", e),
                    }
                }
            }
            Err(e) => eprintln!("Error: {}", e),
        },
        Commands::Install {
            jdk,
            force,
            skip_check,
        } => match Config::load() {
            Ok(cfg) => {
                if full_install_process(&jdk, &cfg, force, skip_check)
                    .await
                    .is_ok()
                {
                    println!("Install JDK: {}", jdk);
                } else {
                    eprintln!("Install JDK {} failed.", jdk);
                }
            }
            Err(e) => eprintln!("Error: {}", e),
        },
        Commands::Switch { jdk } => match Config::load() {
            Ok(cfg) => match switch_jdk(&jdk, &cfg) {
                Ok(true) => println!("JDK switched to {}.", jdk),
                Ok(false) => eprintln!("JDK {} not found.", jdk),
                Err(e) => eprintln!("Switch failed: {}", e),
            },
            Err(e) => eprintln!("Error: {}", e),
        },
        Commands::Uninstall { jdk } => match Config::load() {
            Ok(cfg) => {
                if uninstall_jdk(&jdk, &cfg) {
                    println!("JDK {} uninstalled successfully.", jdk);
                } else {
                    eprintln!("JDK {} not found.", jdk);
                }
            }
            Err(e) => eprintln!("Error: {}", e),
        },
    }
}
