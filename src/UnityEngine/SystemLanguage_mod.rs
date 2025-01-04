#[cfg(feature = "UnityEngine+SystemLanguage")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum SystemLanguage {
    #[default]
    Afrikaans = 0i32,
    Arabic = 1i32,
    Basque = 2i32,
    Belarusian = 3i32,
    Bulgarian = 4i32,
    Catalan = 5i32,
    Chinese = 6i32,
    ChineseSimplified = 40i32,
    ChineseTraditional = 41i32,
    Czech = 7i32,
    Danish = 8i32,
    Dutch = 9i32,
    English = 10i32,
    Estonian = 11i32,
    Faroese = 12i32,
    Finnish = 13i32,
    French = 14i32,
    German = 15i32,
    Greek = 16i32,
    Hebrew = 17i32,
    Hindi = 42i32,
    Hungarian = 18i32,
    Icelandic = 19i32,
    Indonesian = 20i32,
    Italian = 21i32,
    Japanese = 22i32,
    Korean = 23i32,
    Latvian = 24i32,
    Lithuanian = 25i32,
    Norwegian = 26i32,
    Polish = 27i32,
    Portuguese = 28i32,
    Romanian = 29i32,
    Russian = 30i32,
    SerboCroatian = 31i32,
    Slovak = 32i32,
    Slovenian = 33i32,
    Spanish = 34i32,
    Swedish = 35i32,
    Thai = 36i32,
    Turkish = 37i32,
    Ukrainian = 38i32,
    Unknown = 43i32,
    Vietnamese = 39i32,
}
#[cfg(feature = "UnityEngine+SystemLanguage")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::SystemLanguage => "UnityEngine"
    ."SystemLanguage"
);
