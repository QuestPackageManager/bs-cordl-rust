#[cfg(feature = "UnityEngine+TextCore+Text+LinkInfo")]
#[repr(C)]
#[derive(Debug, Clone, Default)]
pub struct LinkInfo {
    pub hashCode: i32,
    pub linkIdFirstCharacterIndex: i32,
    pub linkIdLength: i32,
    pub linkTextfirstCharacterIndex: i32,
    pub linkTextLength: i32,
    pub linkId: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<char>>,
    pub m_LinkIdString: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub m_LinkTextString: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
}
#[cfg(feature = "UnityEngine+TextCore+Text+LinkInfo")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::TextCore::Text::LinkInfo =>
    "UnityEngine.TextCore.Text"."LinkInfo"
);
#[cfg(feature = "UnityEngine+TextCore+Text+LinkInfo")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::TextCore::Text::LinkInfo {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+TextCore+Text+LinkInfo")]
impl crate::UnityEngine::TextCore::Text::LinkInfo {
    pub fn GetLinkId(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = quest_hook::libil2cpp::ValueTypeExt::invoke(self, "GetLinkId", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetLinkText(
        &mut self,
        textInfo: quest_hook::libil2cpp::Gc<crate::UnityEngine::TextCore::Text::TextInfo>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "GetLinkText",
            (textInfo),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn SetLinkId(
        &mut self,
        text: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<char>>,
        startIndex: i32,
        length: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "SetLinkId",
            (text, startIndex, length),
        )?;
        Ok(__cordl_ret.into())
    }
}
