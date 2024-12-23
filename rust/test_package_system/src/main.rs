// mod my_modules;
// use my_modules::my_module::hello;

use test_package_system::my_modules::my_module::hello;

// #[path = "my_modules/my_module.rs"]
// mod my_module;
// use my_module::hello;

mod example_main {
    pub fn main() {
        println!("exmaple main")
    }
}
fn main() {
    println!("Hello, world!");
    example_main::main();
    hello()
}
