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
        "add" => create(&args.arg.unwrap()),
        "list" => list(),
        "delete" => println!("delete"),
        "done" => println!("done"),
        "help" => help(),
        _ => help()
    }
}

fn create(taskName: &String){
    let mut tasks: Vec<Task> = match get_task_content() {
        Ok(content) => serde_json::from_str(&content).unwrap(),
        _ => { 
            println!("Error retrieving tasks. Aborting...");
            return;
        }
    };

    let newTask: Task = Task {
        id: tasks.last().map_or(0, |t| t.id + 1),
        name: taskName.to_string(),
        done: false
    };

    tasks.push(newTask);
    
    match create_task(&tasks) {
        Ok(()) => 
        {
            println!("Successfully added task to list\n");
            list();
        },
        _ => println!("Error occurred adding task to list.")
    };
}

fn list() {
    let tasks: Vec<Task> = match get_task_content() {
        Ok(content) => serde_json::from_str(&content).unwrap(),
        _ => { 
            println!("Error retrieving tasks. Aborting...");
            return;
        }
    };

    println!(
        r#"
Tasks
Id      Name        Done
-------------------------
        "#
    );

    for task in &tasks {
        println!("{}       {}        {}\n", &task.id, &task.name, &task.done);
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