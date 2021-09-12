#[derive(Debug)]
pub struct Builder(u8);

impl Builder {
    pub fn new() -> Self {
        Self(1)
    }

    pub fn take(self) -> Self {
        println!("Hello");
        self
    }

    pub fn take_2(mut self) -> Self {
        println!("World");
        self.0 = 5;
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let a = Builder::new().take().take_2();
        println!("{:?}", a);
    }
}
