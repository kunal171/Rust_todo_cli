use serde::{Serialize, Deserialize};
use std::collections::HashMap;


#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub password: String,
    pub todo: Vec<String>,
}

pub fn add_user(data: &mut HashMap<String, User>, username: String, password: String) {
    let user_data = User {
        password: password,
        todo: vec![],
    };
    data.insert(username, user_data);
}

// pub fn add_content(data: &mut HashMap<String, User>, username: String, content: String){
//     let user = data.get(&username).unwrap();
//     let todo = &mut user.todo.clone();
//     todo.push(content);
//     let user_data = User {
//         password: user.password.clone(),
//         todo: todo.to_vec(),
//     };
//     data.insert(username, user_data);
//     println!("Your new todo list: {:?}", &todo);
// }

pub fn add_content(todo: &mut Vec<String>, content: String){
    todo.push(content);
    println!("Your new todo list: {:?}", &todo);
}

pub fn delete_content(todo: &mut Vec<String>, idx: usize){
    todo.remove(idx);
    println!("Your new todo list: {:?}", &todo);
}

pub fn edit_content(todo: &mut Vec<String>, idx: usize, content: String){
    todo[idx] = content;
    println!("Your new todo list: {:?}", &todo);
}


pub fn empty_todo(todo: &mut Vec<String>){
    *todo = Vec::new();
    println!("Your todo list is empty!");
}