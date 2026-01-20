//! Test module with intentional errors and warnings for testing get_workspace_issues
//!
//! This module is private and should not affect the main crate compilation.

#![allow(dead_code)]  // Allow unused code warnings for test purposes

/// Module with intentional compilation errors
mod errors {
    // ERROR 1: Undefined variable
    pub fn error_undefined_variable() {
        let x = undefined_var;  // Error: cannot find value `undefined_var`
    }
    
    // ERROR 2: Type mismatch
    pub fn error_type_mismatch() -> i32 {
        "not an integer"  // Error: expected `i32`, found `&str`
    }
    
    // ERROR 3: Missing field in struct
    pub struct MyStruct {
        pub field1: i32,
        pub field2: String,
    }
    
    pub fn error_missing_field() {
        let _ = MyStruct {
            field1: 42,
            // Error: missing field `field2`
        };
    }
}

/// Module with intentional warnings
mod warnings {
    // WARNING 1: Unused variable
    pub fn warning_unused_variable() {
        let unused = 42;  // Warning: unused variable
        println!("Hello");
    }
    
    // WARNING 2: Unused function
    fn warning_unused_function() {  // Warning: function is never used
        println!("This function is not used");
    }
    
    // WARNING 3: Unreachable code
    pub fn warning_unreachable() -> i32 {
        return 10;
        let x = 5;  // Warning: unreachable statement
        x
    }
}

// Re-export to make it accessible for testing (but module is still private to the crate)
pub use errors::*;
pub use warnings::*;
