use std::collections::{HashMap,HashSet};
use std::rc::Rc;
use std::cell::RefCell;
use std::hash::{Hash, Hasher};
use std::clone::Clone;
use std::fmt::Display;
static mut STATE_COUNT:u64 = 0;
static ALPHABET: [char; 62] = [
    'a', 'b', 'c', 'd', 'e', 
    'f', 'g', 'h', 'i', 'j', 
    'k', 'l', 'm', 'n', 'o',
    'p', 'q', 'r', 's', 't', 
    'u', 'v', 'w', 'x', 'y', 
    'z',

    'A', 'B', 'C', 'D', 'E', 
    'F', 'G', 'H', 'I', 'J', 
    'K', 'L', 'M', 'N', 'O',
    'P', 'Q', 'R', 'S', 'T', 
    'U', 'V', 'W', 'X', 'Y', 
    'Z',

    '0', '1', '2', '3', '4', 
    '5', '6', '7', '8', '9', 
];


macro_rules! c{
    ($z:expr) => {
        NFA::c($z)
    };
}

macro_rules! or{
    ($x:expr,$z:expr) => {
        NFA::or($x,$z)
    };
}

macro_rules! s{
    ($x:expr,$z:expr) => {
        NFA::seq($x,$z)
    };
}

macro_rules! e{
    () => {
        NFA::e()
    };
}

macro_rules! dfa{
    ($x:expr) => {
        DFA::from_nfa($x)
    };
}

#[derive(Debug)]
struct State{
    is_final:bool,
    state_num:u64,
    char_transitions:HashMap<char,HashSet<HashedState>>,
    empty_transitions:HashSet<HashedState>
}

#[derive(Debug)]
struct HashedState(Rc<RefCell<State>>);

impl Hash for HashedState{
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.0.borrow().state_num.hash(state);
    }
}

impl PartialEq for HashedState{
    fn eq(&self, other: &HashedState) -> bool {
        self.0.borrow().state_num == other.0.borrow().state_num
    }
}

impl Eq for HashedState{}

impl Clone for HashedState{

    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

impl Display for HashedState{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0.borrow().state_num)
    }
}

impl Drop for HashedState{
    fn drop(&mut self) {
        unsafe {
            STATE_COUNT = 0;
        }
    }
}

impl State{
    fn new() -> Self{
        unsafe{STATE_COUNT += 1};
        Self{
            is_final:false,
            state_num:unsafe{STATE_COUNT},
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

    fn get_final_states(&self) -> Vec<HashedState>{
        if self.empty_transitions.len() + self.char_transitions.len() == 0{
            return vec![]
        }

        self.empty_transitions.iter()
        .chain(self.char_transitions.values().flatten())
        .filter(|&x|x.0.borrow().is_final == true)
        .cloned()
        .collect()
    }
}

#[derive(Debug,Clone)]
struct NFA{
    start:HashedState,
    exit:HashedState
}

/*impl Display for NFA{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "START: ({}) -> EXIT: ({})", self.start,self.exit)
    }
}*/

impl NFA{

    fn new(start:State, exit:State) -> Self{
        Self{
            start:HashedState(Rc::new(RefCell::new(start))),
            exit:HashedState(Rc::new(RefCell::new(exit)))
        }
    }

    pub fn c(c: char) -> Self{
        let start = HashedState(Rc::new(RefCell::new(State::new())));
        let exit = HashedState(Rc::new(RefCell::new(State::new())));
        exit.0.borrow_mut().set_final(true);
        start.0.borrow_mut().connect(c, exit.clone());

        Self{
            exit,
            start
        }
    }

    pub fn e() -> Self{
        let start = HashedState(Rc::new(RefCell::new(State::new())));
        let exit = HashedState(Rc::new(RefCell::new(State::new())));
        exit.0.borrow_mut().set_final(true);
        start.0.borrow_mut().connect_empty(exit.clone());
        exit.0.borrow_mut().connect_empty(start.clone());
        Self{
            exit,
            start
        }
    }

    pub fn or(choice1: NFA,choice2: NFA) -> Self{
        let start = HashedState(Rc::new(RefCell::new(State::new())));
        let exit = HashedState(Rc::new(RefCell::new(State::new())));
        exit.0.borrow_mut().set_final(true);

        choice1.exit.0.borrow_mut().set_final(false);
        choice2.exit.0.borrow_mut().set_final(false);

        start.0.borrow_mut().connect_empty(choice1.start);
        start.0.borrow_mut().connect_empty(choice2.start);

        choice1.exit.0.borrow_mut().connect_empty(exit.clone());
        choice2.exit.0.borrow_mut().connect_empty(exit.clone());

        Self{
            start,
            exit
        }
    }

    pub fn star(nfa:NFA) -> Self{
        let start = nfa.start;
        let exit = nfa.exit;
        start.0.borrow_mut().connect_empty(exit.clone());
        exit.0.borrow_mut().connect_empty(start.clone());
        Self{
            start,
            exit
        }
    }

    pub fn seq(part1:NFA,part2:NFA) -> Self{
        
        part1.exit.0.borrow_mut().set_final(false);
        part2.exit.0.borrow_mut().set_final(true);
        part1.exit.0.borrow_mut().connect_empty(part2.start);
        
        Self{
            start:part1.start.clone(),
            exit:part2.exit.clone()
        }
    }

    fn empty_string_closure(states: HashSet<HashedState>,ch:&char) -> HashSet<HashedState>{
        
        
        let mut result:HashSet<HashedState> = HashSet::new();
        
        for state in states{
            result.extend(state.0.borrow().empty_transitions.clone());
        }

        let mut answer = HashSet::new();

        for state_i in result{
            if let Some(val) = state_i.0.borrow().char_transitions.get(ch){
                answer.extend::<HashSet<HashedState>>(val.into_iter().cloned().collect());
            }
        }
        answer
    }

}

struct DFA{
    table:HashMap<(Vec<HashedState>,char),HashSet<HashedState>>
}

impl Display for DFA{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut z = write!(f,"");
        for (x,y) in self.table.iter(){            
            for state in x.0.iter(){
                z = write!(f, "(States: ({}), Input: {}) --> (Exit: {:#?})\n\n\n",state, x.1,y.iter().map(|x| x.0.borrow().get_final_states()));
            }
        }
        z
    }
}

impl DFA{
    pub fn from_nfa(nfa:NFA) -> Self{
        let mut start = HashSet::new();
        start.insert(nfa.start);
        let mut Q = vec![HashSet::new()];
        let mut work_list = vec![HashSet::new()];
        work_list.push(start.clone());
        let mut table = HashMap::new();
        Q.push(start);

        while !work_list.is_empty(){
            let current = work_list.pop().unwrap();
                for ch in ALPHABET.iter(){
                    let t = NFA::empty_string_closure(current.clone(),ch);

                    if t.len() == 0{
                        continue;
                    }

                    table.insert((current.iter().cloned().collect(), *ch),t.clone());

                    for set in t.iter(){
                        if !t.contains(&set){
                            work_list.push(t.clone());
                            Q.push(t);
                            break;
                        }
                    }
                }
        }

        Self{
            table,
        }
    }
}

fn main() {

    //p(aaa|bbb|ccc)(aaa|bbb|ccc)z

    let nfa1 = or!(c!('a'),c!('b'));
    //println!("{}",nfa1);
    //println!("{}",dfa!(or!(c!('a'),c!('b'))));

    let nfa2 = s!(or!(c!('a'),c!('b')),c!('z'));
    println!("{}",dfa!(nfa2));
//    println!("{}",dfa!(s!(or!(c!('a'),c!('b')),c!('z'))));

}
