#[cfg(feature = "UnityEngine+UIElements+Hashes+_hashes_e__FixedBuffer")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct Hashes__hashes_e__FixedBuffer {
    pub FixedElementField: i32,
}
#[cfg(feature = "UnityEngine+UIElements+Hashes+_hashes_e__FixedBuffer")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::UIElements::Hashes__hashes_e__FixedBuffer => "UnityEngine.UIElements"
    ."Hashes/<hashes>e__FixedBuffer"
);
#[cfg(feature = "UnityEngine+UIElements+Hashes+_hashes_e__FixedBuffer")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::UIElements::Hashes__hashes_e__FixedBuffer {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+UIElements+Hashes+_hashes_e__FixedBuffer")]
impl crate::UnityEngine::UIElements::Hashes__hashes_e__FixedBuffer {}
#[cfg(feature = "UnityEngine+UIElements+Hashes")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct Hashes {
    pub hashes: crate::UnityEngine::UIElements::Hashes__hashes_e__FixedBuffer,
}
#[cfg(feature = "UnityEngine+UIElements+Hashes")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::UIElements::Hashes =>
    "UnityEngine.UIElements"."Hashes"
);
#[cfg(feature = "UnityEngine+UIElements+Hashes")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::UIElements::Hashes {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+UIElements+Hashes")]
impl crate::UnityEngine::UIElements::Hashes {
    pub const kSize: i32 = 4i32;
    #[cfg(feature = "UnityEngine+UIElements+Hashes+_hashes_e__FixedBuffer")]
    pub type _hashes_e__FixedBuffer = crate::UnityEngine::UIElements::Hashes__hashes_e__FixedBuffer;
}
