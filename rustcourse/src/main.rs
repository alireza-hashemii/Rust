//? What is the difference between print! and println!?
//? Newline and special characters like \n \t \r
//? \r is called cariage return and it simply ignores the text comes before it.
//? \n works exacly like python. whenever code reaches a \n it means to carry on from a newline
//? \t provides one tab of space
fn main() {
    print!("the next line comes exacly after this one. -> \t");
    println!("we are doing sth");
    println!("{}", 45);
    print!("my name is {} and i'm {} years old","Alireza", 20);
    print!("test if \n works well, and let's see about \r which does a good job");
    print!("{name} is here and trying to {verb} in his job", 
    name="alireza",
    verb= "excel"
    )
}
