use configparser::ini::Ini;

pub fn read_config() -> Vec<String>{

    let mut config = Ini::new();
    config.load("content/global.ini").ok();

    let token = config.get("discord", "token").unwrap();
    let bot_id = config.get("discord", "bot_id").unwrap();

    return vec!(token, bot_id);
}