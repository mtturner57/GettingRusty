mod structs;
use structs::cli_arg::CliArg;
use clap::Parser;

fn main() {
    let args: CliArg = CliArg::parse();

    //println!("Command: {:?} \n Arg: {:?}", args.command, args.arg);
    
    match &*args.command {
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