use std::net::UdpSocket;

fn main() {
    let socket: UdpSocket = UdpSocket::bind("127.0.0.1:7896").expect("Couldn't bind to address");
    socket.connect("127.0.0.1:8080").expect("Connection failed");
    let message = "Hello server\n";
    socket.send(message.as_bytes()).expect("Did not send data");
    let mut buffer = [0; 1024];
    let (received_bytes, _) = socket.recv_from(&mut buffer)
        .expect("Failed to receive data");

    let received_message = String::from_utf8_lossy(&buffer[..received_bytes]);
    println!("Received message: {}", received_message);
}