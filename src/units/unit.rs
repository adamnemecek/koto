use std::cmp::{PartialEq, Eq};
use std::sync::{Arc, Mutex};

use super::super::time::Time;
use super::super::event::Event;

pub struct Mut<T: ?Sized> (pub Mutex<T>);
type Amut<T> = Arc<Mut<T>>;

impl<T> Mut<T> {
    pub fn amut(value: T) -> Amut<T> {
        Arc::new(Mut(Mutex::new(value)))
    }
}

impl<T: PartialEq> PartialEq for Mut<T> {
    fn eq(&self, other: &Self) -> bool {
        *self.0.lock().unwrap() == *other.0.lock().unwrap()
    }
}
impl<T: Eq> Eq for Mut<T> {}

#[derive(Debug)]
pub enum Dump {
    Str(String),
    Op(String, Vec<Box<Dump>>),
}

pub type Signal = (f64, f64);

pub trait Unit {
    fn proc(&mut self, time: &Time) -> Signal;
    fn dump(&self) -> Dump;
}

pub trait Osc: Unit {
    fn set_freq(&mut self, freq: AUnit);
}

pub enum ADSR {
    Attack,
    Decay,
    Sustin,
    Release,
    None,
}

pub trait Eg: Unit {
    fn set_state(&mut self, state: ADSR, eplaced: u64);
}

pub enum Node {
    Val(f64),
    Sig(Amut<Unit + Send>),
    Osc(Amut<Osc + Send>),
    Eg(Amut<Eg + Send>),
}

impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        match self {
            Node::Val(s) => match other {
                Node::Val(o) => s == o,
                _ => false,
            },
            Node::Sig(s) => match other {
                Node::Sig(o) => Arc::ptr_eq(&s, &o),
                _ => false,
            },
            Node::Osc(s) => match other {
                Node::Osc(o) => Arc::ptr_eq(&s, &o),
                _ => false,
            },
            Node::Eg(s) => match other {
                Node::Eg(o) => Arc::ptr_eq(&s, &o),
                _ => false,
            },
        }
    }
}
impl Eq for Node {}

pub struct UnitGraph {
    pub last_tick: u64,
    pub last_sig: Signal,
    pub node: Node,
}

pub type AUnit = Amut<UnitGraph>;

impl PartialEq for UnitGraph {
    fn eq(&self, other: &Self) -> bool {
        self.node == other.node
    }
}
impl Eq for UnitGraph {}

impl UnitGraph {
    pub fn new(node: Node) -> UnitGraph {
        UnitGraph {
            last_tick: 0,
            last_sig: (0.0, 0.0),
            node: node,
        }
    }
}

impl Unit for Node {
    fn proc(&mut self, time: &Time) -> Signal {
        match self {
            Node::Val(v) => (*v, *v),
            Node::Sig(u) => u.0.lock().unwrap().proc(time),
            Node::Osc(u) => u.0.lock().unwrap().proc(time),
            Node::Eg(u) => u.0.lock().unwrap().proc(time),
        }
    }

    fn dump(&self) -> Dump {
        match self {
            Node::Val(v) => Dump::Str(v.to_string()),
            Node::Sig(u) => u.0.lock().unwrap().dump(),
            Node::Osc(u) => u.0.lock().unwrap().dump(),
            Node::Eg(u) => u.0.lock().unwrap().dump(),
        }
    }
}

impl Osc for Node {
    fn set_freq(&mut self, freq: Amut<UnitGraph>) {
        match self {
            Node::Osc(u) => u.0.lock().unwrap().set_freq(freq),
            _ => (),
        }
    }
}

impl Eg for Node {
    fn set_state(&mut self, state: ADSR, eplaced: u64) {
        match self {
            Node::Eg(u) => u.0.lock().unwrap().set_state(state, eplaced),
            _ => (),
        }
    }
}

impl Unit for UnitGraph {
    fn proc(&mut self, time: &Time) -> Signal {
        if self.last_tick < time.tick {
            self.last_sig = self.node.proc(&time);
            self.last_tick = time.tick;
            self.last_sig
        } else {
            self.last_sig
        }
    }

    fn dump(&self) -> Dump {
        self.node.dump()
    }
}

pub type Table = Vec<f64>;

impl Unit for Table {
    fn proc(&mut self, _time: &Time) -> Signal {  // dummy
        (0.0, 0.0)
    }

    fn dump(&self) -> Dump {
        let mut vec = Vec::new();
        for v in self.iter() {
            vec.push(Box::new(Dump::Str(v.to_string())));
        }
        Dump::Op("table".to_string(), vec)
    }
}

impl Unit for Event {
    fn proc(&mut self, _time: &Time) -> Signal {  // dummy
        (0.0, 0.0)
    }
    fn dump(&self) -> Dump {
        // TODO: dump event
        Dump::Str("event".to_string())
    }
}

pub type Pattern = Vec<Box<Event>>;

impl Unit for Pattern {
    fn proc(&mut self, _time: &Time) -> Signal {  // dummy
        (0.0, 0.0)
    }

    fn dump(&self) -> Dump {
        let mut vec = Vec::new();
        for ev in self.iter() {
            vec.push(Box::new(ev.dump()));
        }
        Dump::Op("pat".to_string(), vec)
    }
}
