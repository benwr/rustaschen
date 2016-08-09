use std::net::UdpSocket;
use std::io;

mod text;

pub struct Flaschen {
    socket: UdpSocket,
    host: String,
    width: usize,
    height: usize,
}

#[derive(Debug, Copy, Clone)]
pub struct Pixel {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Flaschen{
    pub fn new(hostname: String, width: usize, height: usize) -> io::Result<Flaschen> {
        println!("connecting to {}: ({}, {})", hostname, width, height);
        let local_socket = "0.0.0.0:0";
        let port_string = ":1337";
        match UdpSocket::bind(&*local_socket) {
            Result::Ok(s)  => Ok(Flaschen{socket: s, height: height, width: width, host: hostname + port_string}),
            Result::Err(e) => Err(e),
        }
    }

    pub fn fill(&self, image: &[Vec<Pixel>]) -> io::Result<usize> {
        let height = image.len();
        if height == 0 {
            return Ok(0);
        }
        let width = image[0].len();
        // this is dumb - should make a packet and then send it rather than sending a million packets
        // (for small fill images, packets get lost in a buffer somewhere)
        let rows: usize = self.height / height + 1;
        let cols: usize = self.width / width + 1;
        for row in 0..rows {
            for col in 0..cols {
                try!(self.put(row * height, col * width, 0, image));
            }
        }
        let parts = self.height * self.width;
        Ok(parts)
    }

    pub fn putchar(&self, x: usize, y: usize, z: usize, c: char, color: Pixel, background: Pixel) -> io::Result<usize> {
        let index = ((c as u32) - ('a' as u32)) as usize;
        let mut display = text::LOWERCASE[index];
        let mut image: Vec<Vec<Pixel>> = vec![];
        for row in 0..6 {
            image.push(vec![]);
            for pt in 0..3 {
                if ((display >> (17 - (row * 3 + pt))) & 1) == 1 {
                    image.last_mut().unwrap().push(color);
                } else {
                    image.last_mut().unwrap().push(background);
                }
            }
        }
        self.put(x, y, z, &image)
    }

    pub fn put(&self, x: usize, y: usize, z: usize, image: &[Vec<Pixel>]) -> io::Result<usize> {
        let height = image.len();
        if height == 0 {
            return Ok(0);
        }
        let width = image[0].len();
        let header = format!("P6\n{} {}\n255\n", width.to_string(), height.to_string());
        let mut packet = header.into_bytes();
        for row in image {
            for pixel in row {
                packet.extend_from_slice(&[pixel.r, pixel.g, pixel.b]);
            }
        }
        let footer = format!("\n{}\n{}\n{}", x, y, z);
        packet.extend_from_slice(&footer.into_bytes());
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
        f.fill(&image);
    }
}
