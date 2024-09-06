use std::fmt::Result;
use std::io::Result as IoResult;

fn collisions() {
    fn function1() -> Result {
        Ok(())
    }

    fn function2() -> IoResult<()> {
        Ok(())
    }

    let r1 = function1();
    let r2 = function2();
    println!("{:?}", r1);
    println!("{:?}", r2);
}

fn external_import(){
    use rand::Rng; // cargo add rand
    let secret_number = rand::thread_rng().gen_range(1..=100);
    println!("The secret number is: {}", secret_number);
}

pub mod use_external_resources{
    pub fn external_resources(){
        super::collisions();
        super::external_import();
    }
}
