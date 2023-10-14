pub const COLLABORATORS: &str = "/collaborators";

pub const COMMENTS: &str = "/comments";

pub const LABELS: &str = "/labels";
pub const LABELS_SHARED: &str = "/labels/shared";
pub const LABELS_SHARED_RENAME: &str = "/labels/shared/rename";
pub const LABELS_SHARED_REMOVE: &str = "/labels/shared/remove";

pub const PROJECTS: &str = "/projects";

pub const SECTIONS: &str = "/sections";

pub const PARAM_PROJECT_ID: &str = "project_id=";

pub fn create_path(paths: &[impl ToString]) -> String {
    let mut final_path = String::new();
    let mut last_was_param = false;
    let mut has_param_yet = false;
    for path in paths {
        let str = path.to_string();
        if str.starts_with('/') {
            if has_param_yet {
                panic!("Cannot have a path with a slash after a parameter");
            }
            final_path.push_str(&str);
            continue;
        }
        if str.ends_with('=') {
            if last_was_param {
                panic!("Cannot have a path with a parameter key right after another parameter key (parameter value required)");
            }
            final_path.push(if has_param_yet { '&' } else { '?' });
            final_path.push_str(&str);
            last_was_param = true;
            has_param_yet = true;
            continue;
        }
        if last_was_param {
            final_path.push_str(&str);
            last_was_param = false;
            continue;
        }
        if has_param_yet {
            if has_param_yet {
                panic!("Cannot have a path with a slash after a parameter");
            }
        }
        final_path.push('/');
        final_path.push_str(&str);
    }
    if last_was_param {
        panic!("Cannot have a path with a parameter key at the end of the path (parameter value required)");
    }
    final_path
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn path_no_params() {
        assert_eq!(create_path(&["/test", "/test2"]), "/test/test2");
    }

    #[test]
    fn path_no_slash_no_params() {
        assert_eq!(create_path(&["test", "test2"]), "/test/test2");
    }

    #[test]
    fn path_with_params() {
        assert_eq!(
            create_path(&["/test", "test2=", "test3"]),
            "/test?test2=test3"
        );
    }

    #[test]
    fn path_with_multiple_params() {
        assert_eq!(
            create_path(&["/test", "test2=", "test3", "test4=", "test5"]),
            "/test?test2=test3&test4=test5"
        );
    }

    #[test]
    fn complex_path() {
        assert_eq!(
            create_path(&["/test", "test2", "/test3", "test4=", "test5", "test6=", "test7"]),
            "/test/test2/test3?test4=test5&test6=test7"
        )
    }

    #[test]
    #[should_panic]
    fn invalid_path_with_slash_after_param_key() {
        create_path(&["/test", "test2=", "/test3"]);
    }

    #[test]
    #[should_panic]
    fn invalid_path_with_slash_after_param_value() {
        create_path(&["/test", "test2=", "test4", "/test5"]);
    }

    #[test]
    #[should_panic]
    fn invalid_path_with_skipped_param_value() {
        create_path(&["/test", "test2=", "test3="]);
    }

    #[test]
    #[should_panic]
    fn invalid_path_with_no_param_value() {
        create_path(&["/test", "test2="]);
    }
}
