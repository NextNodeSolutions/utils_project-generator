use std::cell::RefCell;
use std::collections::HashMap;

thread_local! {
    static VARIABLES: RefCell<HashMap<String, String>> = RefCell::new(HashMap::new());
}

pub fn set_variables(vars: HashMap<String, String>) {
    VARIABLES.with(|v| *v.borrow_mut() = vars);
}

pub fn get_variable(key: &str) -> Option<String> {
    VARIABLES.with(|v| v.borrow().get(key).cloned())
}
