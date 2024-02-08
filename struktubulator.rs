use std::fmt::*;

struct Car {
    brand: String,
    model: String,
    color: [u8; 3],
    mileage: u32,
    is_new: bool
}

impl Display for Car {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{} {} {} {} {} {} {}", self.brand, self.model, self.color[0], self.color[1], self.color[2], self.mileage, self.is_new)
    }
}

fn main() {
    
    let car1 = Car {
        brand: "Mercedes".to_string(),
        model: "Sprinter".to_string(),
        color: [255, 255, 0],
        mileage: 21000,
        is_new: false
    };

    println!("{}", car1);
}
