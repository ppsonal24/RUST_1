_____________________________________________________________________________________

#[derive(Debug , PartialEq)]
struct AlwaysEqual ;
fn no_data(cs_type : AlwaysEqual)-> AlwaysEqual{
    println!("Custom data type is {:?}" , cs_type) ;
    return cs_type ;
}
fn main(){
    let re : AlwaysEqual = no_data(AlwaysEqual) ;
    assert_eq!(re , AlwaysEqual) ;
    println!("Both of them should be same !") ;
}

i have to use partialeq because i want to compare by assert_eq!()
  
