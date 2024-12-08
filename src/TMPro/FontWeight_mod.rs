#[cfg(feature = "TMPro+FontWeight")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FontWeight {
    Black = 900i32,
    Bold = 700i32,
    ExtraLight = 200i32,
    Heavy = 800i32,
    Light = 300i32,
    Medium = 500i32,
    Regular = 400i32,
    SemiBold = 600i32,
    Thin = 100i32,
}
#[cfg(feature = "TMPro+FontWeight")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::TMPro::FontWeight => "TMPro"."FontWeight"
);
