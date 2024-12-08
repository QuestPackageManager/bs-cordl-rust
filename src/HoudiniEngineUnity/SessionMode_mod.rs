#[cfg(feature = "HoudiniEngineUnity+SessionMode")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SessionMode {
    Pipe = 1i32,
    Socket = 0i32,
}
#[cfg(feature = "HoudiniEngineUnity+SessionMode")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::HoudiniEngineUnity::SessionMode =>
    "HoudiniEngineUnity"."SessionMode"
);
