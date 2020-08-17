
pub fn get_settings() -> Vec<(u64, String, String)> {
    let itm: (u64, String, String) = (
        1 * 60 * 1000,
        String::from("http://elasticsearch.logging.local/slack-notifications-development*/_search?filter_path=hits.hits"),
        String::from(r#"{"query": {"range": {"time": {"gte": "now-1m","lt": "now"}}},"_source": ["message","_index","_id"]}"#),
    );
    let settings: Vec<(u64, String, String)> = vec![itm];
    return settings;
}
