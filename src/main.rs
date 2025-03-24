use std::collections::HashMap;

fn users_to_hashmap(users: Vec<User>) -> HashMap<String, i32> {
    let mut hm = HashMap::new();
    for user in users {
        hm.insert(user.name, user.age);
    }
    return hm;
}

struct User {
    name: String,
    age: i32,
}

fn main() {
    // Using User struct
    let users = vec![
        User { name: String::from("myname1"), age: 21 },
        User { name: String::from("myname2"), age: 22 },
        User { name: String::from("myname3"), age: 23 },
    ];
    
    let user_hm = users_to_hashmap(users);
    println!("User HashMap: {:?}", user_hm);
}