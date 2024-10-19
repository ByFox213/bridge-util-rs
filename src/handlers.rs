use regex::Captures;

pub async fn rcon_handler(caps: Captures<'_>, commands: &[String]) -> String {
    let rcon = caps.get(1).unwrap().as_str();
    if commands.iter().any(|text| rcon.starts_with(text.as_str())) {
        return rcon.to_string();
    }
    String::default()
}
