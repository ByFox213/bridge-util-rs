use once_cell::sync::Lazy;
use crate::model::RegexModel;


pub static DD_PATTERNS: Lazy<Vec<RegexModel>> = Lazy::new(|| {
    vec![
        RegexModel::new("rcon", r"^\d{4}-\d{2}-\d{2} \d{2}:\d{2}:\d{2} I server: ClientI[dD]=\d+ rcon='([^']+)'$"),
        RegexModel::new("rcon", r"^\[server]: ClientID=\d+ rcon='([^']+)'$"),
    ]
});