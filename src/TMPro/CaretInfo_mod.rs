#[cfg(feature = "TMPro+CaretInfo")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct CaretInfo {
    pub index: i32,
    pub position: crate::TMPro::CaretPosition,
}
#[cfg(feature = "TMPro+CaretInfo")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::TMPro::CaretInfo => "TMPro"."CaretInfo"
);
#[cfg(feature = "TMPro+CaretInfo")]
unsafe impl quest_hook::libil2cpp::ThisArgument for crate::TMPro::CaretInfo {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "TMPro+CaretInfo")]
impl crate::TMPro::CaretInfo {
    pub fn _ctor(
        &mut self,
        index: i32,
        position: crate::TMPro::CaretPosition,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (index, position),
        )?;
        Ok(__cordl_ret.into())
    }
}
