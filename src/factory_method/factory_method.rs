pub trait Creator {
    fn get_product(&self) -> impl Product;
    fn do_thing(&self) -> String {
        self.get_product().operate()
    }
}

pub trait Product {
    fn operate(&self) -> String;
}

pub struct SpamProduct;

impl Product for SpamProduct {
    fn operate(&self) -> String {
        "spam".to_string()
    }
}

pub struct PassThroughSpamCreator;

impl Creator for PassThroughSpamCreator {
    fn get_product(&self) -> impl Product {
        SpamProduct
    }
}
