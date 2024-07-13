use std::sync::atomic::{AtomicBool, AtomicU32, Ordering};

pub static EXIT_FACT: AtomicU32 = AtomicU32::new(0);
pub static MAXIM_FACT: AtomicU32 = AtomicU32::new(0);
pub static MINIM_FACT: AtomicU32 = AtomicU32::new(0);

pub static EXIT_OVER: AtomicBool = AtomicBool::new(false);
pub static MAXIM_OVER: AtomicBool = AtomicBool::new(false);
pub static MINIM_OVER: AtomicBool = AtomicBool::new(false);

pub static STOP: AtomicBool = AtomicBool::new(false);

pub fn reset_bools() {
    EXIT_OVER.store(false, Ordering::SeqCst);
    MAXIM_OVER.store(false, Ordering::SeqCst);
    MINIM_OVER.store(false, Ordering::SeqCst);
}

pub fn activ_exit_over() {
    EXIT_OVER.store(true, Ordering::SeqCst);
}

pub fn activ_maxim_over() {
    MAXIM_OVER.store(true, Ordering::SeqCst);
}

pub fn activ_minim_over() {
    MINIM_OVER.store(true, Ordering::SeqCst);
}
