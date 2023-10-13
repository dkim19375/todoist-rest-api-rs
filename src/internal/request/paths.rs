pub const COMMENTS: &str = "/comments";

pub const LABELS: &str = "/labels";
pub const LABELS_SHARED: &str = "/labels/shared";
pub const LABELS_SHARED_RENAME: &str = "/labels/shared/rename";
pub const LABELS_SHARED_REMOVE: &str = "/labels/shared/remove";

pub fn create_path(paths: &[impl ToString]) -> String {
    paths
        .iter()
        .map(|x| {
            let str = x.to_string();
            if str.starts_with('/') {
                str
            } else {
                format!("/{}", str)
            }
        })
        .collect::<Vec<String>>()
        .concat()
}
