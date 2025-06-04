// Fix the code so that it compiles.

fn main() {
    let mut s = String::from("Hello, ");
    let s_ref = &mut s;
    change_string(s_ref);
    println!("{s_ref}");
}

fn change_string(s: &mut String) -> &String{
    s.push_str(" world!");
    s
}