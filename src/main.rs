mod process_operator;

fn main() {
    unsafe {
        println!("Hello, world!");
        let _process_operator = process_operator::default_process_operator(0);
    }
}
