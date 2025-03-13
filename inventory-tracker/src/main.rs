

struct Item{
    name:String,
    quantity:u32,
    price:f32,
}
impl Item{
    fn new(name:String,quantity:u32,price:f32)->Item{
        Item{
            name,
            quantity,
            price
        }
    }
}


struct Inventory{
    items:Vec<Item>,
}
impl Inventory{
    fn add_item(&mut self,name:String,quantity:u32,price:f32){
        self.items.push(Item::new(name,quantity,price));
    }
    fn add_existing_item(&mut self,item:Item){
        self.items.push(item);
    }
    fn change_item_quantity(&mut self,name:String,quantity:u32){
        match self.find_item_for_update(&name) {
            Some(item) => {
                item.quantity = quantity;
            },
            None => {println!("{},item not found",&name)}
        }
    }
    fn find_item_for_update(&mut self,name:&String)-> Option<&mut Item>{
        self.items.iter_mut().find(|item|{item.name == *name})
    }
    fn calculate_total_value(&self)->f32{
        self.items.iter().map(|item|{item.price*item.quantity as f32}).sum()
    }
    fn print(&self){
        for item in &self.items{
            println!("{},{},{}", item.name, item.quantity, item.price);
        }
    }

}
fn main() {

    let mut inventory = Inventory{items:Vec::new()};
    inventory.add_item(String::from("test"),5,1f32);
    inventory.add_item(String::from("test2"),5,6f32);
    inventory.change_item_quantity(String::from("test"),7);

    //demo how to use copy syntaxt, since u32 and f32 implement copy trait, i will create another item with a new name and same data as test2
    let item=&inventory.items[0];
    let item_copy= Item{name:String::from("copied"),..*item};
    inventory.add_existing_item(item_copy);

    println!("total value is {}",inventory.calculate_total_value());
    inventory.print();
}
