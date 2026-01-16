#[cfg(feature = "cordl_class_UnityEngine+InputSystem+Key")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[repr(i32)]
pub enum Key {
    #[default]
    A = 15i32,
    AltGr = 54i32,
    B = 16i32,
    Backquote = 4i32,
    Backslash = 10i32,
    Backspace = 65i32,
    C = 17i32,
    CapsLock = 72i32,
    Comma = 7i32,
    ContextMenu = 59i32,
    D = 18i32,
    Delete = 71i32,
    Digit0 = 50i32,
    Digit1 = 41i32,
    Digit2 = 42i32,
    Digit3 = 43i32,
    Digit4 = 44i32,
    Digit5 = 45i32,
    Digit6 = 46i32,
    Digit7 = 47i32,
    Digit8 = 48i32,
    Digit9 = 49i32,
    DownArrow = 64i32,
    E = 19i32,
    End = 69i32,
    Enter = 2i32,
    Equals = 14i32,
    Escape = 60i32,
    F = 20i32,
    F1 = 94i32,
    F10 = 103i32,
    F11 = 104i32,
    F12 = 105i32,
    F2 = 95i32,
    F3 = 96i32,
    F4 = 97i32,
    F5 = 98i32,
    F6 = 99i32,
    F7 = 100i32,
    F8 = 101i32,
    F9 = 102i32,
    G = 21i32,
    H = 22i32,
    Home = 68i32,
    I = 23i32,
    IMESelected = 111i32,
    Insert = 70i32,
    J = 24i32,
    K = 25i32,
    L = 26i32,
    LeftAlt = 53i32,
    LeftApple = 57i32,
    LeftArrow = 61i32,
    LeftBracket = 11i32,
    LeftCtrl = 55i32,
    LeftShift = 51i32,
    M = 27i32,
    Minus = 13i32,
    N = 28i32,
    None = 0i32,
    NumLock = 73i32,
    Numpad0 = 84i32,
    Numpad1 = 85i32,
    Numpad2 = 86i32,
    Numpad3 = 87i32,
    Numpad4 = 88i32,
    Numpad5 = 89i32,
    Numpad6 = 90i32,
    Numpad7 = 91i32,
    Numpad8 = 92i32,
    Numpad9 = 93i32,
    NumpadDivide = 78i32,
    NumpadEnter = 77i32,
    NumpadEquals = 83i32,
    NumpadMinus = 81i32,
    NumpadMultiply = 79i32,
    NumpadPeriod = 82i32,
    NumpadPlus = 80i32,
    O = 29i32,
    OEM1 = 106i32,
    OEM2 = 107i32,
    OEM3 = 108i32,
    OEM4 = 109i32,
    OEM5 = 110i32,
    P = 30i32,
    PageDown = 66i32,
    PageUp = 67i32,
    Pause = 76i32,
    Period = 8i32,
    PrintScreen = 74i32,
    Q = 31i32,
    Quote = 5i32,
    R = 32i32,
    RightApple = 58i32,
    RightArrow = 62i32,
    RightBracket = 12i32,
    RightCtrl = 56i32,
    RightShift = 52i32,
    S = 33i32,
    ScrollLock = 75i32,
    Semicolon = 6i32,
    Slash = 9i32,
    Space = 1i32,
    T = 34i32,
    Tab = 3i32,
    U = 35i32,
    UpArrow = 63i32,
    V = 36i32,
    W = 37i32,
    X = 38i32,
    Y = 39i32,
    Z = 40i32,
}
#[cfg(feature = "cordl_class_UnityEngine+InputSystem+Key")]
unsafe impl quest_hook::libil2cpp::Type for crate::UnityEngine::InputSystem::Key {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine.InputSystem";
    const CLASS_NAME: &'static str = "Key";
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
#[cfg(feature = "cordl_class_UnityEngine+InputSystem+Key")]
unsafe impl quest_hook::libil2cpp::Argument for crate::UnityEngine::InputSystem::Key {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "cordl_class_UnityEngine+InputSystem+Key")]
unsafe impl quest_hook::libil2cpp::Parameter for crate::UnityEngine::InputSystem::Key {
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
#[cfg(feature = "cordl_class_UnityEngine+InputSystem+Key")]
unsafe impl quest_hook::libil2cpp::Returned for crate::UnityEngine::InputSystem::Key {
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
#[cfg(feature = "cordl_class_UnityEngine+InputSystem+Key")]
unsafe impl quest_hook::libil2cpp::Return for crate::UnityEngine::InputSystem::Key {
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
