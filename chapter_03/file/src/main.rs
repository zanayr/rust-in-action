#![allow(unused_variables)]

#[derive(Debug)]
struct File {
    name: String,
    data: Vec<u8>,
}

fn open(f: &mut File) -> bool {
    true
}

fn close(f: &mut File) -> bool {
    true
}

fn read(f: &File, save_to: &mut Vec<u8>) -> usize {
    let mut tmp = f.data.clone();
    let read_length = tmp.len();

    save_to.reserve(read_length);
    save_to.append(&mut tmp);
    read_length
}

fn main() {
    let mut file = File {
        name: String::from("readme.md"),
        data: vec![114, 117, 115, 116, 33],
    };

    let mut buffer: Vec<u8> = vec![];


    open(&mut file);
    let file_length = read(&file, &mut buffer);
    close(&mut file);

    let text = String::from_utf8_lossy(&buffer);

    println!("{:?}", file);
    println!("{} is {} bytes long", &file.name, file_length);
    println!("{}", text);
}