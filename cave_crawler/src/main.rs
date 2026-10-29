use std::collections::HashSet;
use std::fs;
use std::io;
use std::io::Write;
use std::thread;
use std::time::{Duration, Instant};

fn get_process_ids() -> HashSet<u32> {
    let mut ids: HashSet<u32> = HashSet::new();
    //create a mutable hashset
    let entries = fs::read_dir("/proc").expect("Failed to read /proc");
    //this retrieves the directory entries from /proc, the expect is the exception

    for entry in entries {
        //iterates over entries
        let entry = entry.expect("Failed to read directory entry");
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

fn main() {
    let mut cont = String::new();
    println!("Continous Scan (1) or Burst? (2)");
    println!("Please enter 1 or 2: ");
    io::stdin()
        .read_line( &mut cont)
        .expect("Failed to read");
    let num: i32 = cont.trim().parse().unwrap();
    if num == 1{
        loop {
            let duration = Duration::from_secs(2);
            println!("Press 'q' to end.");
            io::stdout().flush().unwrap();

            let mut quit = String::new();
            io::stdin()
                .read_line(&mut quit)
                .expect("Failed to read");
            let command = quit.trim();

            let mut proc: Vec<&u32> = vec![];

            let ids_a = get_process_ids();

            thread::sleep(Duration::from_secs(2));

            let ids_b = get_process_ids();

            let differ = ids_b.difference(&ids_a);

            println!("Snapshot A Processes Found: {}", ids_a.len());

            println!("Snapshot B Processes Found: {}", ids_b.len());

            for pid in differ {
                println!("New processes (pids): {}", pid);
                proc.push(pid);
            }

            let start = Instant::now();
            if start.elapsed() > duration{
                continue;
            }

            if command == "q"{
                println!("All new processes: {:#?}", proc);
                break;
            }
            
            
            
        }
    }
    else{
        let mut count = 10;
        while count > 0 {
            let ids_a = get_process_ids();

            thread::sleep(Duration::from_secs(2));

            let ids_b = get_process_ids();

            let differ = ids_b.difference(&ids_a);

            println!("Snapshot A Processes Found: {}", ids_a.len());

            println!("Snapshot B Processes Found: {}", ids_b.len());

            for pid in differ {
                println!("New processes (pids): {}", pid);
            }
            if count == 1 {
                let mut answer = String::new();

                println!("Keep checking processes? (y/n) ");
                io::stdin()
                    .read_line(&mut answer)
                    .expect("Failed to read line");

                let lowered = answer.trim().to_lowercase();
                if lowered == "y" || lowered == "yes" {
                    count = 11;
                }
            }
            count -= 1;
        }
}
}
