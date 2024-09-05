mod vars;
use vars::vars;

mod data_types;
use data_types::data_types;

mod statements_vs_expressions;
use statements_vs_expressions::statements_vs_expressions;

mod flow_control;
use flow_control::flow_control;

mod ownership;
use ownership::ownership;
use ownership::references;
use ownership::slices;

mod structs;
use structs::structs;
use structs::struct_factory;
use structs::tuples_struct;
use structs::structs_in_use;

mod enums;
use enums::enums;
use enums::enum_methods;
use enums::optionals;

mod matcher;
use matcher::matcher;
use matcher::if_let;



fn main(){
    // vars();
    // data_types();
    // statements_vs_expressions()
    // flow_control()

    // ownership()
    // references()
    // slices();

    // structs()
    // struct_factory()
    // tuples_struct();
    // structs_in_use()

    // enums();
    // enum_methods();
    // optionals();

    // matcher();
    // if_let()
}
