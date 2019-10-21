use std::io::Write;

fn main() {
    loop {
        print!("> ");
        std::io::stdout().flush().unwrap();

        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();

        let mut parts = input.trim().split_whitespace();
        let command = parts.next().unwrap();
        let args = parts;

        let mut child = std::process::Command::new(command)
            .args(args)
            .spawn()
            .unwrap();
        child.wait().unwrap();
    }
}
