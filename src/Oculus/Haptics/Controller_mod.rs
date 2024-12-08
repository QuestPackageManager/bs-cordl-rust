#[cfg(feature = "Oculus+Haptics+Controller")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Controller {
    Both = 2i32,
    Left = 0i32,
    Right = 1i32,
}
#[cfg(feature = "Oculus+Haptics+Controller")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::Oculus::Haptics::Controller => "Oculus.Haptics"
    ."Controller"
);
