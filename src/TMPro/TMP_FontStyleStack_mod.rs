#[cfg(feature = "TMPro+TMP_FontStyleStack")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct TMP_FontStyleStack {
    pub bold: u8,
    pub italic: u8,
    pub underline: u8,
    pub strikethrough: u8,
    pub highlight: u8,
    pub superscript: u8,
    pub subscript: u8,
    pub uppercase: u8,
    pub lowercase: u8,
    pub smallcaps: u8,
}
#[cfg(feature = "TMPro+TMP_FontStyleStack")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::TMPro::TMP_FontStyleStack => "TMPro"
    ."TMP_FontStyleStack"
);
#[cfg(feature = "TMPro+TMP_FontStyleStack")]
unsafe impl quest_hook::libil2cpp::ThisArgument for crate::TMPro::TMP_FontStyleStack {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "TMPro+TMP_FontStyleStack")]
impl crate::TMPro::TMP_FontStyleStack {
    pub fn Add(
        &mut self,
        style: crate::TMPro::FontStyles,
    ) -> quest_hook::libil2cpp::Result<u8> {
        let __cordl_ret: u8 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Add",
            (style),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn Clear(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Clear",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn Remove(
        &mut self,
        style: crate::TMPro::FontStyles,
    ) -> quest_hook::libil2cpp::Result<u8> {
        let __cordl_ret: u8 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Remove",
            (style),
        )?;
        Ok(__cordl_ret.into())
    }
}
