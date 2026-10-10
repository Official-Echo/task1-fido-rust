use clap::{Arg, Command, value_parser};
use node_task::TwoThirdsOfLinkedList;

const USAGE: &str = r#"
Usage: node-task <n | "a -> b -> c">
  n            list of n nodes with data 0..n-1
  a -> b -> c  list of the given values. Use quotes to write since > redirects in shell
  (no args)    input is read from the console
"#;

fn main() {
    let matches = Command::new("node-task")
        .version(env!("CARGO_PKG_VERSION"))
        .after_help(USAGE)
        .arg_required_else_help(true)
        .arg(
            Arg::new("input")
                .required(true)
                .value_parser(value_parser!(String)),
        )
        .get_matches();

    let input = matches.get_one::<String>("input").unwrap();

    let values: Vec<String> = if let Ok(n) = input.parse::<u32>() {
        (0..n).map(|i| i.to_string()).collect()
    } else {
        let parsed = input.split("->").map(|s| s.trim().to_string()).collect();
        println!("Parsed values: {:?}", parsed);
        parsed
    };

    let mut list = TwoThirdsOfLinkedList::new();
    list.prepend_iter(values.into_iter().rev());

    match list.get_two_thirds_node() {
        Some(node) => println!("{}", node.data),
        None => println!("null"),
    }

    // Due to the list sometimes being too large,
    // the default drop crashes with a stack overflow
    // Instead, it is better for the system to just take the space that was taken by the list
    // without taking time to destroy all its values
    std::mem::forget(list);
}
