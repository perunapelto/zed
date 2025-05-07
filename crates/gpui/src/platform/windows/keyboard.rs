use anyhow::{Context, Result};
use windows::Win32::UI::{
    Input::KeyboardAndMouse::{
        GetKeyboardLayoutNameW, MAPVK_VK_TO_VSC, MapVirtualKeyW, ToUnicode, VIRTUAL_KEY, VK_SHIFT,
        VkKeyScanW,
    },
    WindowsAndMessaging::KL_NAMELENGTH,
};
use windows_core::HSTRING;

use crate::{
    Modifiers, PlatformKeyboardLayout, PlatformKeyboardMapper, is_alphabetic_key, is_immutable_key,
};

pub(crate) struct WindowsKeyboardLayout {
    id: String,
    name: String,
}

impl PlatformKeyboardLayout for WindowsKeyboardLayout {
    fn id(&self) -> &str {
        &self.id
    }

    fn name(&self) -> &str {
        &self.name
    }
}

impl WindowsKeyboardLayout {
    pub(crate) fn new() -> Result<Self> {
        let mut buffer = [0u16; KL_NAMELENGTH as usize];
        unsafe { GetKeyboardLayoutNameW(&mut buffer)? };
        let id = HSTRING::from_wide(&buffer).to_string();
        let entry = windows_registry::LOCAL_MACHINE.open(format!(
            "System\\CurrentControlSet\\Control\\Keyboard Layouts\\{}",
            id
        ))?;
        let name = entry.get_hstring("Layout Text")?.to_string();
        Ok(Self { id, name })
    }

    pub(crate) fn unknown() -> Self {
        Self {
            id: "unknown".to_string(),
            name: "unknown".to_string(),
        }
    }
}

pub(crate) struct WindowsKeyboardMapper;

impl PlatformKeyboardMapper for WindowsKeyboardMapper {
    fn get_shifted_key(&self, key: &str) -> Result<String> {
        if is_immutable_key(key) {
            return Ok(key.to_string());
        }
        if is_alphabetic_key(key) {
            return Ok(key.to_uppercase());
        }
        get_shifted_character(key)
    }
}

impl WindowsKeyboardMapper {
    pub(crate) fn new() -> Self {
        Self
    }
}

fn get_vkey_from_char(key: &str, modifiers: &mut Modifiers) -> Result<VIRTUAL_KEY> {
    if key.chars().count() != 1 {
        anyhow::bail!("Key must be a single character, but got: {}", key);
    }
    let key_char = key
        .encode_utf16()
        .next()
        .context("Empty key in keystorke")?;
    let result = unsafe { VkKeyScanW(key_char) };
    if result == -1 {
        anyhow::bail!("Failed to get vkey from char: {}", key);
    }
    let high = (result >> 8) as i8;
    let low = result as u8;
    let (shift, ctrl, alt) = get_modifiers(high);
    if ctrl {
        if modifiers.control {
            anyhow::bail!(
                "Error parsing: {}, Ctrl modifier already set, but ctrl is required for this key: {}, you may be unable to use this shortcut.",
                display_keystroke(key, modifiers),
                key
            );
        }
        modifiers.control = true;
    }
    if alt {
        if modifiers.alt {
            anyhow::bail!(
                "Error parsing: {}, Alt modifier already set, but alt is required for this key: {}, you may be unable to use this shortcut.",
                display_keystroke(key, modifiers),
                key
            );
        }
        modifiers.alt = true;
    }
    if shift {
        if modifiers.shift {
            anyhow::bail!(
                "Error parsing: {}, Shift modifier already set, but shift is required for this key: {}, you may be unable to use this shortcut.",
                display_keystroke(key, modifiers),
                key
            );
        }
        modifiers.shift = true;
    }
    Ok(VIRTUAL_KEY(low as u16))
}

fn get_modifiers(high: i8) -> (bool, bool, bool) {
    let shift = high & 1;
    let ctrl = (high >> 1) & 1;
    let alt = (high >> 2) & 1;
    (shift != 0, ctrl != 0, alt != 0)
}

fn get_shifted_character(key: &str) -> Result<String> {
    let mut modifiers = Modifiers::default();
    let virtual_key = get_vkey_from_char(key, &mut modifiers).context(format!(
        "Failed to get virtual key from char while key_to_shifted: {}",
        key
    ))?;
    if modifiers != Modifiers::default() {
        return Err(anyhow::anyhow!(
            "Key is not a single character or has modifiers: {}",
            key
        ));
    }

    let mut state = [0; 256];
    state[VK_SHIFT.0 as usize] = 0x80;

    let scan_code = unsafe { MapVirtualKeyW(virtual_key.0 as u32, MAPVK_VK_TO_VSC) };
    let mut buffer = [0; 4];
    let len = unsafe {
        ToUnicode(
            virtual_key.0 as u32,
            scan_code,
            Some(&state),
            &mut buffer,
            0,
        )
    };

    if len > 0 {
        let candidate = String::from_utf16_lossy(&buffer[..len as usize]);
        if !candidate.is_empty() && !candidate.chars().next().unwrap().is_control() {
            return Ok(candidate);
        }
    }

    Err(anyhow::anyhow!("Failed to get shifted key for: {}", key))
}

fn display_keystroke(key: &str, modifiers: &Modifiers) -> String {
    let mut display = String::new();
    if modifiers.platform {
        display.push_str("win-");
    }
    if modifiers.control {
        display.push_str("ctrl-");
    }
    if modifiers.shift {
        display.push_str("shift-");
    }
    if modifiers.alt {
        display.push_str("alt-");
    }
    display.push_str(key);
    display
}
