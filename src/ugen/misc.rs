extern crate num;

use super::super::mtime::Time;
use super::core::{Aug, Dump, Proc, Setv, Signal, Slot, UGen, UgNode, Value, Walk, UG};

pub struct Pan {
    pub pan: Aug,
    pub src: Aug,
}

impl Pan {
    pub fn new(pan: Aug, src: Aug) -> Aug {
        Aug::new(UGen::new(UG::Proc(Box::new(Pan { pan: pan, src: src }))))
    }
}

impl Walk for Pan {
    fn walk(&self, f: &mut dyn FnMut(&Aug) -> bool) {
        if f(&self.pan) {
            self.pan.walk(f);
        }
        if f(&self.src) {
            self.src.walk(f);
        }
    }
}

impl Dump for Pan {
    fn dump(&self, shared_ug: &Vec<Aug>) -> UgNode {
        let mut slots = Vec::new();

        slots.push(Slot {
            ug: self.pan.clone(),
            name: "pan".to_string(),
            value: match shared_ug.iter().position(|e| *e == self.pan) {
                Some(n) => Value::Shared(n, shared_ug.iter().nth(n).unwrap().clone()),
                None => Value::Ug(self.pan.clone()),
            },
        });
        slots.push(Slot {
            ug: self.src.clone(),
            name: "src".to_string(),
            value: match shared_ug.iter().position(|e| *e == self.src) {
                Some(n) => Value::Shared(n, shared_ug.iter().nth(n).unwrap().clone()),
                None => Value::Ug(self.src.clone()),
            },
        });

        UgNode::Ug("pan".to_string(), slots)
    }
}

impl Setv for Pan {
    fn setv(&mut self, pname: &str, data: String, shared: &Vec<Aug>) {}
}

impl Proc for Pan {
    fn proc(&mut self, time: &Time) -> Signal {
        let (l, r) = self.src.proc(&time);
        let v = self.pan.proc(&time).0;

        if v > 0.0 {
            (l * (1.0 - v), r)
        } else if v < 0.0 {
            (l, r * (1.0 - v))
        } else {
            (l, r)
        }
    }
}

pub struct Clip {
    pub min: Aug,
    pub max: Aug,
    pub src: Aug,
}

impl Clip {
    pub fn new(min: Aug, max: Aug, src: Aug) -> Aug {
        Aug::new(UGen::new(UG::Proc(Box::new(Clip {
            min: min,
            max: max,
            src: src,
        }))))
    }
}

impl Walk for Clip {
    fn walk(&self, f: &mut dyn FnMut(&Aug) -> bool) {
        if f(&self.min) {
            self.min.walk(f);
        }
        if f(&self.max) {
            self.max.walk(f);
        }
        if f(&self.src) {
            self.src.walk(f);
        }
    }
}

impl Dump for Clip {
    fn dump(&self, shared_ug: &Vec<Aug>) -> UgNode {
        let mut slots = Vec::new();

        slots.push(Slot {
            ug: self.min.clone(),
            name: "min".to_string(),
            value: match shared_ug.iter().position(|e| *e == self.min) {
                Some(n) => Value::Shared(n, shared_ug.iter().nth(n).unwrap().clone()),
                None => Value::Ug(self.min.clone()),
            },
        });
        slots.push(Slot {
            ug: self.max.clone(),
            name: "max".to_string(),
            value: match shared_ug.iter().position(|e| *e == self.max) {
                Some(n) => Value::Shared(n, shared_ug.iter().nth(n).unwrap().clone()),
                None => Value::Ug(self.max.clone()),
            },
        });
        slots.push(Slot {
            ug: self.src.clone(),
            name: "src".to_string(),
            value: match shared_ug.iter().position(|e| *e == self.src) {
                Some(n) => Value::Shared(n, shared_ug.iter().nth(n).unwrap().clone()),
                None => Value::Ug(self.src.clone()),
            },
        });

        UgNode::Ug("clip".to_string(), slots)
    }
}

impl Setv for Clip {
    fn setv(&mut self, pname: &str, data: String, shared: &Vec<Aug>) {}
}

impl Proc for Clip {
    fn proc(&mut self, time: &Time) -> Signal {
        let (l, r) = self.src.proc(&time);
        let min = self.min.proc(&time).0;
        let max = self.max.proc(&time).0;
        (num::clamp(l, min, max), num::clamp(r, min, max))
    }
}

pub struct Offset {
    pub val: Aug,
    pub src: Aug,
}

impl Offset {
    pub fn new(val: Aug, src: Aug) -> Aug {
        Aug::new(UGen::new(UG::Proc(Box::new(Offset { val: val, src: src }))))
    }
}

impl Walk for Offset {
    fn walk(&self, f: &mut dyn FnMut(&Aug) -> bool) {
        if f(&self.val) {
            self.val.walk(f);
        }
        if f(&self.src) {
            self.src.walk(f);
        }
    }
}

impl Dump for Offset {
    fn dump(&self, shared_ug: &Vec<Aug>) -> UgNode {
        let mut slots = Vec::new();

        slots.push(Slot {
            ug: self.val.clone(),
            name: "val".to_string(),
            value: match shared_ug.iter().position(|e| *e == self.val) {
                Some(n) => Value::Shared(n, shared_ug.iter().nth(n).unwrap().clone()),
                None => Value::Ug(self.val.clone()),
            },
        });
        slots.push(Slot {
            ug: self.src.clone(),
            name: "src".to_string(),
            value: match shared_ug.iter().position(|e| *e == self.src) {
                Some(n) => Value::Shared(n, shared_ug.iter().nth(n).unwrap().clone()),
                None => Value::Ug(self.src.clone()),
            },
        });

        UgNode::Ug("offset".to_string(), slots)
    }
}

impl Setv for Offset {
    fn setv(&mut self, pname: &str, data: String, shared: &Vec<Aug>) {}
}

impl Proc for Offset {
    fn proc(&mut self, time: &Time) -> Signal {
        let (l, r) = self.src.proc(&time);
        let val = self.val.proc(&time).0;
        (l + val, r + val)
    }
}

pub struct Gain {
    pub gain: Aug,
    pub src: Aug,
}

impl Gain {
    pub fn new(gain: Aug, src: Aug) -> Aug {
        Aug::new(UGen::new(UG::Proc(Box::new(Gain {
            gain: gain,
            src: src,
        }))))
    }
}

impl Walk for Gain {
    fn walk(&self, f: &mut dyn FnMut(&Aug) -> bool) {
        if f(&self.src) {
            self.src.walk(f);
        }
    }
}

impl Dump for Gain {
    fn dump(&self, shared_ug: &Vec<Aug>) -> UgNode {
        let mut slots = Vec::new();

        slots.push(Slot {
            ug: self.gain.clone(),
            name: "gain".to_string(),
            value: match shared_ug.iter().position(|e| *e == self.gain) {
                Some(n) => Value::Shared(n, shared_ug.iter().nth(n).unwrap().clone()),
                None => Value::Ug(self.gain.clone()),
            },
        });
        slots.push(Slot {
            ug: self.src.clone(),
            name: "src".to_string(),
            value: match shared_ug.iter().position(|e| *e == self.src) {
                Some(n) => Value::Shared(n, shared_ug.iter().nth(n).unwrap().clone()),
                None => Value::Ug(self.src.clone()),
            },
        });

        UgNode::Ug("gain".to_string(), slots)
    }
}

impl Setv for Gain {
    fn setv(&mut self, pname: &str, data: String, shared: &Vec<Aug>) {}
}

impl Proc for Gain {
    fn proc(&mut self, time: &Time) -> Signal {
        let (l, r) = self.src.proc(&time);
        let gain = self.gain.proc(&time).0;
        (l * gain, r * gain)
    }
}

pub struct Add {
    pub sources: Vec<Aug>,
}

impl Add {
    pub fn new(sources: Vec<Aug>) -> Aug {
        Aug::new(UGen::new(UG::Proc(Box::new(Add { sources: sources }))))
    }
}

impl Walk for Add {
    fn walk(&self, f: &mut dyn FnMut(&Aug) -> bool) {
        for s in self.sources.iter() {
            if f(s) {
                s.walk(f);
            }
        }
    }
}

impl Dump for Add {
    fn dump(&self, shared_ug: &Vec<Aug>) -> UgNode {
        let mut values = Vec::new();

        for u in self.sources.iter() {
            match shared_ug.iter().position(|e| *e == *u) {
                Some(n) => values.push(Box::new(Value::Shared(
                    n,
                    shared_ug.iter().nth(n).unwrap().clone(),
                ))),
                None => values.push(Box::new(Value::Ug(u.clone()))),
            };
        }
        UgNode::UgRest("+".to_string(), Vec::new(), "src".to_string(), values)
    }
}

impl Setv for Add {
    fn setv(&mut self, pname: &str, data: String, shared: &Vec<Aug>) {}
}

impl Proc for Add {
    fn proc(&mut self, time: &Time) -> Signal {
        let mut l = 0.0;
        let mut r = 0.0;
        for u in self.sources.iter_mut() {
            let (l2, r2) = u.proc(&time);
            l += l2;
            r += r2;
        }
        (l, r)
    }
}

pub struct Multiply {
    pub sources: Vec<Aug>,
}

impl Multiply {
    pub fn new(sources: Vec<Aug>) -> Aug {
        Aug::new(UGen::new(UG::Proc(Box::new(Multiply { sources: sources }))))
    }
}

impl Walk for Multiply {
    fn walk(&self, f: &mut dyn FnMut(&Aug) -> bool) {
        for s in self.sources.iter() {
            if f(s) {
                s.walk(f);
            }
        }
    }
}

impl Dump for Multiply {
    fn dump(&self, shared_ug: &Vec<Aug>) -> UgNode {
        let mut values = Vec::new();

        for u in self.sources.iter() {
            match shared_ug.iter().position(|e| *e == *u) {
                Some(n) => values.push(Box::new(Value::Shared(
                    n,
                    shared_ug.iter().nth(n).unwrap().clone(),
                ))),
                None => values.push(Box::new(Value::Ug(u.clone()))),
            };
        }

        UgNode::UgRest("*".to_string(), Vec::new(), "src".to_string(), values)
    }
}

impl Setv for Multiply {
    fn setv(&mut self, pname: &str, data: String, shared: &Vec<Aug>) {}
}

impl Proc for Multiply {
    fn proc(&mut self, time: &Time) -> Signal {
        let mut l = 1.0;
        let mut r = 1.0;
        for u in self.sources.iter_mut() {
            let (l2, r2) = u.proc(&time);
            l *= l2;
            r *= r2;
        }
        (l, r)
    }
}

pub struct Out {
    vol: Aug,
    sources: Vec<Aug>,
}

impl Out {
    pub fn new(vol: Aug, sources: Vec<Aug>) -> Aug {
        Aug::new(UGen::new(UG::Proc(Box::new(Out {
            vol: vol,
            sources: sources,
        }))))
    }
}

impl Walk for Out {
    fn walk(&self, f: &mut dyn FnMut(&Aug) -> bool) {
        if f(&self.vol) {
            self.vol.walk(f);
        }
        for s in self.sources.iter() {
            if f(s) {
                s.walk(f);
            }
        }
    }
}

impl Dump for Out {
    fn dump(&self, shared_ug: &Vec<Aug>) -> UgNode {
        let mut slots = Vec::new();
        let mut values = Vec::new();

        slots.push(Slot {
            ug: self.vol.clone(),
            name: "vol".to_string(),
            value: match shared_ug.iter().position(|e| *e == self.vol) {
                Some(n) => Value::Shared(n, shared_ug.iter().nth(n).unwrap().clone()),
                None => Value::Ug(self.vol.clone()),
            },
        });

        for u in self.sources.iter() {
            match shared_ug.iter().position(|e| *e == *u) {
                Some(n) => values.push(Box::new(Value::Shared(
                    n,
                    shared_ug.iter().nth(n).unwrap().clone(),
                ))),
                None => values.push(Box::new(Value::Ug(u.clone()))),
            }
        }
        UgNode::UgRest("out".to_string(), slots, "src".to_string(), values)
    }
}

impl Setv for Out {
    fn setv(&mut self, pname: &str, data: String, shared: &Vec<Aug>) {
        match pname {
            "vol" => {
                let mut vol = data.clone();
                vol.retain(|c| c != '\n' && c != ' ');
                if let Ok(vol) = vol.parse::<f64>() {
                    self.vol = Aug::val(vol);
                } else {
                    println!("error while parsing out.vol");
                }
            }
            _ => (),
        }
    }
}

impl Proc for Out {
    fn proc(&mut self, time: &Time) -> Signal {
        let mut l = 0.0;
        let mut r = 0.0;
        let vol = self.vol.proc(&time).0;
        for u in self.sources.iter_mut() {
            let (l2, r2) = u.proc(&time);
            l += l2;
            r += r2;
        }
        (l * vol, r * vol)
    }
}
