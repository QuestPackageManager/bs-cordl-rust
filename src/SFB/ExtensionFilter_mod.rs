#[cfg(feature = "SFB+ExtensionFilter")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct ExtensionFilter {
    pub _name: *mut quest_hook::libil2cpp::Il2CppString,
    pub _extensions: *mut quest_hook::libil2cpp::Il2CppArray<
        *mut quest_hook::libil2cpp::Il2CppString,
    >,
}
#[cfg(feature = "SFB+ExtensionFilter")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::SFB::ExtensionFilter => "SFB"."ExtensionFilter"
);
#[cfg(feature = "SFB+ExtensionFilter")]
unsafe impl quest_hook::libil2cpp::ThisArgument for crate::SFB::ExtensionFilter {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "SFB+ExtensionFilter")]
impl crate::SFB::ExtensionFilter {
    pub fn _ctor(
        &mut self,
        filterName: *mut quest_hook::libil2cpp::Il2CppString,
        filterExtensions: *mut quest_hook::libil2cpp::Il2CppArray<
            *mut quest_hook::libil2cpp::Il2CppString,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (filterName, filterExtensions),
        )?;
        Ok(__cordl_ret)
    }
}
