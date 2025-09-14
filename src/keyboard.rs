use anyhow::Result;
use log::debug;

use std::collections::HashMap;
use std::{thread, time};

use evdev::{
    uinput::VirtualDevice, AttributeSet, EventType as EvdevEventType, InputEvent,
    KeyCode as EvdevKey,
};

pub struct Keyboard {
    device: Option<VirtualDevice>,
    key_map: HashMap<char, (EvdevKey, bool)>,
    progress_count: u32,
    no_draw_progress: bool,
}

impl Keyboard {
    pub fn new(no_draw: bool, no_draw_progress: bool) -> Self {
        let device = if no_draw {
            None
        } else {
            Some(Self::create_virtual_device())
        };

        Self {
            device,
            key_map: Self::create_key_map(),
            progress_count: 0,
            no_draw_progress,
        }
    }

    fn create_virtual_device() -> VirtualDevice {
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

        // Add punctuation and special keys
        keys.insert(EvdevKey::KEY_SPACE);
        keys.insert(EvdevKey::KEY_ENTER);
        keys.insert(EvdevKey::KEY_TAB);
        keys.insert(EvdevKey::KEY_LEFTSHIFT);
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

        keys.insert(EvdevKey::KEY_BACKSPACE);
        keys.insert(EvdevKey::KEY_ESC);

        keys.insert(EvdevKey::KEY_LEFTCTRL);
        keys.insert(EvdevKey::KEY_LEFTALT);

        VirtualDevice::builder()
            .unwrap()
            .name("Virtual Keyboard")
            .with_keys(&keys)
            .unwrap()
            .build()
            .unwrap()
    }

    fn create_key_map() -> HashMap<char, (EvdevKey, bool)> {
        let mut key_map = HashMap::new();

        // Basic ASCII characters
        let basic_chars = [
            // Lowercase letters
            ('a', EvdevKey::KEY_A, false),
            ('b', EvdevKey::KEY_B, false),
            ('c', EvdevKey::KEY_C, false),
            ('d', EvdevKey::KEY_D, false),
            ('e', EvdevKey::KEY_E, false),
            ('f', EvdevKey::KEY_F, false),
            ('g', EvdevKey::KEY_G, false),
            ('h', EvdevKey::KEY_H, false),
            ('i', EvdevKey::KEY_I, false),
            ('j', EvdevKey::KEY_J, false),
            ('k', EvdevKey::KEY_K, false),
            ('l', EvdevKey::KEY_L, false),
            ('m', EvdevKey::KEY_M, false),
            ('n', EvdevKey::KEY_N, false),
            ('o', EvdevKey::KEY_O, false),
            ('p', EvdevKey::KEY_P, false),
            ('q', EvdevKey::KEY_Q, false),
            ('r', EvdevKey::KEY_R, false),
            ('s', EvdevKey::KEY_S, false),
            ('t', EvdevKey::KEY_T, false),
            ('u', EvdevKey::KEY_U, false),
            ('v', EvdevKey::KEY_V, false),
            ('w', EvdevKey::KEY_W, false),
            ('x', EvdevKey::KEY_X, false),
            ('y', EvdevKey::KEY_Y, false),
            ('z', EvdevKey::KEY_Z, false),
            
            // Uppercase letters
            ('A', EvdevKey::KEY_A, true),
            ('B', EvdevKey::KEY_B, true),
            ('C', EvdevKey::KEY_C, true),
            ('D', EvdevKey::KEY_D, true),
            ('E', EvdevKey::KEY_E, true),
            ('F', EvdevKey::KEY_F, true),
            ('G', EvdevKey::KEY_G, true),
            ('H', EvdevKey::KEY_H, true),
            ('I', EvdevKey::KEY_I, true),
            ('J', EvdevKey::KEY_J, true),
            ('K', EvdevKey::KEY_K, true),
            ('L', EvdevKey::KEY_L, true),
            ('M', EvdevKey::KEY_M, true),
            ('N', EvdevKey::KEY_N, true),
            ('O', EvdevKey::KEY_O, true),
            ('P', EvdevKey::KEY_P, true),
            ('Q', EvdevKey::KEY_Q, true),
            ('R', EvdevKey::KEY_R, true),
            ('S', EvdevKey::KEY_S, true),
            ('T', EvdevKey::KEY_T, true),
            ('U', EvdevKey::KEY_U, true),
            ('V', EvdevKey::KEY_V, true),
            ('W', EvdevKey::KEY_W, true),
            ('X', EvdevKey::KEY_X, true),
            ('Y', EvdevKey::KEY_Y, true),
            ('Z', EvdevKey::KEY_Z, true),
            
            // Numbers
            ('0', EvdevKey::KEY_0, false),
            ('1', EvdevKey::KEY_1, false),
            ('2', EvdevKey::KEY_2, false),
            ('3', EvdevKey::KEY_3, false),
            ('4', EvdevKey::KEY_4, false),
            ('5', EvdevKey::KEY_5, false),
            ('6', EvdevKey::KEY_6, false),
            ('7', EvdevKey::KEY_7, false),
            ('8', EvdevKey::KEY_8, false),
            ('9', EvdevKey::KEY_9, false),
            
            // Special characters
            ('!', EvdevKey::KEY_1, true),
            ('@', EvdevKey::KEY_2, true),
            ('#', EvdevKey::KEY_3, true),
            ('$', EvdevKey::KEY_4, true),
            ('%', EvdevKey::KEY_5, true),
            ('^', EvdevKey::KEY_6, true),
            ('&', EvdevKey::KEY_7, true),
            ('*', EvdevKey::KEY_8, true),
            ('(', EvdevKey::KEY_9, true),
            (')', EvdevKey::KEY_0, true),
            ('_', EvdevKey::KEY_MINUS, true),
            ('+', EvdevKey::KEY_EQUAL, true),
            ('{', EvdevKey::KEY_LEFTBRACE, true),
            ('}', EvdevKey::KEY_RIGHTBRACE, true),
            ('|', EvdevKey::KEY_BACKSLASH, true),
            (':', EvdevKey::KEY_SEMICOLON, true),
            ('"', EvdevKey::KEY_APOSTROPHE, true),
            ('<', EvdevKey::KEY_COMMA, true),
            ('>', EvdevKey::KEY_DOT, true),
            ('?', EvdevKey::KEY_SLASH, true),
            ('~', EvdevKey::KEY_GRAVE, true),
            
            // Common punctuation
            ('-', EvdevKey::KEY_MINUS, false),
            ('=', EvdevKey::KEY_EQUAL, false),
            ('[', EvdevKey::KEY_LEFTBRACE, false),
            (']', EvdevKey::KEY_RIGHTBRACE, false),
            ('\\', EvdevKey::KEY_BACKSLASH, false),
            (';', EvdevKey::KEY_SEMICOLON, false),
            ('\'', EvdevKey::KEY_APOSTROPHE, false),
            (',', EvdevKey::KEY_COMMA, false),
            ('.', EvdevKey::KEY_DOT, false),
            ('/', EvdevKey::KEY_SLASH, false),
            ('`', EvdevKey::KEY_GRAVE, false),
            
            // Whitespace
            (' ', EvdevKey::KEY_SPACE, false),
            ('\t', EvdevKey::KEY_TAB, false),
            ('\n', EvdevKey::KEY_ENTER, false),
            
            // Action keys
            ('\x08', EvdevKey::KEY_BACKSPACE, false),
            ('\x1b', EvdevKey::KEY_ESC, false),
        ];

        for (char, key, shift) in basic_chars {
            key_map.insert(char, (key, shift));
        }

        // Unicode character handling - map accented characters to their base letters
        // This handles French, Spanish, German, and many other European languages
        let unicode_mappings = [
            // French accented characters
            ('à', 'a'), ('â', 'a'), ('ä', 'a'), ('è', 'e'), ('é', 'e'), ('ê', 'e'), ('ë', 'e'),
            ('î', 'i'), ('ï', 'i'), ('ô', 'o'), ('ö', 'o'), ('ù', 'u'), ('û', 'u'), ('ü', 'u'),
            ('ÿ', 'y'), ('ç', 'c'), ('œ', 'o'), ('æ', 'a'),
            
            // Uppercase versions
            ('À', 'A'), ('Â', 'A'), ('Ä', 'A'), ('È', 'E'), ('É', 'E'), ('Ê', 'E'), ('Ë', 'E'),
            ('Î', 'I'), ('Ï', 'I'), ('Ô', 'O'), ('Ö', 'O'), ('Ù', 'U'), ('Û', 'U'), ('Ü', 'U'),
            ('Ÿ', 'Y'), ('Ç', 'C'), ('Œ', 'O'), ('Æ', 'A'),
            
            // Spanish characters
            ('ñ', 'n'), ('Ñ', 'N'), ('¿', '?'), ('¡', '!'),
            
            // German characters
            ('ß', 's'), 
            
            // Scandinavian characters
            ('å', 'a'), ('Å', 'A'), ('ø', 'o'), ('Ø', 'O'), ('æ', 'a'), ('Æ', 'A'),
            
            // Other common European characters
            ('€', 'e'), ('£', 'l'), ('¥', 'y'),
        ];

        for (accented_char, base_char) in unicode_mappings {
            if let Some(&(key, shift)) = key_map.get(&base_char) {
                key_map.insert(accented_char, (key, shift));
            }
        }

        key_map
    }

    pub fn key_down(&mut self, key: EvdevKey) -> Result<()> {
        if let Some(device) = &mut self.device {
            device.emit(&[(InputEvent::new(EvdevEventType::KEY.0, key.code(), 1))])?;
            device.emit(&[InputEvent::new(EvdevEventType::SYNCHRONIZATION.0, 0, 0)])?;
            thread::sleep(time::Duration::from_millis(1));
        }
        Ok(())
    }

    pub fn key_up(&mut self, key: EvdevKey) -> Result<()> {
        if let Some(device) = &mut self.device {
            device.emit(&[(InputEvent::new(EvdevEventType::KEY.0, key.code(), 0))])?;
            device.emit(&[InputEvent::new(EvdevEventType::SYNCHRONIZATION.0, 0, 0)])?;
            thread::sleep(time::Duration::from_millis(1));
        }
        Ok(())
    }

    pub fn string_to_keypresses(&mut self, input: &str) -> Result<()> {
        if let Some(device) = &mut self.device {
            // make sure we are synced before we start; this might be paranoia
            device.emit(&[InputEvent::new(EvdevEventType::SYNCHRONIZATION.0, 0, 0)])?;
            thread::sleep(time::Duration::from_millis(10));

            for c in input.chars() {
                if let Some(&(key, shift)) = self.key_map.get(&c) {
                    if shift {
                        // Press Shift
                        device.emit(&[InputEvent::new(
                            EvdevEventType::KEY.0,
                            EvdevKey::KEY_LEFTSHIFT.code(),
                            1,
                        )])?;
                    }

                    // Press key
                    device.emit(&[InputEvent::new(EvdevEventType::KEY.0, key.code(), 1)])?;

                    // Release key
                    device.emit(&[InputEvent::new(EvdevEventType::KEY.0, key.code(), 0)])?;

                    if shift {
                        // Release Shift
                        device.emit(&[InputEvent::new(
                            EvdevEventType::KEY.0,
                            EvdevKey::KEY_LEFTSHIFT.code(),
                            0,
                        )])?;
                    }

                    // Sync event
                    device.emit(&[InputEvent::new(EvdevEventType::SYNCHRONIZATION.0, 0, 0)])?;
                    thread::sleep(time::Duration::from_millis(10));
                }
            }
        }
        Ok(())
    }

    fn key_cmd(&mut self, button: &str, shift: bool) -> Result<()> {
        self.key_down(EvdevKey::KEY_LEFTCTRL)?;
        if shift {
            self.key_down(EvdevKey::KEY_LEFTSHIFT)?;
        }
        self.string_to_keypresses(button)?;
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
        self.string_to_keypresses(note)?;
        self.progress_count += note.len() as u32;
        Ok(())
    }

    pub fn progress_end(&mut self) -> Result<()> {
        if self.no_draw_progress {
            return Ok(());
        }
        // Send a backspace for each progress
        for _ in 0..self.progress_count {
            self.string_to_keypresses("\x08")?;
        }
        self.progress_count = 0;
        Ok(())
    }
}
