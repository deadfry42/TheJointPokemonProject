pub struct Logger {}

impl Logger {
    pub fn log(msg: String) {
        println!("[LOG] // {}", msg);
    }

    pub fn log_literal(msg: &'static str) {
        println!("[LOG] // {}", msg);
    }

    pub fn warn(msg: String) {
        println!("[WRN] // {}", msg);
    }

    pub fn warn_literal(msg: &'static str) {
        println!("[WRN] // {}", msg);
    }

    pub fn error(msg: String) {
        println!("[ERR] // {}", msg);
    }

    pub fn error_literal(msg: &'static str) {
        println!("[ERR] // {}", msg);
    }
}
