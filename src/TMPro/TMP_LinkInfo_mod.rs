#[cfg(feature = "TMPro+TMP_LinkInfo")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct TMP_LinkInfo {
    pub textComponent: *mut crate::TMPro::TMP_Text,
    pub hashCode: i32,
    pub linkIdFirstCharacterIndex: i32,
    pub linkIdLength: i32,
    pub linkTextfirstCharacterIndex: i32,
    pub linkTextLength: i32,
    pub linkID: *mut quest_hook::libil2cpp::Il2CppArray<char>,
}
#[cfg(feature = "TMPro+TMP_LinkInfo")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::TMPro::TMP_LinkInfo => "TMPro"."TMP_LinkInfo"
);
#[cfg(feature = "TMPro+TMP_LinkInfo")]
unsafe impl quest_hook::libil2cpp::ThisArgument for crate::TMPro::TMP_LinkInfo {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "TMPro+TMP_LinkInfo")]
impl crate::TMPro::TMP_LinkInfo {
    pub fn GetLinkText(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_ret: *mut crate::System::String = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "GetLinkText",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn GetLinkID(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_ret: *mut crate::System::String = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "GetLinkID",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn SetLinkID(
        &mut self,
        text: *mut quest_hook::libil2cpp::Il2CppArray<char>,
        startIndex: i32,
        length: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "SetLinkID",
            (text, startIndex, length),
        )?;
        Ok(__cordl_ret)
    }
}
