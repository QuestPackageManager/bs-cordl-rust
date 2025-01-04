#[cfg(feature = "UnityEngine+NetworkReachability")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum NetworkReachability {
    #[default]
    NotReachable = 0i32,
    ReachableViaCarrierDataNetwork = 1i32,
    ReachableViaLocalAreaNetwork = 2i32,
}
#[cfg(feature = "UnityEngine+NetworkReachability")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::NetworkReachability =>
    "UnityEngine"."NetworkReachability"
);
