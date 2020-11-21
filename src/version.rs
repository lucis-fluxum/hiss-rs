use std::{cmp::Ordering, fmt};

#[derive(Debug, Clone, Eq, Ord)]
pub struct Version {
    epoch: usize,
    release: Vec<usize>,
    pre_release: Option<usize>,
    post_release: Option<usize>,
    dev_release: Option<usize>,
    // TODO: Local version segment
}

impl Version {
    pub fn new(
        epoch: usize,
        release: Vec<usize>,
        pre_release: Option<usize>,
        post_release: Option<usize>,
        dev_release: Option<usize>,
    ) -> Self {
        Self {
            epoch,
            release,
            pre_release,
            post_release,
            dev_release,
        }
    }

    pub fn zero() -> Self {
        Version::new(0, vec![0], None, None, None)
    }

    // See https://github.com/pypa/packaging/blob/master/packaging/version.py#L495-L556
    // for how we generate this key.
    fn compare_key(&self) -> (usize, Vec<usize>, i64, i64, i64) {
        // Remove trailing zeros from release
        let mut release: Vec<usize> = self
            .release
            .iter()
            .copied()
            .rev()
            .skip_while(|&n| n == 0)
            .collect();
        release.reverse();

        let pre_release = match (self.pre_release, self.post_release, self.dev_release) {
            (None, None, Some(_)) => i64::MIN,
            (None, _, _) => i64::MAX,
            // TODO: Technically this could overflow
            (Some(n), _, _) => n as i64,
        };

        let post_release = match self.post_release {
            // TODO: Technically this could overflow
            Some(n) => n as i64,
            None => i64::MIN,
        };

        let dev_release = match self.dev_release {
            // TODO: Technically this could overflow
            Some(n) => n as i64,
            None => i64::MAX,
        };

        // TODO: Local version compare key

        (self.epoch, release, pre_release, post_release, dev_release)
    }
}

impl fmt::Display for Version {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let epoch = if self.epoch == 0 {
            String::new()
        } else {
            format!("{}!", self.epoch)
        };
        let release: Vec<String> = self.release.iter().map(|n| n.to_string()).collect();
        let pre = self
            .pre_release
            .map(|n| format!(".pre{}", n))
            .unwrap_or_default();
        let post = self
            .post_release
            .map(|n| format!(".post{}", n))
            .unwrap_or_default();
        let dev = self
            .dev_release
            .map(|n| format!(".dev{}", n))
            .unwrap_or_default();
        f.write_fmt(format_args!(
            "{}{}{}{}{}",
            epoch,
            release.join("."),
            pre,
            post,
            dev
        ))
    }
}

impl PartialEq for Version {
    fn eq(&self, other: &Self) -> bool {
        self.compare_key() == other.compare_key()
    }
}

impl PartialOrd for Version {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.compare_key().partial_cmp(&other.compare_key())
    }
}

impl pubgrub::version::Version for Version {
    fn lowest() -> Self {
        Version::zero()
    }

    fn bump(&self) -> Self {
        let mut release = self.release.clone();
        *release.last_mut().unwrap() += 1;
        Version::new(
            self.epoch,
            release,
            self.pre_release,
            self.post_release,
            self.dev_release,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn display_test() {
        let version = Version::new(2, vec![1, 2, 3], Some(3), Some(4), Some(1));
        assert_eq!(
            format!("{}", version),
            String::from("2!1.2.3.pre3.post4.dev1")
        );
        assert_eq!(format!("{}", Version::zero()), String::from("0"));
    }

    #[test]
    fn equality_test() {
        let left = Version::new(0, vec![0, 1], None, None, None);
        let right = Version::new(0, vec![0, 1, 0, 0, 0, 0], None, None, None);
        assert_eq!(left, right);

        let random = Version::new(2, vec![1, 2, 3], Some(3), Some(4), Some(1));
        assert_eq!(random, random);
    }

    #[test]
    fn version_impl_test() {
        let version = Version::new(0, vec![0, 1], None, None, None);
        assert_eq!(
            pubgrub::version::Version::bump(&version),
            Version::new(0, vec![0, 2], None, None, None)
        );
    }

    #[test]
    fn compare_test() {
        let v0_1 = Version::new(0, vec![0, 1], None, None, None);
        let v0_1_pre1 = Version::new(0, vec![0, 1], Some(1), None, None);
        let v0_1_post2 = Version::new(0, vec![0, 1], None, Some(2), None);
        let v0_1_dev3 = Version::new(0, vec![0, 1], None, None, Some(3));
        let v0_2_0_0 = Version::new(0, vec![0, 2, 0, 0], None, None, None);

        assert!(v0_1 < v0_2_0_0);
        assert!(v0_2_0_0 >= v0_1);
        assert!(v0_1_pre1 < v0_1);
        assert!(v0_1 <= v0_1_post2);
        assert!(v0_1_dev3 < v0_1);
    }
}
