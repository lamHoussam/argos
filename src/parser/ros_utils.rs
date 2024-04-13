

// TODO: Replace var_type with sizeof_type
#[derive(Debug)]
pub struct Variable {
    pub name: String,
    pub size: usize,
    pub max_bounds_checked: bool,
}

impl Variable {
    pub fn new(var_name: String, var_size: usize) -> Self {
        Variable {
            name: var_name,
            size: var_size,
            max_bounds_checked: false,
        }
    }
}
