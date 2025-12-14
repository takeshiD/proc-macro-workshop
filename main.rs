// Write code here.
//
// To see what the code looks like after macro expansion:
//     $ cargo expand
//
// To run the code:
//     $ cargo run

use derive_builder::Builder;

#[derive(Builder)]
pub struct Command {
    executable: String,
    args: Vec<String>,
    env: Vec<String>,
    current_dir: String,
}

fn main() {
    // let cmd = Command {
    //     executable: "hello".to_string(),
    //     args: vec!["hello".to_string(), "world".to_string()],
    //     env: vec!["hello".to_string(), "world".to_string()],
    //     current_dir: "yeah".to_string(),
    // };
    let cmd_builder = Command::builder();
    cmd_builder.print_derive_input();
}
