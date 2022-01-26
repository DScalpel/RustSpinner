use std::io;
use std::io::Write;
use std::{thread, time};

fn main() {
    

    println!("Spinner Example");
    let frames= ['⠋', '⠙', '⠹', '⠸', '⠼', '⠴', '⠦', '⠧', '⠇', '⠏'];
    let frames1 = ['⢹','⢸','⣸','⡇','⣇','⡇','⡏','⡇'];
    let frames2 = ['⡇','⡏','⠋', '⠙', '⠹','⢸', '⣸','⣠','⣄' ];

    for i in 1..101{
        //print!("{} {} [{}]", frames[(i % frames.len())],frames1[(i % frames1.len())], i);
        print!("{} {} {} [{}]", frames[i % frames.len()],frames1[i % frames1.len()], frames2[i % frames2.len()], i);
        io::stdout().flush().unwrap();
        thread::sleep(time::Duration::from_millis(100));
        if i==101 {
            println!("\n\n");
        }
        else{
            print!("\r");
        }

    }

}
