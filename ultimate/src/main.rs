mod functions;
mod variables;

// main has some special rules: cannot take any prarameters, can only have some special result types.
fn main() {
    println!("Hello, world!");
    variables::var_run();
    functions::func_run();
}
