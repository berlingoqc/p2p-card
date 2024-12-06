use crate::logic::cards::{card::Card, color::CardColor, color_card::ColorCard};


pub struct PickCard {
    quantity: u32,

    color: Option<CardColor>
}


impl ColorCard for PickCard {

    fn color(&self) -> CardColor {
        return self.color.unwrap();
    }
    
}


impl Card for PickCard  {
    
}