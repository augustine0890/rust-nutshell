mod lesson;
use lesson::function;
use lesson::variable;
// main has some special rules: cannot take any prarameters, can only have some special result types.
fn main() {
    println!("Hello, world!");
    function::run();
    variable::run();
}
