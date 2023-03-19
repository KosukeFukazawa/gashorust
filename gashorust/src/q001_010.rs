mod q001;
mod q002;
mod q003;
mod q004;
mod q005;

pub fn run(code_num: u8) {
    match code_num {
        1 => q001::run(),
        2 => q002::run(),
        3 => q003::run(),
        4 => q004::run(),
        5 => q005::run(),
        _ => {},
    }
}