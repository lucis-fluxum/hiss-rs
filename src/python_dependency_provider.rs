use std::{borrow::Borrow, error::Error};

use pubgrub::{
    range::Range,
    solver::{Dependencies, DependencyProvider},
};

use crate::Version;

#[derive(Debug, Clone, Default)]
struct PythonDependencyProvider {}

impl DependencyProvider<String, Version> for PythonDependencyProvider {
    fn choose_package_version<T: Borrow<String>, U: Borrow<Range<Version>>>(
        &self,
        _potential_packages: impl Iterator<Item = (T, U)>,
    ) -> Result<(T, Option<Version>), Box<dyn Error>> {
        todo!()
    }

    fn get_dependencies(
        &self,
        package: &String,
        version: &Version,
    ) -> Result<Dependencies<String, Version>, Box<dyn Error>> {
        let url = format!("https://pypi.org/pypi/{}/{}/json", package, version);
        let json: serde_json::Value = reqwest::blocking::get(&url)?.json()?;
        let requirements: Vec<String> =
            serde_json::from_value(json["info"]["requires_dist"].clone())?;
        println!("{:#?}", requirements);
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_dependencies_test() {
        let provider = PythonDependencyProvider {};
        provider
            .get_dependencies(
                &String::from("matisse-controller"),
                &Version::new("0.4.0").unwrap(),
            )
            .unwrap();
    }
}
