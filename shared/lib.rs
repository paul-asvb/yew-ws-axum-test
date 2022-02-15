use fake::{Dummy, Fake};

#[derive(Debug, Dummy)]
pub struct Foo {
    #[dummy(faker = "1000..2000")]
    some_number: usize,
    //customer: String,
    //paid: bool,
}

