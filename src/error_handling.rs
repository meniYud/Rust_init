
mod unrecoverables;
use unrecoverables::panicing;

mod recoverables;
use recoverables::result_enum;



pub fn error_handling_main(){
    // panicing()
    result_enum()
}