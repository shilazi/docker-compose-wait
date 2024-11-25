use clap::Parser;

fn main() {
    env_logger::init_from_env(env_logger::Env::default().filter_or("WAIT_LOGGER_LEVEL", "debug"));

    let parameter = wait::Parameter::parse();
    let mut sleep = wait::sleeper::new();
    wait::wait(&mut sleep, &wait::config_from_env(parameter), &mut on_timeout);
}

fn on_timeout() {
    std::process::exit(1);
}
