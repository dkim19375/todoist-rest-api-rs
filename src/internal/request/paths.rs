pub const COMMENTS: &str = "/comments";

pub fn create_path(paths: &[impl ToString]) -> String {
    paths.iter().map(|x| {
        let str = x.to_string();
        if str.starts_with('/') {
            str
        } else {
            format!("/{}", str)
        }
    }).collect::<Vec<String>>().concat()
}