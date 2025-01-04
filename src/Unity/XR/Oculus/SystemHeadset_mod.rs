#[cfg(feature = "Unity+XR+Oculus+SystemHeadset")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum SystemHeadset {
    #[default]
    Meta_Link_Quest_Pro = 4103i32,
    Meta_Quest_Pro = 10i32,
    None = 0i32,
    Oculus_Link_Quest = 4101i32,
    Oculus_Link_Quest_2 = 4102i32,
    Oculus_Quest = 8i32,
    Oculus_Quest_2 = 9i32,
    PC_Placeholder_4104 = 4104i32,
    PC_Placeholder_4105 = 4105i32,
    PC_Placeholder_4106 = 4106i32,
    PC_Placeholder_4107 = 4107i32,
    Placeholder_11 = 11i32,
    Placeholder_12 = 12i32,
    Placeholder_13 = 13i32,
    Placeholder_14 = 14i32,
    Rift_CB = 4099i32,
    Rift_CV1 = 4098i32,
    Rift_DK1 = 4096i32,
    Rift_DK2 = 4097i32,
    Rift_S = 4100i32,
}
#[cfg(feature = "Unity+XR+Oculus+SystemHeadset")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::Unity::XR::Oculus::SystemHeadset =>
    "Unity.XR.Oculus"."SystemHeadset"
);
