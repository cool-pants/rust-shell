use std::io::{self,Write};
use std::str::FromStr;
use std::process::Command;

enum Builtin {
  Echo,
  Clear,
  Cd,
  Pwd,
  Debug,
  Exit
}
  
impl FromStr for Builtin {
  type Err = ();
  fn from_str(s : &str) -> Result<Self, Self::Err> {
      match s {
      "echo" => Ok(Builtin::Echo),
      "cls" => Ok(Builtin::Clear),
      "cd" => Ok(Builtin::Cd),
      "pwd" => Ok(Builtin::Pwd),
      "debug" => Ok(Builtin::Debug),
      "exit" => Ok(Builtin::Exit),
      _ => Err(()),
    }
  }
}

fn builtin_echo(args : &Vec<String>) -> i32 {
    let mut vec:Vec<String> = Vec::new();
    vec.push("/C".to_string());
    vec.extend(args.to_vec());
    let output = if cfg!(target_os = "windows") {
        Command::new("cmd")
                .args(&vec)
                .output()
                .expect("failed to execute process")
    } else {
        Command::new("sh")
                .arg("-c")
                .arg(vec!["echo".to_string(),args.join(" ").to_string()].join(" "))
                .output()
                .expect("failed to execute process")
    };
    
    let hello = output.stdout;
    println!("{}", String::from_utf8(hello).expect("got output"));
    0
}

fn builtin_pwd() -> i32 {
    let mut vec:Vec<String> = Vec::new();
    vec.push("dir".to_string());
    let output = if cfg!(target_os = "windows") {
        Command::new("cmd")
                .args(&vec)
                .output()
                .expect("failed to execute process")
    } else {
        Command::new("sh")
                .arg("-c")
                .arg(vec!["ls"].join(" "))
                .output()
                .expect("failed to execute process")
    };
    
    let hello = output.stdout;
    println!("{}", String::from_utf8(hello).expect("got output"));
    0
}

fn builtin_cd(args : &Vec<String>) -> i32 {
    let mut vec:Vec<String> = Vec::new();
    vec.push("cd".to_string());
    vec.extend(args.to_vec());
    if cfg!(target_os = "windows") {
        Command::new("cmd")
                .args(&vec)
                .output()
                .expect("failed to execute process")
    } else {
        Command::new("sh")
                .current_dir(vec![args.join(" ").to_string()].join(" "))
                .output()
                .expect("failed to execute process")
    };
    
    println!("Moved to directory : {0}",vec![args.join(" ").to_string()].join(" "));
    0
}

fn builtin_clear() -> i32 {
  let mut vec:Vec<String> = Vec::new();
    vec.push("cls".to_string());
    let output = if cfg!(target_os = "windows") {
        Command::new("cmd")
                .args(&vec)
                .output()
                .expect("failed to execute process")
    } else {
        Command::new("sh")
                .arg("-c")
                .arg(vec!["clear"].join(" "))
                .output()
                .expect("failed to execute process")
    };
    
    let hello = output.stdout;
    println!("{}", String::from_utf8(hello).expect("got output"));
    0
}

fn builtin_debug(args : &Vec<String>) -> i32 {
  
  if args[0] == "0\n" {
    println!("Debug turned OFF...\n");
    return 1;
  } else {
    println!("Debug turned ON...\n");
    return 0;
  }
}

fn builtin_exit() -> i32 {
  let mut vec:Vec<String> = Vec::new();
    vec.push("exit".to_string());
    let output = if cfg!(target_os = "windows") {
        Command::new("cmd")
                .args(&vec)
                .output()
                .expect("failed to execute process")
    } else {
        Command::new("sh")
                .arg("-c")
                .arg(vec!["exit 0"].join(" "))
                .output()
                .expect("failed to execute process")
    };
    
    let hello = output.stdout;
    println!("{}", String::from_utf8(hello).expect("got output"));
    0
}

struct Comms {
    keyword: String,
    args: Vec<String>,
}

fn process_command(c : Comms) -> i32 {
  match Builtin::from_str(&c.keyword) {
    Ok(Builtin::Echo) => builtin_echo(&c.args),
    Ok(Builtin::Clear) => builtin_clear(),
    Ok(Builtin::Cd) => builtin_cd(&c.args),
    Ok(Builtin::Pwd) => builtin_pwd(),
    Ok(Builtin::Debug) => builtin_debug(&c.args),
    Ok(Builtin::Exit) => builtin_exit(),
    _ => {
        println!("{}: command not found", &c.keyword);
        1
    },
  }
}

fn main() {
  let prompt_char = "%";
  let mut debug_flag = "0".to_string();
  loop {
    print!("{0} ", prompt_char);
    io::stdout().flush().unwrap();

    let mut command = String::new();
    io::stdin().read_line(&mut command)
      .expect("Failed to read in command");
    let command_split : Vec<&str> = command.split(' ').collect();
    let mut keyword = command_split[0];
    let arguments = &command_split[1..];
    let mut args0: Vec<String> = Vec::new();
    if arguments.len() != 0 {
        let argsvec: Vec<&str> = command_split[1..].to_vec();

        for a in &argsvec{
            args0.push(a.to_string());
        }
    }
    if arguments.len() == 0{
        keyword = keyword.trim_end();
    }

    else if keyword == "cd" {
        args0[0] = args0[0].trim_end().to_string();
    }

    if debug_flag == "0"{
      println!("DEBUG: Raw input: {:?}", command);

      
      println!("DEBUG: Split input: {:?}", command_split);

      println!("DEBUG: Keyword: {0}", keyword);
      println!("DEBUG: Number of arguments: {0:?}\nDEBUG: Arguments: {1:?}", args0.len(), args0);
    }

    if keyword == "debug" {
      args0.push(debug_flag.to_string());
      debug_flag = process_command(Comms {
                                    keyword: String::from(keyword),
                                    args: args0,
                                  }).to_string();
    } else {
      process_command(Comms {
                                  keyword: String::from(keyword),
                                  args: args0,
                              });
    }

    if keyword == "exit"{
      println!("\n\nExiting Shell....\n");
      break;
    }
  }
}