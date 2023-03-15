use std::process::Command;

use std::env;

fn main() {

	let args: Vec<_> = env::args().collect();
	let mut search  = "https://google.com/search?q=".to_string();
	let len :i32 = (args.len()-1) as i32;
	if len >= 1{
	let  mut iter = 0;
	for text in args {
		if iter >= 1{
			search = format!("{}{}{}", &search, "+".to_string(), &text);
		}
		iter += 1;
	}
	println!("{search}");


    let _browse = Command::new("firefox")
		.arg(search)
        .output()
			.expect("failed to start browser");
	}
	else {
		println!("Please insert a search query!");
	}

}
