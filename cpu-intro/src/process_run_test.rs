use crate::process_run::Scheduler;

use crate::seed;

#[cfg(test)]
mod tests {
    use crate::process_run::{Scheduler, ProcessState};

    #[test]
    fn test_load_program() {
        let program = "c7,i,c1,i,c33,i";
        for inst in program.split(',') {
            let opcode = inst.chars().nth(0).unwrap();
            println!("opcode: {}", opcode);
            match opcode {
                'c' => {
                    let reg = inst[1..].parse::<i32>().unwrap();
                    println!("reg: {}", reg)
                },
                'i' => {
                    println!("immediate");
                },
                _ => {
                    panic!("invalid opcode");
                }
            }
        }
    }

    #[test]
    fn test_new_process_basic() {
        let mut manager = Scheduler::new();
        let proc_id = manager.new_process();

        assert_eq!(proc_id, 0);
        assert_eq!(manager.proc_info.len(), 1);
        assert_eq!(manager.proc_info[&proc_id].proc_id, 0);
        assert_eq!(manager.proc_info[&proc_id].proc_pc, 0);
        assert_eq!(manager.proc_info[&proc_id].proc_code.len(), 0);
        assert_eq!(manager.proc_info[&proc_id].proc_state, ProcessState::Ready);
    }
}