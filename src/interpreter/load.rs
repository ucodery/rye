pub fn load_from_file(path: String) {
    let byte_file = std::fs::read(path).expect("Nothing in the file");

    for byte_code in byte_file {
        println!("Got: {:X}", byte_code)
    }
}
