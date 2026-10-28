use std::fs;
use std::collections::HashSet;
use std::thread;
use std::time::Duration;

fn get_process_ids() -> HashSet<u32> {
    let mut ids: HashSet<u32> = HashSet::new();
    //create a mutable hashset
    let entries = fs::read_dir("/proc").expect("Failed to read /proc");
    //this retrieves the directory entries from /proc, the expect is the exception 


    for entry in entries{
        //iterates over entries
        let entry = entry.expect("Failed to read directory entry");
        //assigns the iterated entry to a var with the exception of not able to be read
        let name = entry.file_name();
        //declares name to be the file_name of the entry
        let name = name.to_string_lossy();
        //makes the name a string

        if let Ok(pid) = name.parse::<u32>(){
            ids.insert(pid);
            //insert pid into hashset
        }
    }
    ids
}


fn main() {
    let ids_a = get_process_ids();

    thread::sleep(Duration::from_secs(2));

    let ids_b = get_process_ids();

    let differ = ids_b.difference(&ids_a);
    
    println!("Snapshot A Processes Found: {}", ids_a.len());

    println!("Snapshot B Processes Found: {}", ids_b.len());

    for pid in differ{

        println!("New processes: {}", pid);
    }
}
