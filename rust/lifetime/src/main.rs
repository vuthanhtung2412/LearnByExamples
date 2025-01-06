fn main() {
    let mut a: &str = &String::from("aa");
    // Normally ths line above wouldn't work because String::from() create a String internally and
    // return ownership
    // However, the `let` construct allow the life time to be extended as long as the liveness of the scompe
    // The one below breaks 
    // a  = &String::from("aaaa");
 
    // But this doesn't
    // let mut a : &str = &String::from("aaaa");

    // but the one below works
    // TODO: my guess is shadowing bypass borrow checker
    // let mut a: &str = &String::from("aa");
    // a  = &String::from("aaaa");
    // let mut a : &str = &String::from("aaaa");

    let mut c: &str;

    // This will cause an error since num doesn't live long enough
    // let num = 0;
    // let num_pointer: &'static i32 = &num;

    {
        // this still works because the lifetime of "bbb" is static
        let b: &str = "bbb";

        // the one below will fail
        // because *c get deallocated as soon as the scope end
        // let s = String::from("bbb");
        // The binding below also determine the lifetime `b,
        // which is the same as the underlying value which ends when deallocation happen (end of the scope)
        // let b: &str = &s; 
       
        // 
        // The lifetime calculation will proceed like below
        c = b;                   // ---------+-- 'c = `b
        println!("{}", c);       //          |
        c = longest(a, b); //          +-- 'c = `b U 'a
    }                            //          +-- the liveness continue since the current value of c is used in the `longest()` function below
                                 //          |
    a = "aaaa";                  //          |
    c = longest(c, a);     //          +-- 'c = `c U 'a = `b U 'a U 'c = `b U 'a
    println!("{}", c);           // ---------+
}

// The function below is the same as
// fn longest<'a>(x: &'a str, y: &'a str) -> &'a str
fn longest<'a, 'b, 'c>(x: &'a str, y: &'b str) -> &'c str
where
    'a: 'c,
    'b: 'c,
{
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
