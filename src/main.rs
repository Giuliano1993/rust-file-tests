
use std::{fs::{File, self, OpenOptions}, io::Write};

fn main() -> std::io::Result<()>{
    println!("Hello, world!");

    let mut f = File::create("test.txt")?;
    

    //the b before the string converts the string to byte strign literal
    // converting the string to a series of bytes representing the single characters
    f.write_all(b"buf")?;

    match fs::copy("test.txt", "test_copy.txt"){
        Ok(filebytes) => {

            let mut cloned_file  = OpenOptions::new()
                .read(true)
                .write(true)
                .create(false)
                .append(true)
                .open("test_copy.txt")?;
            
            //File open opens the file in readonly mode
            //let mut cloned_file = File::open("test_copy.txt")?;
            cloned_file.write(b"\n solo cose belle aggiunte al file copiato")?;
        },
        Err(_)=> {panic!("no valid file was read");}
    }

    

    return Ok(());

    


    
    
}
