use std::env::Args;
use std::fs;
fn main() {
    let neg = std::env::args();
    if neg.len() == 1{
        println!("eihgt 0.0.1\nHow to use:\n   -s string : string into hex\n   -f path_to_file : file into hex\n");
        return;
    }
    let mut gsd:Vec<String> = vec![String::new() ; 3];
    let mut t = 0;
    for i in neg.enumerate(){
        gsd[t] = i.1;
        t += 1;
    }
    if gsd[1] == "-s"{
        println!("{:?}", to_hex(gsd[2].as_bytes()))
    }
    else if gsd[1] == "-f"{
        let contents = fs::read(&gsd[2]).expect("No such file");
        println!("{:?}", to_hex(&contents))
    }
    else {
        println!("eihgt 0.0.1\nHow to use:\n   -s string : string into hex\n   -f path_to_file : file into hex\n");
    }


}
fn to_hex(data: &[u8]) -> Vec<String>{
    let mut new_block:Vec<String> = vec![];
    let mut hex_str = String::from("0");
    for i in data.into_iter(){
        hex_str = format!("{:x}", &i);
        new_block.push(hex_str);
    }
    return new_block;
}