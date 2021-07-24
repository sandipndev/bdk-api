use bdk_api;

fn setup_log() {
    let def_env = env_logger::Env::default().filter_or(env_logger::DEFAULT_FILTER_ENV, "debug");
    env_logger::init_from_env(def_env);
    log::debug!("log setup successfully");
}

#[tokio::main]
pub async fn main() {
    setup_log();
    bdk_api::server().launch().await.unwrap();
}
