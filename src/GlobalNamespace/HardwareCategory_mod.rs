#[cfg(feature = "HardwareCategory")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HardwareCategory {
    PlayStation4 = 5i32,
    PlayStation4Pro = 6i32,
    PlayStation5 = 7i32,
    Quest1 = 1i32,
    Quest2 = 2i32,
    Quest3 = 3i32,
    QuestPro = 4i32,
    Standalone = 0i32,
}
#[cfg(feature = "HardwareCategory")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::HardwareCategory => ""
    ."HardwareCategory"
);
