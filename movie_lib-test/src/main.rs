extern crate movies_lib;

use movies_lib::movies;
use std::fs::File;

fn main() {
    println!("Inide main of test");
    let name = String::from("hello,world");  
    
    movies::play(name);

    //panic
    //let v = vec![1,2,3];P
    //let _value = v[99];

    //file
    /*
    let f = File::open("../hello.txt");

     match f {
        Ok(file)=>file,
        Err(error) => {
            panic!("Problem opening the file {:?}",error);
        }
    };
    */

    let integer = Point{x:5,y:10};
    let float = Point{x:0.5,y:0.1};
    let cha = Point{x:'a',y:'b'};

    println!("cha {}",cha.x);
}

//泛型 
/*
fn largest<T>(list: &[T]) -> T{
    let mut largest = list[0];
    
    for &item in list.iter() {
        if item > largest{
            largest =item;
        }
    }
    largest
}
*/

struct Point<T> {
    x: T,
    y: T,
}

