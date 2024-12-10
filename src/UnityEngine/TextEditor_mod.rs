#[cfg(feature = "UnityEngine+TextEditor")]
#[repr(C)]
#[derive(Debug)]
pub struct TextEditor {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub keyboardOnScreen: *mut crate::UnityEngine::TouchScreenKeyboard,
    pub controlID: i32,
    pub style: *mut crate::UnityEngine::GUIStyle,
    pub multiline: bool,
    pub hasHorizontalCursorPos: bool,
    pub isPasswordField: bool,
    pub scrollOffset: crate::UnityEngine::Vector2,
    pub m_Content: *mut crate::UnityEngine::GUIContent,
    pub m_CursorIndex: i32,
    pub m_SelectIndex: i32,
    pub m_RevealCursor: bool,
    pub m_MouseDragSelectsWholeWords: bool,
    pub m_DblClickInitPos: i32,
    pub m_DblClickSnap: crate::UnityEngine::TextEditor_DblClickSnapping,
    pub m_bJustSelected: bool,
    pub m_iAltCursorPos: i32,
}
#[cfg(feature = "UnityEngine+TextEditor")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::TextEditor => "UnityEngine"
    ."TextEditor"
);
#[cfg(feature = "UnityEngine+TextEditor")]
impl std::ops::Deref for crate::UnityEngine::TextEditor {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+TextEditor")]
impl std::ops::DerefMut for crate::UnityEngine::TextEditor {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+TextEditor")]
impl crate::UnityEngine::TextEditor {
    #[cfg(feature = "UnityEngine+TextEditor+DblClickSnapping")]
    pub type DblClickSnapping = crate::UnityEngine::TextEditor_DblClickSnapping;
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+TextEditor")]
impl quest_hook::libil2cpp::ObjectType for crate::UnityEngine::TextEditor {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "UnityEngine+TextEditor+DblClickSnapping")]
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TextEditor_DblClickSnapping {
    PARAGRAPHS = 1u8,
    WORDS = 0u8,
}
#[cfg(feature = "UnityEngine+TextEditor+DblClickSnapping")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::TextEditor_DblClickSnapping =>
    "UnityEngine"."TextEditor/DblClickSnapping"
);
