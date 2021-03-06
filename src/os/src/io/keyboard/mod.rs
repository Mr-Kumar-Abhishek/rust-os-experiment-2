pub use self::scancode::ScanCode;

use stream::{Stream, Subscriber};

mod scancode;
mod qwerty;
mod qwertz;

pub fn init<S>(key_presses: S) where S: Stream<Item=ScanCode> {
    let mut parser = scancode::Parser::new();
    let mut qwerty_parser = qwertz::Parser::new();
    key_presses.filter_map(move |code| parser.parse_code(code))
        .filter_map(move |key_press| qwerty_parser.parse(key_press))
        .subscribe(Dummy);
}

struct Dummy;

impl Subscriber<Input> for Dummy {
    fn on_value(&mut self, input: Input) {
        if let Input::Char(c) = input {
            print!("{}", c);
        }
    }
}

#[derive(Debug)]
enum Input {
    Char(char),
    ControlKeyPressed(ControlKey),
    ControlKeyReleased(ControlKey),
}

#[derive(Debug)]
enum KeyPress {
    KeyPressed(Key),
    KeyReleased(Key),
}

#[derive(Debug)]
enum Key {
    Control(ControlKey),
    Printable(PrintableKey),
}

#[derive(Debug)]
enum ControlKey {
    Escape,
    Backspace,
    Tab,
    Enter,
    KeypadEnter,

    LeftCtrl,
    LeftAlt,
    LeftShift,
    LeftGui,
    RightCtrl,
    RightAlt,
    RightShift,
    RightGui,

    Home,
    End,
    Delete,
    Insert,
    PageUp,
    PageDown,

    CursorUp,
    CursorDown,
    CursorLeft,
    CursorRight,

    NumberLock,
    CapsLock,
    ScrollLock,
    PrintScreen,
    Pause,

    F1,
    F2,
    F3,
    F4,
    F5,
    F6,
    F7,
    F8,
    F9,
    F10,
    F11,
    F12,

    AcpiPower,
    AcpiSleep,
    AcpiWake,

    MultimediaStop,
    MultimediaPlayPause,
    MultmediaNext,
    MultimediaPrevious,
    MultimediaHome,
    MultimediaEmail,
    MultimediaSearch,
    MultimediaRefresh,
    MultimediaForward,
    MultimediaBack,
    MultmediaMediaSelect,
    MultimediaComputer,
    MultimediaVolumeUp,
    MultimediaVolumeDown,
    MultimediaMute,
    MultimediaCalculator,
    MultimediaFavourites,
    Apps,
}

#[derive(Debug)]
enum PrintableKey {
    Space,

    Comma,
    Point,
    Semicolon,
    Slash,
    Backslash,
    LeftBracket,
    RightBracket,
    Equal,
    SingleQuote,
    BackTick,
    Minus,

    Number0,
    Number1,
    Number2,
    Number3,
    Number4,
    Number5,
    Number6,
    Number7,
    Number8,
    Number9,

    A,
    B,
    C,
    D,
    E,
    F,
    G,
    H,
    I,
    J,
    K,
    L,
    M,
    N,
    O,
    P,
    Q,
    R,
    S,
    T,
    U,
    V,
    W,
    X,
    Y,
    Z,

    Keypad0,
    Keypad1,
    Keypad2,
    Keypad3,
    Keypad4,
    Keypad5,
    Keypad6,
    Keypad7,
    Keypad8,
    Keypad9,

    KeypadPlus,
    KeypadMinus,
    KeypadStar,
    KeypadSlash,
    KeypadPoint,
}
