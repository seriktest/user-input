use std::{collections::HashMap, io};

#[derive(Debug, Clone)]
pub struct Bill {
    name: String,
    amount: f64,
}

pub struct Bills {
    inner: HashMap<String, Bill>,
}

impl Default for Bills {
    fn default() -> Self {
        Self::new()
    }
}

impl Bills {
    pub fn new() -> Self {
        Self {
            inner: HashMap::new(),
        }
    }

    fn add(&mut self, bill: Bill) {
        self.inner.insert(bill.name.to_string(), bill);
    }

    fn get_all(&self) -> Vec<&Bill> {
        self.inner.values().collect()
    }

    fn remove(&mut self, name: &str) -> bool {
        self.inner.remove(name).is_some()
    }

    fn update(&mut self, name: &str, amount: f64) -> bool {
        if let Some(bill) = self.inner.get_mut(name) {
            bill.amount = amount;
            true
        } else {
            false
        }
    }
}

fn get_input() -> Option<String> {
    let mut buffer = String::new();
    while io::stdin().read_line(&mut buffer).is_err() {
        println!("Please type a value");
    }
    let trimmed = buffer.trim().to_owned();
    if trimmed.is_empty() {
        None
    } else {
        Some(trimmed)
    }
}

fn get_bill_amount() -> Option<f64> {
    loop {
        let input = match get_input() {
            Some(input) => input,
            None => return None,
        };
        if input.is_empty() {
            return None;
        }
        let parsed_input: Result<f64, _> = input.parse();
        match parsed_input {
            Ok(amount) => return Some(amount),
            Err(_) => println!("Please enter a number"),
        }
    }
}

mod menu {
    use crate::{get_bill_amount, get_input, Bill, Bills};

    pub fn add_bill(bills: &mut Bills) {
        println!("Bill name: ");
        let name = match get_input() {
            Some(name) => name,
            None => return,
        };
        println!("Bill amount: ");
        let amount = match get_bill_amount() {
            Some(amount) => amount,
            None => return,
        };
        let bill = Bill { name, amount };
        bills.add(bill);
        println!("Bill added");
    }

    pub fn view_bill(bills: &Bills) {
        for bill in bills.get_all() {
            println!("{:?}", bill);
        }
    }

    pub fn remove_bill(bills: &mut Bills) {
        for bill in bills.get_all() {
            println!("{:?}", bill);
        }
        println!("Enter bill name to remove");
        let name = match get_input() {
            Some(name) => name,
            None => return,
        };
        if bills.remove(&name) {
            println!("Bill removed");
        } else {
            println!("Bill not found");
        }
    }

    pub fn update_bill(bills: &mut Bills) {
        for bill in bills.get_all() {
            println!("{:?}", bill);
        }

        println!("Enter bill name to update");
        let name = match get_input() {
            Some(name) => name,
            None => return,
        };
        let amount = match get_bill_amount() {
            Some(amount) => amount,
            None => return,
        };
        if bills.update(&name, amount) {
            println!("Bill updated");
        } else {
            println!("Bill not found");
        }
    }
}

enum Menu {
    AddBill,
    ViewBill,
    RemoveBill,
    UpdateBill
}

impl Menu {
    fn from_str(input: &str) -> Option<Menu> {
        match input {
            "1" => Some(Menu::AddBill),
            "2" => Some(Menu::ViewBill),
            "3" => Some(Menu::RemoveBill),
            "4" => Some(Menu::UpdateBill),
            _ => None,
        }
    }
    fn show() {
        println!();
        println!(" == Bill Manager == ");
        println!("1. Add Bill");
        println!("2. View Bills");
        println!("3. Remove Bill");
        println!("4. Update Bill");
        println!();
        println!("Enter selection: ");
    }
}

fn main() {
    let mut bills = Bills::new();
    loop {
        Menu::show();
        let input = get_input().expect("no data entered");
        match Menu::from_str(input.as_str()) {
            Some(Menu::AddBill) => menu::add_bill(&mut bills),
            Some(Menu::RemoveBill) => menu::remove_bill(&mut bills),
            Some(Menu::ViewBill) => menu::view_bill(&bills),
            Some(Menu::UpdateBill) => menu::update_bill(&mut bills),
            None => return,
        }
    }
}
