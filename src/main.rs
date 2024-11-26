fn main() {
    env_logger::init_from_env(env_logger::Env::default().filter_or("WAIT_LOGGER_LEVEL", "debug"));

    let args: Vec<String> = std::env::args().collect();
    let mut sleep = wait::sleeper::new();
    wait::wait(&mut sleep, &wait::config_from_env(args), &mut on_timeout);
}

fn on_timeout() {
    std::process::exit(1);
}
