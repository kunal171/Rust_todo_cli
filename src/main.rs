use std::io;
use std::fs::File;
use serde_json::Result;
use std::collections::HashMap;
use std::io::BufReader;

mod todo;
use crate::todo::User;
use crate::todo::{add_user, add_content, delete_content, edit_content, empty_todo};


// Reading the dataset
fn read_dataset(path: String) -> Result<HashMap<String, User>> {
    let file = File::open(path).expect("Failed to load file");
    let reader = BufReader::new(file);
    let dataset: HashMap<String, User> = serde_json::from_reader(reader)?;

    Ok(dataset)
}

// saving the dataset
fn save_dataset(path: String, dataset: &HashMap<String, User>) {
    let file = File::create(path).unwrap();
    serde_json::to_writer(file, &dataset).expect("Falied to save data!");
}


fn main() {
    let mut dataset = read_dataset(String::from("dataset.json")).unwrap();

    println!("Please enter username:- ");
    let mut username = String::new();
    io::stdin().read_line(&mut username).expect("failed to read the data");
    println!("your username: {}", &username);

    username = username.trim_end().to_string();
    // let user = dataset.get(&username);
    let user = &mut dataset.get_mut(&username);

    // Pattern match to retrieve the value
    match user {
        Some(x) => {
            println!("Please enter your password");
            let mut password = String::new();
            io::stdin().read_line(&mut password).expect("failed to read the data");
            let saved_password = &x.password;
            
            if &password.trim_end().to_string() == saved_password {
                println!("Here is your todo list: {:?}", &x.todo);
                println!("What do you want to with list now? add?, delete?, edit?, empty?");
                let mut action = String::new();
                io::stdin().read_line(&mut action).expect("failed to read the data");
                
                if action.trim_end().to_lowercase() == "add" {
                    println!("Please add the content!");
                    let mut content = String::new();
                    io::stdin().read_line(&mut content).expect("failed to read the data");
                    content = content.trim_end().to_string();
                    add_content(&mut x.todo, content);
                    save_dataset(String::from("dataset.json"), &dataset);
                }
                else if action.trim_end().to_lowercase() == "delete"{
                    println!("Here is your current todo list: {:?}", &x.todo);
                    println!("Please enter the number which you want to delete!");
                    let mut idx = String::new();
                    io::stdin().read_line(&mut idx).expect("failed to read the data");
                    idx = idx.trim_end().to_string();
                    let idx = idx.parse::<i32>().unwrap() - 1;
                    let idx = idx.try_into().unwrap();
                    delete_content(&mut x.todo, idx);
                    save_dataset(String::from("dataset.json"), &dataset);
                }
                else if action.trim_end().to_lowercase() == "edit"{
                    println!("Here is your current todo list: {:?}", &x.todo);
                    println!("Please enter the number which you want to edit!");
                    let mut idx = String::new();
                    io::stdin().read_line(&mut idx).expect("failed to read the data");
                    idx = idx.trim_end().to_string();
                    let idx = idx.parse::<i32>().unwrap() - 1;
                    let idx = idx.try_into().unwrap();
                    println!("Please enter the new todo!");
                    let mut content = String::new();
                    io::stdin().read_line(&mut content).expect("failed to read the data");
                    content = content.trim_end().to_string();
                    edit_content(&mut x.todo, idx, content);
                    save_dataset(String::from("dataset.json"), &dataset);
                }
                else if action.trim_end().to_lowercase() == "empty"{
                    println!("Here is your current todo list: {:?}", &x.todo);
                    empty_todo(&mut x.todo);
                    save_dataset(String::from("dataset.json"), &dataset);

                }
                else{
                    println!("Please select the right opration!");
                }
            }

            else {
                println!("Password is wrong");
            }
        },
        None => {
            println!("Wrong username or user doesn't exist");
            println!("Do you want to create a new user? press Y or N");
            let mut new_user = String::new();
            io::stdin().read_line(&mut new_user).expect("failed to read the data");

            if new_user.trim_end().to_lowercase() == "y" {
                println!("Please enter username:- ");
                let mut new_username = String::new();
                io::stdin().read_line(&mut new_username).expect("failed to read the data");
                new_username = new_username.trim_end().to_string();
                let new_user = dataset.get(&new_username);
                match new_user {
                    Some(_) => println!("{:?} this user already exist, Please provide different username!", &new_username),
                    None => {
                        println!("Please enter the password");
                        let mut password = String::new();
                        io::stdin().read_line(&mut password).expect("failed to read the data");
                        add_user(&mut dataset, new_username, password.trim_end().to_string());
                        save_dataset(String::from("dataset.json"), &dataset);
                        println!("user created succssefully!");
                    }
                }
            }
            else {
                println!("Please enter a valid option");
            }
        },
    }
}
