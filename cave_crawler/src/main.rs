use std::collections::HashSet;
use std::fs;
use std::io;
use std::thread;
use std::time::Duration;
// use std::time::Instant;
use crossbeam_channel::unbounded;
use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*; 

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
    //path for folder of pid to find status, where ppids will be found
    let yargle = fs::read_to_string(path).ok()?;
    //i picked yargle as a var name because i was watching a mtg video while coding, path to string
    for line in yargle.lines() {
        //for line in the lines of yargle
        if line.starts_with("PPid:") {
            //find the line with PPid:
            if let Some((_, value)) = line.split_once(":") {
                //splits line at :
                let ppid = value.trim().parse::<u32>();
                //makes ppid value, returns it below
                return ppid.ok();
            }
        }
    }

    None
    //return nothing otherwise
}

fn get_thread_id(pid: u32) -> HashSet<u32> {
    let mut ids: HashSet<u32> = HashSet::new();
    //make hashset for ids
    let path = format!("/proc/{}/task", pid);
    //path like the others, task being for threads
    let entries = match fs::read_dir(path) {
        Ok(entries) => entries,
        Err(_) => return ids,
    };
    //reads directory at path

    for entry in entries {
        let entry = match entry {
            Ok(entry) => entry,
            Err(_) => continue,
        };
        //for entry in entries

        let id = entry.file_name();
        let id = id.to_string_lossy();
        //id equals the file name

        if let Ok(thread) = id.parse::<u32>() {
            ids.insert(thread);
        }
        //add that string to the hashset
    }

    ids 
    //return hashset
}

fn main() -> std::io::Result<()>{
    let (tx, rx) = unbounded();
    //send and recieve input
    thread::spawn(move || {
        //create the thread
        loop {
            let mut quit = String::new();
            //quit input which should be 'q'
            if io::stdin().read_line(&mut quit).is_ok() {
                if quit.trim() == "q" {
                    let _ = tx.send(());
                    //if quit == q, end the fn
                    break;
                }
            }
        }
    });
    println!("Press 'q' to end.");
    let mut proc: Vec<(u32, String, u32)> = vec![];
    let mut missing: Vec<(u32, String, u32)> = vec![];
    //make vecs that accept tuples of int, string, int
    let mut ids_a: HashSet<u32> = get_process_ids();
    //make hashset for ids, needs to be out here for the way that the loop works
    let mut processes: HashMap<u32, (String, u32, HashSet<u32>)> = HashMap::new();
    //make hashmap for the processes

    for pid in &ids_a {
        let name = get_process_names(*pid).unwrap_or("Unknown".to_string());
        //name equals the names from the fn
        let ppid = get_process_parent_id(*pid).unwrap_or(0);
        //parent pid from the fn
        let threads = get_thread_id(*pid);
        //threads from the pid
        processes.insert(*pid, (name, ppid, threads));
        //add to the hashmap with the key being *pid and the tuple of other vars
    }

    loop {
        thread::sleep(Duration::from_secs(2));
        if rx.try_recv().is_ok() {
            break;
        }
        //if q is recieved, break loop

        let ids_b = get_process_ids();
        //second snapshot 
        let differ = ids_b.difference(&ids_a);
        //differences in new processes
        let gone = ids_a.difference(&ids_b);
        //processes that have disappeared
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
                //print all threads
            }

            processes.insert(*pid, (thename.clone(), ppid, thread));
            //insert into hashmap with key *pid and tuple of above vars
            proc.push((*pid, thename, ppid));
            //push to vec
        }

        for pid in gone {
            println!("Missing processes (pids): {}", pid);

            if let Some((name, ppid, thread)) = processes.remove(pid) {
                //remove from hashmap instead
                println!("Process Name: {}", name);
                println!("Parent process id: {}", ppid);
                for thr in &thread {
                    println!("Thread IDs: {}", thr);
                }
                missing.push((*pid, name, ppid));
            }
            //same thing as above but with missing things
        }

        for pid in &ids_b {
            if let Some(process) = processes.get_mut(pid) {
                //get mutable from pid in processes
                let old_threads = &process.2;
                let new_threads = get_thread_id(*pid);
                //check from the hashmap and make new thread hashset
                if new_threads.is_empty() {
                    continue;
                }
                //dont crash if no new threads
                let new = new_threads.difference(old_threads);
                let gone = old_threads.difference(&new_threads);
                //compare differences for new and missing threads
                for tid in new {
                    println!("New Threads detected: PID {} | TID {}", pid, tid);
                }
                for tid in gone {
                    println!("Thread Terminated: PID {} | TID {}", pid, tid);
                }
                //print them
                process.2 = new_threads;
                //add new_threads to process.2
            }
        }

        if rx.try_recv().is_ok() {
            break;
        }
        //again press 'q' and exit loop

        ids_a = ids_b;
        //reset loop by making ids_a (which is intialized outside the loop) equal to the current ids_b
    }
    let mut file = File::create("log.txt")?;

    println!("New Processes Detected: {}", proc.len());
    for (pid, name, ppid) in &proc {
        println!(
            "Process ID: {} | Process Name: {} | Parent ID: {}",
            pid, name, ppid,
        );
        writeln!(file, "Process ID: {} | Process Name: {} | Parent ID: {}",
            pid, name, ppid)?;
    }

    println!("Missing Processes Detected: {}", missing.len());
    for (pid, name, ppid) in &missing {
        println!(
            "Process ID: {} | Process Name: {} | Parent ID: {}",
            pid, name, ppid,
        );
        writeln!(file, "Process ID: {} | Process Name: {} | Parent ID: {}",
            pid, name, ppid)?;

    }
    println!("Log File Written!");
    println!("Check log.txt");
    //print the stuff
    Ok(())
}
