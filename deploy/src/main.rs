fn main() {

    let args: Vec<String> = std::env::args().collect();

    let mut port = serialport::new(&args[1], 115_200)
        .timeout(std::time::Duration::from_millis(10))
        .open().expect("Failed to open port");

    let file = std::fs::read(&args[2]).unwrap();
    
    port.write("~~~~~~\r".as_bytes()).unwrap();
    std::thread::sleep(std::time::Duration::from_millis(50));
    port.write("lpc\r".as_bytes()).unwrap();
    std::thread::sleep(std::time::Duration::from_millis(50));
    port.write(&0x80000000u64.to_be_bytes()).unwrap();
    std::thread::sleep(std::time::Duration::from_millis(50));
    port.write(&(file.len() as u64).to_be_bytes()).unwrap();
    std::thread::sleep(std::time::Duration::from_millis(50));
    port.write(&file).unwrap();
}
