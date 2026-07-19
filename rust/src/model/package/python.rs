// This is free and unencumbered software released into the public domain.

impl TryFrom<export::python::PyprojectToml> for Package {
    type Error = export::python::LoadPyprojectError;

    fn try_from(input: export::python::PyprojectToml) -> Result<Self, Self::Error> {
        use export::python::{Contact, License};
        let project = input.project.unwrap();
        let project_urls = project.urls.unwrap_or_default();
        Ok(Self {
            name: project.name,
            version: project.version.map(|v| v.to_string()).unwrap_or_default(),
            authors: project
                .authors
                .unwrap_or_default()
                .iter()
                .filter_map(Contact::name)
                .map(ToString::to_string)
                .collect(),
            description: project.description,
            homepage: project_urls.get("Homepage").cloned(),
            keywords: project.keywords.unwrap_or_default(),
            categories: project.classifiers.unwrap_or_default(),
            licenses: match project.license {
                Some(License::Spdx(s)) => vec![s],
                _ => vec![], // TODO
            },
            repository: project_urls.get("Repository").cloned(),
            metadata: None, // TODO
        })
    }
}
