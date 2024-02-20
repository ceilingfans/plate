use std::process::Command;

fn main() {
    if !cfg!(unix) {
        println!("os not impl");
        return;
    }

    println!("building object file");
    Command::new("nasm")
        .arg("-felf64")
        .arg("test.asm")
        .arg("-o")
        .arg("out.o")
        .output()
        .expect("failed to execute process");
    println!("linking");
    Command::new("ld")
        .arg("out.o")
        .arg("-o")
        .arg("plate")
        .output()
        .expect("failed to execute process");
    println!("complete");
}
