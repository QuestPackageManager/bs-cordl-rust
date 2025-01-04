#[cfg(feature = "System+Net+Sockets+IPProtectionLevel")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum IPProtectionLevel {
    #[default]
    EdgeRestricted = 20i32,
    Restricted = 30i32,
    Unrestricted = 10i32,
    Unspecified = -1i32,
}
#[cfg(feature = "System+Net+Sockets+IPProtectionLevel")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::System::Net::Sockets::IPProtectionLevel =>
    "System.Net.Sockets"."IPProtectionLevel"
);
