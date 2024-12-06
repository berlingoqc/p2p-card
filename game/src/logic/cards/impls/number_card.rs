use crate::logic::cards::{card::Card, color::CardColor, color_card::ColorCard};


pub struct NumberCard {
    number: u32,
    col: CardColor,
}


impl NumberCard {
    fn number(&self) -> u32 {
        return self.number;
    }
}


impl ColorCard for NumberCard {
    fn color(&self) -> crate::logic::cards::color::CardColor {
        return self.col; 
    }
}


impl Card for NumberCard {

}