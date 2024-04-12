use::std::io;
fn get_input() -> io::Result<String> {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer)?;
    Ok(buffer.trim().to_owned())

}
fn main() {
    let mut all_input = Vec::new();
    let mut times_input=0;
    while times_input<2 {
        match get_input(){
            Ok(words) => {
                all_input.push(words);
                times_input+=1;
            }
            Err(e) => {
                eprintln!("Error: {:?}", e);
                break;
            }
       }
    }
    for input in all_input {
        println!("{:?}", input);
    }
    println!("Times of input: {:?}", times_input);
}
