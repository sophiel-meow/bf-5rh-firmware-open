use crate::board;
use crate::hal::delay;
use crate::hal::uptime;

use at32f421_pac as pac;
use cortex_m::peripheral::SYST;

const SETTLE_US: u32 = 5;
const LONG_PRESS_TICKS: u32 = 10_000; // 1000ms @ 100us/tick
const REPEAT_FIRE_TICKS: u32 = 15_000; // 1500ms @ 100us/tick
const REPEAT_RESET_TICKS: u32 = 14_000; // re-fires every 1000 ticks (100ms) after
const EVENT_QUEUE_CAP: usize = 8;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum KeyId {
    Digit0,
    Digit1,
    Digit2,
    Digit3,
    Digit4,
    Digit5,
    Digit6,
    Digit7,
    Digit8,
    Digit9,
    Asterisk,
    Pound,
    Menu,
    Exit,
    Up,
    Down,
    Vm,
    Ab,
    Band,
    Side1,
    Side2,
}

impl KeyId {
    fn supports_long(self) -> bool {
        matches!(
            self,
            KeyId::Up
                | KeyId::Down
                | KeyId::Vm
                | KeyId::Band
                | KeyId::Asterisk
                | KeyId::Pound
                | KeyId::Digit0
                | KeyId::Exit
                | KeyId::Side1
                | KeyId::Side2
        )
    }

    fn supports_repeat(self) -> bool {
        matches!(self, KeyId::Up | KeyId::Down)
    }
}

#[rustfmt::skip]
const KEY_TABLE: [[Option<KeyId>; 4]; 5] = [
    [Some(KeyId::Ab),     Some(KeyId::Vm),     Some(KeyId::Band),   Some(KeyId::Side1)],
    [Some(KeyId::Digit1), Some(KeyId::Digit2), Some(KeyId::Digit3), Some(KeyId::Asterisk)],
    [Some(KeyId::Digit4), Some(KeyId::Digit5), Some(KeyId::Digit6), Some(KeyId::Digit0)],
    [Some(KeyId::Digit7), Some(KeyId::Digit8), Some(KeyId::Digit9), Some(KeyId::Pound)],
    [Some(KeyId::Up),     Some(KeyId::Down),   Some(KeyId::Menu),   Some(KeyId::Exit)],
];

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum KeyEventKind {
    Press,
    Release,
    Single,
    Long,
    Repeat,
}

#[derive(Clone, Copy, Debug)]
pub struct KeyEvent {
    pub key: KeyId,
    pub kind: KeyEventKind,
}

struct EventQueue {
    buf: [Option<KeyEvent>; EVENT_QUEUE_CAP],
    head: usize,
    len: usize,
}

impl EventQueue {
    const fn new() -> Self {
        EventQueue {
            buf: [None; EVENT_QUEUE_CAP],
            head: 0,
            len: 0,
        }
    }

    fn push(&mut self, ev: KeyEvent) {
        if self.len == EVENT_QUEUE_CAP {
            self.head = (self.head + 1) % EVENT_QUEUE_CAP;
            self.len -= 1;
        }
        let tail = (self.head + self.len) % EVENT_QUEUE_CAP;
        self.buf[tail] = Some(ev);
        self.len += 1;
    }

    fn pop(&mut self) -> Option<KeyEvent> {
        if self.len == 0 {
            return None;
        }
        let ev = self.buf[self.head].take();
        self.head = (self.head + 1) % EVENT_QUEUE_CAP;
        self.len -= 1;
        ev
    }
}

pub struct Keypad<'a> {
    gpioa: &'a pac::Gpioa,
    gpiob: &'a pac::Gpiob,
    raw_prev: Option<KeyId>,
    stable: Option<KeyId>,
    hold_ticks: u32,
    last_poll: u16,
    long_fired: bool,
    queue: EventQueue,
}

impl<'a> Keypad<'a> {
    pub fn new(gpioa: &'a pac::Gpioa, gpiob: &'a pac::Gpiob) -> Self {
        Keypad {
            gpioa,
            gpiob,
            raw_prev: None,
            stable: None,
            hold_ticks: 0,
            last_poll: uptime::now(),
            long_fired: false,
            queue: EventQueue::new(),
        }
    }

    fn scan(&mut self, syst: &mut SYST) -> Option<KeyId> {
        if board::read_side_key2(self.gpioa) {
            return Some(KeyId::Side2);
        }
        for phase in 0..5u8 {
            board::set_keypad_row(self.gpiob, phase);
            delay::us(syst, SETTLE_US);
            if let Some(col) = board::read_keypad_column(self.gpiob) {
                if let Some(key) = KEY_TABLE[phase as usize][col as usize] {
                    board::set_keypad_rows_idle(self.gpiob);
                    return Some(key);
                }
            }
        }
        board::set_keypad_rows_idle(self.gpiob);
        None
    }

    pub fn poll(&mut self, syst: &mut SYST) {
        let now = uptime::now();
        let elapsed = now.wrapping_sub(self.last_poll);
        self.last_poll = now;

        let raw = self.scan(syst);

        if raw != self.raw_prev {
            self.raw_prev = raw;
            return;
        }

        if raw == self.stable {
            if let Some(key) = self.stable {
                self.hold_ticks += elapsed as u32;
                if !self.long_fired
                    && key.supports_long()
                    && self.hold_ticks >= LONG_PRESS_TICKS
                {
                    self.long_fired = true;
                    self.queue.push(KeyEvent {
                        key,
                        kind: KeyEventKind::Long,
                    });
                }
                if key.supports_repeat() && self.hold_ticks >= REPEAT_FIRE_TICKS
                {
                    self.hold_ticks = REPEAT_RESET_TICKS;
                    self.queue.push(KeyEvent {
                        key,
                        kind: KeyEventKind::Repeat,
                    });
                }
            }
            return;
        }

        if let Some(old) = self.stable {
            if !self.long_fired {
                self.queue.push(KeyEvent {
                    key: old,
                    kind: KeyEventKind::Single,
                });
            }
            self.queue.push(KeyEvent {
                key: old,
                kind: KeyEventKind::Release,
            });
        }
        self.stable = raw;
        self.hold_ticks = 0;
        self.long_fired = false;
        if let Some(new) = self.stable {
            self.queue.push(KeyEvent {
                key: new,
                kind: KeyEventKind::Press,
            });
        }
    }

    pub fn pop_event(&mut self) -> Option<KeyEvent> {
        self.queue.pop()
    }

    pub fn any_pressed(&mut self, syst: &mut SYST) -> bool {
        self.scan(syst).is_some()
    }

    pub fn is_pressed(&mut self, syst: &mut SYST, key: KeyId) -> bool {
        self.scan(syst) == Some(key)
    }
}
