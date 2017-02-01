use std::process::Command;

fn main() {
    for _ in 1..5 {
        use std::io::{stdin,stdout,Write};
        let mut s=String::new();

        print!("Please enter some text: ");
        let _=stdout().flush();

        stdin().read_line(&mut s).expect("Did not enter a correct string");

        if let Some('\n')=s.chars().next_back() {
            s.pop();
        }

        if let Some('\r')=s.chars().next_back() {
            s.pop();
        }

         let mut cmd = Command::new(s);

         let output = cmd.output().unwrap_or_else( |e| {panic!("failed to execute process: {}", e)});
         let s = String::from_utf8_lossy(&output.stdout);

         println!("output: {}",s);
         //println!("You typed: {}",s);
    }
}

/*let cmd = Command::new("ls").arg("/").
    .stdout(Stdio::piped()).spawn();

let cmd2 = Command::new("grep").arg("etc")
    .stdin(cmd.stdout)
    .output()
    .unwrap_or_else(|e| { panic!("failed to execute process: {}", e) });
*/
