#[cfg(feature = "HoudiniEngineUnity+SessionConnectionState")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum SessionConnectionState {
    #[default]
    CONNECTED = 1i32,
    FAILED_TO_CONNECT = 2i32,
    NOT_CONNECTED = 0i32,
}
#[cfg(feature = "HoudiniEngineUnity+SessionConnectionState")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::HoudiniEngineUnity::SessionConnectionState =>
    "HoudiniEngineUnity"."SessionConnectionState"
);
