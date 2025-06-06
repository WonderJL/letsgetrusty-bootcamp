fn main() {
    let i_woke_up_late = true;
    morning_routine(i_woke_up_late);
}

fn morning_routine(i_am_late: bool) {
    println!("This morning, I...");
    if i_am_late {
        go_to_work();
        return;
    }
    exercise();
    eat_breakfast();
    make_coffee();
    go_to_work();
}

fn exercise() {
    println!("I went to the gym.");
}

fn eat_breakfast() {
    println!("I had a healthy breakfast!");
}

fn make_coffee() {
    println!("I made myself coffee. Now that I'm ready..."); 
}

fn go_to_work() {
    println!("I went straight to work!");
}
