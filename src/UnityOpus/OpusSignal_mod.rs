#[cfg(feature = "UnityOpus+OpusSignal")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum OpusSignal {
    #[default]
    Auto = -1000i32,
    Music = 3002i32,
    Voice = 3001i32,
}
#[cfg(feature = "UnityOpus+OpusSignal")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityOpus::OpusSignal => "UnityOpus"
    ."OpusSignal"
);
