use std::collections::HashSet;
use std::fs;
use std::io;
use std::thread;
use std::time::Duration;
// use std::time::Instant;
use crossbeam_channel::unbounded;
use std::collections::HashMap;

fn get_process_ids() -> HashSet<u32> {
    let mut ids: HashSet<u32> = HashSet::new();
    //create a mutable hashset
    let entries = fs::read_dir("/proc").expect("Failed to read /proc");
    //this retrieves the directory entries from /proc, the expect is the exception

    for entry in entries {
        //iterates over entries
        let entry = match entry {
            Ok(entry) => entry,
            Err(_) => continue,
        };
        //assigns the iterated entry to a var with the exception of not able to be read
        let name = entry.file_name();
        //declares name to be the file_name of the entry
        let name = name.to_string_lossy();
        //makes the name a string

        if let Ok(pid) = name.parse::<u32>() {
            ids.insert(pid);
            //insert pid into hashset
        }
    }
    ids
}

fn get_process_names(pid: u32) -> Option<String> {
    //fn needs pid to run, returns a string
    let path = format!("/proc/{}/comm", pid);
    //path is the pid to the comm directory where the process names are stored
    match fs::read_to_string(path) {
        Ok(name) => Some(name.trim().to_string()),
        Err(_) => None,
    }
    //reads the directory names as strings
}

fn get_process_parent_id(pid: u32) -> Option<u32> {
    let path = format!("/proc/{}/status", pid);

    let yargle = fs::read_to_string(path).ok()?;

    for line in yargle.lines() {
        if line.starts_with("PPid:") {
            if let Some((_, value)) = line.split_once(":") {
                let ppid = value.trim().parse::<u32>();
                return ppid.ok();
            }
        }
    }

    None
}

fn get_thread_id(pid: u32) -> HashSet<u32> {
    let mut ids: HashSet<u32> = HashSet::new();

    let path = format!("/proc/{}/task", pid);

    let entries = match fs::read_dir(path) {
        Ok(entries) => entries,
        Err(_) => return ids,
    };

    for entry in entries {
        let entry = match entry {
            Ok(entry) => entry,
            Err(_) => continue,
        };

        let id = entry.file_name();
        let id = id.to_string_lossy();

        if let Ok(thread) = id.parse::<u32>() {
            ids.insert(thread);
        }
    }

    ids
}

fn main() {
    let (tx, rx) = unbounded();
    thread::spawn(move || {
        loop {
            let mut quit = String::new();
            if io::stdin().read_line(&mut quit).is_ok() {
                if quit.trim() == "q" {
                    let _ = tx.send(());
                    break;
                }
            }
        }
    });
    println!("Press 'q' to end.");
    let mut proc: Vec<(u32, String, u32)> = vec![];
    let mut missing: Vec<(u32, String, u32)> = vec![];

    let mut ids_a: HashSet<u32> = get_process_ids();

    let mut processes: HashMap<u32, (String, u32, HashSet<u32>)> = HashMap::new();

    for pid in &ids_a {
        let name = get_process_names(*pid).unwrap_or("Unknown".to_string());
        let ppid = get_process_parent_id(*pid).unwrap_or(0);
        let threads = get_thread_id(*pid);
        processes.insert(*pid, (name, ppid, threads));
    }

    // let duration = Duration::from_secs(2);

    loop {
        thread::sleep(Duration::from_secs(2));
        if rx.try_recv().is_ok() {
            break;
        }

        let ids_b = get_process_ids();

        let differ = ids_b.difference(&ids_a);

        let gone = ids_a.difference(&ids_b);

        println!("Snapshot A Processes Found: {}", ids_a.len());

        println!("Snapshot B Processes Found: {}", ids_b.len());

        for pid in differ {
            println!("New processes (pids): {}", pid);
            let proname: Option<String> = get_process_names(*pid);
            let thename = proname.unwrap_or("Unknown".to_string());
            let ppid = get_process_parent_id(*pid).unwrap_or(0);
            let thread = get_thread_id(*pid);
            println!("Process name: {}", thename);
            println!("Parent process id: {}", ppid);
            for thr in &thread {
                println!("Thread IDs: {}", thr);
            }

            processes.insert(*pid, (thename.clone(), ppid, thread));
            proc.push((*pid, thename, ppid));
        }

        for pid in gone {
            println!("Missing processes (pids): {}", pid);

            if let Some((name, ppid, thread)) = processes.remove(pid) {
                println!("Process Name: {}", name);
                println!("Parent process id: {}", ppid);
                for thr in &thread {
                    println!("Thread IDs: {}", thr);
                }
                missing.push((*pid, name, ppid));
            }
        }

        for pid in &ids_b {
            if let Some(process) = processes.get_mut(pid) {
                let old_threads = &process.2;
                let new_threads = get_thread_id(*pid);

                if new_threads.is_empty() {
                    continue;
                }

                let new = new_threads.difference(old_threads);
                let gone = old_threads.difference(&new_threads);

                for tid in new {
                    println!("New Threads detected: PID {} | TID {}", pid, tid);
                }
                for tid in gone {
                    println!("Thread Terminated: PID {} | TID {}", pid, tid);
                }
                process.2 = new_threads;
            }
        }

        if rx.try_recv().is_ok() {
            break;
        }

        ids_a = ids_b;
    }

    println!("New Processes Detected: {}", proc.len());
    for (pid, name, ppid) in &proc {
        println!(
            "Process ID: {} | Process Name: {} | Parent ID: {}",
            pid, name, ppid
        );
    }

    println!("New Processes Detected: {}", missing.len());
    for (pid, name, ppid) in &missing {
        println!(
            "Process ID: {} | Process Name: {} | Parent ID: {}",
            pid, name, ppid
        );
    }
}
