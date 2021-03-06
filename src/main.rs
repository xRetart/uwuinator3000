mod uwu;


fn main() {
    use std::process::exit;

    exit(code_main())
}
fn code_main() -> i32 {
    use std::env::args;


    let english = args().skip(1).collect::<Vec<String>>();
    if english.is_empty() { return 1; }

    println!("{}", uwu::translate(english.join(" ")));
    0
}
