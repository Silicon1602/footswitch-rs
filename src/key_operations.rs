// http://www.freebsddiary.org/APC/usb_hid_usages.php
static KEY_MAP : &[(&str, u8)] = &[
    ("<00>"                , 0x00),
    ("<01>"                , 0x01),
    ("<02>"                , 0x02),
    ("<03>"                , 0x03),
    ("a"                   , 0x04),
    ("b"                   , 0x05),
    ("c"                   , 0x06),
    ("d"                   , 0x07),
    ("e"                   , 0x08),
    ("f"                   , 0x09),
    ("g"                   , 0x0a),
    ("h"                   , 0x0b),
    ("i"                   , 0x0c),
    ("j"                   , 0x0d),
    ("k"                   , 0x0e),
    ("l"                   , 0x0f),
    ("m"                   , 0x10),
    ("n"                   , 0x11),
    ("o"                   , 0x12),
    ("p"                   , 0x13),
    ("q"                   , 0x14),
    ("r"                   , 0x15),
    ("s"                   , 0x16),
    ("t"                   , 0x17),
    ("u"                   , 0x18),
    ("v"                   , 0x19),
    ("w"                   , 0x1a),
    ("x"                   , 0x1b),
    ("y"                   , 0x1c),
    ("z"                   , 0x1d),
    ("1"                   , 0x1e),
    ("2"                   , 0x1f),
    ("3"                   , 0x20),
    ("4"                   , 0x21),
    ("5"                   , 0x22),
    ("6"                   , 0x23),
    ("7"                   , 0x24),
    ("8"                   , 0x25),
    ("9"                   , 0x26),
    ("0"                   , 0x27),
    ("enter"               , 0x28),
    ("Return"              , 0x28),
    ("esc"                 , 0x29),
    ("Escape"              , 0x29),
    ("backspace"           , 0x2a),
    ("tab"                 , 0x2b),
    (" "                   , 0x2c),
    ("space"               , 0x2c),
    ("-"                   , 0x2d),
    ("="                   , 0x2e),
    ("["                   , 0x2f),
    ("]"                   , 0x30),
    ("\\"                  , 0x31),
    ("\\"                  , 0x32), // yes a repeat
    (";"                   , 0x33),
    ("\'"                  , 0x34),
    ("`"                   , 0x35),
    (","                   , 0x36),
    ("."                   , 0x37),
    ("/"                   , 0x38),
    ("capslock"            , 0x39),
    ("f1"                  , 0x3a),
    ("f2"                  , 0x3b),
    ("f3"                  , 0x3c),
    ("f4"                  , 0x3d),
    ("f5"                  , 0x3e),
    ("f6"                  , 0x3f),
    ("f7"                  , 0x40),
    ("f8"                  , 0x41),
    ("f9"                  , 0x42),
    ("f10"                 , 0x43),
    ("f11"                 , 0x44),
    ("f12"                 , 0x45),
    ("f13"                 , 0x68),
    ("f14"                 , 0x69),
    ("f15"                 , 0x6A),
    ("f16"                 , 0x6B),
    ("f17"                 , 0x6C),
    ("f18"                 , 0x6D),
    ("f19"                 , 0x6E),
    ("f20"                 , 0x6F),
    ("f21"                 , 0x70),
    ("f22"                 , 0x71),
    ("f23"                 , 0x72),
    ("f24"                 , 0x73),
    ("printscreen"         , 0x46),
    ("scrollock"           , 0x47),
    ("pause"               , 0x48),
    ("insert"              , 0x49),
    ("home"                , 0x4a),
    ("pageup"              , 0x4b),
    ("Prior"               , 0x4b),
    ("delete"              , 0x4c),
    ("end"                 , 0x4d),
    ("pagedown"            , 0x4e),
    ("Next"                , 0x4e),
    ("right"               , 0x4f),
    ("down"                , 0x51),
    ("left"                , 0x50),
    ("up"                  , 0x52),
    ("numlock"             , 0x53),
    ("KP_Divide"           , 0x54),
    ("KP_Multiply"         , 0x55),
    ("KP_Subtract"         , 0x56),
    ("KP_Add"              , 0x57),
    ("KP_Enter"            , 0x58),
    ("KP_End"              , 0x59),
    ("KP_Down"             , 0x5a),
    ("KP_Next"             , 0x5b),
    ("KP_Left"             , 0x5c),
    ("KP_Begin"            , 0x5d),
    ("KP_Right"            , 0x5e),
    ("KP_Home"             , 0x5f),
    ("KP_Up"               , 0x60),
    ("KP_Prior"            , 0x61),
    ("KP_Insert"           , 0x62),
    ("KP_Delete"           , 0x63),
    ("less"                , 0x64),
    ("Multi_key"           , 0x65),
    ("compose"             , 0x65),
    ("XF86PowerOff"        , 0x66),
    ("KP_Equal"            , 0x67),
    ("XF86Tools"           , 0x68),
    ("XF86Launch5"         , 0x69),
    ("XF86MenuKB"          , 0x6a),
    ("XF86Launch7"         , 0x6b),
    ("XF86Launch8"         , 0x6c),
    ("XF86Launch9"         , 0x6d),
    ("<6e>"                , 0x6e),
    ("<6f>"                , 0x6f),
    ("XF86TouchpadToggle"  , 0x70),
    ("XF86TouchpadToggle"  , 0x71),
    ("XF86TouchpadOff"     , 0x72),
    ("<73>"                , 0x73),
    ("SunOpen"             , 0x74),
    ("Help"                , 0x75),
    ("SunProps"            , 0x76),
    ("SunFront"            , 0x77),
    ("Cancel"              , 0x78),
    ("Redo"                , 0x79),
    ("Undo"                , 0x7a),
    ("XF86Cut"             , 0x7b),
    ("XF86Copy"            , 0x7c),
    ("XF86Paste"           , 0x7d),
    ("Find"                , 0x7e),
    ("XF86AudioMute"       , 0x7f),
    ("XF86AudioRaiseVolume", 0x80),
    ("XF86AudioLowerVolume", 0x81),
    ("<82>"                , 0x82),
    ("<83>"                , 0x83),
    ("A"                   , 0x84),
    ("B"                   , 0x85),
    ("C"                   , 0x86),
    ("D"                   , 0x87),
    ("E"                   , 0x88),
    ("F"                   , 0x89),
    ("G"                   , 0x8a),
    ("H"                   , 0x8b),
    ("I"                   , 0x8c),
    ("J"                   , 0x8d),
    ("K"                   , 0x8e),
    ("L"                   , 0x8f),
    ("M"                   , 0x90),
    ("N"                   , 0x91),
    ("O"                   , 0x92),
    ("P"                   , 0x93),
    ("Q"                   , 0x94),
    ("R"                   , 0x95),
    ("S"                   , 0x96),
    ("T"                   , 0x97),
    ("U"                   , 0x98),
    ("V"                   , 0x99),
    ("W"                   , 0x9a),
    ("X"                   , 0x9b),
    ("Y"                   , 0x9c),
    ("Z"                   , 0x9d),
    ("!"                   , 0x9e),
    ("@"                   , 0x9f),
    ("#"                   , 0xa0),
    ("$"                   , 0xa1),
    ("%"                   , 0xa2),
    ("^"                   , 0xa3),
    ("&"                   , 0xa4),
    ("*"                   , 0xa5),
    ("("                   , 0xa6),
    (")"                   , 0xa7),
    ("<a8>"                , 0xa8),
    ("<a9>"                , 0xa9),
    ("<aa>"                , 0xaa),
    ("<ab>"                , 0xab),
    ("<ac>"                , 0xac),
    ("_"                   , 0xad),
    ("+"                   , 0xae),
    ("("                   , 0xaf),
    (")"                   , 0xb0),
    ("|"                   , 0xb1),
    ("|"                   , 0xb2),
    (":"                   , 0xb3),
    ("\""                  , 0xb4),
    ("~"                   , 0xb5),
    ("<"                   , 0xb6),
    (">"                   , 0xb7),
    ("?"                   , 0xb8),
    ("<b9>"                , 0xb9),
    ("<ba>"                , 0xba),
    ("<bb>"                , 0xbb),
    ("<bc>"                , 0xbc),
    ("<bd>"                , 0xbd),
    ("<be>"                , 0xbe),
    ("<bf>"                , 0xbf),
    ("<c0>"                , 0xc0),
    ("<c1>"                , 0xc1),
    ("<c2>"                , 0xc2),
    ("<c3>"                , 0xc3),
    ("<c4>"                , 0xc4),
    ("<c5>"                , 0xc5),
    ("<c6>"                , 0xc6),
    ("<c7>"                , 0xc7),
    ("<c8>"                , 0xc8),
    ("<c9>"                , 0xc9),
    ("<ca>"                , 0xca),
    ("<cb>"                , 0xcb),
    ("<cc>"                , 0xcc),
    ("<cd>"                , 0xcd),
    ("<ce>"                , 0xce),
    ("<cf>"                , 0xcf),
    ("<d0>"                , 0xd0),
    ("<d1>"                , 0xd1),
    ("<d2>"                , 0xd2),
    ("<d3>"                , 0xd3),
    ("<d4>"                , 0xd4),
    ("<d5>"                , 0xd5),
    ("<d6>"                , 0xd6),
    ("<d7>"                , 0xd7),
    ("<d8>"                , 0xd8),
    ("<d9>"                , 0xd9),
    ("<da>"                , 0xda),
    ("<db>"                , 0xdb),
    ("<dc>"                , 0xdc),
    ("<dd>"                , 0xdd),
    ("<de>"                , 0xde),
    ("<df>"                , 0xdf),
    ("Control_L"           , 0xe0),
    ("Shift_L"             , 0xe1),
    ("Alt_L"               , 0xe2),
    ("Super_L"             , 0xe3),
    ("Control_R"           , 0xe4),
    ("Shift_R"             , 0xe5),
    ("Meta_R"              , 0xe6),
    ("Super_R"             , 0xe7),
    ("XF86AudioPause"      , 0xe8),
    ("XF86Eject"           , 0xe9),
    ("XF86AudioPrev"       , 0xea),
    ("XF86AudioNext"       , 0xeb),
    ("XF86Eject"           , 0xec),
    ("XF86AudioRaiseVolume", 0xed),
    ("XF86AudioLowerVolume", 0xee),
    ("XF86AudioMute"       , 0xef),
    ("XF86WWW"             , 0xf0),
    ("XF86Back"            , 0xf1),
    ("XF86Forward"         , 0xf2),
    ("Cancel"              , 0xf3),
    ("Find"                , 0xf4),
    ("XF86ScrollUp"        , 0xf5),
    ("XF86ScrollDown"      , 0xf6),
    ("<f7>"                , 0xf7),
    ("XF86Sleep"           , 0xf8),
    ("XF86ScreenSaver"     , 0xf9),
    ("XF86Reload"          , 0xfa),
    ("XF86Calculator"      , 0xfb),
    ("<fc>"                , 0xfc),
    ("<fd>"                , 0xfd),
    ("<fe>"                , 0xfe),
    ("<ff>"                , 0xff),
];

pub enum Modifier {
    Ctrl = 1,
    Shift = 2,
    Alt = 4,
    Win = 8,
}

impl Modifier {
    pub fn str_to_enum(modifier:&str) -> Option<Modifier> {
        match modifier {
            "ctrl" => Some(Modifier::Ctrl),
            "shift" => Some(Modifier::Shift),
            "alt" => Some(Modifier::Alt),
            "win" => Some(Modifier::Win),
            _ => None,
        }
    }

    pub fn enum_to_string(modifier:Modifier) -> String {
        match modifier {
            Modifier::Ctrl => "ctrl".to_string(),
            Modifier::Shift => "shift".to_string(),
            Modifier::Alt => "alt".to_string(),
            Modifier::Win => "win".to_string(),
        }
    }
}

pub enum MouseButton {
    MouseLeft = 1,
    MouseRight = 2,
    MouseMiddle = 4,
    MouseDouble = 8,
}

impl MouseButton {
    pub fn str_to_enum(mousebutton:&str) -> Option<MouseButton> {
        match mousebutton {
            "left" => Some(MouseButton::MouseLeft),
            "right" => Some(MouseButton::MouseRight),
            "middle" => Some(MouseButton::MouseMiddle),
            "double" => Some(MouseButton::MouseDouble),
            _ => None,
        }
    }

    pub fn u8_to_enum(mousebutton:u8) -> Option<MouseButton> {
        match mousebutton {
            1 => Some(MouseButton::MouseLeft),
            2 => Some(MouseButton::MouseRight),
            3 => Some(MouseButton::MouseMiddle),
            4 => Some(MouseButton::MouseDouble),
            _ => None,
        }
    }

    pub fn enum_to_string(mousebutton:MouseButton) -> String {
        match mousebutton {
            MouseButton::MouseLeft => "left".to_string(),
            MouseButton::MouseRight => "right".to_string(),
            MouseButton::MouseMiddle => "middle".to_string(),
            MouseButton::MouseDouble => "double".to_string(),
        }
    }
}

pub fn encode_byte(c: &str) -> Option<u8> {
    for key in KEY_MAP.iter() {
        if key.0 == c {
            return Some(key.1)
        }
    }

    None
}

pub fn decode_byte(u: &u8) -> Option<String> {
    for key in KEY_MAP.iter() {
        if key.1 == *u {
            let return_value = format!("{}{}{}", "<", key.0.to_string(), ">");
            return Some(return_value)
        }
    }

    None
}

pub fn encode_string(s: &str) -> Option<Vec<u8> > {
    let mut char_vec = Vec::new();

    for c in s.chars() {
        match encode_byte(&c.to_string()) {
           Some(x) => char_vec.push(x),
           None => return None,
        };
    }

    Some(char_vec)
}

pub fn print_key(response: &[u8]) -> Option<String> {
    let mut key_combo = String::new();

    // Handle modifiers
    if response[2] & Modifier::Ctrl as u8 != 0 {
        key_combo.push_str(&Modifier::enum_to_string(Modifier::Ctrl)[..]);
        key_combo.push_str("+");
    }
    if response[2] & Modifier::Shift as u8 != 0 {
        key_combo.push_str(&Modifier::enum_to_string(Modifier::Shift)[..]);
        key_combo.push_str("+");
    }
    if response[2] & Modifier::Alt as u8 != 0 {
        key_combo.push_str(&Modifier::enum_to_string(Modifier::Alt)[..]);
        key_combo.push_str("+");
    }
    if response[2] & Modifier::Win as u8 != 0 {
        key_combo.push_str(&Modifier::enum_to_string(Modifier::Win)[..]);
        key_combo.push_str("+");
    }

    // Handle others keys
    if response[3] != 0 {
        if let Some(key_str) = decode_byte(&response[3]) {
            key_combo.push_str(&key_str);

            return Some(key_combo);
        }
    }

    None
}

pub fn print_mousebutton(response: &[u8]) -> Option<String> {
    let mut mouse_string = String::new();

    match MouseButton::u8_to_enum(response[4]) {
        Some(x) => mouse_string.push_str(&MouseButton::enum_to_string(x)[..]),
        None => {}
    }

    let x:i16 = if response[5] > 127 { response[5] as i16 - 256 } else { response[5] as i16 };
    let y:i16 = if response[6] > 127 { response[6] as i16 - 256 } else { response[6] as i16 };
    let w:i16 = if response[7] > 127 { response[7] as i16 - 256 } else { response[7] as i16 };

    mouse_string.push_str(&format!(" X = {}", x)[..]);
    mouse_string.push_str(&format!(" Y = {}", y)[..]);
    mouse_string.push_str(&format!(" W = {}", w)[..]);

    Some(mouse_string)

}

pub fn print_mouse_key(response: &[u8]) -> Option<String> {
    let mut key_mouse_string = String::new();

    match print_key(&response) {
        Some(x) => {
            key_mouse_string.push_str(&x[..]);
            key_mouse_string.push_str(" ");
        },
        None => {}
    }

    match print_mousebutton(&response) {
        Some(x) => {
            key_mouse_string.push_str(&x[..]);
            key_mouse_string.push_str(" ");
        },
        None => {}
    }

    Some(key_mouse_string)
}

pub fn print_key_map(rows: usize) {
    print!("{}", " ‖ Key Name             ¦ Value     ".repeat(rows));
    println!(" ‖");

    println!(" {}", "-".repeat(rows*36));
    for (i, key) in KEY_MAP.iter().enumerate() {

        print!(" ‖ {name:<-name_width$} ¦ <0x{value:>0value_width$x}>    ",
                 name=key.0, name_width=20, value = key.1, value_width = 2);

        if (i + 1) % rows == 0 {
            println!(" ‖");
        }
    }
}
