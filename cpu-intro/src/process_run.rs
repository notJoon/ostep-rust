use std::collections::HashMap;

use crate::{push_process, assign_state, seed};

const DO_COMPUTE: &'static str = "cpu";
const DO_IO: &'static str = "io";
const DO_IO_DONE: &'static str = "io_done";

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum ProcessState {
    Running,
    Ready,
    Blocked,
    Terminated,
}

pub struct ProcessStructure {
    pub proc_id: i32,
    pub proc_pc: usize,
    pub proc_code: Vec<&'static str>, // should find better way to do this
    pub proc_state: ProcessState,
}

pub struct Scheduler {
    pub proc_info: HashMap<i32, ProcessStructure>,
    pub curr_proc: i32,
    io_finish_time: HashMap<i32, i32>,
}

impl Scheduler {
    pub fn new() -> Self {
        Self {
            proc_info: HashMap::new(),
            curr_proc: 0,
            io_finish_time: HashMap::new(),
        }
    }

    pub fn new_process(&mut self) -> i32 {
        let proc_id = self.proc_info.len() as i32;
        let proc_info = ProcessStructure {
            proc_id,
            proc_pc: 0,
            proc_code: Vec::new(),
            proc_state: ProcessState::Ready,
        };
        self.proc_info.insert(proc_id, proc_info);
        proc_id
    } 

    pub fn load_program(&mut self, program: &str) {
        let proc_id = self.new_process();

        for p in program.split(',') {
            let opcode = p.chars().nth(0).unwrap();
            self.check_opcode(opcode, p, proc_id);
        }
    }

    pub fn load(&mut self, program: &str) {
        let proc_id = self.new_process();

        // program is a string of colon-separated instructions (like "5:100")
        // which denotes `5` compute instructions and `100` cpu chances.
        let tmp = program.split(':').collect::<Vec<&str>>();
        if tmp.len() != 2 {
            panic!("Bad description `{}`: Must be integer `x:y`", program)
        }

        let compute = tmp[0].parse::<i32>().unwrap();
        let chances = tmp[1].parse::<i32>().unwrap() as f32 / 100.0;

        for _ in 0..compute {
            if (seed::seed() as f32) < chances {
                push_process!(self.proc_info, proc_id, DO_COMPUTE);
            } else {
                push_process!(self.proc_info, proc_id, DO_IO);
                push_process!(self.proc_info, proc_id, DO_IO_DONE);
            }
        }
    }

    fn move_to_ready(&mut self, pid: i32, expected: ProcessState) {
        if pid == -1 {
            self.curr_proc;
        }
        assert_eq!(self.proc_info[&pid].proc_state, expected);
        assign_state!(
            self.proc_info, 
            pid, 
            ProcessState::Ready
        );
    }

    fn move_to_running (&mut self, expected: ProcessState) {
        assert_eq!(self.proc_info[&self.curr_proc].proc_state, expected);
        assign_state!(
            self.proc_info, 
            self.curr_proc, 
            ProcessState::Running
        );
    }

    fn move_to_done(&mut self, expected: ProcessState) {
        assert_eq!(self.proc_info[&self.curr_proc].proc_state, expected);
        assign_state!(
            self.proc_info, 
            self.curr_proc, 
            ProcessState::Terminated
        );
    }

    fn next_proc(&mut self, pid: i32) {
        if pid != -1 {
            self.curr_proc = pid;
            self.move_to_running(ProcessState::Ready);
        }

        let curr = self.curr_proc + 1;
        let end = self.proc_info.len() as i32;

        self.state_ready_to_running(curr, end);
        self.state_ready_to_running(0, curr);
    }

    pub fn get_num_processes(&self) -> usize {
        self.proc_info.len()
    }

    pub fn get_num_instructions(&self, pid: i32) -> usize {
        self.proc_info[&pid].proc_code.len()
    }

    // pub fn get_instruction(&self, pid: i32, idx: i32) {
    //     self.proc_info[&pid].proc_state
    // }

    pub fn get_num_actives(&self) -> usize {
        let mut result = 0;
        for p in 0..self.proc_info.len() {
            let state = self.proc_info[&(p as i32)].proc_state;

            if state != ProcessState::Terminated {
                result += 1;
            }
        }
        result
    }

    pub fn get_num_runnable(&self) -> usize {
        let mut result = 0;
        for p in 0..self.proc_info.len() {
            let state = self.proc_info[&(p as i32)].proc_state;

            if state == ProcessState::Ready 
            || state == ProcessState::Running {
                result += 1;
            }
        }
        result
    }

    // TODO
    // pub fn get_ios_in_flight(&self, curr_time: i32) -> usize {
    //     let mut flights = 0;

    //     for pid in 0..self.proc_info.len() {
    //         for time in self.io_finish_time[&(pid as i32)] {
    //             if *time >= curr_time {
    //                 flights += 1;
    //             }
    //         }
    //     }
    //     flights
    // }

    fn check_for_switch(self) {
        return
    }

    fn state_ready_to_running(&mut self, start: i32, end: i32) {
        for i in start..end {
            if self.proc_info[&i].proc_state == ProcessState::Ready {
                self.curr_proc = i;
                self.move_to_running(ProcessState::Ready);
                return;
            }
        }
    }

    fn check_opcode(&mut self, opcode: char, program: &str, proc_id: i32) {
        match opcode {
            'c' => {
                let reg = program[1..].parse::<i32>().unwrap();
                for _ in 0..reg {
                    push_process!(self.proc_info, proc_id, DO_COMPUTE);
                }
            },
            'i' => {
                push_process!(self.proc_info, proc_id, DO_IO);
                push_process!(self.proc_info, proc_id, DO_IO_DONE);
            },
            _ => {
                panic!(
                    "invalid opcode `{}`: opcode must be 'c' or 'i'.", 
                    opcode
                );
            }
        }
    }
}

#[macro_export]
macro_rules! push_process {
    ($proc_info:expr, $proc_id:expr, $process:expr) => {
        $proc_info
            .get_mut(&$proc_id)
            .unwrap()
            .proc_code
            .push($process);
    };
}

#[macro_export]
macro_rules! assign_state {
    ($proc_info:expr, $proc_id:expr, $state:expr) => {
        $proc_info
            .get_mut(&$proc_id)
            .unwrap()
            .proc_state = $state;
    };
}