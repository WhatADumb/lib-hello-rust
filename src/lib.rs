#[cfg(feature="greeting")]
pub fn saying_hello(name: &str) -> String{
    format!("Hello {}", name)
}

#[cfg(feature="farewell")]
pub fn saying_bye(name: &str) -> String{
    format!("Good Bye {}", name)
}

#[cfg(feature="communication")]
pub fn hello_and_bye(name: &str){
    println!("Hello {}, hope we meet again someday...", name);
}