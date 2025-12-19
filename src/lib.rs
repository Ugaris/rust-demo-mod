//! Ugaris Rust Demo Mod
//!
//! A demonstration of native mod development using Rust.
//! Shows basic API usage: commands, rendering, and game data access.
//!
//! Commands:
//!   #hello   - Display a greeting message
//!   #stats   - Show current HP/Mana/Gold
//!   #overlay - Toggle a simple HUD overlay

use std::ffi::{c_char, c_int, CStr};
use std::sync::atomic::{AtomicBool, AtomicU32, Ordering};

// ============================================================================
// FFI Declarations - Client-exported functions and data
// ============================================================================

// Stat indices
const V_HP: usize = 0;
const V_MANA: usize = 2;
const V_STR: usize = 6;
const V_AGI: usize = 5;
const V_INT: usize = 4;
const V_WIS: usize = 3;
const V_MAX: usize = 200;

// Screen anchor points
const DOT_TL: c_int = 0;

// Color macro (RGB 5-5-5)
const fn irgb(r: u16, g: u16, b: u16) -> u16 {
    (r << 10) | (g << 5) | b
}

extern "C" {
    // Logging
    fn note(format: *const c_char, ...) -> c_int;
    fn addline(format: *const c_char, ...);

    // Rendering
    fn render_rect(sx: c_int, sy: c_int, ex: c_int, ey: c_int, color: u16);
    fn render_line(fx: c_int, fy: c_int, tx: c_int, ty: c_int, color: u16);
    fn render_text(sx: c_int, sy: c_int, color: u16, flags: c_int, text: *const c_char) -> c_int;

    // GUI helpers
    fn dotx(didx: c_int) -> c_int;
    fn doty(didx: c_int) -> c_int;

    // Utilities
    fn exp2level(val: c_int) -> c_int;

    // Game state (imported globals)
    static hp: c_int;
    static mana: c_int;
    static gold: c_int;
    static experience: c_int;
    static value: [[c_int; V_MAX]; 2];
    static username: [c_char; 40];

    // Colors
    static whitecolor: u16;
    static textcolor: u16;
    static healthcolor: u16;
    static manacolor: u16;
}

// ============================================================================
// Mod State
// ============================================================================

static SHOW_OVERLAY: AtomicBool = AtomicBool::new(false);
static FRAME_COUNT: AtomicU32 = AtomicU32::new(0);

// ============================================================================
// Helper Functions
// ============================================================================

macro_rules! cstr {
    ($s:expr) => {
        concat!($s, "\0").as_ptr() as *const c_char
    };
}

fn get_username() -> String {
    unsafe {
        let cstr = CStr::from_ptr(username.as_ptr());
        cstr.to_string_lossy().into_owned()
    }
}

// ============================================================================
// Mod Callbacks
// ============================================================================

#[no_mangle]
pub extern "C" fn amod_version() -> *const c_char {
    cstr!("Rust Demo Mod 1.0.0")
}

#[no_mangle]
pub extern "C" fn amod_init() {
    unsafe {
        note(cstr!("Rust Demo Mod initializing..."));
    }
}

#[no_mangle]
pub extern "C" fn amod_exit() {
    unsafe {
        note(cstr!("Rust Demo Mod shutting down."));
    }
}

#[no_mangle]
pub extern "C" fn amod_gamestart() {
    let name = get_username();
    unsafe {
        note(cstr!("Rust Demo Mod: Game started! Welcome, %s"), name.as_ptr() as *const c_char);
        addline(cstr!("Rust Demo Mod loaded. Type #hello for commands."));
    }
}

#[no_mangle]
pub extern "C" fn amod_tick() {
    // Called 24 times per second
}

#[no_mangle]
pub extern "C" fn amod_frame() {
    FRAME_COUNT.fetch_add(1, Ordering::Relaxed);

    if !SHOW_OVERLAY.load(Ordering::Relaxed) {
        return;
    }

    unsafe {
        let x = dotx(DOT_TL) + 10;
        let y = doty(DOT_TL) + 10;
        let w = 180;
        let h = 80;

        // Panel background
        render_rect(x, y, x + w, y + h, irgb(4, 4, 6));

        // Panel border
        let border_color = irgb(12, 12, 16);
        render_line(x, y, x + w, y, border_color);
        render_line(x, y + h, x + w, y + h, border_color);
        render_line(x, y, x, y + h, border_color);
        render_line(x + w, y, x + w, y + h, border_color);

        // Title
        render_text(x + 4, y + 4, whitecolor, 0, cstr!("Rust Demo Mod"));

        // Stats
        let mut text_y = y + 20;

        // HP
        let hp_text = format!("HP: {} / {}\0", hp, value[0][V_HP]);
        render_text(x + 4, text_y, healthcolor, 0, hp_text.as_ptr() as *const c_char);
        text_y += 14;

        // Mana
        let mana_text = format!("Mana: {} / {}\0", mana, value[0][V_MANA]);
        render_text(x + 4, text_y, manacolor, 0, mana_text.as_ptr() as *const c_char);
        text_y += 14;

        // Gold
        let gold_text = format!("Gold: {}\0", gold);
        render_text(x + 4, text_y, irgb(31, 31, 0), 0, gold_text.as_ptr() as *const c_char);
        text_y += 14;

        // Frame counter
        let frame_text = format!("Frame: {}\0", FRAME_COUNT.load(Ordering::Relaxed));
        render_text(x + 4, text_y, textcolor, 0, frame_text.as_ptr() as *const c_char);
    }
}

#[no_mangle]
pub extern "C" fn amod_mouse_move(_x: c_int, _y: c_int) {}

#[no_mangle]
pub extern "C" fn amod_mouse_click(_x: c_int, _y: c_int, _what: c_int) -> c_int {
    0 // Don't consume
}

#[no_mangle]
pub extern "C" fn amod_keydown(_key: c_int) -> c_int {
    0 // Don't consume
}

#[no_mangle]
pub extern "C" fn amod_keyup(_key: c_int) -> c_int {
    0
}

#[no_mangle]
pub extern "C" fn amod_client_cmd(buf: *const c_char) -> c_int {
    let cmd = unsafe {
        if buf.is_null() {
            return 0;
        }
        match CStr::from_ptr(buf).to_str() {
            Ok(s) => s,
            Err(_) => return 0,
        }
    };

    unsafe {
        match cmd {
            "#hello" => {
                addline(cstr!("=== Rust Demo Mod Commands ==="));
                addline(cstr!("#hello   - Show this help"));
                addline(cstr!("#stats   - Display current stats"));
                addline(cstr!("#overlay - Toggle HUD overlay"));
                1
            }
            "#stats" => {
                let level = exp2level(experience);
                addline(cstr!("=== Player Stats (from Rust) ==="));

                let level_text = format!("Level: {}  Experience: {}\0", level, experience);
                addline(level_text.as_ptr() as *const c_char);

                let hp_text = format!("HP: {}/{}  Mana: {}/{}\0", hp, value[0][V_HP], mana, value[0][V_MANA]);
                addline(hp_text.as_ptr() as *const c_char);

                let stats_text = format!("STR: {}  AGI: {}  INT: {}  WIS: {}\0",
                    value[0][V_STR], value[0][V_AGI], value[0][V_INT], value[0][V_WIS]);
                addline(stats_text.as_ptr() as *const c_char);

                let gold_text = format!("Gold: {}\0", gold);
                addline(gold_text.as_ptr() as *const c_char);
                1
            }
            "#overlay" => {
                let new_state = !SHOW_OVERLAY.load(Ordering::Relaxed);
                SHOW_OVERLAY.store(new_state, Ordering::Relaxed);
                if new_state {
                    addline(cstr!("Overlay: ON"));
                } else {
                    addline(cstr!("Overlay: OFF"));
                }
                1
            }
            _ => 0,
        }
    }
}
