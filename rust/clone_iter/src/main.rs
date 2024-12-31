use std::fmt::Display;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() -> io::Result<()> {
    // Example 1: Cloning an iterator from a vector
    let vec = vec![1, 2, 3, 4, 5];
    let iter1 = vec.iter(); // Create an iterator over the vector
    let mut iter2 = iter1.clone(); // Clone the iterator

    println!("Iterating with the original iterator:");
    for val in iter1 {
        println!("{}", val);
    }

    println!("\nIterating with the cloned iterator:");
    for val in iter2 {
        println!("{}", val);
    }

    println!("---File reader example---");
    // Example 2: Cloning an iterator from a file reader
    let current_file_path = Path::new(file!());
    let path = current_file_path.parent().unwrap().join("./sample.txt");
    let file = File::open(&path)?;
    let buf_reader = io::BufReader::new(file);
    let line_reader1 = buf_reader.lines();

    // Lines is a non clonable iterator
    // let mut line_reader2 = line_reader1.clone();

    for line in line_reader1 {
        // The example below will cause daggling pointer
        // `line?`` will return a String owned by the `?` function or an Error
        // split_whitespace take a &str as an input a return an iterator
        // this iterator point to the underlying text with a &str
        // However the `line` is moved to the `?` function -> dangling pointer

        // 1st Example
        // let words1 = line?.split_whitespace();
        // let words2 = words1.clone();

        // 2nd Example
        // let s = String::from("hello").as_str(); // Error: temporary String dropped while borrowed
        // println!("{}", s)
        // println!("{}", words.next().unwrap())

        // // Bind the owned String to a variable
        let line = line?;
        // Now, split_whitespace borrows from `line`, which has a proper lifetime
        let mut words1 = line.split_whitespace();
        clone_iterator(&words1);
        println!("1st word: {}", words1.next().unwrap());

        // If you need to clone the iterator:
        let words2 = words1.clone();
        clone_iterator(&words1);
        println!("Other words:");
        for w in words2 {
            print!("{} ", w)
        }
        println!();
        println!("---")
    }

    // Create an iterator over lines

    Ok(())
}

// This doesn't work because it for the iterator to has the same lifetime as the &str element
// which makes `let mut words1 = line.split_whitespace();` has the same lifetime as `let line`
// -> borrows not terminated after the scope of clone_iterator but at the same time as `let line`
// -> clash with mutable ref `words1.next().unwrap()`
// fn clone_iterator<'a, I>(iter: &'aI)
// where
//     I: Iterator<Item = &'a str> + Clone,
fn clone_iterator<'a, I>(iter: &I)
where
    I: Iterator<Item = &'a str> + Clone,
{
    println!("### clone_iterator ###");
    for e in iter.clone() {
        print!("{} ", e);
    }
    println!();
    println!("###")
}
