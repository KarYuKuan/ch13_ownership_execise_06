fn main() {
    let s1 = give_ownership_back();

    println!("----------the value of s1 : {s1}---------");

    let s2 = String::from("goodye");

    let s3 = take_and_give_back(s2);

    println!("----------the value of s1 : {s3}---------");

    //
    println!("----------the value of s1 : {s2}---------");


}

fn give_ownership_back ()->String{
    let some = String::from("Hello");
    return some
}

fn take_and_give_back(anything:String)->String{
    return anything
}
