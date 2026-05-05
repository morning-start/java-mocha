pub mod core;
pub mod func;

pub use crate::core::state::{JdkRequest, VersionSpec, OperationState, OperationEvent, OperationResult};
pub use crate::core::converter::{InputConverter, CliInputConverter, CustomInputConverter, VersionSpecConverter, JdkRequestConverter};
pub use crate::core::installer::JdkInstaller;