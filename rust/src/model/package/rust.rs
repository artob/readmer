// This is free and unencumbered software released into the public domain.

/// Converts Cargo package metadata with the `rust` feature enabled.
///
/// Returns a load error if `[package]` is absent or a metadata field still needs
/// workspace inheritance. Use [`Package::load`] to resolve inheritance from disk
/// before conversion; this conversion itself performs no filesystem access.
impl TryFrom<distrib::rust::Manifest> for Package {
    type Error = distrib::rust::LoadManifestError;

    fn try_from(input: distrib::rust::Manifest) -> Result<Self, Self::Error> {
        use distrib::rust::{Edition, Value};
        let package = input
            .package
            .ok_or_else(|| Self::Error::Other("Cargo manifest has no [package] section".into()))?;
        let rust_edition = package.edition.get()?;
        let rust_version = package
            .rust_version
            .map(|x| x.get().cloned())
            .transpose()?
            .unwrap_or_else(|| {
                match rust_edition {
                    Edition::E2024 => "2024",
                    Edition::E2021 => "2021",
                    Edition::E2018 => "2018",
                    Edition::E2015 => "2015",
                    _ => "2015", // Cargo assumes 2015 if absent
                }
                .into()
            });
        Ok(Self {
            language: Language {
                name: "rust".into(),
                label: "Rust".into(),
                extensions: vec![".rs".into(), ".rs.in".into()],
                version: Some(rust_version.clone()),
                minimum_version: Some(rust_version),
                ..Default::default()
            },
            languages: vec![],
            name: package.name,
            version: package.version.get()?.to_string(),
            authors: package.authors.get()?.clone(),
            description: package.description.map(|x| x.get().cloned()).transpose()?,
            homepage: package.homepage.map(|x| x.get().cloned()).transpose()?,
            keywords: package.keywords.get()?.clone(),
            categories: package.categories.get()?.clone(),
            licenses: match package.license {
                None => vec![],
                Some(x) => vec![x.get()?.clone()],
            },
            repository: package.repository.map(|x| x.get().cloned()).transpose()?,
            metadata: package.metadata.map(|x| x.try_into()).transpose()?,
        })
    }
}
