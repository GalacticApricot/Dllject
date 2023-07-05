use std::env;
use std::fs;


fn main() -> std::io::Result<()> {
    println!("collecting arguments...");
    let args: Vec<String> = env::args().collect();
    println!("loading locations...");
    let dllloc: &String = &args[1];
    let targetloc: &String = &args[2];
    if args.len() > 3 {
        let storeloc: &String = &args[3];
        println!("backing up to {}...", storeloc);
        fs::copy(targetloc, storeloc)?;
    }
    println!("injecting {} in {}...", dllloc, targetloc);
    fs::copy(dllloc, targetloc)?;
    println!("injection successful!");
    Ok(())
}
