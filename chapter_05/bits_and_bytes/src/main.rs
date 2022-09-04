use std::mem::transmute;

fn main() {
    {   // Listing 5.2
        let a: u16 = 50115;
        let b: i16 = -15421;

        println!("a: {:016b} {}", a, a);
        println!("b: {:016b} {}", b, b);
    }

    {   // Listing 5.2
        let a: f32 = 42.42;
        let frankentype: u32 = unsafe {
            transmute(a)
        };

        println!("{}", frankentype);
        println!("{:032b}", frankentype);

        let b: f32 = unsafe {
            transmute(frankentype)
        };
        println!("{}", b);
        assert_eq!(a, b,);
    }

    { // Listing 5.6
        let big_endian: [u8; 4]     = [0xAA, 0xBB, 0xCC, 0xDD];
        let little_endian: [u8; 4]  = [0xDD, 0xCC, 0xBB, 0xAA];

        let a: i32 = unsafe { transmute(big_endian) };
        let b: i32 = unsafe { transmute(little_endian) };

        println!("{} vs {}", a, b);
    }
}
