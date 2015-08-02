/// This contains the platform dependent code
/// For each platform the following must be provided:
///  * A `NativeConnection` type, which implements the `Drop` trait,
///    the `std::io::Read` and `std::io::Write` traits
///  * The following functions:
///    * connect(name: &str) -> Result<NativeConnection, std::io::Error>;

#[cfg(not(target_os="windows"))]
pub mod unix;
#[cfg(not(target_os="windows"))]
pub use self::unix::{NativeConnection, connect};

