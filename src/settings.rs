
pub fn get_settings() -> Vec<(u64, String, String)> {
    let itm: (u64, String, String) = (
        15000,
        String::from("http://elasticsearch.logging.local/lambda-production-*/_search?filter_path=hits.hits"),
        String::from(r#"{"query": {"match_phrase": {"message": "error"}}, "_source": ["message", "_index", "_id", "log_group"]}"#),
    );
    let settings: Vec<(u64, String, String)> = vec![itm];
    return settings;
}
