pub struct Logger {}

impl Logger {
    pub fn log(msg: String) {
        println!("[LOG] // {}", msg);
    }

    pub fn warn(msg: String) {
        println!("[WRN] // {}", msg);
    }

    pub fn error(msg: String) {
        println!("[ERR] // {}", msg);
    }
}
