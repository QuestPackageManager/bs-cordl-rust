#[cfg(feature = "UnityEngine+RangeInt")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct RangeInt {
    pub start: i32,
    pub length: i32,
}
#[cfg(feature = "UnityEngine+RangeInt")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::RangeInt => "UnityEngine"
    ."RangeInt"
);
#[cfg(feature = "UnityEngine+RangeInt")]
unsafe impl quest_hook::libil2cpp::ThisArgument for crate::UnityEngine::RangeInt {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+RangeInt")]
impl crate::UnityEngine::RangeInt {
    pub fn _ctor(
        &mut self,
        start: i32,
        length: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (start, length),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_end(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_end",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
}
