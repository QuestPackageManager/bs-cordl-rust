#[cfg(feature = "UnityEngine+ResourceManagement+Util+BundleSource")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum BundleSource {
    #[default]
    Cache = 2i32,
    Download = 4i32,
    Local = 1i32,
    None = 0i32,
}
#[cfg(feature = "UnityEngine+ResourceManagement+Util+BundleSource")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::ResourceManagement::Util::BundleSource =>
    "UnityEngine.ResourceManagement.Util"."BundleSource"
);
