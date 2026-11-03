fn main() {
    let s = "hello"; //string literal 
    let mut s = String::from("hello"); 
    s.push_str(", world!"); //appends literal to a string 
    
    println!("{s}"); 

    takes_ownership(s);

    let x = 5; 
    makes_copy(x); 

    let s1 = gives_ownership();
    let s2 = String::from("hello");
    let s3 = takes_and_gives_back(s2); 
    println!("{s3}");

    let (s4, len) = calculate_length(s1);

    println!("The length of '{s4}' is {len}.");

}// implicitly calls drop here 

fn takes_ownership(some_string: String) {
    println!("{some_string}");
} // drop on whatever arg we used, hence s becomes inaccessible after line 8

fn makes_copy(some_integer: i32){
    println!("{some_integer}")
} //drop on whatever arg we used, hence x becomes inaccessible after line 11

fn gives_ownership() -> String {

    let some_string = String::from("yours"); 

    some_string //returning a value allows this string to be persisted and drop does not execute here 
}

fn takes_and_gives_back(a_string: String) -> String {
    a_string //returning a value allows this string to be persisted and drop does not execute here
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len(); 
    
    (s, length)
}