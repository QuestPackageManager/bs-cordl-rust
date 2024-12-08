#[cfg(feature = "OVR+OpenVR+EVRNotificationStyle")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EVRNotificationStyle {
    Application = 100i32,
    Contact_Active = 202i32,
    Contact_Disabled = 200i32,
    Contact_Enabled = 201i32,
    None = 0i32,
}
#[cfg(feature = "OVR+OpenVR+EVRNotificationStyle")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::OVR::OpenVR::EVRNotificationStyle =>
    "OVR.OpenVR"."EVRNotificationStyle"
);
