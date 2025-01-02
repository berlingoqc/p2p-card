use msg::ClientHandlers;

pub mod chain;
pub mod client;
pub mod msg;
pub mod card;


impl From<u32> for ClientHandlers {
   fn from(value: u32) -> Self {
      ClientHandlers::from_i32(value as i32).unwrap()
   } 
}

impl Into<u32> for ClientHandlers {
   fn into(self) -> u32 {
      let i32_v: i32 = self.into();
      i32_v as u32
   }
}
