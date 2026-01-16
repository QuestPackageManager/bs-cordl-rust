#[cfg(feature = "cordl_class_System+ConsoleKey")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[repr(i32)]
pub enum ConsoleKey {
    #[default]
    A = 65i32,
    Add = 107i32,
    Applications = 93i32,
    Attention = 246i32,
    B = 66i32,
    Backspace = 8i32,
    BrowserBack = 166i32,
    BrowserFavorites = 171i32,
    BrowserForward = 167i32,
    BrowserHome = 172i32,
    BrowserRefresh = 168i32,
    BrowserSearch = 170i32,
    BrowserStop = 169i32,
    C = 67i32,
    Clear = 12i32,
    CrSel = 247i32,
    D = 68i32,
    D0 = 48i32,
    D1 = 49i32,
    D2 = 50i32,
    D3 = 51i32,
    D4 = 52i32,
    D5 = 53i32,
    D6 = 54i32,
    D7 = 55i32,
    D8 = 56i32,
    D9 = 57i32,
    Decimal = 110i32,
    Delete = 46i32,
    Divide = 111i32,
    DownArrow = 40i32,
    E = 69i32,
    End = 35i32,
    Enter = 13i32,
    EraseEndOfFile = 249i32,
    Escape = 27i32,
    ExSel = 248i32,
    Execute = 43i32,
    F = 70i32,
    F1 = 112i32,
    F10 = 121i32,
    F11 = 122i32,
    F12 = 123i32,
    F13 = 124i32,
    F14 = 125i32,
    F15 = 126i32,
    F16 = 127i32,
    F17 = 128i32,
    F18 = 129i32,
    F19 = 130i32,
    F2 = 113i32,
    F20 = 131i32,
    F21 = 132i32,
    F22 = 133i32,
    F23 = 134i32,
    F24 = 135i32,
    F3 = 114i32,
    F4 = 115i32,
    F5 = 116i32,
    F6 = 117i32,
    F7 = 118i32,
    F8 = 119i32,
    F9 = 120i32,
    G = 71i32,
    H = 72i32,
    Help = 47i32,
    Home = 36i32,
    I = 73i32,
    Insert = 45i32,
    J = 74i32,
    K = 75i32,
    L = 76i32,
    LaunchApp1 = 182i32,
    LaunchApp2 = 183i32,
    LaunchMail = 180i32,
    LaunchMediaSelect = 181i32,
    LeftArrow = 37i32,
    LeftWindows = 91i32,
    M = 77i32,
    MediaNext = 176i32,
    MediaPlay = 179i32,
    MediaPrevious = 177i32,
    MediaStop = 178i32,
    Multiply = 106i32,
    N = 78i32,
    NoName = 252i32,
    NumPad0 = 96i32,
    NumPad1 = 97i32,
    NumPad2 = 98i32,
    NumPad3 = 99i32,
    NumPad4 = 100i32,
    NumPad5 = 101i32,
    NumPad6 = 102i32,
    NumPad7 = 103i32,
    NumPad8 = 104i32,
    NumPad9 = 105i32,
    O = 79i32,
    Oem1 = 186i32,
    Oem102 = 226i32,
    Oem2 = 191i32,
    Oem3 = 192i32,
    Oem4 = 219i32,
    Oem5 = 220i32,
    Oem6 = 221i32,
    Oem7 = 222i32,
    Oem8 = 223i32,
    OemClear = 254i32,
    OemComma = 188i32,
    OemMinus = 189i32,
    OemPeriod = 190i32,
    OemPlus = 187i32,
    P = 80i32,
    Pa1 = 253i32,
    Packet = 231i32,
    PageDown = 34i32,
    PageUp = 33i32,
    Pause = 19i32,
    Play = 250i32,
    Print = 42i32,
    PrintScreen = 44i32,
    Process = 229i32,
    Q = 81i32,
    R = 82i32,
    RightArrow = 39i32,
    RightWindows = 92i32,
    S = 83i32,
    Select = 41i32,
    Separator = 108i32,
    Sleep = 95i32,
    Spacebar = 32i32,
    Subtract = 109i32,
    T = 84i32,
    Tab = 9i32,
    U = 85i32,
    UpArrow = 38i32,
    V = 86i32,
    VolumeDown = 174i32,
    VolumeMute = 173i32,
    VolumeUp = 175i32,
    W = 87i32,
    X = 88i32,
    Y = 89i32,
    Z = 90i32,
    Zoom = 251i32,
}
#[cfg(feature = "cordl_class_System+ConsoleKey")]
unsafe impl quest_hook::libil2cpp::Type for crate::System::ConsoleKey {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "System";
    const CLASS_NAME: &'static str = "ConsoleKey";
    fn matches_value_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        !ty.is_ref()
            && ty
                .class()
                .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref()
            && ty
                .class()
                .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_value_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        !ty.is_ref()
            && <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref() && <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
}
#[cfg(feature = "cordl_class_System+ConsoleKey")]
unsafe impl quest_hook::libil2cpp::Argument for crate::System::ConsoleKey {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "cordl_class_System+ConsoleKey")]
unsafe impl quest_hook::libil2cpp::Parameter for crate::System::ConsoleKey {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_parameter(ty)
    }
    fn from_actual(actual: <Self as quest_hook::libil2cpp::Parameter>::Actual) -> Self {
        actual
    }
    fn into_actual(self) -> <Self as quest_hook::libil2cpp::Parameter>::Actual {
        self
    }
}
#[cfg(feature = "cordl_class_System+ConsoleKey")]
unsafe impl quest_hook::libil2cpp::Returned for crate::System::ConsoleKey {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_returned(ty)
    }
    fn from_object(object: Option<&mut quest_hook::libil2cpp::Il2CppObject>) -> Self {
        unsafe {
            quest_hook::libil2cpp::raw::unbox(quest_hook::libil2cpp::WrapRaw::raw(object.unwrap()))
        }
    }
}
#[cfg(feature = "cordl_class_System+ConsoleKey")]
unsafe impl quest_hook::libil2cpp::Return for crate::System::ConsoleKey {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_return(ty)
    }
    fn into_actual(self) -> <Self as quest_hook::libil2cpp::Return>::Actual {
        self
    }
    fn from_actual(actual: <Self as quest_hook::libil2cpp::Return>::Actual) -> Self {
        actual
    }
}
