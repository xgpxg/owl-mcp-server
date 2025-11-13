mod log_config;

pub use log;

pub fn init_log(){
    log_config::init_log();
}