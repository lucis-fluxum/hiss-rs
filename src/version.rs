use std::{cmp::Ordering, fmt};

use pyo3::{prelude::*, types::IntoPyDict};

#[derive(Debug, Clone)]
pub struct Version {
    inner: PyObject,
}

impl Version {
    pub fn new(version: &str) -> PyResult<Self> {
        Python::with_gil(|py| {
            Ok(Version {
                inner: py
                    .import("packaging.version")?
                    .get("Version")?
                    .call1((version,))?
                    .to_object(py),
            })
        })
    }

    pub fn zero() -> Self {
        Version::new("0").unwrap()
    }
}

impl<'p> fmt::Display for Version {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let string = Python::with_gil(|py| {
            self.inner
                .as_ref(py)
                .call_method0("__str__")
                .unwrap()
                .extract::<String>()
                .unwrap()
        });
        f.write_str(&string)
    }
}

impl<'p> PartialEq for Version {
    fn eq(&self, other: &Self) -> bool {
        Python::with_gil(|py: Python| {
            self.inner.as_ref(py).compare(&other.inner).unwrap() == Ordering::Equal
        })
    }
}
impl<'p> Eq for Version {}

impl<'p> PartialOrd for Version {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Python::with_gil(|py: Python| Some(self.inner.as_ref(py).compare(&other.inner).unwrap()))
    }
}
impl<'p> Ord for Version {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl<'p> pubgrub::version::Version for Version {
    fn lowest() -> Self {
        Version::zero()
    }

    fn bump(&self) -> Self {
        Python::with_gil(|py: Python| {
            let (epoch, mut release, pre_release, post_release, dev_release, local_version): (
                usize,
                Vec<usize>,
                Option<(String, usize)>,
                Option<(String, usize)>,
                Option<(String, usize)>,
                Option<String>,
            ) = py
                .eval(
                    "(v.epoch, v.release, v.pre, v.post, v.dev, v.local)",
                    None,
                    Some([("v", self.inner.as_ref(py))].into_py_dict(py)),
                )
                .unwrap()
                .extract()
                .unwrap();

            *release.last_mut().unwrap() += 1;
            let release = release
                .into_iter()
                .map(|n| n.to_string())
                .collect::<Vec<String>>()
                .join(".");

            Version::new(&format!(
                "{}!{}{}{}{}{}",
                epoch,
                release,
                pre_release
                    .map(|(s, n)| format!("{}{}", s, n))
                    .unwrap_or_default(),
                post_release
                    .map(|(s, n)| format!(".{}{}", s, n))
                    .unwrap_or_default(),
                dev_release
                    .map(|(s, n)| format!(".{}{}", s, n))
                    .unwrap_or_default(),
                local_version.unwrap_or_default()
            ))
            .unwrap()
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn display_test() {
        let version = Version::new("2!1.2.3pre3.rev4.dev1").unwrap();
        assert_eq!(
            format!("{}", version),
            String::from("2!1.2.3rc3.post4.dev1")
        );
        assert_eq!(format!("{}", Version::zero()), String::from("0"));
    }

    #[test]
    fn equality_test() {
        let left = Version::new("0.1").unwrap();
        let right = Version::new("0.1.0.0.0.0").unwrap();
        assert_eq!(left, right);

        let random = Version::new("2!1.2.3pre3.post4.dev1").unwrap();
        assert_eq!(random, random);
    }

    #[test]
    fn version_impl_test() {
        let version = Version::new("0.1.3").unwrap();
        assert_eq!(
            pubgrub::version::Version::bump(&version),
            Version::new("0.1.4").unwrap()
        );
    }

    #[test]
    fn compare_test() {
        let v0_1 = Version::new("0.1").unwrap();
        let v0_1_pre1 = Version::new("0.1a1").unwrap();
        let v0_1_post2 = Version::new("0.1.post2").unwrap();
        let v0_1_dev3 = Version::new("0.1.dev3").unwrap();
        let v0_2_0_0 = Version::new("0.2.0.0").unwrap();

        assert!(v0_1 < v0_2_0_0);
        assert!(v0_2_0_0 >= v0_1);
        assert!(v0_1_pre1 < v0_1);
        assert!(v0_1 <= v0_1_post2);
        assert!(v0_1_dev3 < v0_1);
    }
}
