#[cfg(feature = "UnityEngine+UIElements+TouchScreenTextEditorEventHandler")]
#[repr(C)]
#[derive(Debug)]
pub struct TouchScreenTextEditorEventHandler {
    __cordl_parent: crate::UnityEngine::UIElements::TextEditorEventHandler,
    pub m_TouchKeyboardPoller: *mut crate::UnityEngine::UIElements::IVisualElementScheduledItem,
    pub m_TouchKeyboardAllowsInPlaceEditing: bool,
    pub m_IsClicking: bool,
}
#[cfg(feature = "UnityEngine+UIElements+TouchScreenTextEditorEventHandler")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::UIElements::TouchScreenTextEditorEventHandler =>
    "UnityEngine.UIElements"."TouchScreenTextEditorEventHandler"
);
#[cfg(feature = "UnityEngine+UIElements+TouchScreenTextEditorEventHandler")]
impl std::ops::Deref
for crate::UnityEngine::UIElements::TouchScreenTextEditorEventHandler {
    type Target = crate::UnityEngine::UIElements::TextEditorEventHandler;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+TouchScreenTextEditorEventHandler")]
impl std::ops::DerefMut
for crate::UnityEngine::UIElements::TouchScreenTextEditorEventHandler {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+TouchScreenTextEditorEventHandler")]
impl crate::UnityEngine::UIElements::TouchScreenTextEditorEventHandler {
    pub fn OpenTouchScreenKeyboard(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OpenTouchScreenKeyboard", ())?;
        Ok(__cordl_ret)
    }
    pub fn OnPointerUpEvent(
        &mut self,
        evt: *mut crate::UnityEngine::UIElements::PointerUpEvent,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnPointerUpEvent", (evt))?;
        Ok(__cordl_ret)
    }
    pub fn OnFocusInEvent(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnFocusInEvent", ())?;
        Ok(__cordl_ret)
    }
    pub fn CloseTouchScreenKeyboard(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("CloseTouchScreenKeyboard", ())?;
        Ok(__cordl_ret)
    }
    pub fn ExecuteDefaultActionAtTarget(
        &mut self,
        evt: *mut crate::UnityEngine::UIElements::EventBase,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ExecuteDefaultActionAtTarget", (evt))?;
        Ok(__cordl_ret)
    }
    pub fn UpdateStringPositionFromKeyboard(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UpdateStringPositionFromKeyboard", ())?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
        textElement: *mut crate::UnityEngine::UIElements::TextElement,
        editingUtilities: *mut crate::UnityEngine::TextEditingUtilities,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (textElement, editingUtilities))?;
        Ok(__cordl_ret)
    }
    pub fn PollTouchScreenKeyboard(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("PollTouchScreenKeyboard", ())?;
        Ok(__cordl_ret)
    }
    pub fn DoPollTouchScreenKeyboard(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("DoPollTouchScreenKeyboard", ())?;
        Ok(__cordl_ret)
    }
    pub fn OnFocusOutEvent(
        &mut self,
        evt: *mut crate::UnityEngine::UIElements::FocusOutEvent,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnFocusOutEvent", (evt))?;
        Ok(__cordl_ret)
    }
    pub fn OnPointerDownEvent(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnPointerDownEvent", ())?;
        Ok(__cordl_ret)
    }
    pub fn New(
        textElement: *mut crate::UnityEngine::UIElements::TextElement,
        editingUtilities: *mut crate::UnityEngine::TextEditingUtilities,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (textElement, editingUtilities))?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "UnityEngine+UIElements+TouchScreenTextEditorEventHandler")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::UIElements::TouchScreenTextEditorEventHandler {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
