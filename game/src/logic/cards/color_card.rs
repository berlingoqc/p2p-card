use super::color::CardColor;



pub trait ColorCard {
    
    fn color(&self) -> CardColor;
}