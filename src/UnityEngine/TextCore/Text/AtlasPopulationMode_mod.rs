#[cfg(feature = "UnityEngine+TextCore+Text+AtlasPopulationMode")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AtlasPopulationMode {
    Dynamic = 1i32,
    DynamicOS = 2i32,
    Static = 0i32,
}
#[cfg(feature = "UnityEngine+TextCore+Text+AtlasPopulationMode")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::TextCore::Text::AtlasPopulationMode
    => "UnityEngine.TextCore.Text"."AtlasPopulationMode"
);
