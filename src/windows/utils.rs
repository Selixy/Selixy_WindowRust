pub fn loword(lparam: u32) -> u16 {
    (lparam & 0xFFFF) as u16
}

pub fn hiword(lparam: u32) -> u16 {
    ((lparam >> 16) & 0xFFFF) as u16
}
