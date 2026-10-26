//define the shape of a laptop
struct Laptop {
   brand:String,
   price:u32
}

//logic to cost a purchase of laptops of one brand
impl Laptop {
   fn cost(&self, quantity:u32)->u32 {
      //use the . operator to fetch the price via the self keyword
      self.price * quantity
   }
}

fn main() {
   //initialize one instance per brand
   let hp = Laptop {
      brand:String::from("HP"),
      price:650000
   };
   let ibm = Laptop {
      brand:String::from("IBM"),
      price:755000
   };
   let toshiba = Laptop {
      brand:String::from("Toshiba"),
      price:550000
   };
   let dell = Laptop {
      brand:String::from("Dell"),
      price:850000
   };

   //the customer buys 3 laptops from each brand
   let quantity = 3;

   //call the method on each instance
   let hp_cost = hp.cost(quantity);
   let ibm_cost = ibm.cost(quantity);
   let toshiba_cost = toshiba.cost(quantity);
   let dell_cost = dell.cost(quantity);

   println!("{} x {} = N{}",quantity,hp.brand,hp_cost);
   println!("{} x {} = N{}",quantity,ibm.brand,ibm_cost);
   println!("{} x {} = N{}",quantity,toshiba.brand,toshiba_cost);
   println!("{} x {} = N{}",quantity,dell.brand,dell_cost);

   //one total across the four brands
   let total = hp_cost + ibm_cost + toshiba_cost + dell_cost;
   println!("Total cost = N{}",total);
}