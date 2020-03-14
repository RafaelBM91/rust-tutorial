pub mod mod_one {

    use crate::module::mod_two::index::mod_two::function as function_two;
    
    use crate::module::mod_one::index::mod_one_two::function as function_one_two;

    pub fn function (value: &'static str) {
        println!("{:?}", value);
    }

    pub fn call_function_two (value: &'static str) {
        function_two(value);
    }

    pub fn call_function_one_two (value: &'static str) {
        function_one_two(value);
    }
}

pub mod mod_one_two {
    
    pub fn function (value: &'static str) {
        println!("{:?}", value);
    }

}
