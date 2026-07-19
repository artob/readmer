// This is free and unencumbered software released into the public domain.

impl TryFrom<export::ruby::Gemspec> for Package {
    type Error = export::ruby::LoadGemspecError;

    fn try_from(input: export::ruby::Gemspec) -> Result<Self, Self::Error> {
        let input_metadata = input.metadata.unwrap_or_default();
        Ok(Self {
            name: input.name,
            version: input.version.version,
            authors: input.authors,
            description: input.description,
            homepage: input.homepage,
            keywords: vec![],
            categories: vec![],
            licenses: input.licenses,
            repository: input_metadata.source_code_uri,
            metadata: Some(serde_json::Value::Object(input_metadata.other)),
        })
    }
}
