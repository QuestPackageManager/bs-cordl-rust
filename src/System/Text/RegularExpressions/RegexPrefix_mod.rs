#[cfg(feature = "System+Text+RegularExpressions+RegexPrefix")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct RegexPrefix {
    pub _CaseInsensitive_k__BackingField: bool,
    pub _Prefix_k__BackingField: *mut crate::System::String,
}
#[cfg(feature = "System+Text+RegularExpressions+RegexPrefix")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::System::Text::RegularExpressions::RegexPrefix =>
    "System.Text.RegularExpressions"."RegexPrefix"
);
#[cfg(feature = "System+Text+RegularExpressions+RegexPrefix")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::System::Text::RegularExpressions::RegexPrefix {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "System+Text+RegularExpressions+RegexPrefix")]
impl crate::System::Text::RegularExpressions::RegexPrefix {
    pub fn _ctor(
        &mut self,
        prefix: *mut crate::System::String,
        ci: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (prefix, ci),
        )?;
        Ok(__cordl_ret)
    }
    pub fn get_CaseInsensitive(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_CaseInsensitive",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn get_Prefix(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_ret: *mut crate::System::String = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_Prefix",
            (),
        )?;
        Ok(__cordl_ret)
    }
}