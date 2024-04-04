fn main() {
    let string1 = String::from("abcd");
    let strig2 = "xyz";

    let result = longest(string1.as_str(), string2);
    println!("The longest string is {}", result);
}
