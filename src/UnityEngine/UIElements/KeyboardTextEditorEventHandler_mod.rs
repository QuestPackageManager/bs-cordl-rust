#[cfg(feature = "UnityEngine+UIElements+KeyboardTextEditorEventHandler")]
#[repr(C)]
#[derive(Debug)]
pub struct KeyboardTextEditorEventHandler {
    __cordl_parent: crate::UnityEngine::UIElements::TextEditorEventHandler,
    pub m_ImguiEvent: *mut crate::UnityEngine::Event,
    pub m_Changed: bool,
}
#[cfg(feature = "UnityEngine+UIElements+KeyboardTextEditorEventHandler")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::UIElements::KeyboardTextEditorEventHandler => "UnityEngine.UIElements"
    ."KeyboardTextEditorEventHandler"
);
#[cfg(feature = "UnityEngine+UIElements+KeyboardTextEditorEventHandler")]
impl std::ops::Deref for crate::UnityEngine::UIElements::KeyboardTextEditorEventHandler {
    type Target = crate::UnityEngine::UIElements::TextEditorEventHandler;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+KeyboardTextEditorEventHandler")]
impl std::ops::DerefMut
for crate::UnityEngine::UIElements::KeyboardTextEditorEventHandler {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+KeyboardTextEditorEventHandler")]
impl crate::UnityEngine::UIElements::KeyboardTextEditorEventHandler {
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
    pub fn New(
        textElement: *mut crate::UnityEngine::UIElements::TextElement,
        editingUtilities: *mut crate::UnityEngine::TextEditingUtilities,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (textElement, editingUtilities))?;
        Ok(__cordl_object)
    }
    pub fn OnBlur(
        &mut self,
        _: *mut crate::UnityEngine::UIElements::BlurEvent,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnBlur", (_))?;
        Ok(__cordl_ret)
    }
    pub fn OnExecuteCommandEvent(
        &mut self,
        evt: *mut crate::UnityEngine::UIElements::ExecuteCommandEvent,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnExecuteCommandEvent", (evt))?;
        Ok(__cordl_ret)
    }
    pub fn OnFocus(
        &mut self,
        _: *mut crate::UnityEngine::UIElements::FocusEvent,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnFocus", (_))?;
        Ok(__cordl_ret)
    }
    pub fn OnKeyDown(
        &mut self,
        evt: *mut crate::UnityEngine::UIElements::KeyDownEvent,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnKeyDown", (evt))?;
        Ok(__cordl_ret)
    }
    pub fn OnNavigationEvent<TEvent>(
        &mut self,
        evt: *mut crate::UnityEngine::UIElements::NavigationEventBase_1<TEvent>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TEvent: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnNavigationEvent", (evt))?;
        Ok(__cordl_ret)
    }
    pub fn OnValidateCommandEvent(
        &mut self,
        evt: *mut crate::UnityEngine::UIElements::ValidateCommandEvent,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnValidateCommandEvent", (evt))?;
        Ok(__cordl_ret)
    }
    pub fn UpdateLabel(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UpdateLabel", ())?;
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
}
#[cfg(feature = "UnityEngine+UIElements+KeyboardTextEditorEventHandler")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::UIElements::KeyboardTextEditorEventHandler {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
