use std::cell::RefCell;
use std::clone::Clone;
use std::collections::{HashMap, HashSet, VecDeque};
use std::fmt::Display;
use std::hash::{Hash, Hasher};
use std::rc::Rc;

static mut STATE_COUNT: u32 = 0;
static ALPHABET: [char; 62] = [
    'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's',
    't', 'u', 'v', 'w', 'x', 'y', 'z', 'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L',
    'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z', '0', '1', '2', '3', '4',
    '5', '6', '7', '8', '9',
];

macro_rules! c {
    ($z:expr) => {
        NFA::c($z)
    };
}

macro_rules! or {
    ($x:expr,$z:expr) => {
        NFA::or($x, $z)
    };
}

macro_rules! s {
    ($x:expr,$z:expr) => {
        NFA::seq($x, $z)
    };
}

macro_rules! dfa {
    ($x:expr) => {
        DFA::from_nfa($x)
    };
}

#[derive(Debug)]
struct State {
    is_final: bool,
    state_num: u32,
    char_transitions: HashMap<char, HashSet<HashedState>>,
    empty_transitions: HashSet<HashedState>,
}

#[derive(Debug)]
struct HashedState(Rc<RefCell<State>>);

impl Hash for HashedState {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.0.borrow().state_num.hash(state);
    }
}

impl PartialEq for HashedState {
    fn eq(&self, other: &HashedState) -> bool {
        self.0.borrow().state_num == other.0.borrow().state_num
    }
}

impl Eq for HashedState {}

impl Clone for HashedState {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

impl Display for HashedState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0.borrow().state_num)
    }
}

impl State {
    fn new() -> Self {
        unsafe { STATE_COUNT += 1 };
        Self {
            is_final: false,
            state_num: unsafe { STATE_COUNT },
            char_transitions: HashMap::new(),
            empty_transitions: HashSet::new(),
        }
    }

    fn connect(&mut self, c: char, exit: HashedState) {
        match self.char_transitions.get_mut(&c) {
            Some(list) => {
                list.insert(exit);
            }
            None => {
                let mut set = HashSet::new();
                set.insert(exit);
                self.char_transitions.insert(c, set);
            }
        }
    }

    fn connect_empty(&mut self, exit: HashedState) {
        self.empty_transitions.insert(exit);
    }

    fn set_final(&mut self, final_state: bool) {
        self.is_final = final_state;
    }

    fn apply_empty(&self) -> HashSet<HashedState> {
        let mut stack: Vec<HashedState> = self.empty_transitions.iter().cloned().collect();

        let mut result = HashSet::new();

        while !stack.is_empty() {
            let item = stack.pop().unwrap();
            stack.extend(item.0.borrow().empty_transitions.clone());
            result.insert(item);
        }

        result
    }
}

#[derive(Debug, Clone)]
struct NFA {
    start: HashedState,
    exit: HashedState,
}

impl Display for NFA {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "START: ({}) -> EXIT: ({})", self.start, self.exit)
    }
}

impl NFA {
    #[allow(dead_code)]
    fn new(start: State, exit: State) -> Self {
        Self {
            start: HashedState(Rc::new(RefCell::new(start))),
            exit: HashedState(Rc::new(RefCell::new(exit))),
        }
    }

    pub fn c(c: char) -> Self {
        let start = HashedState(Rc::new(RefCell::new(State::new())));
        let exit = HashedState(Rc::new(RefCell::new(State::new())));
        exit.0.borrow_mut().set_final(true);
        start.0.borrow_mut().connect(c, exit.clone());

        Self { exit, start }
    }

    #[allow(dead_code)]
    pub fn e() -> Self {
        let start = HashedState(Rc::new(RefCell::new(State::new())));
        let exit = HashedState(Rc::new(RefCell::new(State::new())));
        exit.0.borrow_mut().set_final(true);
        start.0.borrow_mut().connect_empty(exit.clone());
        exit.0.borrow_mut().connect_empty(start.clone());
        Self { exit, start }
    }

    pub fn or(choice1: NFA, choice2: NFA) -> Self {
        let start = HashedState(Rc::new(RefCell::new(State::new())));
        let exit = HashedState(Rc::new(RefCell::new(State::new())));
        exit.0.borrow_mut().set_final(true);

        choice1.exit.0.borrow_mut().set_final(false);
        choice2.exit.0.borrow_mut().set_final(false);

        start.0.borrow_mut().connect_empty(choice1.start);
        start.0.borrow_mut().connect_empty(choice2.start);

        choice1.exit.0.borrow_mut().connect_empty(exit.clone());
        choice2.exit.0.borrow_mut().connect_empty(exit.clone());

        Self { start, exit }
    }

    #[allow(dead_code)]
    pub fn star(nfa: NFA) -> Self {
        let start = nfa.start;
        let exit = nfa.exit;
        start.0.borrow_mut().connect_empty(exit.clone());
        exit.0.borrow_mut().connect_empty(start.clone());
        Self { start, exit }
    }

    pub fn seq(part1: NFA, part2: NFA) -> Self {
        part1.exit.0.borrow_mut().set_final(false);
        part2.exit.0.borrow_mut().set_final(true);
        part1.exit.0.borrow_mut().connect_empty(part2.start);

        Self {
            start: part1.start.clone(),
            exit: part2.exit.clone(),
        }
    }

    fn empty_string_closure(states: HashSet<HashedState>, ch: &char) -> HashSet<HashedState> {
        let mut result: HashSet<HashedState> = HashSet::new();

        for state in states {
            result.extend(state.0.borrow().apply_empty());
            if let Some(val) = state.0.borrow().char_transitions.get(ch).cloned() {
                result.extend(val);
            }
        }

        result
    }
}

struct DFA {
    table: HashMap<(HashedState, char), HashSet<HashedState>>,
}

impl DFA {
    pub fn from_nfa(nfa: NFA) -> Self {
        let mut start = nfa.start.0.borrow().apply_empty();
        let mut Q = vec![HashSet::new()];
        let mut work_list = vec![HashSet::new()];
        work_list.push(start.clone());
        let mut table = HashMap::new();
        Q.push(start);

        while !work_list.is_empty() {
            let current = work_list.pop().unwrap();
            for ch in ALPHABET.iter() {
                let t = NFA::empty_string_closure(current.clone(), ch);

                //for z in t.iter(){
                //println!("CH: {} | SET: {}",ch,z);
                //}

                if t.len() == 0 {
                    continue;
                }

                table.insert((current.iter().cloned().collect(), *ch), t.clone());

                for set in t.iter() {
                    if !t.contains(&set) {
                        work_list.push(t.clone());
                        Q.push(t.clone());
                    }
                }
            }
        }

        DFA {
            table: DFA::build_table(table),
        }
    }

    fn build_table(
        structure: HashMap<(Vec<HashedState>, char), HashSet<HashedState>>,
    ) -> HashMap<(HashedState, char), HashSet<HashedState>> {
        let mut table = HashMap::new();

        for ((states, ch), v) in structure {
            for state in states {
                table.insert((state, ch), v.clone());
            }
        }

        table
    }
}

struct Scanner {
    dfa: DFA,
    stream: VecDeque<Vec<char>>,
}

impl Scanner {
    fn generate(nfa: NFA) -> Scanner {
        Scanner {
            dfa: DFA::from_nfa(nfa),
            stream: VecDeque::new(),
        }
    }

    fn feed(&mut self, stream: Vec<Vec<char>>) {
        self.stream = stream.into_iter().collect();
    }

    fn scan(&mut self) {
        let buffer = self.stream.pop_front().unwrap();
        //let mut state = &self.dfa.start;
        //let mut stack = vec![];
        let mut lexeme = vec![];
        for i in 0..buffer.len() {
            let ch = buffer[i];
            lexeme.push(ch);

            //if state.0.borrow().is_final{
            //    stack.clear();
            //}
            //stack.push(state);
            //state = &state.0.borrow().c
        }
    }
}

fn main() {
    let nfa1 = c!('a');
    let nfa2 = c!('b');
    let nfa3 = or!(nfa1, nfa2);
    let table = &dfa!(nfa3).table;
    println!(
        "{:#?}",
        table
            .keys()
            .filter(|&(_, ch)| *ch == 'a')
            .collect::<Vec<_>>()
    );
}
