#[cfg(feature = "TMPro+TMP_WordInfo")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct TMP_WordInfo {
    pub textComponent: quest_hook::libil2cpp::Gc<crate::TMPro::TMP_Text>,
    pub firstCharacterIndex: i32,
    pub lastCharacterIndex: i32,
    pub characterCount: i32,
}
#[cfg(feature = "TMPro+TMP_WordInfo")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::TMPro::TMP_WordInfo => "TMPro"."TMP_WordInfo"
);
#[cfg(feature = "TMPro+TMP_WordInfo")]
unsafe impl quest_hook::libil2cpp::ThisArgument for crate::TMPro::TMP_WordInfo {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "TMPro+TMP_WordInfo")]
impl crate::TMPro::TMP_WordInfo {
    pub fn GetWord(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = quest_hook::libil2cpp::ValueTypeExt::invoke(self, "GetWord", ())?;
        Ok(__cordl_ret.into())
    }
}
