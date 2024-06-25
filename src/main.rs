#[macro_use] extern crate rocket;

use rocket::config::Config;
use dotenv::dotenv;
use std::env;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[launch]
fn rocket() -> _ {
    // 加载环境变量
    dotenv().ok();

    // 从环境变量获取端口，默认为 8000
    let port: u16 = env::var("PORT")
        .unwrap_or_else(|_| "8000".to_string())
        .parse()
        .expect("PORT must be a valid u16");

    // 获取环境模式（默认开发模式）
    let env: String = env::var("APP_ENV").unwrap_or_else(|_| "development".to_string());

    // 配置 Rocket
    let config: rocket::figment::Figment = match env.as_str() {
        "production" => Config::figment()
            .merge(("port", port))
            .merge(("address", "0.0.0.0")),
        _ => Config::figment()
            .merge(("port", port))
            .merge(("address", "127.0.0.1")),
    };

    rocket::custom(config).mount("/", routes![index])
}
