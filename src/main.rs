use std::io;
fn main() {
    // loop used if input is not correct
    loop {
        // bit_size is the bit size used later for calc

        print!("\x1B[2J\x1B[1;1H"); // clears console
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

        // ask user if limit is signed or unsigned

        println!("signed or unsigned?");
        let mut signed_unsigned = String::new();

        io::stdin()
            .read_line(&mut signed_unsigned)
            .expect("Failed to read String");

        // use 2 as the base for the power calculation
        let base: u128 = 2;

        print!("\x1B[2J\x1B[1;1H"); // clears console
        if signed_unsigned.trim() == "signed" {
            signed(bit_size, base);
        } else if signed_unsigned.trim() == "unsigned" {
            unsigned(bit_size, base);
        }
        break;
    }
}

fn signed(bit_size: u32, base: u128) {
    //signed integer calc
    print!("\x1B[2J\x1B[1;1H"); // clears console
    if bit_size == 128 {
        // special case because 128bit
        println!("128bit signed can store from -1.701411835x10³⁸ to 1.701411835x10³⁸");
    }
    println!(
        "{}bit signed can store from -{} to {}",
        bit_size,
        base.pow(bit_size - 1),
        base.pow(bit_size - 1) - 1
    );
}

fn unsigned(bit_size: u32, base: u128) {
    //unsigned integer calc
    if bit_size == 128 {
        // special case because 128bit
        println!("128bit signed can store from 0 to 3.402823669x10³⁸");
    }
    println!(
        "{}bit unsigned can store from 0 to {}",
        bit_size,
        base.pow(bit_size) - 1
    );
}
