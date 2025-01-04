#[cfg(feature = "LightAxis")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum LightAxis {
    #[default]
    X = 0i32,
    Y = 1i32,
    Z = 2i32,
}
#[cfg(feature = "LightAxis")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::LightAxis => ""."LightAxis"
);
