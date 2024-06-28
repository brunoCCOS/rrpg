#[derive(Debug,Clone)]
pub struct Inventory {
    capacity: u16,
    load: u16,
    items: Vec<usize>,
    gold: u32,
}

impl Inventory {
    pub fn new(capacity: u16) -> Inventory {
        Inventory {
            capacity,
            load: 0,
            items: vec![0],
            gold: 0,
        }
    }

    fn add_item(&mut self,item: usize) -> () {
        if !self.check_full() {
            self.items.push(item);
        }
        else {
            println!("Inventory full");
        }
    }

    fn add_gold(&mut self, ammount:u32) {
        self.gold += ammount;
    }


    fn spend_gold(&mut self, ammount:u32) {
        if self.gold >= ammount {
            self.gold -= ammount;
        }
        else {
            println!("Not enough gold");
        }
    }

    fn remove_item(&mut self, item: usize) -> () {
        if let Some(index) = self.items.iter().position(|value| *value == item) {
            self.items.swap_remove(index);
        }
        else{
            println!("There's no such item in the inventory")
        }
    }

    fn check_full(&self) -> bool {
        self.load == self.capacity
    }

    fn check_empty(&self) -> bool {
        self.load == 0
    }
}