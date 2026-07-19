// This is free and unencumbered software released into the public domain.

/// Support for Dart projects.
#[cfg(feature = "dart")]
pub mod dart {
    mod error;
    pub use error::*;

    mod load;
    pub use load::*;

    pub mod pubspec;
    pub use pubspec::Pubspec;
}

/// Support for JavaScript/TypeScript projects.
#[cfg(feature = "js")]
pub mod js {
    mod error;
    pub use error::*;

    mod load;
    pub use load::*;

    pub mod package;
    pub use package::*;
}

/// Support for Python projects.
#[cfg(feature = "python")]
pub mod python {
    mod error;
    pub use error::*;

    mod load;
    pub use load::*;

    pub mod pyproject;
    pub use pyproject::*;
}

/// Support for Ruby projects.
#[cfg(feature = "ruby")]
pub mod ruby {
    mod error;
    pub use error::*;

    pub mod gemspec;
    pub use gemspec::*;

    mod load;
    pub use load::*;
}

/// Support for Rust projects.
#[cfg(feature = "rust")]
pub mod rust {
    mod error;
    pub use error::*;

    mod load;
    pub use load::*;

    pub mod manifest;
    pub use manifest::*;
}
