mod structs;
mod services;

use structs::cli_arg::*;
use structs::task::*;
use services::file_system::*;
use clap::Parser;

fn main() {
    let args: CliArg = CliArg::parse();

    //println!("Command: {:?} \n Arg: {:?}", args.command, args.arg);
    
    match check_exists(){
        Ok(true) => (),
        Ok(false) => match create_file() {
            Ok(()) => (),
            _ => { println!("Error occurred creating file. Aborting..."); return }
        },
        _ => { println!("Error occurred checking if file exists. Aborting..."); return }
    }

    match &*args.command {
        "add" => println!("add"),
        "list" => println!("list"),
        "delete" => println!("delete"),
        "done" => println!("done"),
        "help" => help(),
        _ => help()
    }
}

fn help() {
    println!(r#"
TodoCli creates, updates, and removes tasks from a list to help you stay on top of your work.

Commands:
    add <task>          Add a task to the list
    list                Lists all tasks
    delete <task_id>    Deletes a task from the list
    done <task_id>      Marks a task as done
    help                Displays this help page
    "#);
}