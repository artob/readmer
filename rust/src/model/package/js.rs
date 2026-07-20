// This is free and unencumbered software released into the public domain.

impl TryFrom<export::js::PackageJson> for Package {
    type Error = export::js::LoadPackageError;

    fn try_from(input: export::js::PackageJson) -> Result<Self, Self::Error> {
        use package_json_schema::{Person, PersonObject, Repository};
        let js_version = None; // TODO
        Ok(Self {
            language: Language {
                name: "js".into(),
                label: "JavaScript".into(),
                extensions: vec![".js".into(), ".jsx".into(), ".cjs".into(), ".mjs".into()],
                version: js_version.clone(),
                minimum_version: js_version,
                ..Default::default()
            },
            languages: vec![],
            name: input.name.unwrap_or_default(),
            version: input.version.unwrap_or_default(),
            authors: input
                .author
                .into_iter()
                .map(|person| match person {
                    Person::String(name) => name,
                    Person::Object(person) => person.name,
                })
                .collect(),
            description: input.description,
            homepage: input.homepage,
            keywords: input.keywords.unwrap_or_default(),
            categories: vec![], // N/A?
            licenses: input.license.into_iter().collect(),
            repository: match input.repository {
                Some(Repository::Object {
                    url: Some(mut url), ..
                }) => Some(if let Some(s) = url.strip_suffix(".git") {
                    url.truncate(s.len());
                    url
                } else {
                    url
                }),
                _ => None,
            },
            metadata: None, // TODO: Some(serde_json::Value::Object(input_metadata.other)),
        })
    }
}
