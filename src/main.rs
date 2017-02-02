extern crate duct;
use duct::{Expression,cmd};
use std::io::{stdin,stdout,Write};

/* Tried using the duct crate to handle piping but get various errors when 
 * compiling the needed crates. Might be because the rustc version is 1.11.0
 * and Cargo is at 0.12.0-nightly*/
fn main() {
    loop {
        let mut s = String::new();
        print!("mish: ");
        let _=stdout().flush();
        stdin().read_line(&mut s).expect("Did not enter a correct string");


        let commands = s.split('|').map(|x| x.split_whitespace());
        //if commands.is_empty() { println!("no input"); break; }
        let mut executable : Option<Expression> = None;

        for mut command in commands {
            if let Some(com) = command.next() {
                let mut args = vec![];
                //print!("{} ", com);
                for argument in command {
                    //print!("{} ", argument);
                    args.push(argument);
                }

                if executable.is_some() {
                    //print!("piping");
                    executable = Some(executable.unwrap().pipe(cmd(com, args)));
                }
                else {
                    //print!("first command");
                    executable = Some(cmd(com, args));
                }
                //print!("\n");
            }
        }

        match executable.unwrap().read() {
            Ok(a) => println!("{}",a),
            Err(e) => println!("{}", e),
        };
    }
}

/*Cannot create pipes between child processes :( */
/*use std::process::{Command,Stdio,ChildStdout};

fn main() {
    for _ in 1..5 {
        use std::io::{stdin,stdout,Write};
        let mut s=String::new();

        /*Get the users command*/
        print!("Please enter some text: ");
        let _=stdout().flush();
        stdin().read_line(&mut s).expect("Did not enter a correct string");

        /*Create the commands and add their arguments*/
        let commands = s.split('|').map(|x| x.split_whitespace());
        let mut cmds = Vec::new();

        for mut command in commands {
            let mut cmd = Command::new(command.next().expect(":("));
            for argument in command {
                cmd.arg(argument);
            }
            cmds.push(cmd);
        }

        let mut pipehandle : Option<ChildStdout>;
        for mut cmd in cmds {

            if let Some(handle) = pipehandle {
                cmd.stdin(handle);
            }
            let process = cmd.stdout(Stdio::piped()).spawn().expect("Failed to spawn child");
            pipehandle = process.stdout;
            //let output = cmd.output().unwrap_or_else( |e| {panic!("failed to execute process: {}", e)});
            //let s = String::from_utf8_lossy(&output.stdout);

        }
        //println!("You typed: {}",s);
    }
}*/

/*let cmd = Command::new("ls").arg("/").
    .stdout(Stdio::piped()).spawn();

let cmd2 = Command::new("grep").arg("etc")
    .stdin(cmd.stdout)
    .output()
    .unwrap_or_else(|e| { panic!("failed to execute process: {}", e) });
*/
