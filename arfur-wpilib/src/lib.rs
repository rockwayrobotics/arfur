pub mod error;
pub mod robot;
pub mod util;

pub mod prelude {
    //! Common re-exports.

    pub use crate::error::{Error, Result};
    pub use crate::robot::{Robot, RobotBuilder};
}

#[allow(
    rustdoc::broken_intra_doc_links,
    rustdoc::bare_urls,
    rustdoc::invalid_rust_codeblocks
)]
pub mod ffi {
    //! A raw interface to any WPILib function.
    //!
    //! All of these functions should be considered unsafe and difficult to use.

    #[cfg(feature = "bindgen")]
    include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

    #[cfg(not(feature = "bindgen"))]
    include!("./bindings.rs");
}
