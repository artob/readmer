// This is free and unencumbered software released into the public domain.

impl TryFrom<export::dart::Pubspec> for Package {
    type Error = export::dart::LoadPubspecError;

    fn try_from(input: export::dart::Pubspec) -> Result<Self, Self::Error> {
        use itertools::Itertools;
        let dart_version = input
            .environment
            .map(|e| e.sdk.replace('^', "").split('.').take(2).join(".")); // FIXME
        Ok(Self {
            language: Language {
                name: "dart".into(),
                label: "Dart".into(),
                extensions: vec![".dart".into()],
                version: dart_version.clone(),
                minimum_version: dart_version,
                ..Default::default()
            },
            languages: vec![],
            name: input.name,
            version: input.version.unwrap_or_default(),
            authors: vec![], // N/A: deprecated since Dart 2.7
            description: input.description,
            homepage: input.homepage,
            keywords: input.topics.unwrap_or_default(),
            categories: vec![], // N/A
            licenses: vec![],   // TODO: detect from `LICENSE` file
            repository: input.repository,
            metadata: None, // TODO
        })
    }
}
