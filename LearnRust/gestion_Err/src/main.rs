use std::{fs, io};

fn read_cfg()  -> io::Result<String> {
    let cfg1 = fs::read_to_string("./hello.cfg")?;
    let cfg2 = fs::read_to_string("./world.cfg")?;
    Ok(cfg1 + &
        cfg2)
}
fn main() {
    let s = read_cfg().expect(" reading cfg");
    println!("{}", s)
}
