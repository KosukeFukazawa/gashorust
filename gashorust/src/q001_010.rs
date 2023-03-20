mod q001;
mod q002;
mod q003;
mod q004;
mod q005;
mod q006;
mod q007;
mod q008;
mod q009;
mod q010;

pub fn run(code_num: u8) {
    match code_num {
        1  => q001::run(),
        2  => q002::run(),
        3  => q003::run(),
        4  => q004::run(),
        5  => q005::run(),
        6  => q006::run(),
        7  => q007::run(),
        8  => q008::run(),
        9  => q009::run(),
        10 => q010::run(),
        _ => {},
    }
}