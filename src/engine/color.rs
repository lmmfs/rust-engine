pub struct Color {
    bytes: [u8; 4],
}

impl Color {
    pub fn from_bytes(bytes: &[u8]) -> Self {
        let sized_bytes: &[u8; 4] = bytes.try_into().unwrap();
        Self {
            bytes: sized_bytes.clone(),
        }
    }

    pub fn from_rgba(r: u8, b: u8, g: u8, a: u8) -> Self {
        Self {
            bytes: [r, g, b, a],
        }
    }
    pub fn from_rgb(r: u8, b: u8, g: u8) -> Self {
        Self::from_rgba(r, g, b, 255)
    }

    pub fn as_bytes(&self) -> &[u8; 4] {
        &self.bytes
    }
}