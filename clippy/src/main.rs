#![warn(clippy::all, clippy::pedantic)]
fn main() {
    // let MYLIST = ["One", "Two", "Three"];
    // for i in 0..MYLIST.len() {
    //     println!("{}: {}", i, MYLIST[i]);
    // }
    let my_list = ["One", "Two", "Three"];
    for (i, num) in my_list.iter().enumerate(){
        // println!("{}: {}", i, num);
        println!("{i}: {num}");
    }
}
