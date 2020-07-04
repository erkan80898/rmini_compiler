use std::collections::HashMap;
use std::rc::Rc;
use std::cell::RefCell;

static mut state_count:u64 = 0;
#[derive(Debug)]
struct NFA_state{
    is_final:bool,
    state_num:u64,
    char_transitions:HashMap<char,Vec<Rc<RefCell<NFA_state>>>>,
    empty_transitions:Vec<Rc<RefCell<NFA_state>>>
}

impl PartialEq for NFA_state{
    fn eq(&self, other: &Self) -> bool {
        self.state_num == other.state_num
    }
}

impl NFA_state{
    fn new() -> Self{
        unsafe{state_count += 1};
        Self{
            is_final:false,
            state_num: unsafe{state_count-1},
            char_transitions:HashMap::new(),
            empty_transitions:Vec::new()
        }
    }

    fn connect(&mut self, c: char, exit: Rc<RefCell<NFA_state>>){
        match self.char_transitions.get_mut(&c){
            Some(list) =>{
                list.push(exit);
            }
            None =>{
                self.char_transitions.insert(c, vec![exit]);
            }
        }
    }

    fn connect_empty(&mut self,exit: Rc<RefCell<NFA_state>>){
        let inner = exit.borrow();
        for state in self.empty_transitions.iter(){
            if state.borrow().eq(&inner){
                return
            }
        }
        self.empty_transitions.push(exit.clone());
    }
   
    fn finalize(&mut self){
        self.is_final = true;
    }
}

#[derive(Debug)]
struct NFA{
    start:Rc<RefCell<NFA_state>>,
    exit:Rc<RefCell<NFA_state>>
}

impl NFA{

    fn new(start:NFA_state, exit:NFA_state) -> Self{
        Self{
            start:Rc::new(RefCell::new(start)),
            exit:Rc::new(RefCell::new(exit))
        }
    }

    fn character_nfa(c: char) -> Self{
        let mut exit = NFA_state::new();
        let mut start = NFA_state::new();
        exit.finalize();
        let exit = Rc::new(RefCell::new(exit));
        start.connect(c, exit.clone());

        let start = Rc::new(RefCell::new(start));
        Self{
            exit,
            start
        }
    }

    fn empty_nfa() -> Self{
        let mut exit = NFA_state::new();
        let mut start = NFA_state::new();
        exit.finalize();
        let exit = Rc::new(RefCell::new(exit));
        start.connect_empty(exit.clone());
        let start = Rc::new(RefCell::new(start));

        Self{
            exit,
            start
        }
    }

    fn or_nfa(choice1: NFA, choice2: NFA) -> Self{
        let mut start = Rc::new(RefCell::new(NFA_state::new()));
        let mut exit = Rc::new(RefCell::new(NFA_state::new()));
        start.borrow_mut().connect_empty(choice1.start);
        start.borrow_mut().connect_empty(choice2.start);

        choice1.exit.borrow_mut().connect_empty(exit.clone());
        choice2.exit.borrow_mut().connect_empty(exit.clone());

        Self{
            start,
            exit
        }
    }

    fn rep_nfa(nfa:NFA) -> Self{
        let start = nfa.start;
        let exit = nfa.exit;
        start.borrow_mut().connect_empty(exit.clone());
        exit.borrow_mut().connect_empty(start.clone());
        Self{
            start,
            exit
        }
    }

    fn seq_nfa(part1:NFA, part2:NFA) -> Self{
        part2.exit.borrow_mut().finalize();
        part1.exit.borrow_mut().connect_empty(part2.start.clone());
        Self{
            start:part1.start,
            exit:part2.exit
        }
    }

}

fn main() {

    let nfa = NFA::character_nfa('c');
    println!("{:#?}",NFA::rep_nfa(nfa));
}
