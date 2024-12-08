#[cfg(feature = "UnityEngine+TouchScreenKeyboardType")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TouchScreenKeyboardType {
    ASCIICapable = 1i32,
    DecimalPad = 11i32,
    Default = 0i32,
    EmailAddress = 7i32,
    NamePhonePad = 6i32,
    NintendoNetworkAccount = 8i32,
    NumberPad = 4i32,
    NumbersAndPunctuation = 2i32,
    OneTimeCode = 12i32,
    PhonePad = 5i32,
    Search = 10i32,
    Social = 9i32,
    URL = 3i32,
}
#[cfg(feature = "UnityEngine+TouchScreenKeyboardType")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::TouchScreenKeyboardType =>
    "UnityEngine"."TouchScreenKeyboardType"
);
