mod simple_paths;
use simple_paths::front_house;

mod super_path;
use super_path::back_of_house;

pub fn paths_main(){
    simple_paths::eat_at_restaurant();
    back_of_house::fix_incorrect_order()
}