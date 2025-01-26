use which_cli::which;
use std::env;

fn main() {
	if let Some(targets) = {
		let args: Vec<String> = env::args().collect();
		if args.len() >= 2 {
			Some(args[1..].to_owned().clone())
		} else {
			None
		}
	} {
		for target in targets {
			match which(&target) {
				Ok(result) => match result.into_os_string().into_string() {
					Ok(string) => println!("{}", string),
					Err(osstring) => println!("{:?}", osstring),
				},
				Err(_) => println!("(command '{target}' not found)"),
			}
		}
	} else {
		println!();
	};
}
