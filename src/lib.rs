pub fn saying_hello(name: &str) -> String{
    format!("Hello {}", name)
}

pub fn saying_bye(name: &str) -> String{
    format!("Good Bye {}", name)
}

pub fn hello_and_bye(name: &str){
    println!("Hello {}, hope we meet again someday...", name);
}