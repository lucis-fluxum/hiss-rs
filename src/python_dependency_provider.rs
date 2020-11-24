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
        _package: &String,
        _version: &Version,
    ) -> Result<Dependencies<String, Version>, Box<dyn Error>> {
        todo!()
    }
}
