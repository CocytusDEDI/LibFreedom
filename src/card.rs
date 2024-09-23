use std::fs::File;

use std::os::unix::io::AsFd;
use std::os::unix::io::BorrowedFd;


pub struct Card(File);

impl AsFd for Card {
    fn as_fd(&self) -> BorrowedFd<'_> {
        self.0.as_fd()
    }
}

impl Card {
    pub fn open(path: &str) -> Result<Self, String> {
        match File::open(path) {
            Ok(file) => Ok(Card(file)),
            Err(_) => Err(String::from("Couldn't open ") + path)
        }
    }
}

impl drm::Device for Card {}
impl drm::control::Device for Card {}
