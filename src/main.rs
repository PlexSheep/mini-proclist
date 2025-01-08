use std::process::exit;

use sysinfo::System;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        usage(&args[0]);
    }
    proclist(&args[1]);
}

fn proclist(name: &str) {
    let s = System::new_all();
    let mut processes: Vec<&sysinfo::Process> = s.processes_by_name(name.as_ref()).collect();
    processes.sort_by_key(|a| a.pid());

    println!("{:<16}{:<10}{:<30}{:<40}", "UID", "PID", "NAME", "CMD");

    for process in processes {
        println!(
            "{:<16}{:<10}{:<30}{:<40}",
            process
                .user_id()
                .map(|a| a.to_string())
                .unwrap_or("(bad id)".to_string()),
            process.pid().to_string(),
            process.name().to_string_lossy(),
            process
                .cmd()
                .iter()
                .map(|a| a.to_string_lossy())
                .collect::<Vec<_>>()
                .join(" ")
        );
    }
}

fn usage(program: &str) {
    eprintln!("Usage: {} [PROCESS_NAME]", program);
    exit(1);
}
