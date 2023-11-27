use std::net::TcpListener;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:4221").unwrap();
    // listener is a iterator to a array of streams
    for stream in listener.incoming(){
        let stream = stream.unwrap();// overshadowing stream
        // stream is a connection attempt and when os limit reaches and throws error
        println!("Connection established");
    }
}
