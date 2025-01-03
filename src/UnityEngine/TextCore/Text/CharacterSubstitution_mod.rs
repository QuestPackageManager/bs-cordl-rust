#[cfg(feature = "UnityEngine+TextCore+Text+CharacterSubstitution")]
#[repr(C)]
#[derive(Debug, Clone, Default)]
pub struct CharacterSubstitution {
    pub index: i32,
    pub unicode: u32,
}
#[cfg(feature = "UnityEngine+TextCore+Text+CharacterSubstitution")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::TextCore::Text::CharacterSubstitution => "UnityEngine.TextCore.Text"
    ."CharacterSubstitution"
);
#[cfg(feature = "UnityEngine+TextCore+Text+CharacterSubstitution")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::TextCore::Text::CharacterSubstitution {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+TextCore+Text+CharacterSubstitution")]
impl crate::UnityEngine::TextCore::Text::CharacterSubstitution {
    pub fn _ctor(
        &mut self,
        index: i32,
        unicode: u32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (index, unicode),
        )?;
        Ok(__cordl_ret.into())
    }
}
