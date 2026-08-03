// This is free and unencumbered software released into the public domain.

impl TryFrom<distrib::gleam::PackageConfig> for Package {
    type Error = distrib::gleam::LoadPackageError;

    fn try_from(input: distrib::gleam::PackageConfig) -> Result<Self, Self::Error> {
        use itertools::Itertools;
        Ok(Self {
            language: Language {
                name: "gleam".into(),
                label: "Gleam".into(),
                extensions: vec![".gleam".into()],
                version: None,
                minimum_version: None, // TODO: detect from `gleam.toml`
                ..Default::default()
            },
            languages: vec![],
            name: input.name,
            version: input.version,
            authors: vec![], // N/A
            description: input.description,
            homepage: input
                .links
                .unwrap_or_default()
                .into_iter()
                .filter_map(|link| {
                    if link.title == "Website" {
                        Some(link.href.clone())
                    } else {
                        None
                    }
                })
                .next(),
            keywords: vec![],   // N/A
            categories: vec![], // N/A
            licenses: input.licenses.unwrap_or_default(),
            repository: None, // TODO: input.repository
            metadata: None,   // TODO
        })
    }
}
