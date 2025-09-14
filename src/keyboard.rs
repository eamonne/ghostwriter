use anyhow::Result;
use log::debug;

use std::collections::HashMap;
use std::{thread, time};

use evdev::{
    uinput::{VirtualDeviceBuilder, VirtualDevice},
    AttributeSet, EventType as EvdevEventType, InputEvent,
    KeyCode as EvdevKey,
};

pub struct Keyboard {
    device: Option<VirtualDevice>,
    key_map: HashMap<char, (EvdevKey, bool)>,
    progress_count: u32,
    no_draw_progress: bool,
}

impl Keyboard {
    pub fn progress_end(&mut self) -> Result<()> {
        // Implement any cleanup or finalization logic for progress here if needed
        Ok(())
    }
    pub fn new(no_draw: bool, no_draw_progress: bool) -> Self {
        let device = if no_draw {
            None
        } else {
            Some(Self::create_virtual_device_builder().expect("Failed to create virtual device"))
        };

        Self {
            device,
            key_map: Self::create_key_map(),
            progress_count: 0,
            no_draw_progress,
        }
    }

    fn create_virtual_device_builder() -> Result<VirtualDevice> {
        debug!("Creating virtual keyboard");
        let mut keys = AttributeSet::new();

        keys.insert(EvdevKey::KEY_A);
        keys.insert(EvdevKey::KEY_B);
        keys.insert(EvdevKey::KEY_C);
        keys.insert(EvdevKey::KEY_D);
        keys.insert(EvdevKey::KEY_E);
        keys.insert(EvdevKey::KEY_F);
        keys.insert(EvdevKey::KEY_G);
        keys.insert(EvdevKey::KEY_H);
        keys.insert(EvdevKey::KEY_I);
        keys.insert(EvdevKey::KEY_J);
        keys.insert(EvdevKey::KEY_K);
        keys.insert(EvdevKey::KEY_L);
        keys.insert(EvdevKey::KEY_M);
        keys.insert(EvdevKey::KEY_N);
        keys.insert(EvdevKey::KEY_O);
        keys.insert(EvdevKey::KEY_P);
        keys.insert(EvdevKey::KEY_Q);
        keys.insert(EvdevKey::KEY_R);
        keys.insert(EvdevKey::KEY_S);
        keys.insert(EvdevKey::KEY_T);
        keys.insert(EvdevKey::KEY_U);
        keys.insert(EvdevKey::KEY_V);
        keys.insert(EvdevKey::KEY_W);
        keys.insert(EvdevKey::KEY_X);
        keys.insert(EvdevKey::KEY_Y);
        keys.insert(EvdevKey::KEY_Z);
        keys.insert(EvdevKey::KEY_1);
        keys.insert(EvdevKey::KEY_2);
        keys.insert(EvdevKey::KEY_3);
        keys.insert(EvdevKey::KEY_4);
        keys.insert(EvdevKey::KEY_5);
        keys.insert(EvdevKey::KEY_6);
        keys.insert(EvdevKey::KEY_7);
        keys.insert(EvdevKey::KEY_8);
        keys.insert(EvdevKey::KEY_9);
        keys.insert(EvdevKey::KEY_0);
        keys.insert(EvdevKey::KEY_MINUS);
        keys.insert(EvdevKey::KEY_EQUAL);
        keys.insert(EvdevKey::KEY_LEFTBRACE);
        keys.insert(EvdevKey::KEY_RIGHTBRACE);
        keys.insert(EvdevKey::KEY_BACKSLASH);
        keys.insert(EvdevKey::KEY_SEMICOLON);
        keys.insert(EvdevKey::KEY_APOSTROPHE);
        keys.insert(EvdevKey::KEY_GRAVE);
        keys.insert(EvdevKey::KEY_COMMA);
        keys.insert(EvdevKey::KEY_DOT);
        keys.insert(EvdevKey::KEY_SLASH);
        keys.insert(EvdevKey::KEY_SPACE);
        keys.insert(EvdevKey::KEY_ENTER);
        keys.insert(EvdevKey::KEY_BACKSPACE);
        keys.insert(EvdevKey::KEY_TAB);
        keys.insert(EvdevKey::KEY_CAPSLOCK);
        keys.insert(EvdevKey::KEY_LEFTCTRL);
        keys.insert(EvdevKey::KEY_LEFTSHIFT);
        keys.insert(EvdevKey::KEY_LEFTALT);
        keys.insert(EvdevKey::KEY_RIGHTCTRL);
        keys.insert(EvdevKey::KEY_RIGHTSHIFT);
        keys.insert(EvdevKey::KEY_RIGHTALT);
        keys.insert(EvdevKey::KEY_UP);
        keys.insert(EvdevKey::KEY_DOWN);
        keys.insert(EvdevKey::KEY_LEFT);
        keys.insert(EvdevKey::KEY_RIGHT);

        let builder = VirtualDeviceBuilder::new()?.name("Remarkable keyboard").with_keys(&keys)?;
        Ok(builder.build()?)
    }

    fn create_key_map() -> HashMap<char, (EvdevKey, bool)> {
        let mut map = HashMap::new();
        map.insert('a', (EvdevKey::KEY_A, false));
        map.insert('b', (EvdevKey::KEY_B, false));
        map.insert('c', (EvdevKey::KEY_C, false));
        map.insert('d', (EvdevKey::KEY_D, false));
        map.insert('e', (EvdevKey::KEY_E, false));
        map.insert('f', (EvdevKey::KEY_F, false));
        map.insert('g', (EvdevKey::KEY_G, false));
        map.insert('h', (EvdevKey::KEY_H, false));
        map.insert('i', (EvdevKey::KEY_I, false));
        map.insert('j', (EvdevKey::KEY_J, false));
        map.insert('k', (EvdevKey::KEY_K, false));
        map.insert('l', (EvdevKey::KEY_L, false));
        map.insert('m', (EvdevKey::KEY_M, false));
        map.insert('n', (EvdevKey::KEY_N, false));
        map.insert('o', (EvdevKey::KEY_O, false));
        map.insert('p', (EvdevKey::KEY_P, false));
        map.insert('q', (EvdevKey::KEY_Q, false));
        map.insert('r', (EvdevKey::KEY_R, false));
        map.insert('s', (EvdevKey::KEY_S, false));
        map.insert('t', (EvdevKey::KEY_T, false));
        map.insert('u', (EvdevKey::KEY_U, false));
        map.insert('v', (EvdevKey::KEY_V, false));
        map.insert('w', (EvdevKey::KEY_W, false));
        map.insert('x', (EvdevKey::KEY_X, false));
        map.insert('y', (EvdevKey::KEY_Y, false));
        map.insert('z', (EvdevKey::KEY_Z, false));
        map.insert('A', (EvdevKey::KEY_A, true));
        map.insert('B', (EvdevKey::KEY_B, true));
        map.insert('C', (EvdevKey::KEY_C, true));
        map.insert('D', (EvdevKey::KEY_D, true));
        map.insert('E', (EvdevKey::KEY_E, true));
        map.insert('F', (EvdevKey::KEY_F, true));
        map.insert('G', (EvdevKey::KEY_G, true));
        map.insert('H', (EvdevKey::KEY_H, true));
        map.insert('I', (EvdevKey::KEY_I, true));
        map.insert('J', (EvdevKey::KEY_J, true));
        map.insert('K', (EvdevKey::KEY_K, true));
        map.insert('L', (EvdevKey::KEY_L, true));
        map.insert('M', (EvdevKey::KEY_M, true));
        map.insert('N', (EvdevKey::KEY_N, true));
        map.insert('O', (EvdevKey::KEY_O, true));
        map.insert('P', (EvdevKey::KEY_P, true));
        map.insert('Q', (EvdevKey::KEY_Q, true));
        map.insert('R', (EvdevKey::KEY_R, true));
        map.insert('S', (EvdevKey::KEY_S, true));
        map.insert('T', (EvdevKey::KEY_T, true));
        map.insert('U', (EvdevKey::KEY_U, true));
        map.insert('V', (EvdevKey::KEY_V, true));
        map.insert('W', (EvdevKey::KEY_W, true));
        map.insert('X', (EvdevKey::KEY_X, true));
        map.insert('Y', (EvdevKey::KEY_Y, true));
        map.insert('Z', (EvdevKey::KEY_Z, true));
        map.insert('1', (EvdevKey::KEY_1, false));
        map.insert('2', (EvdevKey::KEY_2, false));
        map.insert('3', (EvdevKey::KEY_3, false));
        map.insert('4', (EvdevKey::KEY_4, false));
        map.insert('5', (EvdevKey::KEY_5, false));
        map.insert('6', (EvdevKey::KEY_6, false));
        map.insert('7', (EvdevKey::KEY_7, false));
        map.insert('8', (EvdevKey::KEY_8, false));
        map.insert('9', (EvdevKey::KEY_9, false));
        map.insert('0', (EvdevKey::KEY_0, false));
        map.insert('-', (EvdevKey::KEY_MINUS, false));
        map.insert('=', (EvdevKey::KEY_EQUAL, false));
        map.insert('[', (EvdevKey::KEY_LEFTBRACE, false));
        map.insert(']', (EvdevKey::KEY_RIGHTBRACE, false));
        map.insert('\\', (EvdevKey::KEY_BACKSLASH, false));
        map.insert(';', (EvdevKey::KEY_SEMICOLON, false));
        map.insert('\'', (EvdevKey::KEY_APOSTROPHE, false));
        map.insert('`', (EvdevKey::KEY_GRAVE, false));
        map.insert(',', (EvdevKey::KEY_COMMA, false));
        map.insert('.', (EvdevKey::KEY_DOT, false));
        map.insert('/', (EvdevKey::KEY_SLASH, false));
        map.insert(' ', (EvdevKey::KEY_SPACE, false));
        map.insert('\n', (EvdevKey::KEY_ENTER, false));
        map.insert('\t', (EvdevKey::KEY_TAB, false));
        map.insert('!', (EvdevKey::KEY_1, true));
        map.insert('@', (EvdevKey::KEY_2, true));
        map.insert('#', (EvdevKey::KEY_3, true));
        map.insert('$', (EvdevKey::KEY_4, true));
        map.insert('%', (EvdevKey::KEY_5, true));
        map.insert('^', (EvdevKey::KEY_6, true));
        map.insert('&', (EvdevKey::KEY_7, true));
        map.insert('*', (EvdevKey::KEY_8, true));
        map.insert('(', (EvdevKey::KEY_9, true));
        map.insert(')', (EvdevKey::KEY_0, true));
        map.insert('_', (EvdevKey::KEY_MINUS, true));
        map.insert('+', (EvdevKey::KEY_EQUAL, true));
        map.insert('{', (EvdevKey::KEY_LEFTBRACE, true));
        map.insert('}', (EvdevKey::KEY_RIGHTBRACE, true));
        map.insert('|', (EvdevKey::KEY_BACKSLASH, true));
        map.insert(':', (EvdevKey::KEY_SEMICOLON, true));
        map.insert('"', (EvdevKey::KEY_APOSTROPHE, true));
        map.insert('~', (EvdevKey::KEY_GRAVE, true));
        map.insert('<', (EvdevKey::KEY_COMMA, true));
        map.insert('>', (EvdevKey::KEY_DOT, true));
        map.insert('?', (EvdevKey::KEY_SLASH, true));
        map
    }

    fn key_down(&mut self, key: EvdevKey) -> Result<()> {
        if let Some(device) = self.device.as_mut() {
            device.emit(&[
                InputEvent::new(EvdevEventType::KEY.0, key.0, 1),
                InputEvent::new(0, 0, 0),
            ])?;
        }
        thread::sleep(time::Duration::from_millis(50));
        Ok(())
    }

    fn key_up(&mut self, key: EvdevKey) -> Result<()> {
        if let Some(device) = self.device.as_mut() {
            device.emit(&[
                InputEvent::new(EvdevEventType::KEY.0, key.0, 0),
                InputEvent::new(0, 0, 0),
            ])?;
        }
        thread::sleep(time::Duration::from_millis(50));
        Ok(())
    }

    fn key_press(&mut self, key: EvdevKey, shift: bool) -> Result<()> {
        if shift {
            self.key_down(EvdevKey::KEY_LEFTSHIFT)?;
        }
        self.key_down(key)?;
        self.key_up(key)?;
        if shift {
            self.key_up(EvdevKey::KEY_LEFTSHIFT)?;
        }
        Ok(())
    }

    fn key_unicode(&mut self, unicode_char: char) -> Result<()> {
        // Start the Ctrl+Shift+U sequence
        self.key_down(EvdevKey::KEY_LEFTCTRL)?;
        self.key_down(EvdevKey::KEY_LEFTSHIFT)?;
        self.key_down(EvdevKey::KEY_U)?;
        self.key_up(EvdevKey::KEY_U)?;
        self.key_up(EvdevKey::KEY_LEFTSHIFT)?;
        self.key_up(EvdevKey::KEY_LEFTCTRL)?;

        thread::sleep(time::Duration::from_millis(100)); // Delay to allow the system to register the sequence

        // Get the hex value of the character
        let hex_string = format!("{:x}", unicode_char as u32);
        
        // Type out the hex value
        for c in hex_string.chars() {
            if let Some(&(key, shift)) = self.key_map.get(&c) {
                self.key_press(key, shift)?;
            }
        }
        
        // Press Enter to confirm the Unicode character
        self.key_press(EvdevKey::KEY_ENTER, false)?;

        Ok(())
    }

    pub fn string_to_unicode_keys(&mut self, s: &str) -> Result<()> {
        for c in s.chars() {
            if c.is_ascii() {
                if let Some(&(key, shift)) = self.key_map.get(&c) {
                    self.key_press(key, shift)?;
                } else {
                    debug!("Unknown character: {}", c);
                }
            } else {
                self.key_unicode(c)?;
            }
        }
        Ok(())
    }

    pub fn string_to_keypresses(&mut self, s: &str) -> Result<()> {
        for c in s.chars() {
            if let Some(&(key, shift)) = self.key_map.get(&c) {
                self.key_press(key, shift)?;
            } else {
                debug!("Character not in map: {}", c);
                // The original code tried to send EV_MSC events, which we are now replacing
                // with a more robust Unicode handling method.
                if self.device.is_some() {
                    self.key_unicode(c)?;
                }
            }
            thread::sleep(time::Duration::from_millis(10));
        }
        Ok(())
    }

    pub fn key_cmd(&mut self, button: &str, shift: bool) -> Result<()> {
        self.key_down(EvdevKey::KEY_LEFTCTRL)?;
        if shift {
            self.key_down(EvdevKey::KEY_LEFTSHIFT)?;
        }
        self.string_to_unicode_keys(button)?;
        if shift {
            self.key_up(EvdevKey::KEY_LEFTSHIFT)?;
        }
        self.key_up(EvdevKey::KEY_LEFTCTRL)?;
        Ok(())
    }

    pub fn key_cmd_title(&mut self) -> Result<()> {
        self.key_cmd("1", false)?;
        Ok(())
    }

    pub fn key_cmd_subheading(&mut self) -> Result<()> {
        self.key_cmd("2", false)?;
        Ok(())
    }

    pub fn key_cmd_body(&mut self) -> Result<()> {
        self.key_cmd("3", false)?;
        Ok(())
    }

    pub fn key_cmd_bullet(&mut self) -> Result<()> {
        self.key_cmd("4", false)?;
        Ok(())
    }

    pub fn progress(&mut self, note: &str) -> Result<()> {
        if self.no_draw_progress {
            return Ok(());
        }
        self.string_to_unicode_keys(note)?;
        self.progress_count += note.len() as u32;
        Ok(())
    }
}
