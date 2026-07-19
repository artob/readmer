// This is free and unencumbered software released into the public domain.

impl TryFrom<export::rust::Manifest> for Package {
    type Error = export::rust::LoadManifestError;

    fn try_from(input: export::rust::Manifest) -> Result<Self, Self::Error> {
        use export::rust::Value;
        assert!(!input.needs_workspace_inheritance());
        let package = input.package.unwrap();
        let rust_version = package.rust_version.map(|x| x.unwrap());
        Ok(Self {
            language: Language {
                name: "rust".into(),
                label: "Rust".into(),
                extensions: vec![".rs".into(), ".rs.in".into()],
                version: rust_version.clone(),
                minimum_version: rust_version,
                ..Default::default()
            },
            languages: vec![],
            name: package.name,
            version: package.version.unwrap().to_string(),
            authors: package.authors.unwrap(),
            description: package.description.map(|x| x.unwrap()),
            homepage: package.homepage.map(|x| x.unwrap()),
            keywords: package.keywords.unwrap(),
            categories: package.categories.unwrap(),
            licenses: match package.license {
                None => vec![],
                Some(x) => vec![x.unwrap()],
            },
            repository: package.repository.map(|x| x.unwrap()),
            metadata: package.metadata.map(|x| x.try_into()).transpose()?,
        })
    }
}
