use std::path::Path;

use ignore::gitignore::{Gitignore, GitignoreBuilder};

use crate::config::{FileName, IgnoreList};

pub(crate) struct IgnorePathSet {
    ignore_set: Gitignore,
}

impl IgnorePathSet {
    pub(crate) fn from_ignore_list(ignore_list: &IgnoreList) -> Result<Self, ignore::Error> {
        let root = ignore_list
            .rustfmt_toml_path()
            .parent()
            .unwrap_or(&Path::new(""));
        let mut ignore_builder = GitignoreBuilder::new(root);

        for ignore_path in ignore_list {
            ignore_builder.add_line(None, &ignore_path.to_string_lossy())?;
        }

        Ok(Self {
            ignore_set: ignore_builder.build()?,
        })
    }

    pub(crate) fn is_match(&self, file_name: &FileName) -> bool {
        match file_name {
            FileName::Stdin => false,
            FileName::Real(p) => self
                .ignore_set
                .matched_path_or_any_parents(p, false)
                .is_ignore(),
        }
    }
}

#[cfg(test)]
mod test {
    use std::path::{Path, PathBuf};

    use super::IgnorePathSet;
    use crate::config::{Config, FileName};
    use crate::is_nightly_channel;

    #[test]
    fn test_ignore_path_set() {
        if !is_nightly_channel!() {
            // This test requires nightly
            return;
        }
        let config =
            Config::from_toml(r#"ignore = ["foo.rs", "bar_dir/*"]"#, Path::new("")).unwrap();
        let ignore_path_set = IgnorePathSet::from_ignore_list(&config.ignore()).unwrap();

        assert!(ignore_path_set.is_match(&FileName::Real(PathBuf::from("src/foo.rs"))));
        assert!(ignore_path_set.is_match(&FileName::Real(PathBuf::from("bar_dir/baz.rs"))));
        assert!(!ignore_path_set.is_match(&FileName::Real(PathBuf::from("src/bar.rs"))));
    }
    #[test]
    fn test_ignore_path_set_with_dir() {
        if !is_nightly_channel!() {
            // This test requires nightly
            return;
        }
        let config = Config::from_toml(
            r#"ignore = ["tests/**/foo/bar.rs"]"#,
            Path::new("tests/config/"),
        )
        .unwrap();
        info!(
            "rustfmt_toml_path: {:?}",
            &config.ignore().rustfmt_toml_path()
        );
        let ignore_path_set = IgnorePathSet::from_ignore_list(&config.ignore()).unwrap();

        assert_eq!(
            ignore_path_set.is_match(&FileName::Real(PathBuf::from("tests/source/foo/bar.rs"))),
            false
        );
        assert_eq!(
            ignore_path_set.is_match(&FileName::Real(PathBuf::from(
                "tests/tests/source/foo/bar.rs"
            ))),
            true
        );
    }
}
