mod q001;
mod q002;
mod q003;

pub fn run(code_num: u8) {
    match code_num {
        1 => q001::run(),
        2 => q002::run(),
        3 => q003::run(),
        _ => {},
    }
}