use copypasta::{ClipboardContext, ClipboardProvider};
use std::io;

fn main() {
    let mut result = String::new();

    loop {
        let mut input = String::new();
        let stdin = io::stdin().read_line(&mut input);

        match stdin {
            Ok(x) => {
                input = input.trim().to_string();
                if x == 0 {
                    break;
                }
                result.push_str("\n");
                result.push_str(&input);
            }
            Err(_) => {
                panic!("Could not read line")
            }
        }
    }
    if &result[..1] == "\n" {
        result.remove(0);
    }
    set_clipboard(&result);
    println!("Clipboard updated");
}

fn set_clipboard(content: &str) {
    let mut ctx = ClipboardContext::new().unwrap();
    ctx.set_contents(content.to_string().to_owned()).unwrap();
    ctx.get_contents().unwrap();
}
