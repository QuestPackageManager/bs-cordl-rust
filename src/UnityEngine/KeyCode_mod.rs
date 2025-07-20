#[cfg(feature = "UnityEngine+KeyCode")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum KeyCode {
    #[default]
    A = 97i32,
    Alpha0 = 48i32,
    Alpha1 = 49i32,
    Alpha2 = 50i32,
    Alpha3 = 51i32,
    Alpha4 = 52i32,
    Alpha5 = 53i32,
    Alpha6 = 54i32,
    Alpha7 = 55i32,
    Alpha8 = 56i32,
    Alpha9 = 57i32,
    AltGr = 313i32,
    Ampersand = 38i32,
    Asterisk = 42i32,
    At = 64i32,
    B = 98i32,
    BackQuote = 96i32,
    Backslash = 92i32,
    Backspace = 8i32,
    Break = 318i32,
    C = 99i32,
    CapsLock = 301i32,
    Caret = 94i32,
    Clear = 12i32,
    Colon = 58i32,
    Comma = 44i32,
    D = 100i32,
    Delete = 127i32,
    Dollar = 36i32,
    DoubleQuote = 34i32,
    DownArrow = 274i32,
    E = 101i32,
    End = 279i32,
    Equals = 61i32,
    Escape = 27i32,
    Exclaim = 33i32,
    F = 102i32,
    F1 = 282i32,
    F10 = 291i32,
    F11 = 292i32,
    F12 = 293i32,
    F13 = 294i32,
    F14 = 295i32,
    F15 = 296i32,
    F2 = 283i32,
    F3 = 284i32,
    F4 = 285i32,
    F5 = 286i32,
    F6 = 287i32,
    F7 = 288i32,
    F8 = 289i32,
    F9 = 290i32,
    G = 103i32,
    Greater = 62i32,
    H = 104i32,
    Hash = 35i32,
    Help = 315i32,
    Home = 278i32,
    I = 105i32,
    Insert = 277i32,
    J = 106i32,
    Joystick1Button0 = 350i32,
    Joystick1Button1 = 351i32,
    Joystick1Button10 = 360i32,
    Joystick1Button11 = 361i32,
    Joystick1Button12 = 362i32,
    Joystick1Button13 = 363i32,
    Joystick1Button14 = 364i32,
    Joystick1Button15 = 365i32,
    Joystick1Button16 = 366i32,
    Joystick1Button17 = 367i32,
    Joystick1Button18 = 368i32,
    Joystick1Button19 = 369i32,
    Joystick1Button2 = 352i32,
    Joystick1Button3 = 353i32,
    Joystick1Button4 = 354i32,
    Joystick1Button5 = 355i32,
    Joystick1Button6 = 356i32,
    Joystick1Button7 = 357i32,
    Joystick1Button8 = 358i32,
    Joystick1Button9 = 359i32,
    Joystick2Button0 = 370i32,
    Joystick2Button1 = 371i32,
    Joystick2Button10 = 380i32,
    Joystick2Button11 = 381i32,
    Joystick2Button12 = 382i32,
    Joystick2Button13 = 383i32,
    Joystick2Button14 = 384i32,
    Joystick2Button15 = 385i32,
    Joystick2Button16 = 386i32,
    Joystick2Button17 = 387i32,
    Joystick2Button18 = 388i32,
    Joystick2Button19 = 389i32,
    Joystick2Button2 = 372i32,
    Joystick2Button3 = 373i32,
    Joystick2Button4 = 374i32,
    Joystick2Button5 = 375i32,
    Joystick2Button6 = 376i32,
    Joystick2Button7 = 377i32,
    Joystick2Button8 = 378i32,
    Joystick2Button9 = 379i32,
    Joystick3Button0 = 390i32,
    Joystick3Button1 = 391i32,
    Joystick3Button10 = 400i32,
    Joystick3Button11 = 401i32,
    Joystick3Button12 = 402i32,
    Joystick3Button13 = 403i32,
    Joystick3Button14 = 404i32,
    Joystick3Button15 = 405i32,
    Joystick3Button16 = 406i32,
    Joystick3Button17 = 407i32,
    Joystick3Button18 = 408i32,
    Joystick3Button19 = 409i32,
    Joystick3Button2 = 392i32,
    Joystick3Button3 = 393i32,
    Joystick3Button4 = 394i32,
    Joystick3Button5 = 395i32,
    Joystick3Button6 = 396i32,
    Joystick3Button7 = 397i32,
    Joystick3Button8 = 398i32,
    Joystick3Button9 = 399i32,
    Joystick4Button0 = 410i32,
    Joystick4Button1 = 411i32,
    Joystick4Button10 = 420i32,
    Joystick4Button11 = 421i32,
    Joystick4Button12 = 422i32,
    Joystick4Button13 = 423i32,
    Joystick4Button14 = 424i32,
    Joystick4Button15 = 425i32,
    Joystick4Button16 = 426i32,
    Joystick4Button17 = 427i32,
    Joystick4Button18 = 428i32,
    Joystick4Button19 = 429i32,
    Joystick4Button2 = 412i32,
    Joystick4Button3 = 413i32,
    Joystick4Button4 = 414i32,
    Joystick4Button5 = 415i32,
    Joystick4Button6 = 416i32,
    Joystick4Button7 = 417i32,
    Joystick4Button8 = 418i32,
    Joystick4Button9 = 419i32,
    Joystick5Button0 = 430i32,
    Joystick5Button1 = 431i32,
    Joystick5Button10 = 440i32,
    Joystick5Button11 = 441i32,
    Joystick5Button12 = 442i32,
    Joystick5Button13 = 443i32,
    Joystick5Button14 = 444i32,
    Joystick5Button15 = 445i32,
    Joystick5Button16 = 446i32,
    Joystick5Button17 = 447i32,
    Joystick5Button18 = 448i32,
    Joystick5Button19 = 449i32,
    Joystick5Button2 = 432i32,
    Joystick5Button3 = 433i32,
    Joystick5Button4 = 434i32,
    Joystick5Button5 = 435i32,
    Joystick5Button6 = 436i32,
    Joystick5Button7 = 437i32,
    Joystick5Button8 = 438i32,
    Joystick5Button9 = 439i32,
    Joystick6Button0 = 450i32,
    Joystick6Button1 = 451i32,
    Joystick6Button10 = 460i32,
    Joystick6Button11 = 461i32,
    Joystick6Button12 = 462i32,
    Joystick6Button13 = 463i32,
    Joystick6Button14 = 464i32,
    Joystick6Button15 = 465i32,
    Joystick6Button16 = 466i32,
    Joystick6Button17 = 467i32,
    Joystick6Button18 = 468i32,
    Joystick6Button19 = 469i32,
    Joystick6Button2 = 452i32,
    Joystick6Button3 = 453i32,
    Joystick6Button4 = 454i32,
    Joystick6Button5 = 455i32,
    Joystick6Button6 = 456i32,
    Joystick6Button7 = 457i32,
    Joystick6Button8 = 458i32,
    Joystick6Button9 = 459i32,
    Joystick7Button0 = 470i32,
    Joystick7Button1 = 471i32,
    Joystick7Button10 = 480i32,
    Joystick7Button11 = 481i32,
    Joystick7Button12 = 482i32,
    Joystick7Button13 = 483i32,
    Joystick7Button14 = 484i32,
    Joystick7Button15 = 485i32,
    Joystick7Button16 = 486i32,
    Joystick7Button17 = 487i32,
    Joystick7Button18 = 488i32,
    Joystick7Button19 = 489i32,
    Joystick7Button2 = 472i32,
    Joystick7Button3 = 473i32,
    Joystick7Button4 = 474i32,
    Joystick7Button5 = 475i32,
    Joystick7Button6 = 476i32,
    Joystick7Button7 = 477i32,
    Joystick7Button8 = 478i32,
    Joystick7Button9 = 479i32,
    Joystick8Button0 = 490i32,
    Joystick8Button1 = 491i32,
    Joystick8Button10 = 500i32,
    Joystick8Button11 = 501i32,
    Joystick8Button12 = 502i32,
    Joystick8Button13 = 503i32,
    Joystick8Button14 = 504i32,
    Joystick8Button15 = 505i32,
    Joystick8Button16 = 506i32,
    Joystick8Button17 = 507i32,
    Joystick8Button18 = 508i32,
    Joystick8Button19 = 509i32,
    Joystick8Button2 = 492i32,
    Joystick8Button3 = 493i32,
    Joystick8Button4 = 494i32,
    Joystick8Button5 = 495i32,
    Joystick8Button6 = 496i32,
    Joystick8Button7 = 497i32,
    Joystick8Button8 = 498i32,
    Joystick8Button9 = 499i32,
    JoystickButton0 = 330i32,
    JoystickButton1 = 331i32,
    JoystickButton10 = 340i32,
    JoystickButton11 = 341i32,
    JoystickButton12 = 342i32,
    JoystickButton13 = 343i32,
    JoystickButton14 = 344i32,
    JoystickButton15 = 345i32,
    JoystickButton16 = 346i32,
    JoystickButton17 = 347i32,
    JoystickButton18 = 348i32,
    JoystickButton19 = 349i32,
    JoystickButton2 = 332i32,
    JoystickButton3 = 333i32,
    JoystickButton4 = 334i32,
    JoystickButton5 = 335i32,
    JoystickButton6 = 336i32,
    JoystickButton7 = 337i32,
    JoystickButton8 = 338i32,
    JoystickButton9 = 339i32,
    K = 107i32,
    Keypad0 = 256i32,
    Keypad1 = 257i32,
    Keypad2 = 258i32,
    Keypad3 = 259i32,
    Keypad4 = 260i32,
    Keypad5 = 261i32,
    Keypad6 = 262i32,
    Keypad7 = 263i32,
    Keypad8 = 264i32,
    Keypad9 = 265i32,
    KeypadDivide = 267i32,
    KeypadEnter = 271i32,
    KeypadEquals = 272i32,
    KeypadMinus = 269i32,
    KeypadMultiply = 268i32,
    KeypadPeriod = 266i32,
    KeypadPlus = 270i32,
    L = 108i32,
    LeftAlt = 308i32,
    LeftApple = 310i32,
    LeftArrow = 276i32,
    LeftBracket = 91i32,
    LeftControl = 306i32,
    LeftCurlyBracket = 123i32,
    LeftParen = 40i32,
    LeftShift = 304i32,
    LeftWindows = 311i32,
    Less = 60i32,
    M = 109i32,
    Menu = 319i32,
    Minus = 45i32,
    Mouse0 = 323i32,
    Mouse1 = 324i32,
    Mouse2 = 325i32,
    Mouse3 = 326i32,
    Mouse4 = 327i32,
    Mouse5 = 328i32,
    Mouse6 = 329i32,
    N = 110i32,
    None = 0i32,
    Numlock = 300i32,
    O = 111i32,
    P = 112i32,
    PageDown = 281i32,
    PageUp = 280i32,
    Pause = 19i32,
    Percent = 37i32,
    Period = 46i32,
    Pipe = 124i32,
    Plus = 43i32,
    Print = 316i32,
    Q = 113i32,
    Question = 63i32,
    Quote = 39i32,
    R = 114i32,
    Return = 13i32,
    RightAlt = 307i32,
    RightApple = 309i32,
    RightArrow = 275i32,
    RightBracket = 93i32,
    RightControl = 305i32,
    RightCurlyBracket = 125i32,
    RightParen = 41i32,
    RightShift = 303i32,
    RightWindows = 312i32,
    S = 115i32,
    ScrollLock = 302i32,
    Semicolon = 59i32,
    Slash = 47i32,
    Space = 32i32,
    SysReq = 317i32,
    T = 116i32,
    Tab = 9i32,
    Tilde = 126i32,
    U = 117i32,
    Underscore = 95i32,
    UpArrow = 273i32,
    V = 118i32,
    W = 119i32,
    X = 120i32,
    Y = 121i32,
    Z = 122i32,
}
#[cfg(feature = "UnityEngine+KeyCode")]
unsafe impl quest_hook::libil2cpp::Type for crate::UnityEngine::KeyCode {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine";
    const CLASS_NAME: &'static str = "KeyCode";
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
            && <Self as quest_hook::libil2cpp::Type>::class()
                .is_assignable_from(ty.class())
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref()
            && <Self as quest_hook::libil2cpp::Type>::class()
                .is_assignable_from(ty.class())
    }
}
#[cfg(feature = "UnityEngine+KeyCode")]
unsafe impl quest_hook::libil2cpp::Argument for crate::UnityEngine::KeyCode {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "UnityEngine+KeyCode")]
unsafe impl quest_hook::libil2cpp::Parameter for crate::UnityEngine::KeyCode {
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
#[cfg(feature = "UnityEngine+KeyCode")]
unsafe impl quest_hook::libil2cpp::Returned for crate::UnityEngine::KeyCode {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_returned(ty)
    }
    fn from_object(object: Option<&mut quest_hook::libil2cpp::Il2CppObject>) -> Self {
        unsafe {
            quest_hook::libil2cpp::raw::unbox(
                quest_hook::libil2cpp::WrapRaw::raw(object.unwrap()),
            )
        }
    }
}
#[cfg(feature = "UnityEngine+KeyCode")]
unsafe impl quest_hook::libil2cpp::Return for crate::UnityEngine::KeyCode {
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
