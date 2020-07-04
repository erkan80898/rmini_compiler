use std::collections::{HashMap,HashSet};
use std::rc::Rc;
use std::cell::RefCell;
use std::hash::{Hash, Hasher};

static mut state_count:u64 = 0;

#[derive(Debug)]
struct State{
    is_final:bool,
    state_num:u64,
    char_transitions:HashMap<char,HashSet<HashedState>>,
    empty_transitions:HashSet<HashedState>
}
#[derive(Debug,Clone)]
struct HashedState(Rc<RefCell<State>>);

impl Hash for HashedState{
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.0.borrow().state_num.hash(state);
    }
}

impl PartialEq for HashedState{
    fn eq(&self, other: &HashedState) -> bool {
        self.0.borrow().state_num == self.0.borrow().state_num
    }
}

impl Eq for HashedState{}


impl State{
    fn new() -> Self{
        unsafe{state_count += 1};
        Self{
            is_final:false,
            state_num:unsafe{state_count},
            char_transitions:HashMap::new(),
            empty_transitions:HashSet::new()
        }
    }

    fn connect(&mut self, c: char, exit: HashedState){
        match self.char_transitions.get_mut(&c){
            Some(list) =>{
                list.insert(exit);
            }
            None =>{
                let mut set = HashSet::new();
                set.insert(exit);
                self.char_transitions.insert(c, set);
            }
        }
    }

    fn connect_empty(&mut self,exit: HashedState){
        self.empty_transitions.insert(exit);
    }
   
    fn set_final(&mut self,final_state:bool){
        self.is_final = final_state;
    }
}

#[derive(Debug)]
struct NFA{
    start:HashedState,
    exit:HashedState
}

impl NFA{

    fn new(start:State, exit:State) -> Self{
        Self{
            start:HashedState(Rc::new(RefCell::new(start))),
            exit:HashedState(Rc::new(RefCell::new(exit)))
        }
    }

    pub fn character_nfa(c: char) -> Self{
        let mut exit = State::new();
        let mut start = State::new();
        exit.set_final(true);
        let exit = HashedState(Rc::new(RefCell::new(exit)));
        start.connect(c, exit.clone());

        let start = HashedState(Rc::new(RefCell::new(start)));
        Self{
            exit,
            start
        }
    }

    pub fn empty_nfa() -> Self{
        let mut exit = State::new();
        let mut start = State::new();
        exit.set_final(true);
        let exit = HashedState(Rc::new(RefCell::new(exit)));
        start.connect_empty(exit.clone());
        let start = HashedState(Rc::new(RefCell::new(start)));

        Self{
            exit,
            start
        }
    }

    pub fn or_nfa(choice1: NFA, choice2: NFA) -> Self{
        let start = HashedState(Rc::new(RefCell::new(State::new())));
        let exit = HashedState(Rc::new(RefCell::new(State::new())));
        start.0.borrow_mut().connect_empty(choice1.start);
        start.0.borrow_mut().connect_empty(choice2.start);

        choice1.exit.0.borrow_mut().connect_empty(exit.clone());
        choice2.exit.0.borrow_mut().connect_empty(exit.clone());

        Self{
            start,
            exit
        }
    }

    pub fn rep_nfa(nfa:NFA) -> Self{
        let start = nfa.start;
        let exit = nfa.exit;
        start.0.borrow_mut().connect_empty(exit.clone());
        exit.0.borrow_mut().connect_empty(start.clone());
        Self{
            start,
            exit
        }
    }

    pub fn seq_nfa(part1:NFA, part2:NFA) -> Self{
        part2.exit.0.borrow_mut().set_final(true);
        part1.exit.0.borrow_mut().connect_empty(part2.start);
        part1.exit.0.borrow_mut().set_final(false);
        Self{
            start:part1.start,
            exit:part2.exit
        }
    }

    fn empty_string_closure(set:HashSet<State>) -> HashSet<HashedState>{
        let mut result = HashSet::new();
        for item in set{
            result.extend(item.empty_transitions);
        }
        result
    }

}

struct DFA{
    start:Rc<RefCell<State>>,
    exit:Rc<RefCell<State>>
}

impl DFA{
    
}

fn main() {

    let nfa1 = NFA::character_nfa('a');
    let nfa2 = NFA::character_nfa('b');
    let nfa3 = NFA::character_nfa('c');
    let nfa4 = NFA::seq_nfa(NFA::or_nfa(NFA::character_nfa('a'),NFA::character_nfa('b')),NFA::seq_nfa(nfa1, NFA::seq_nfa(nfa2, nfa3)));
    println!("{:#?}",nfa4);
}
