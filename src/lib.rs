pub mod uart {
    use std::convert::TryFrom;

    #[derive(Debug)]
    pub struct IndexError(pub u8);

    #[derive(Debug, PartialEq, Eq, Clone, Copy)]
    pub struct Idx {
        relay_number: u8,
    }

    impl Idx {
        pub fn new(value: u8) -> Self {
            Self::try_from(value).unwrap()
        }

        pub fn as_byte(&self) -> u8 {
            b'0' + self.relay_number
        }
    }

    impl TryFrom<u8> for Idx {
        type Error = IndexError;
        fn try_from(x: u8) -> Result<Self, Self::Error> {
            if x < 1 || x > 9 {
                Err(IndexError(x))
            } else {
                Ok(Idx { relay_number: x })
            }
        }
    }

    #[derive(Debug, Clone, PartialEq, Eq)]
    pub enum Command {
        // Emergency stop all cards, regardless of address
        Emergency, // E
        // Display address
        Display, // D
        // Set a relay
        Set(Idx), // S
        // Clear a relay
        Clear(Idx), // C
        // Toggle a relay
        Toggle(Idx), // T
        // Change the current address of a card
        Address(Idx), // A
        // Force all cards to address 1 (default)
        Force, // F
        // Send a byte
        Byte(u8), // B
    }

    impl Command {
        pub fn to_bytes(&self, address: u8) -> [u8; 5] {
            let (cmd, idx) = self.as_tuple();
            let mut buf = [13, address, cmd, idx, 0];
            buf[4] = buf[0..=3].iter().fold(0, |a, x| a.overflowing_sub(*x).0);
            buf
        }

        fn as_tuple(&self) -> (u8, u8) {
            match *self {
                Command::Emergency => (b'E', 0),
                Command::Display => (b'D', 0),
                Command::Set(x) => (b'S', x.as_byte()),
                Command::Clear(x) => (b'C', x.as_byte()),
                Command::Toggle(x) => (b'T', x.as_byte()),
                Command::Address(x) => (b'A', x.as_byte()),
                Command::Force => (b'F', 0),
                Command::Byte(x) => (b'B', x),
            }
        }
    }
}


#[cfg(test)]
mod tests {
    // use super::*;

    // #[test]

}
