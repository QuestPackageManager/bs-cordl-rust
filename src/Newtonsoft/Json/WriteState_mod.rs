#[cfg(feature = "Newtonsoft+Json+WriteState")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WriteState {
    Array = 3i32,
    Closed = 1i32,
    Constructor = 4i32,
    Error = 0i32,
    Object = 2i32,
    Property = 5i32,
    Start = 6i32,
}
#[cfg(feature = "Newtonsoft+Json+WriteState")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::Newtonsoft::Json::WriteState =>
    "Newtonsoft.Json"."WriteState"
);
