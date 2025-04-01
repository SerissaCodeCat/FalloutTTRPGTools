use std::io;
pub fn convert_input_to_bool() -> bool {
    loop {
        let mut tmp = String::new();
        io::stdin()
            .read_line(&mut tmp)
            .expect("failed to read line.");
        if tmp.trim() == "Y" || tmp.trim() == "y" {
            return true;
        } else if tmp.trim() == "N" || tmp.trim() == "n" {
            return false;
        } else {
            println!("please enter Y or N");
        };
    }
}
pub fn convert_input_to_int() -> i32 {
    let mut tmp = String::new();
    println!("please enter a number");
    io::stdin()
        .read_line(&mut tmp)
        .expect("could not read input.");
    return tmp.trim().parse().expect("please enter a number");
}
