use crate::logic::cards::{card::Card, color::CardColor, color_card::ColorCard};




pub struct ReverseCard {
    color: CardColor
}


impl ColorCard for ReverseCard {

    fn color(&self) -> CardColor {
        return self.color;
    }
    
}


impl Card for ReverseCard  {
    
}