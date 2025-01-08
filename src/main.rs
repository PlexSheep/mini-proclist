use std::process::exit;

use sysinfo::{ProcessRefreshKind, RefreshKind, System};

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() > 2 {
        usage(&args[0]);
    }
    proclist(args.get(1).map(|a| a.as_str()));
}

fn proclist(name: Option<&str>) {
    let s = System::new_with_specifics(
        RefreshKind::nothing().with_processes(ProcessRefreshKind::everything()),
    );
    let mut processes: Vec<&sysinfo::Process> = if let Some(name) = name {
        s.processes_by_name(name.as_ref()).collect::<Vec<_>>()
    } else {
        s.processes().values().collect::<Vec<_>>()
    }
    .into_iter()
    .filter(
        |p| p.thread_kind().is_none(), /* I only want real processes, not just threads */
    )
    .collect();
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
