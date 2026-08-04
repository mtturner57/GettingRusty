fn main() {
    let args: Vec<String> = std::env::args().collect();

    println!("Total arguments: {}", args.len());

    for (index, arg) in args.iter().enumerate() {
        println!("  args[{}] = {:?}", index, arg);
    }
    // if args.len() < 1
    
    // match mode {
    //     '--help' => returnHelp(),
    //     _ => returnHelp()
    // }
}

fn listOfCommands() {

}

fn help() {
    
}