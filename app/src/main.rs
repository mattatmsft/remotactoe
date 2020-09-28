mod runtime;

fn main() {
    println!("Welcome to MWSoft's RemoTacToe, tic tac toe with a network twist.");
    let mut runtime = runtime::Runtime::new();
    runtime.start().unwrap();
}
