fn main() {
    let mut a: &str = "aa";
    let mut c: &str;

    {
        // this still works because the lifetime of "ccc" is static
        let b: &str = "ccc";

        // the one below will fail
        // because *c get deallocated as soon as the scope end
        // let s = String::from("ccc");
        // The binding below also determine the lifetime `b,
        // which is the same as the underlying value which ends when deallocation happen (end of the scope)
        // let b: &str = &s; 
       
        // 
        // The lifetime calculation will proceed like below
        c = b;                   // ---------+-- 'c = `b
        println!("{}", c);       //          |
        c = longest(a, b); //          +-- 'c = `b U 'a
    }                            //          |
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
