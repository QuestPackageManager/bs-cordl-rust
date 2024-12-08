#[cfg(feature = "BGLib+Polyglot+Language")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Language {
    Arabic = 26i32,
    Bosnian = 27i32,
    Bulgarian = 24i32,
    Czech = 20i32,
    Danish = 10i32,
    Debug_English_Reverted = 29i32,
    Debug_Keys = 28i32,
    Debug_Word_With_Max_Lenght = 30i32,
    Dutch = 13i32,
    English = 0i32,
    Finnish = 15i32,
    French = 1i32,
    German = 3i32,
    Greek = 8i32,
    Hebrew = 25i32,
    Hungarian = 21i32,
    Italian = 4i32,
    Japanese = 16i32,
    Korean = 19i32,
    Norwegian = 11i32,
    Polish = 14i32,
    Portuguese = 6i32,
    Portuguese_Brazil = 5i32,
    Romanian = 22i32,
    Russian = 7i32,
    Simplified_Chinese = 17i32,
    Spanish = 2i32,
    Swedish = 12i32,
    Thai = 23i32,
    Traditional_Chinese = 18i32,
    Turkish = 9i32,
}
#[cfg(feature = "BGLib+Polyglot+Language")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::BGLib::Polyglot::Language => "BGLib.Polyglot"
    ."Language"
);
