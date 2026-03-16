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
