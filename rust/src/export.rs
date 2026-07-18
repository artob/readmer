// This is free and unencumbered software released into the public domain.

#[cfg(feature = "dart")]
pub mod dart {
    mod error;
    pub use error::*;
    mod pubspec;
    pub use pubspec::*;
}

#[cfg(feature = "js")]
pub mod js {
    mod error;
    pub use error::*;
    mod package;
    pub use package::*;
}

#[cfg(feature = "python")]
pub mod python {
    mod error;
    pub use error::*;
    mod pyproject;
    pub use pyproject::*;
}

#[cfg(feature = "ruby")]
pub mod ruby {
    mod error;
    pub use error::*;
    mod gemspec;
    pub use gemspec::*;
}

#[cfg(feature = "rust")]
pub mod rust {
    mod error;
    pub use error::*;
    mod manifest;
    pub use manifest::*;
}
