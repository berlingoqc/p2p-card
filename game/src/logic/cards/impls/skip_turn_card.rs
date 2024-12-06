
use crate::logic::cards::{card::Card, color::CardColor, color_card::ColorCard};




pub struct SkipTurnCard {
    color: CardColor
}


impl ColorCard for SkipTurnCard {

    fn color(&self) -> CardColor {
        return self.color;
    }
    
}


impl Card for SkipTurnCard  {
    
}