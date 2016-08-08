use std::net::UdpSocket;
use std::io;

pub struct Flaschen {
    socket: UdpSocket,
    host: String,
    width: u16,
    height: u16,
}

#[derive(Debug, Copy, Clone)]
pub struct Pixel {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Flaschen{
    pub fn new(hostname: String, width: u16, height: u16) -> io::Result<Flaschen> {
        println!("connecting to {}: ({}, {})", hostname, width, height);
        let local_socket = "0.0.0.0:0";
        let port_string = ":1337";
        match UdpSocket::bind(&*local_socket) {
            Result::Ok(s)  => Ok(Flaschen{socket: s, height: height, width: width, host: hostname + port_string}),
            Result::Err(e) => Err(e),
        }
    }

    pub fn send(&self, image: &[Vec<Pixel>]) -> io::Result<usize> {
        let header = format!("P6\n{} {}\n255\n", self.width.to_string(), self.height.to_string());
        let mut packet = header.into_bytes();
        for row in image {
            for pixel in row {
                packet.extend_from_slice(&[pixel.r, pixel.g, pixel.b]);
            }
        }
        println!("{}", self.host);
        self.socket.send_to(&packet, &*(self.host))
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let host = "localhost";
        let f = ::Flaschen::new(host.to_string(), 45, 35).unwrap();
        let p = ::Pixel {r: 0, g: 255, b: 20};
        let image = vec![vec![p, p, p]]; 
        f.send(&image);
    }
}
