use std::fs;


fn main() {
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

        if name.parse::<u32>().is_ok(){
            //checks if the name can be parsed into an integer that is non-zero and fits u32
            println!("{}", name);
            //prints the names
        }
    }



}
