#[cfg(feature = "UnityEngine+UIElements+TouchScreenTextEditorEventHandler")]
#[repr(C)]
#[derive(Debug)]
pub struct TouchScreenTextEditorEventHandler {
    __cordl_parent: crate::UnityEngine::UIElements::TextEditorEventHandler,
    pub m_TouchKeyboardPoller: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::UIElements::IVisualElementScheduledItem,
    >,
    pub m_TouchKeyboardAllowsInPlaceEditing: bool,
    pub m_IsClicking: bool,
}
#[cfg(feature = "UnityEngine+UIElements+TouchScreenTextEditorEventHandler")]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::UIElements::TouchScreenTextEditorEventHandler {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine.UIElements";
    const CLASS_NAME: &'static str = "TouchScreenTextEditorEventHandler";
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.class().is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_value_argument(_: &quest_hook::libil2cpp::Il2CppType) -> bool {
        false
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
    fn matches_value_parameter(_: &quest_hook::libil2cpp::Il2CppType) -> bool {
        false
    }
}
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
    pub fn CloseTouchScreenKeyboard(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("CloseTouchScreenKeyboard", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn DoPollTouchScreenKeyboard(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("DoPollTouchScreenKeyboard", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn ExecuteDefaultActionAtTarget(
        &mut self,
        evt: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::EventBase>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ExecuteDefaultActionAtTarget", (evt))?;
        Ok(__cordl_ret.into())
    }
    pub fn New(
        textElement: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::TextElement,
        >,
        editingUtilities: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::TextEditingUtilities,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (textElement, editingUtilities))?;
        Ok(__cordl_object.into())
    }
    pub fn OnFocusInEvent(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnFocusInEvent", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn OnFocusOutEvent(
        &mut self,
        evt: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::FocusOutEvent>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnFocusOutEvent", (evt))?;
        Ok(__cordl_ret.into())
    }
    pub fn OnPointerDownEvent(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnPointerDownEvent", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn OnPointerUpEvent(
        &mut self,
        evt: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::PointerUpEvent>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnPointerUpEvent", (evt))?;
        Ok(__cordl_ret.into())
    }
    pub fn OpenTouchScreenKeyboard(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OpenTouchScreenKeyboard", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn PollTouchScreenKeyboard(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("PollTouchScreenKeyboard", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn UpdateStringPositionFromKeyboard(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UpdateStringPositionFromKeyboard", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        textElement: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::TextElement,
        >,
        editingUtilities: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::TextEditingUtilities,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (textElement, editingUtilities))?;
        Ok(__cordl_ret.into())
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
