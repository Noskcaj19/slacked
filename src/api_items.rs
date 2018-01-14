#[derive(Deserialize, Debug)]
pub struct Config {
    pub token: String,
}

#[derive(Deserialize, Debug)]
pub struct Connect {
    pub ok: bool,
    pub url: String,
}
