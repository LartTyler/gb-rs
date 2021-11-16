use gb_rs_core::bytes::{bytes_to_word, word_to_bytes};
use std::cell::RefCell;
use std::time::{Instant, SystemTime, UNIX_EPOCH};

const RTC_REGISTER_SECS: usize = 0;
const RTC_REGISTER_MINS: usize = 1;
const RTC_REGISTER_HOURS: usize = 2;
const RTC_REGISTER_DAYS_LOW: usize = 3;
const RTC_REGISTER_DAYS_HIGH: usize = 4;

struct RtcInner {
    seconds: u8,
    minutes: u8,
    hours: u8,
    days_low: u8,
    days_high: u8,
}

impl RtcInner {
    pub fn new() -> Self {
        Self {
            seconds: 0,
            minutes: 0,
            hours: 0,
            days_low: 0,
            days_high: 0,
        }
    }

    pub fn load(parts: [u8; 5]) -> Self {
        Self {
            seconds: parts[RTC_REGISTER_SECS],
            minutes: parts[RTC_REGISTER_MINS],
            hours: parts[RTC_REGISTER_HOURS],
            days_low: parts[RTC_REGISTER_DAYS_LOW],
            days_high: parts[RTC_REGISTER_DAYS_HIGH],
        }
    }
}

pub struct Rtc {
    latch_prev: u8,
    latched: bool,
    inner: RefCell<RtcInner>,
    timer: RefCell<Option<Instant>>,
}

const RTC_FLAG_HALT: u8 = 0x40;
const RTC_FLAG_CARRY: u8 = 0x80;
const RTC_FLAGS: u8 = RTC_FLAG_HALT | RTC_FLAG_CARRY;

impl Rtc {
    /// Creates a new [`RTC`] with default contents.
    pub fn new() -> Self {
        Self {
            latch_prev: 0xFF,
            latched: false,
            inner: RefCell::new(RtcInner::new()),
            timer: RefCell::new(Some(Instant::now())),
        }
    }

    /// Creates a new [`RTC`] using the specified initial data, usually from a save state.
    ///
    /// The `last_timestamp` should be the last Unix timestamp that the RTC was active for (e.g.
    /// from the moment it was saved). `time_parts` is an array of RTC registers to initialize the
    /// [`RTC`] with, in the order `[secs, mins, hours, days_low, days_high]`.
    pub fn load(last_timestamp: u64, time_parts: [u8; 5]) -> Self {
        let rtc = Self {
            latch_prev: 0xFF,
            latched: false,
            inner: RefCell::new(RtcInner::load(time_parts)),
            timer: RefCell::new(Some(Instant::now())),
        };

        let unix_now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("System time is before Unix epoch")
            .as_secs();

        if last_timestamp > unix_now {
            panic!("Loaded timestamp is greater than the current system time and everything is broken.");
        }

        rtc.update_from_elapsed_secs(unix_now - last_timestamp);

        rtc
    }

    /// Returns `true` if the RTC has been latched.
    ///
    /// A "latched" RTC freezes the values of its time registers, but will continue to count time.
    /// Once an RTC is "unlatched", any time that passed is added to the registers.
    pub fn is_latched(&self) -> bool {
        self.latched
    }

    /// Returns `true` if the RTC has been halted.
    ///
    /// A "halted" RTC stops counting time until it's "un-halted". Any time that passes while the
    /// RTC has been halted is ignored.
    pub fn is_halted(&self) -> bool {
        self.timer.borrow().is_none()
    }

    /// Returns the RTC's current time parts, in the order `[secs, mins, hours, days_low,
    /// days_high]`.
    pub fn get_time_parts(&self) -> [u8; 5] {
        self.refresh();

        let inner = self.inner.borrow();

        [
            inner.seconds,
            inner.minutes,
            inner.hours,
            inner.days_low,
            inner.days_high,
        ]
    }

    /// Writes a value to the RTC's latch register.
    ///
    /// The RTC's latch is toggled by writing `0x0` followed by `0x1`.
    pub fn latch_write(&mut self, value: u8) {
        if self.latch_prev == 0 && value == 1 {
            self.latched = !self.latched;
        }

        self.latch_prev = value;
    }

    /// Writes a value to one of the RTC's time registers.
    ///
    /// The `register` should be one of the registers named by the `RTC_REGISTER_*` constants.
    pub fn register_write(&mut self, register: u8, value: u8) {
        self.refresh();

        let mut inner = self.inner.borrow_mut();

        match register as usize {
            RTC_REGISTER_SECS => inner.seconds = value & 0x3B,
            RTC_REGISTER_MINS => inner.minutes = value & 0x3B,
            RTC_REGISTER_HOURS => inner.hours = value & 0x17,
            RTC_REGISTER_DAYS_LOW => inner.days_low = value,
            RTC_REGISTER_DAYS_HIGH => {
                inner.days_high = (value & RTC_FLAGS) | (value & 1);

                if inner.days_high & RTC_FLAG_HALT != 0 {
                    self.timer.borrow_mut().take();
                } else if self.is_halted() {
                    self.timer.borrow_mut().replace(Instant::now());
                }
            }
            _ => (),
        };
    }

    /// Reads a value from one of the RTC's time registers.
    ///
    /// The `register` should be one of the registers named by the `RTC_REGISTER_*` constants.
    pub fn register_read(&self, register: u8) -> u8 {
        self.refresh();

        let inner = self.inner.borrow();

        match register as usize {
            RTC_REGISTER_SECS => inner.seconds,
            RTC_REGISTER_MINS => inner.minutes,
            RTC_REGISTER_HOURS => inner.hours,
            RTC_REGISTER_DAYS_LOW => inner.days_low,
            RTC_REGISTER_DAYS_HIGH => inner.days_high,
            _ => 0xFF,
        }
    }

    fn refresh(&self) -> bool {
        if !self.is_latched() {
            let mut timer = self.timer.borrow_mut();

            if let Some(t) = *timer {
                self.update_from_elapsed_secs(t.elapsed().as_secs());
                timer.replace(Instant::now());

                return true;
            }
        }

        false
    }

    fn update_from_elapsed_secs(&self, elapsed: u64) {
        if elapsed == 0 {
            return;
        }

        let mut inner = self.inner.borrow_mut();

        let new_seconds = inner.seconds + (elapsed % 60) as u8;
        inner.seconds = new_seconds % 60;

        let mut carry = if new_seconds >= 60 {
            Some(new_seconds as u64 / 60)
        } else {
            None
        };

        if elapsed >= 60 || carry.is_some() {
            let new_minutes = inner.minutes as u64 + (elapsed / 60) + carry.unwrap_or(0);
            inner.minutes = (new_minutes % 60) as u8;

            if new_minutes >= 60 {
                carry = Some(new_minutes / 60);
            } else {
                carry = None;
            }
        }

        if elapsed >= 3600 || carry.is_some() {
            let new_hours = inner.hours as u64 + carry.unwrap_or(0);
            inner.hours = (new_hours % 24) as u8;

            if new_hours >= 24 {
                carry = Some(new_hours / 24);
            } else {
                carry = None;
            }
        }

        if elapsed >= 86_400 || carry.is_some() {
            let days = bytes_to_word(inner.days_low, inner.days_high) as u64;
            let days = days + carry.unwrap_or(0);
            let (low, high) = word_to_bytes((days % 512) as u16);

            inner.days_low = low;
            inner.days_high = (inner.days_high & 0xFE) | high;

            if days >= 512 {
                // The actual carried value doesn't matter here, we just need to know that a carry
                // happened.
                carry = Some(1);
            } else {
                carry = None;
            }
        }

        if carry.is_some() {
            inner.days_high = inner.days_high | RTC_FLAG_CARRY;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;

    macro_rules! assert_time {
        ( $rtc:expr, $expected:expr ) => {
            let mut actual = $rtc.get_time_parts();
            actual[4] &= 1;

            assert_eq!(actual, $expected);
        };
    }

    #[test]
    fn sleep_progress() {
        let rtc = Rtc::new();
        assert_time!(rtc, [0, 0, 0, 0, 0]);

        std::thread::sleep(Duration::new(2, 0));
        assert_time!(rtc, [2, 0, 0, 0, 0]);
    }

    #[test]
    fn set_initial_state() {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();

        let rtc = Rtc::load(now - 4 * 3600, [0, 0, 0, 0, 0]);
        assert_time!(rtc, [0, 0, 4, 0, 0]);

        //                          s    m        h           d
        let rtc = Rtc::load(
            now - (34 + 18 * 60 + 7 * 3600 + 37 * 86_400),
            [0, 0, 0, 0, 0],
        );
        assert_time!(rtc, [34, 18, 7, 37, 0]);
    }

    #[test]
    fn elapsed() {
        let rtc = Rtc::new();

        rtc.update_from_elapsed_secs(34);
        assert_time!(rtc, [34, 0, 0, 0, 0]);

        rtc.update_from_elapsed_secs(40);
        assert_time!(rtc, [14, 1, 0, 0, 0]);

        rtc.update_from_elapsed_secs(64 * 60); // 64 minutes
        assert_time!(rtc, [14, 5, 1, 0, 0]);

        rtc.update_from_elapsed_secs(4 * 86_400);
        assert_time!(rtc, [14, 5, 1, 4, 0]);
    }

    #[test]
    fn latch() {
        let mut rtc = Rtc::new();

        assert!(!rtc.is_latched());
        std::thread::sleep(Duration::new(1, 0));
        assert_time!(rtc, [1, 0, 0, 0, 0]);

        rtc.latch_write(0);
        rtc.latch_write(1);
        assert!(rtc.is_latched());

        std::thread::sleep(Duration::new(1, 0));
        assert_time!(rtc, [1, 0, 0, 0, 0]);

        rtc.latch_write(0);
        rtc.latch_write(1);
        assert!(!rtc.is_latched());
        assert_time!(rtc, [2, 0, 0, 0, 0]);
    }

    #[test]
    fn halt() {
        let mut rtc = Rtc::new();
        assert!(!rtc.is_halted());

        rtc.register_write(4, RTC_FLAG_HALT);
        assert!(rtc.is_halted());

        std::thread::sleep(Duration::new(2, 0));
        assert_time!(rtc, [0, 0, 0, 0, 0]);
    }
}
