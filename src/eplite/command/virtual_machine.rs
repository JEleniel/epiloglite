/// Virtual machine - executes bytecode

use super::code_generator::{Instruction, Opcode, PreparedStatement};
use crate::eplite::error::{Error, Result};

/// Register in the virtual machine
#[derive(Debug, Clone)]
pub enum Register {
	Null,
	Integer(i64),
	Real(f64),
	Text(String),
	Blob(Vec<u8>),
}

/// Virtual machine state
pub struct VirtualMachine {
	registers: Vec<Register>,
	program_counter: usize,
}

impl VirtualMachine {
	pub fn new() -> Self {
		VirtualMachine {
			registers: vec![Register::Null; 256],
			program_counter: 0,
		}
	}

	/// Execute a prepared statement
	pub fn execute(&mut self, statement: &PreparedStatement) -> Result<Vec<Vec<Register>>> {
		let mut results = Vec::new();
		self.program_counter = 0;

		while self.program_counter < statement.instructions.len() {
			let instruction = &statement.instructions[self.program_counter];
			
			match self.execute_instruction(instruction) {
				Ok(ExecutionResult::Continue) => {
					self.program_counter += 1;
				}
				Ok(ExecutionResult::Jump(addr)) => {
					self.program_counter = addr;
				}
				Ok(ExecutionResult::Halt) => {
					break;
				}
				Ok(ExecutionResult::Yield(row)) => {
					results.push(row);
					self.program_counter += 1;
				}
				Err(e) => return Err(e),
			}
		}

		Ok(results)
	}

	fn execute_instruction(&mut self, instruction: &Instruction) -> Result<ExecutionResult> {
		match instruction.opcode {
			Opcode::Init => Ok(ExecutionResult::Continue),
			Opcode::Halt => Ok(ExecutionResult::Halt),
			_ => Err(Error::NotSupported(format!(
				"Opcode {:?} not yet implemented",
				instruction.opcode
			))),
		}
	}
}

impl Default for VirtualMachine {
	fn default() -> Self {
		Self::new()
	}
}

enum ExecutionResult {
	Continue,
	Jump(usize),
	Halt,
	Yield(Vec<Register>),
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::eplite::command::code_generator::P4Type;

	#[test]
	fn test_vm_creation() {
		let vm = VirtualMachine::new();
		assert_eq!(vm.registers.len(), 256);
		assert_eq!(vm.program_counter, 0);
	}

	#[test]
	fn test_execute_halt() {
		let mut vm = VirtualMachine::new();
		let stmt = PreparedStatement {
			instructions: vec![Instruction {
				opcode: Opcode::Halt,
				p1: 0,
				p2: 0,
				p3: 0,
				p4: P4Type::None,
				p5: 0,
			}],
		};
		let result = vm.execute(&stmt);
		assert!(result.is_ok());
	}
}
