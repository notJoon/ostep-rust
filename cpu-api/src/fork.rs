use std::collections::HashMap;
use std::string::String;

use clap::Parser;
use rand::thread_rng;
use rand::Rng;
use regex::Regex;

use crate::hash_map;
use crate::seed;

// from this sort of things:
// a forks b
// a forks c
// a forks d
// b forks e
// b forks f
// d forks g

// process tree
// a --- b --- e
//    |     |
//    |     |- f
//    |- c
//    |
//    |- d --- g

// TODO : add `seed` to Forker, for enabled to use `clap::Parser`
#[derive(Clone, Debug, PartialEq)]
struct Forker {
    fork_percentage: f64,
    max_actions: u32,
    action_list: Vec<String>,
    show_tree: bool,
    just_final: bool,
    leaf_only: bool,
    local_reparent: bool,
    print_style: String,
    solve: bool,
    root_name: char,

    // process list: names of all active processes
    process_list: Vec<char>,

    // for each process, it has a list of its children
    children: HashMap<char, Vec<char>>,

    // track parents
    parents: HashMap<char, char>,

    // process names
    name_length: u32,
    base_names: String,

    curr_names: String,
    curr_index: u32,
}

impl Forker {
    pub fn new(
        fork_percentage: f64,
        max_actions: u32,
        action_list: Vec<String>,
        show_tree: bool,
        just_final: bool,
        leaf_only: bool,
        local_reparent: bool,
        print_style: String,
        solve: bool,
    ) -> Self {
        // root name must set to "a"
        let root_name = 'a';
        let process_list = vec![root_name];
        let children = hash_map! { root_name => vec![] };
        let parents = HashMap::new();
        let base_names = r#"abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ"#.to_string();
        let curr_name = base_names.clone();

        Self {
            fork_percentage,
            max_actions,
            action_list,
            show_tree,
            just_final,
            leaf_only,
            local_reparent,
            print_style,
            solve,
            root_name,
            process_list,
            children,
            parents,
            name_length: 1,
            base_names,
            curr_names: curr_name,
            curr_index: 1,
        }
    }

    fn grow_names(&mut self) {
        let mut new_names: Vec<char> = Vec::new();
        for b1 in self.curr_names.chars() {
            for b2 in self.base_names.chars() {
                new_names.push(format!("{}{}", b1, b2).chars().next().unwrap());
            }
        }

        self.curr_names = new_names.iter().collect();
        self.curr_index = 0;
    }

    fn get_name(&mut self) -> char {
        if self.curr_index == self.curr_names.len() as u32 {
            self.grow_names();
        }

        let name = self
            .curr_names
            .chars()
            .nth(self.curr_index as usize)
            .unwrap();

        self.curr_index += 1;

        name
    }

    fn walk(&self, p: char, level: usize, pmask: &mut [bool], is_last: bool) {
        let chars: [&str; 4];
        match self.print_style.as_str() {
            "basic" => {
                for _ in 0..level {
                    print!("    ");
                }

                println!("{}", p);

                for child in self.children[&p].iter() {
                    self.walk(*child, level + 1, pmask, false);
                }

                return;
            }

            "line1" => chars = ["|", "-", "+", "|"],
            "line2" => chars = ["|", "_", "|", "|"],
            "fancy" => chars = ["│", "─", "├", "└"],

            _ => {
                println!("bad style {}", self.print_style);
                std::process::exit(1);
            }
        }

        // print something before node
        if level > 0 {
            // main printing
            for i in 0..level - 1 {
                match pmask[i] {
                    true => print!("{}   ", chars[0]),
                    false => print!("    "),
                }
            }
            // "|__"
            match pmask[level - 1] {
                true => print!("{}{}{} ", chars[3], chars[1], chars[1]),
                false => print!("{}{}{} ", chars[2], chars[1], chars[1]),
            }
        } else {
            // "___"
            print!(" {} {} ", chars[1], chars[1]);
        }

        // print node
        println!("{}", p);

        // undo parent verticals
        if is_last {
            pmask[level - 1] = false;
        }

        // recurse
        pmask[level] = true;

        for child in self.children[&p][..self.children[&p].len() - 1].iter() {
            self.walk(*child, level + 1, pmask, false);
        }

        for child in self.children[&p][self.children[&p].len() - 1..].iter() {
            self.walk(*child, level + 1, pmask, true);
        }
    }

    fn print_tree(&self) {
        self.walk(self.root_name, 0, &mut [false; 100], false)
    }

    fn do_fork(&mut self, p: char, c: char) -> String {
        self.process_list.push(c);
        self.children.insert(c, vec![]);
        self.children.get_mut(&p).unwrap().push(c);
        self.parents.insert(c, p);

        format!("forks {} -> {}", p, c)
    }

    fn collect_children(&self, p: char) -> Vec<char> {
        match self.children[&p].is_empty() {
            true => vec![p],
            false => {
                let mut l = vec![p];
                for c in self.children[&p].iter() {
                    l.extend(self.collect_children(*c));
                }
                l
            }
        }
    }

    fn do_exit(&mut self, p: char) -> String {
        // remove the process from the process list
        if p == self.root_name {
            println!("root process: cannot exit");
            std::process::exit(1);
        }

        let exit_parent = self.parents[&p];
        self.process_list.retain(|&x| x != p);

        // for each orphan, set its parent to exiting proc's parent or root
        if self.local_reparent {
            for orphan in self.children[&p].clone().iter() {
                self.parents.insert(*orphan, exit_parent);
                self.children.get_mut(&exit_parent).unwrap().push(*orphan);
            }
        } else {
            // should set ALL descendants to be child of ROOT
            let desc = self.collect_children(p);
            for d in desc.iter() {
                self.children.insert(*d, vec![]);
                self.parents.insert(*d, self.root_name);
                self.children.get_mut(&self.root_name).unwrap().push(*d);
            }
        }

        // remove the entry from its parent child list
        self.children
            .get_mut(&exit_parent)
            .unwrap()
            .retain(|&x| x != p);

        // should never use this.
        self.children.insert(p, vec![]);
        self.parents.insert(p, ' ');

        // remove the entry for this proc from children
        format!("{} EXITS", p)
    }

    fn bad_action(&self, action: &str) {
        println!(
            "bad action: {}, must be `X+Y` or `X-` where `X` and `Y` are processes",
            action
        );
        std::process::exit(1);
    }

    fn is_legal<'a>(&'a self, action: &'a str) -> Result<Vec<&str>, &str> {
        let re = Regex::new(r"^(\w+)([+-])(\w+)$").unwrap();
        let capture = re.captures(action);

        if capture.is_none() {
            return Err("bad action");
        }

        let capture = capture.unwrap();
        let p1 = capture.get(1).unwrap().as_str();
        let op = capture.get(2).unwrap().as_str();
        let p2 = capture.get(3).unwrap().as_str();

        match op {
            "+" => Ok(vec![p1, p2]),
            "-" => Ok(vec![p1]),
            _ => Err("bad action"),
        }
    }

    fn run(mut self) {
        println!("                           Process Tree:");
        self.print_tree();
        println!("");

        let action_list: Vec<&str> = match !self.action_list.is_empty() {
            true => {
                // same as `action_list = self.action_list.split(',')` (python)
                self.action_list
                    .iter()
                    .map(|action| action.as_str())
                    .collect()
            }
            false => Vec::new(),
        };

        let mut actions = 0;
        let mut temp_process_list = vec![self.root_name];
        let mut level_list = hash_map! { self.root_name => 1 };

        while actions < self.max_actions {
            let rand = seed::seed();

            if rand < self.fork_percentage as i32 {
                match thread_rng().gen_bool(self.fork_percentage) {
                    true => {
                        let fork_choice = seed::choice(&temp_process_list);
                        let new_child = self.get_name();
                        // todo : action_list.push(fork_choice+new_child)
                    }
                    false => {
                        let exit_choice = seed::choice(&temp_process_list);

                        if exit_choice == &self.root_name {
                            continue;
                        }
                    }
                }
            }
            actions += 1;
        }
    }
}

#[macro_export]
macro_rules! hash_map (
    { $($key:expr => $value:expr), + } => {
        {
            let mut map = HashMap::new();
            $( map.insert($key, $value); )+

            map
        }
    };
);
