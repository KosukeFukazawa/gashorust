mod q000;
mod q001_010;

fn main() {
    let mut code_num = 0;
    let args: Vec<String> = std::env::args().collect();
    if args.len() > 1 {
        code_num = args[1].parse::<i32>().unwrap();
    }

    run(code_num)
}

fn run(code_num: i32) {
    match code_num {
        1 => q001_010::q001::run(),
        _ => q000::run(),
    }
}