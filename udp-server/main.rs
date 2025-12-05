use std::net::UdpSocket;

fn main() {
    println!("Opening UDP socket on 127.0.0.1:8080");
    let socket: UdpSocket = UdpSocket::bind("127.0.0.1:8080").unwrap();
    println!("I guess the socket is opened!");

    loop {
        let mut buffer = [0; 1024];
        let (received_bytes, client_address) = socket.recv_from(&mut buffer)
            .expect("Failed to receive data");

        let received_message: std::borrow::Cow<'_, str> = String::from_utf8_lossy(&buffer[..received_bytes]);
        println!("Received message from {}: {}", client_address, received_message);

        let response = "Hello, client!";
        socket.send_to(response.as_bytes(), client_address)
            .expect("Failed to send data");
    }
}