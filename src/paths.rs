mod simple_paths;
use simple_paths::front_house;

mod super_path;
use super_path::back_of_house;

mod external_paths;
use external_paths::use_external_resources;

pub fn paths_main(){
    simple_paths::eat_at_restaurant();
    back_of_house::fix_incorrect_order();
    use_external_resources::external_resources();
}