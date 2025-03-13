use std::io;

struct User{
    username:String,
    age:Option<u8>,
    email:Option<String>,
    active:bool,
}
impl User{
    fn set_age(&mut self,age:u8){
        self.age = Some(age);
    }
    fn set_email(&mut self,email:String){
        self.email = Some(email);
    }
    fn unset_age(&mut self){
        self.age = None;
    }
    fn unset_email(&mut self){
        self.email = None;
    }
    fn is_complete_profile(&self)-> bool{
        self.age.is_some()&&self.email.is_some()
    }
    fn print_user_profile(&self){
        println!("User Profile: \n name: {}, \n age: {} \n email: {} \n active: {} \n complete:{}\n", self.username,self.age.unwrap_or(0), self.email.as_ref().unwrap_or(&String::from("not provided")),self.active, self.is_complete_profile());
    }
}

fn main() {
    let mut users:Vec<User> = Vec::new();
    
    loop {
        let mut input = String::new();

        println!("1 to list users");
        println!("2 to add a user");
        println!("3 to select an existing user");
        io::stdin().read_line(&mut input).expect("Failed");
        input=input.trim().to_string();
        if (input == "1") {
            input.clear();
            users.iter().for_each(|u|  u.print_user_profile());
        } else if input == "2" {
            input.clear();
            let mut username: String = String::new();
            let mut age: Option<u8> = None;
            let mut email: Option<String> = None;
            let active: bool = false;

            println!("please enter a username");
            io::stdin().read_line(&mut username).expect("Failed");
            username = username.trim().to_string();
            input.clear();
            println!("please enter a age");
            io::stdin()
                .read_line(&mut input)
                .expect("Failed to read input");
            age = input.trim().parse::<u8>().ok();
            input.clear();
            println!("please enter a email");
            io::stdin()
                .read_line(&mut input)
                .expect("Failed to read input");
            email = Option::from(input.trim().to_string());
            input.clear();
            let user = User {
                username,
                age,
                active,
                email,
            };
            users.push(user);
        }
    }
}
