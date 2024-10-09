mod meditation_timer;

pub fn select_module() {
    // let options: Vec<&str> = vec!["Banana", "Apple", "Strawberry", "Grapes", "Lemon", "Tangerine", "Watermelon", "Orange", "Pear", "Avocado", "Pineapple"];
    // 
    // let ans: Result<&str, InquireError> = Select::new("What's your favorite fruit?", options).prompt();
    // 
    // match ans {
    //     Ok(choice) => println!("{}! That's mine too!", choice),
    //     Err(_) => println!("There was an error, please try again"),
    // }
    meditation_timer::run_meditation_timer();
}
