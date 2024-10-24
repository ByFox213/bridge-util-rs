use regex::Captures;

pub async fn rcon_handler(caps: Captures<'_>, commands: &[String]) -> String {
    let rcon = caps.get(1).unwrap().as_str();
    if rcon.is_empty() {
        return String::default();
    }
    if let Some(first_word) = rcon.split_whitespace().next() {
        if commands.contains(&first_word.to_string()) {
            return rcon.to_string();
        }
    }
    String::default()
}
