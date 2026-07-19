// This is free and unencumbered software released into the public domain.

impl TryFrom<export::ruby::Gemspec> for Package {
    type Error = export::ruby::LoadGemspecError;

    fn try_from(input: export::ruby::Gemspec) -> Result<Self, Self::Error> {
        let input_metadata = input.metadata.unwrap_or_default();
        let ruby_version = input
            .required_ruby_version
            .and_then(|r| r.requirements.into_iter().next())
            .map(|e| e.1.version.clone());
        Ok(Self {
            language: Language {
                name: "ruby".into(),
                label: "Ruby".into(),
                extensions: vec![
                    ".rb".into(),
                    ".ru".into(),
                    ".rake".into(),
                    ".gemspec".into(),
                ],
                version: ruby_version.clone(),
                minimum_version: ruby_version,
                ..Default::default()
            },
            languages: vec![],
            name: input.name,
            version: input.version.version,
            authors: input.authors,
            description: input.description,
            homepage: input.homepage,
            keywords: vec![],   // N/A
            categories: vec![], // N/A
            licenses: input.licenses,
            repository: input_metadata.source_code_uri,
            metadata: Some(serde_json::Value::Object(input_metadata.other)),
        })
    }
}
