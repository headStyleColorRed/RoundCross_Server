pub fn current_date() -> String {
    let now = chrono::offset::Utc::now();
    now.to_string()
}