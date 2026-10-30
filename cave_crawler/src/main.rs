use std::collections::HashSet;
use std::fs;
use std::io;
use std::thread;
use std::time::Duration;
// use std::time::Instant;
use crossbeam_channel::unbounded;

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

fn get_process_names(pid: u32) -> Option<String>{
    //fn needs pid to run, returns a string
    let path = format!("/proc/{}/comm", pid);
    //path is the pid to the comm directory where the process names are stored
    match fs::read_to_string(path){
        Ok(name) => Some(name.trim().to_string()),
        Err(_) => None,
    }
    //reads the directory names as strings
}


fn main() {
    if let Some(name) = get_process_names(std::process::id()){
        println!("cave_craweler process name: {}", name);
    }
    let mut cont = String::new();
    println!("Continous Scan (1) or Burst? (2)");
    println!("Please enter 1 or 2: ");
    io::stdin()
        .read_line( &mut cont)
        .expect("Failed to read");
    let num: i32 = cont.trim().parse().unwrap();
    if num == 1{
            let (tx, rx) = unbounded();
            thread::spawn(move || {
                loop{
                    let mut quit = String::new();
                    if io::stdin().read_line(&mut quit).is_ok(){
                        if quit.trim() == "q"{
                            let _ = tx.send(());
                            break;
                        }
                    }
                }
            });
            println!("Press 'q' to end.");
            let mut proc: Vec<u32> = vec![];
            let mut missing: Vec<u32> = vec![];


            let mut ids_a: HashSet<u32> = get_process_ids();


            // let duration = Duration::from_secs(2);

        loop {

            thread::sleep(Duration::from_secs(2));
            if rx.try_recv().is_ok(){
                break;
            }

            let ids_b = get_process_ids();

            let differ = ids_b.difference(&ids_a);

            let gone = ids_a.difference(&ids_b);

            println!("Snapshot A Processes Found: {}", ids_a.len());

            println!("Snapshot B Processes Found: {}", ids_b.len());

            for pid in differ {
                println!("New processes (pids): {}", pid);
                proc.push(*pid);
            }

            for pid in gone{
                println!("Missing processes (pids): {}", pid);
                missing.push(*pid);
            }

            // let start = Instant::now();
            // if start.elapsed() > duration{
            //     continue;
            // }

            if rx.try_recv().is_ok(){
                println!("All new processes from this cycle: {:#?}", proc);
                println!("Processes that disappeared in this cyle: {:#?}", missing);

                break;
            }
            
            ids_a = ids_b;
        }
        println!("All new processes from this cycle: {:#?}", proc);
        println!("Processes that disappeared in this cyle: {:#?}", missing)

    }
    else{
        let mut ids_a: HashSet<u32> = get_process_ids();
        let mut count = 10;
        while count > 0 {

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
            ids_a = ids_b;
            count -= 1;
        }
}
}
