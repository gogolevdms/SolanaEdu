///-------------------------------------------------------------------------------
///
/// This is your calculator implementation task 
/// to practice enums, structs, and methods.
/// 
/// Complete the implementation of the Calculator struct and its methods.
/// 
/// The calculator should support basic arithmetic 
/// operations (addition, subtraction, multiplication)
/// with overflow protection and maintain a history 
/// of operations.
/// 
/// Tasks:
/// 1. Implement the OperationType enum methods
/// 2. Implement the Operation struct constructor
/// 3. Implement all Calculator methods
/// 
///-------------------------------------------------------------------------------

#[derive(Clone)]
pub enum OperationType {
    Addition,
    Subtraction,
    Multiplication
}

impl OperationType {
    // TODO: Return the string representation of the operation sign
    // Addition -> "+", Subtraction -> "-", Multiplication -> "*"
    // Done
    pub fn get_sign(&self) -> &str {
        match self {
            OperationType::Addition => "+",
            OperationType::Subtraction => "-",
            OperationType::Multiplication => "*",
        }
    }
    
    // TODO: Perform the operation on two i64 numbers with overflow protection
    // Return Some(result) on success, None on overflow
    //
    // Example: OperationType::Multiplication.perform(x, y)
    // Done
    pub fn perform(&self, x: i64, y: i64) -> Option<i64> {
        match self {
            OperationType::Addition => x.checked_add(y),
            OperationType::Subtraction => x.checked_sub(y),
            OperationType::Multiplication => x.checked_mul(y),
        }
    }
}

#[derive(Clone)]
pub struct Operation {
    pub first_num: i64,
    pub second_num: i64,
    pub operation_type: OperationType
}

impl Operation {
    // TODO: Create a new Operation with the given parameters
    // Done
    pub fn new(first_num: i64, second_num: i64, operation_type: OperationType) -> Self {
        Self {
            first_num,
            second_num,
            operation_type
        }
    }
}

pub struct Calculator {
    pub history: Vec<Operation>
}

impl Calculator {
    // TODO: Create a new Calculator with empty history
    // Done
    pub fn new() -> Self {
        Self {
            history: Vec::new(),
        }
    }
    
    // TODO: Perform addition and store successful operations in history
    // Return Some(result) on success, None on overflow
    // Done
    pub fn addition(&mut self, x: i64, y: i64) -> Option<i64> {
        let result = OperationType::Addition.perform(x, y)?;
        
        let operation = Operation::new(x, y, OperationType::Addition);
        
        self.history.push(operation);
        
        Some(result)
    }
    
    // TODO: Perform subtraction and store successful operations in history
    // Return Some(result) on success, None on overflow
    // Done
    pub fn subtraction(&mut self, x: i64, y: i64) -> Option<i64> {
        let result = OperationType::Subtraction.perform(x, y)?;
        
        let operation = Operation::new(x, y, OperationType::Subtraction);
        
        self.history.push(operation);
        
        Some(result)
    }
    
    // TODO: Perform multiplication and store successful operations in history
    // Return Some(result) on success, None on overflow
    // Done
    pub fn multiplication(&mut self, x: i64, y: i64) -> Option<i64> {
                let result = OperationType::Multiplication.perform(x, y)?;
        
        let operation = Operation::new(x, y, OperationType::Multiplication);
        
        self.history.push(operation);
        
        Some(result)
    }
    
    // TODO: Generate a formatted string showing all operations in history
    // Format: "index: first_num operation_sign second_num = result\n"
    //
    // Example: "0: 5 + 3 = 8\n1: 10 - 2 = 8\n"
    // Done
    pub fn show_history(&self) -> String{
        let mut history_string = String::new();
        
        for (index, operation) in self.history.iter().enumerate() {

            let result = operation.operation_type
                .perform(operation.first_num, operation.second_num)
                .unwrap();
            
            let operation_line = format!(
                "{}: {} {} {} = {}\n",
                index,
                operation.first_num,
                operation.operation_type.get_sign(),
                operation.second_num,
                result
            );
            
            history_string.push_str(&operation_line);
        }
        
        history_string
    }
    
    // TODO: Repeat an operation from history by index
    // Add the repeated operation to history and return the result
    // Return None if the index is invalid
    // Done
    pub fn repeat(&mut self, operation_index: usize) -> Option<i64>{
        let operation = self.history.get(operation_index)?.clone();
        
        let result = operation.operation_type
            .perform(operation.first_num, operation.second_num)?;
        
        self.history.push(operation);
        
        Some(result)
    }
    
    // TODO: Clear all operations from history
    // Done
    pub fn clear_history(&mut self) {
        self.history.clear();
    }
}
