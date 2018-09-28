

fn do_something<F>(closure: F) where F : Fn(String) -> String {
    print!("{}", closure("x".to_string()));
}


fn main() {

    let c = move |x| x;

    do_something(c);
}
