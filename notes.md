# Notes


## Description


## Project Folder Structure

src/
├── intercept.c
├── libintercept.so
├── lib.rs
├── main.rs
├── parser.rs
├── utils.rs
test/
├── main
└── main.c


## Dependencies


## Utils

### Structs

- VariablePointer



## Static Analyser

### Structs

#### CodeParser

#### Members
- variables: HashMap<String, Variable>
#### Functions
- new() -> Self
- add_new_variable(&mut self, var_name: String, size: usize, var_type: String)
- parse_code(&mut self, entity: &Entity<'_>) {

## Dynamic Analyser

### Structs

- 

