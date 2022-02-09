use video::greet;

fn main() {
    // variables
    // let mut[Optional] VarName: dataType[Optional] = value
    let var1 = 10; // automatically assigned type
    let var2:i16 = 10; // manually assigned type
    let mut var3:i16 = 10;
    // here mut stands for mutable 
    // generally in rust all variables are im-mutable
    // because for thread safety and speed
    
    // also we can use const like C
    // const VAR_NAME:datatype = CONST_VALUE;
    const VAR:i32 = 100;

    let (x, y) = (10, 30);
    {
        let z = 40;
        println!("value of Z = {}", z);
    }
    // Error
    // println!("value of Z = {}", z);
    // bcz out of scope, in rust when a variable goes out of scope
    // its actually deleted form memory



    greet();
}

// fn function_name(variable_name:datatype) -> return_type {}
fn test() { // void return
    println!("called test function");
} 

fn mul(x:i32, y:i32) -> i32{
    // return x*y; 
    x * y;
}

// to use any module inside same package/module
// use full scope or use namespace
// to use namespace just `use module_name::func/struct/name etc` like C++