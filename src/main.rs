#![feature(proc_macro_hygiene, decl_macro)]

mod module;

fn main() {
    module::file::modules::connection(12);

    module::mod_one::index::mod_one::function("Hi, I am mod_one -> index -> function.");
    
    module::mod_one::index::mod_one::call_function_two("Hi, I am mod_one -> index -> call_function_two.");
    
    module::mod_one::index::mod_one::call_function_one_two("Hi, I am mod_one -> index -> call_function_one_two.");
    
    module::mod_two::index::mod_two::function("Hi, I am mod_two -> index -> function.");
}
