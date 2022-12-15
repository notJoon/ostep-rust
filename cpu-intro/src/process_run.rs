use std::collections::HashMap;

/// process switch behavior
/// SCHED_SWITCH_ON_IO = 'SWITCH_ON_IO'
/// SCHED_SWITCH_ON_END = 'SWITCH_ON_END'
#[derive(PartialEq, Eq, Hash)]
#[warn(dead_code)]
enum ScheduleSwitchBehavior {
    SwitchOnIO,
    SwitchOnEnd,
}

/// I/O finished behavior
#[derive(PartialEq, Eq, Hash)]
#[warn(dead_code)]
enum IORunBehavior {
    Later,
    Immediate,
}

/// process states
#[derive(Eq, PartialEq, Hash)]
#[warn(dead_code)]
enum ProcessState {
    Running,
    Ready,
    Done,
    Wait,
}

/// members of process structure
/// PROC_CODE = 'code_'
/// PROC_PC = 'pc_'
/// PROC_ID = 'pid_'
/// PROC_STATE = 'proc_state_'
#[derive(Eq, PartialEq, Hash)]
#[warn(dead_code)]
enum ProcessStructure {
    Code,
    PC,
    ID,
    State,
}

#[warn(dead_code)]
enum ProcessStructureValue {
    Code(Vec<ProcessAction>),
    PC(i32),
    ID(usize),
    State(ProcessState),
}

/// things a process can do
/// DO_COMPUTE = 'cpu'
/// DO_IO = 'io'
/// DO_IO_DONE = 'io_done'
#[derive(Eq, PartialEq, Hash)]
#[warn(dead_code)]
enum ProcessAction {
    Compute,
    IO,
    IODone,
}

#[warn(dead_code)]
struct Scheduler {
    proc_info: HashMap<usize, HashMap<ProcessStructure, usize>>,
    process_switch_behavior: ScheduleSwitchBehavior,
    io_done_behavior: IORunBehavior,
    io_length: usize,
    curr_proc: i32,
}

impl Scheduler {
    fn new(
        self,
        process_switch_behavior: ScheduleSwitchBehavior, 
        io_done_behavior: IORunBehavior, 
        io_length: usize
    ) -> Self {
        let process_switch_behavior = process_switch_behavior;
        let io_done_behavior = io_done_behavior;
        let io_length = io_length;
        let curr_proc = 0;

        Self {
            proc_info: HashMap::new(),
            process_switch_behavior,
            io_done_behavior,
            io_length,
            curr_proc,
        }
    }

    fn new_process(&mut self) -> usize {
        let process_id = self.proc_info.len();
        let mut process_data: HashMap<ProcessStructure, ProcessStructureValue> = HashMap::new();
        process_data.insert(ProcessStructure::PC, ProcessStructureValue::PC(0));
        process_data.insert(ProcessStructure::ID, ProcessStructureValue::ID(process_id));
        process_data.insert(ProcessStructure::State, ProcessStructureValue::State(ProcessState::Ready));
        process_data.insert(ProcessStructure::Code, ProcessStructureValue::Code(Vec::new()));

        process_id
    }

    // program looks like this:
    //     c7,i,c1,i
    // which means compute for 7, then i/o, then compute 1, then i/o
    fn load_program(&mut self, program: String) -> Result<(), &str> {
        let proc_id = self.new_process();

        for token in program.split(',') {
            let opcode = token.chars().nth(0);
            match opcode {
                // compute
                Some('c') => {
                    let num = token[1..].parse::<i32>();
                    for _ in num {
                        unimplemented!()
                    }
                },
                // input I/O
                Some('i') => {
                    unimplemented!()
                },
                _ => {
                    panic!("invalid opcode")
                }
            }
        }
        Err("error")
    }

    fn load(&mut self, desc: String) {
        let proc_id = self.new_process();
        let tmp: Vec<&str> = desc.split(":").collect();
        if tmp.len() != 2 {
            println!("Bad description {}: must be number `<x:y>", desc);
            println!("    where `X` is the number of instructions");
            println!("    and `Y` is the percent change that an instruction is CPU not IO.");
            
        }

        let num_instructions = tmp[0];
        let cpu_chance = tmp[1];
    }

    fn move_to_ready(mut self, mut pid: i32, expected: &str) { 
        if pid == -1 {
            pid = self.curr_proc;
        }
    }

    fn move_to_wait(self) {
        unimplemented!()
    }

    fn move_to_running(self) {
        unimplemented!()
    }

    fn move_to_done(self) {
        unimplemented!()
    }

    fn next_proc(self, pid:i32) {
        unimplemented!()
    }

    fn get_num_processes(&self) -> usize {
        self.proc_info.len()
    }

    fn get_num_instructions(self) {
        unimplemented!()
    }

    fn get_instruction(self) {
        unimplemented!()
    }

    fn get_num_active(self) -> usize {
        unimplemented!()
    }

    fn get_ios_in_flight(self) -> usize {
        unimplemented!()
    }

    fn check_for_switch(self) -> () {
        ()
    }

    fn space(self, cols: i32) {
        unimplemented!()
    }

    fn check_if_done(self) {
        unimplemented!()
    }

    fn run(self) {
        unimplemented!()
    }
}
