mod q001_010;

fn main() {
    let mut code_num: u8 = 0;
    let args: Vec<String> = std::env::args().collect();
    if args.len() > 1 {
        code_num = args[1].parse::<u8>().unwrap();
    }

    run(code_num)
}

fn run(code_num: u8) {
    if code_num <= 10 {
        q001_010::run(code_num);
    }
}