#[cfg(feature = "System+Net+NetworkInformation+AixIoctlRequest")]
#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AixIoctlRequest {
    SIOCGIFCONF = 275334640u32,
    SIOCGIFFLAGS = 677974512u32,
    SIOCGIFMTU = 677992176u32,
    SIOCGIFNETMASK = 677979632u32,
    SIOCGSIZIFCONF = 74017520u32,
}
#[cfg(feature = "System+Net+NetworkInformation+AixIoctlRequest")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::System::Net::NetworkInformation::AixIoctlRequest
    => "System.Net.NetworkInformation"."AixIoctlRequest"
);
