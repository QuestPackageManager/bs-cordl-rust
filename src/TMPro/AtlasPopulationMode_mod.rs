#[cfg(feature = "TMPro+AtlasPopulationMode")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum AtlasPopulationMode {
    #[default]
    Dynamic = 1i32,
    Static = 0i32,
}
#[cfg(feature = "TMPro+AtlasPopulationMode")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::TMPro::AtlasPopulationMode => "TMPro"
    ."AtlasPopulationMode"
);
