use gritshield::GritComponent;

#[derive(Clone, GritComponent)]
pub struct JqlParser {}

impl JqlParser {
    pub fn new() -> Self {
        Self{}
    }

    /// Converts a JQL string into query condition parameters
    pub fn parse_query(&self, raw_jql: &str) -> String {
        println!("[JQL] Parsing search query: '{}'", raw_jql);
        format!("WHERE {}", raw_jql)
    }
}
