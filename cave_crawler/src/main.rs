use std::fs;
use std::collections::HashSet;


fn get_process_ids() -> HashSet<u32> {
    let mut ids: HashSet<u32> = HashSet::new();

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
        }
    }
    ids
}


fn main() {
    let ids = get_process_ids();

    for pid in &ids{
        println!("{}", pid);
    }

    println("Total Processes Found: {}", ids.len());

}
