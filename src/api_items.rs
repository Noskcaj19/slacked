use rustyline;
use std::convert::Into;
use std::default::Default;

#[derive(Deserialize, Clone, Debug)]
#[serde(rename_all = "lowercase")]
pub enum EditMode {
    Vi,
    Emacs,
}

impl Into<rustyline::EditMode> for EditMode {
    fn into(self) -> rustyline::EditMode {
        use self::EditMode::*;
        match self {
            Emacs => rustyline::EditMode::Emacs,
            Vi => rustyline::EditMode::Vi,
        }
    }
}

impl Default for EditMode {
    fn default() -> Self {
        EditMode::Emacs
    }
}

#[derive(Deserialize, Debug)]
pub struct Config {
    pub token: String,
    #[serde(default)] pub edit_mode: EditMode,
}

#[derive(Deserialize, Debug)]
pub struct Connect {
    pub ok: bool,
    pub url: String,
}
