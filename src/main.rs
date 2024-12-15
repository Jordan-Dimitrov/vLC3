mod vm;

use std::env;
use vm::{*};
fn main() {
    let mut vm = Vm::new();

    let program_path = format!("programs/{}", env::args().skip(1).next().unwrap());

    println!("{}", program_path);

    vm.start(program_path);
}
