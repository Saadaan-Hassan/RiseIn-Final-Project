use std::{
    collections::HashMap,
    io::{self, Write},
};

#[derive(Debug)]
struct Product {
    id: u32,
    name: String,
    description: String,
    price: f64,
    quantity: i32,
}

impl Product {
    fn new(id: u32, name: String, description: String, price: f64, quantity: i32) -> Self {
        Self {
            id,
            name,
            description,
            price,
            quantity,
        }
    }
}

fn main() {
    let mut products: Vec<Product> = Vec::new();
    let mut credentials = HashMap::new();
    credentials.insert(String::from("saadaan"), String::from("saadaan"));

    add_dummy_data(&mut products);

    println!("==============Rusty Store Inventory Management System==============");

    loop {
        println!("Please login to continue");
        print!("Username: ");
        io::stdout().flush().unwrap();
        let mut username = String::new();
        io::stdin()
            .read_line(&mut username)
            .expect("Failed to read input");
        let username = username.trim();

        print!("Password: ");
        io::stdout().flush().unwrap();
        let mut password = String::new();
        io::stdin()
            .read_line(&mut password)
            .expect("Failed to read input");
        let password = password.trim();

        if let Some(expected_password) = credentials.get(username) {
            if *expected_password == password {
                println!("Login successful.\n");
                break;
            }
        }
        println!("Invalid username or password. Please try again.\n");
    }

    loop {
        println!("Please select an option:");
        println!("1. Add Product");
        println!("2. Edit Product");
        println!("3. Delete Product");
        println!("4. Generate Report");
        println!("5. Exit");

        let mut choice = String::new();
        io::stdin()
            .read_line(&mut choice)
            .expect("Failed to read input");
        let choice: u32 = match choice.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid input. Please enter a number.");
                continue;
            }
        };

        match choice {
            1 => add_product(&mut products),
            2 => edit_product(&mut products),
            3 => delete_product(&mut products),
            4 => generate_report(&products),
            5 => {
                println!("Exiting program. Goodbye!");
                break;
            }
            _ => println!("Invalid choice. Please select a number from 1 to 5."),
        }
    }
}

fn add_dummy_data(products: &mut Vec<Product>) {
    let dummy_data = vec![
        Product::new(
            1,
            String::from("Wireless Gaming Mouse"),
            String::from("High-precision mouse with customizable RGB lighting"),
            39.99,
            50,
        ),
        Product::new(
            2,
            String::from("Organic Green Tea"),
            String::from("Premium quality green tea leaves sourced from organic farms"),
            9.99,
            100,
        ),
        Product::new(
            3,
            String::from("Bluetooth Speaker"),
            String::from("Portable speaker with deep bass and 360° sound"),
            59.99,
            30,
        ),
        Product::new(
            4,
            String::from("Stainless Steel Water Bottle"),
            String::from("Durable water bottle with vacuum insulation for hot and cold drinks"),
            19.99,
            50,
        ),
        Product::new(
            5,
            String::from("Fitness Tracker Watch"),
            String::from("Smartwatch with built-in heart rate monitor and activity tracking"),
            79.99,
            20,
        ),
    ];

    for product in dummy_data {
        products.push(product);
    }
}

fn add_product(products: &mut Vec<Product>) {
    println!("Enter product details:");
    print!("Name: ");
    io::stdout().flush().unwrap();
    let mut name = String::new();
    io::stdin()
        .read_line(&mut name)
        .expect("Failed to read input");

    print!("Description: ");
    io::stdout().flush().unwrap();
    let mut description = String::new();
    io::stdin()
        .read_line(&mut description)
        .expect("Failed to read input");

    print!("Price: ");
    io::stdout().flush().unwrap();
    let mut price_str = String::new();
    io::stdin()
        .read_line(&mut price_str)
        .expect("Failed to read input");
    let price: f64 = match price_str.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Invalid input for price. Please enter a number.");
            return;
        }
    };

    print!("Quantity: ");
    io::stdout().flush().unwrap();
    let mut quantity_str = String::new();
    io::stdin()
        .read_line(&mut quantity_str)
        .expect("Failed to read input");
    let quantity: i32 = match quantity_str.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Invalid input for quantity. Please enter a number.");
            return;
        }
    };

    let id = products.iter().map(|p| p.id).max().unwrap_or(0) + 1;
    let new_product = Product::new(
        id,
        name.trim().to_string(),
        description.trim().to_string(),
        price,
        quantity,
    );
    products.push(new_product);
    println!("Product added successfully.\n");
}

fn edit_product(products: &mut Vec<Product>) {
    if products.is_empty() {
        println!("No products available to edit.");
        return;
    }

    println!("Enter the ID of the product you want to edit:");
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");
    let input: u32 = match input.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Invalid input for ID. Please enter a number.");
            return;
        }
    };

    if let Some(product) = products.iter_mut().find(|p| p.id == input) {
        println!("Enter new details for the product:");
        print!("Name: ");
        io::stdout().flush().unwrap();
        let mut name = String::new();
        io::stdin()
            .read_line(&mut name)
            .expect("Failed to read input");

        print!("Description: ");
        io::stdout().flush().unwrap();
        let mut description = String::new();
        io::stdin()
            .read_line(&mut description)
            .expect("Failed to read input");

        print!("Price: ");
        io::stdout().flush().unwrap();
        let mut price_str = String::new();
        io::stdin()
            .read_line(&mut price_str)
            .expect("Failed to read input");
        let price: f64 = match price_str.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid input for price. Please enter a number.");
                return;
            }
        };

        print!("Quantity: ");
        io::stdout().flush().unwrap();
        let mut quantity_str = String::new();
        io::stdin()
            .read_line(&mut quantity_str)
            .expect("Failed to read input");
        let quantity: i32 = match quantity_str.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid input for quantity. Please enter a number.");
                return;
            }
        };

        product.name = name.trim().to_string();
        product.description = description.trim().to_string();
        product.price = price;
        product.quantity = quantity;
        println!("Product details updated successfully.\n");
    } else {
        println!("No product found with the given ID.");
    }
}

fn delete_product(products: &mut Vec<Product>) {
    if products.is_empty() {
        println!("No products available to delete.");
        return;
    }

    println!("Enter the ID of the product you want to delete:");
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");
    let input: u32 = match input.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Invalid input for ID. Please enter a number.");
            return;
        }
    };

    if let Some(index) = products.iter().position(|p| p.id == input) {
        products.remove(index);
        println!("Product deleted successfully.\n");
    } else {
        println!("No product found with the given ID.");
    }
}

fn generate_report(products: &Vec<Product>) {
    if products.is_empty() {
        println!("No products available to generate report.");
        return;
    }

    println!("ID | Name | Description | Price | Quantity");
    for product in products {
        println!(
            "{} | {} | {} | {} | {}",
            product.id, product.name, product.description, product.price, product.quantity
        );
    }
}
