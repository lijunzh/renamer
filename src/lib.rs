pub mod cli;
pub mod config;
pub mod error;  // Keep this module
pub mod file_ops;
pub mod renamer;

pub use cli::Cli;
pub use config::merge_config;
pub use error::RenamerError;  // Export from error module
pub use renamer::{PlannedRename, transform_filename, check_warning};
pub use file_ops::should_process_file;
