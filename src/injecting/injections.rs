use wasm_instrument::parity_wasm::elements::{
    BlockType, FuncBody, Instruction, Instructions, Local, Module, ValueType,
};

use super::injector::FunctionMapper;

pub fn inject_infinite_loop(module: &mut Module) -> Result<(), String> {
    module.map_function("validate_block", |func_body: &mut FuncBody| {
        let code = func_body.code_mut();

        let mut code_with_loop = vec![
            // Loop never ends
            Instruction::Loop(BlockType::NoResult),
            Instruction::Nop,
            Instruction::Br(0),
            Instruction::End,
        ];
        code_with_loop.append(code.elements_mut());

        *code.elements_mut() = code_with_loop;
    })
}

pub fn inject_jibberish_return_value(module: &mut Module) -> Result<(), String> {
    module.map_function("validate_block", |func_body: &mut FuncBody| {
        *func_body.code_mut() = Instructions::new(vec![
            // Last value on the stack gets returned
            Instruction::I64Const(123456789),
            Instruction::End,
        ]);
    })
}

pub fn inject_stack_overflow(module: &mut Module) -> Result<(), String> {
    module.map_function("validate_block", |func_body: &mut FuncBody| {
        func_body.locals_mut().append(&mut vec![
            // Creating 100 `i64`s should cause the stack to overflow
            Local::new(100, ValueType::I64),
        ]);
    })
}

pub fn inject_noops(module: &mut Module) -> Result<(), String> {
    module.map_function("validate_block", |func_body: &mut FuncBody| {
        // Add half a billion NoOpeartions to (hopefully) slow down interpretation-time
        let code = func_body.code_mut();

        let mut nops = vec![Instruction::Nop; 500_000_000];
        nops.append(code.elements_mut());

        *code.elements_mut() = nops;
    })
}

pub fn inject_heap_overload(module: &mut Module) -> Result<(), String> {
    module.map_function("validate_block", |func_body: &mut FuncBody| {
        *func_body.locals_mut() = vec![];
        *func_body.code_mut() = Instructions::new(vec![
            // Try to allocate 255 pages
            Instruction::GrowMemory(u8::max_value()),
            Instruction::I64Const(0),
            Instruction::End,
        ]);
    })
}
