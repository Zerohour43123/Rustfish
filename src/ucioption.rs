// SPDX-License-Identifier: GPL-3.0-or-later

use std::sync::RwLock;

use tb;
use threads;
use tt;

type OnChange = Option<fn(&OptVal)>;

struct Opt {
    key: &'static str,
    val: OptVal,
    on_change: OnChange,
}

impl Opt {
    pub const fn new(key: &'static str, val: OptVal, on_change: OnChange) -> Opt {
        Opt {
            key,
            val,
            on_change,
        }
    }
}

enum OptVal {
    StringOpt {
        def: &'static str,
        cur: String,
    },
    Spin {
        def: i32,
        cur: i32,
        min: i32,
        max: i32,
    },
    Check {
        def: bool,
        cur: bool,
    },
    Button,
    Combo {
        def: &'static str,
        cur: String,
    },
}

impl OptVal {
    pub fn string(def: &'static str) -> OptVal {
        OptVal::StringOpt {
            def,
            cur: String::from(def),
        }
    }

    pub fn spin(def: i32, min: i32, max: i32) -> OptVal {
        OptVal::Spin {
            def,
            cur: def,
            min,
            max,
        }
    }

    pub fn check(def: bool) -> OptVal {
        OptVal::Check {
            def,
            cur: def,
        }
    }

    pub fn combo(def: &'static str) -> OptVal {
        OptVal::Combo {
            def,
            cur: String::from(&def[0..def.find(" var").unwrap()])
                .to_lowercase(),
        }
    }
}

fn on_clear_hash(_: &OptVal) {
    tt::clear();
}

fn on_hash_size(opt_val: &OptVal) {
    if let &OptVal::Spin { cur, .. } = opt_val {
        tt::resize(cur as usize);
    }
}

fn on_threads(opt_val: &OptVal) {
    if let &OptVal::Spin { cur, .. } = opt_val {
        threads::set(cur as usize);
    }
}

fn on_tb_path(opt_val: &OptVal) {
    if let &OptVal::StringOpt { ref cur, .. } = opt_val {
        tb::init(String::from(cur.as_str()));
    }
}

static OPTIONS: RwLock<Vec<Opt>> = RwLock::new(Vec::new());

pub fn init()
{
    if let Ok(mut opts) = OPTIONS.write() {
        opts.push(Opt::new("Contempt", OptVal::spin(18, -100, 100), None));
        opts.push(Opt::new("Analysis Contempt",
                           OptVal::combo("Off var Off var White var Black"), None));
        opts.push(Opt::new("Threads", OptVal::spin(1, 1, 512), Some(on_threads)));
        opts.push(Opt::new("Hash", OptVal::spin(16, 1, 128 * 1024),
                           Some(on_hash_size)));
        opts.push(Opt::new("Clear Hash", OptVal::Button, Some(on_clear_hash)));
        opts.push(Opt::new("Ponder", OptVal::check(false), None));
        opts.push(Opt::new("MultiPV", OptVal::spin(1, 1, 500), None));
        opts.push(Opt::new("Move Overhead", OptVal::spin(30, 0, 5000), None));
        opts.push(Opt::new("Minimum Thinking Time", OptVal::spin(20, 0, 5000),
                           None));
        opts.push(Opt::new("Slow Mover", OptVal::spin(84, 10, 1000), None));
        opts.push(Opt::new("UCI_AnalyseMode", OptVal::check(false), None));
        opts.push(Opt::new("UCI_Chess960", OptVal::check(false), None));
        opts.push(Opt::new("SyzygyPath", OptVal::string("<empty>"),
                           Some(on_tb_path)));
        opts.push(Opt::new("SyzygyProbeDepth", OptVal::spin(1, 1, 100), None));
        opts.push(Opt::new("Syzygy50MoveRule", OptVal::check(true), None));
        opts.push(Opt::new("SyzygyProbeLimit", OptVal::spin(6, 0, 6), None));
        opts.push(Opt::new("SyzygyUseDTM", OptVal::check(true), None));
    }
}
// TODO improve pattern matching on functions below

pub fn print() {
    if let Ok(opts) = OPTIONS.read() {
        for opt in opts.iter() {
            print!("\noption name {} type {}", opt.key, match opt.val {
                OptVal::StringOpt { def, .. } => format!("string default {}", def),
                OptVal::Spin { def, min, max, .. } =>
                    format!("spin default {} min {} max {}", def, min, max),
                OptVal::Check { def, .. } =>
                    format!("check default {}", def),
                OptVal::Button => "button".to_string(),
                OptVal::Combo { def, .. } => format!("combo default {}", def),
            });
        }
    }
    println!();
}

pub fn set(key: &str, val: &str) {
    if let Ok(mut opts) = OPTIONS.write() {
        if let Some(opt) = opts.iter_mut().find(|o| o.key == key) {
            match opt.val {
                OptVal::StringOpt { ref mut cur, .. } => *cur = String::from(val),
                OptVal::Spin { ref mut cur, .. } => *cur = val.parse().unwrap(),
                OptVal::Check { ref mut cur, .. } => *cur = val == "true",
                OptVal::Button => {}
                OptVal::Combo { ref mut cur, .. } =>
                    *cur = String::from(val).to_lowercase(),
            }
            if let Some(on_change) = opt.on_change {
                on_change(&opt.val);
            }
        }
    } else {
        println!("No such option: {}", key);
    }
}

pub fn get_i32(key: &str) -> i32 {
    if let Ok(opts) = OPTIONS.read() {
        return {
            let opt = opts.iter().find(|o| o.key == key).unwrap();
            if let OptVal::Spin { cur, .. } = opt.val { cur } else { 0 }
        };
    }
    0
}

pub fn get_bool(key: &str) -> bool {
    if let Ok(opts) = OPTIONS.read() {
        return {
            let opt = opts.iter().find(|o| o.key == key).unwrap();
            if let OptVal::Check { cur, .. } = opt.val { cur } else { false }
        };
    }
    false
}

// todo see if COW ptr can be used here?

pub fn get_string(key: &str) -> String {
    if let Ok(opts) = OPTIONS.read() {
        return {
            let opt = opts.iter().find(|o| o.key == key).unwrap();
            if let OptVal::StringOpt { ref cur, .. } = opt.val {
                String::from(cur.as_str())
            } else if let OptVal::Combo { ref cur, .. } = opt.val {
                String::from(cur.as_str())
            } else {
                String::new()
            }
        };
    }
    String::new()
}
