use std::io;
fn main() {
    // loop used if input is not correct
    loop {
        // bit_size is the bit size used later for calc

        println!("Input the bit size");
        let mut bit_size = String::new();

        // Convert input from String to unsigned 32bit integer

        io::stdin()
            .read_line(&mut bit_size)
            .expect("Could not read String");

        let bit_size: u32 = match bit_size.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        // check if input equals bit size, else new loop iteration

        match bit_size {
            8 => println!("using 8 bit"),
            32 => println!("using 32bit"),
            64 => println!("using 64bit"),
            128 => println!("using 128bit"),
            _ => continue,
        }

        // ask user if limit is signed or unsigned, yes I am too stupid to use Strings for if checks :)

        println!("0 => signed\n1=> unsigned");
        let mut signed_unsigned = String::new();

        // same convertion from String to integer as above

        io::stdin()
            .read_line(&mut signed_unsigned)
            .expect("Failed to read String");

        let signed_unsigned: u32 = match signed_unsigned.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        // use 2 as the base for the power calculation

        let base: u128 = 2;

        if signed_unsigned == 0 {
            //signed integer calc
            print!("\x1B[2J\x1B[1;1H");
            println!(
                "{}bit signed can store from -{} to {}",
                bit_size,
                base.pow(bit_size - 1),
                base.pow(bit_size - 1) - 1
            );
            break;
        } else if signed_unsigned == 1 {
            //unsigned integer calc
            print!("\x1B[2J\x1B[1;1H");
            println!(
                "{}bit unsigned can store from 0 to {}",
                bit_size,
                base.pow(bit_size) - 1
            );
            break;
        } else {
            // if neither 0 nor 1 was the input, new loop iteration
            println!("No signed or unsigned input, try again!");
            continue;
        }
    }
}
