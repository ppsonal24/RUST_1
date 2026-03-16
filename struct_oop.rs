_____________________________________________________
#[derive(Debug)]
struct User{
    name : String ,
    age : i32 ,
    index : usize ,
    email : String ,
}
fn build_user(name : String ,email : String)-> User{
    User{
        name : name ,
        age : 120 ,
        index : 32 ,
        email : email ,    
    }
}
fn main(){
    let per1 = build_user( 
        String::from("Kohee") ,
        String::from("Someone@gmail.com") ,   
    ) ;
    let per2 : User = User{
        name : per1.name.clone() ,
        age : per1.age ,
        index : 12 ,
        email : per1.email.clone() ,
    } ;
    println!("Person1 is {:?}" , per1) ;
    println!("Person2 is {:?}" , per2) ;
}
_________________________________________
struct Info{
    len : i32 ,
    wid : i32 ,
}
fn calculate_area(len : i32 , wid : i32)-> i32{
    return len * wid ;
}
fn main(){
    let info : Info = Info{
        len : 120 ,
        wid : 32 ,
    } ;
    let result : i32 = calculate_area(info.len , info.wid) ;
    println!("The area is {:?}" , result) ;
}
____________________________________________________________________________________
#[derive(Debug)]
struct Num{
    num1 : i32 ,
    num2 : i32 ,
}
fn main(){
    let num : Num = Num{
        num1 : 12 ,
        num2 : 32 ,
    } ;
    dbg!(("Num is {:?}" , num)) ;
}
using dbg like touple (()) 
dbg!("Num is {:?}" , num)    ---> is worng way ;
dbg!(("Num is {:?}" , num)) ----> correct way ; 
