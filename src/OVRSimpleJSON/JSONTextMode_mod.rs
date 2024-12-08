#[cfg(feature = "OVRSimpleJSON+JSONTextMode")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum JSONTextMode {
    Compact = 0i32,
    Indent = 1i32,
}
#[cfg(feature = "OVRSimpleJSON+JSONTextMode")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::OVRSimpleJSON::JSONTextMode => "OVRSimpleJSON"
    ."JSONTextMode"
);
