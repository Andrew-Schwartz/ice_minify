#![allow(incomplete_features)]
#![feature(generic_const_exprs)]

pub trait HasLen {
    const LEN: usize;
}

pub struct Header;
impl HasLen for Header {
    const LEN: usize = 5;
}

pub trait Message: HasLen {
    fn pack(self) -> [u8; Header::LEN + Self::LEN]
    where
        Self: Sized,
    {
        [0; Header::LEN + Self::LEN]
    }
}

pub struct MyMessage;

impl HasLen for MyMessage {
    const LEN: usize = 2;
}

impl Message for MyMessage {}

pub struct Light;

impl Light {
    pub fn send<M: Message>(self, m: M)
    where
        [(); Header::LEN + M::LEN]:,
    {
        let _bytes = m.pack();
    }
}
