pub fn path_exists(path: String) -> bool {
    std::fs::metadata(path).is_ok()
}
