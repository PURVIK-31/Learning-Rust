// time to learn structs in rust : 

struct Naam{
    firstname: String,
    lastname:String,
    rollno: i32,
}

impl Naam {
    //&self means that the method is taking a reference to the struct instance and it is present for the insatace of the struct, not the type.
    fn display(&self){
        print!("{} {} {}",self.firstname,self.lastname,self.rollno);
        }
    fn printsmthing() {
        print!("Hey from somthing !")
    }
    }

fn main(){
   let n = Naam {
    firstname: "Rahul".to_string(),
    lastname: "Kumar".to_string(),
    rollno: 123,
   };
    n.display();
        Naam::printsmthing();
}