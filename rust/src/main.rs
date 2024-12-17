pub mod rust_by_examples;
use rust_by_examples::formatted_print::hello;

mod example_main{
    pub fn main(){
        println!("exmaple main")
    }
}
fn main() {
    println!("Hello, world!");
    example_main::main();
    hello()
}
