#[allow(dead_code)]
pub fn decimal_to_hex(num: u32) -> String {
    format!("{:x}", num)
}

#[allow(dead_code)]
pub fn decimal_to_binary(num: u32) -> String {
    format!("{:b}", num)
}
