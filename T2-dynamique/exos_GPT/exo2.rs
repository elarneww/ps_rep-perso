enum Operation {
    Add,
    Subtract,
    Multiply,
    Divide,
}

fn calculate(op: Operation, a: i32, b: i32) -> f32 {
    match op {
        Operation::Add => (a + b) as f32,
        Operation::Subtract => (a - b) as f32,
        Operation::Multiply => (a * b) as f32,
        Operation::Divide => a as f32 / b as f32, // division flottante
    }
}

fn main() {
    let result = calculate(Operation::Divide, 6, 7);
    println!("{}", result); // 0.85714287
}
