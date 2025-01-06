#[cfg(feature = "UnityEngine+TextCore+LowLevel+FontEngineUtilities")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct FontEngineUtilities {}
#[cfg(feature = "UnityEngine+TextCore+LowLevel+FontEngineUtilities")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::TextCore::LowLevel::FontEngineUtilities =>
    "UnityEngine.TextCore.LowLevel"."FontEngineUtilities"
);
#[cfg(feature = "UnityEngine+TextCore+LowLevel+FontEngineUtilities")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::TextCore::LowLevel::FontEngineUtilities {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+TextCore+LowLevel+FontEngineUtilities")]
impl crate::UnityEngine::TextCore::LowLevel::FontEngineUtilities {
    pub fn MaxValue(a: i32, b: i32, c: i32) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("MaxValue", (a, b, c))?;
        Ok(__cordl_ret.into())
    }
}
