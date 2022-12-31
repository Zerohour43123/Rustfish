// SPDX-License-Identifier: GPL-3.0-or-later

extern crate memmap;

use std::thread;

mod benchmark;
mod bitbases;
#[macro_use]
mod bitboard;
mod endgame;
mod evaluate;
mod material;
mod misc;
mod movegen;
mod movepick;
mod pawns;
mod position;
mod psqt;
mod search;
mod tb;
mod threads;
mod timeman;
mod tt;
mod types;
mod uci;
mod ucioption;

fn main() {
    println!("{}", misc::engine_info(false));

    ucioption::init();
    bitboard::init();
    search::init();
    pawns::init();
    tt::resize(ucioption::get_i32("Hash") as usize);
    threads::init(ucioption::get_i32("Threads") as usize);
    tb::init(ucioption::get_string("SyzygyPath"));
    search::clear();

    // To avoid a stack overflow, we create a thread with a large
    // enough stack size to run the UI.
    let builder = thread::Builder::new().stack_size(16 * 1024 * 1024);
    if let Ok(ui_thread) = builder.spawn(uci::cmd_loop) {
        let _ = ui_thread.join();
    }
    uci::cmd_loop();

    threads::free();
    tb::free();
    tt::free();
    ucioption::free();
}
