#[cfg(feature = "UnityEngine+UIElements+TextEditingManipulator")]
#[repr(C)]
#[derive(Debug)]
pub struct TextEditingManipulator {
    __cordl_parent: crate::System::Object,
    pub m_TextElement: *mut crate::UnityEngine::UIElements::TextElement,
    pub editingEventHandler: *mut crate::UnityEngine::UIElements::TextEditorEventHandler,
    pub editingUtilities: *mut crate::UnityEngine::TextEditingUtilities,
    pub m_TouchScreenTextFieldInitialized: bool,
    pub m_HardwareKeyboardPoller: *mut crate::UnityEngine::UIElements::IVisualElementScheduledItem,
}
#[cfg(feature = "UnityEngine+UIElements+TextEditingManipulator")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::UIElements::TextEditingManipulator
    => "UnityEngine.UIElements"."TextEditingManipulator"
);
#[cfg(feature = "UnityEngine+UIElements+TextEditingManipulator")]
impl std::ops::Deref for crate::UnityEngine::UIElements::TextEditingManipulator {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+TextEditingManipulator")]
impl std::ops::DerefMut for crate::UnityEngine::UIElements::TextEditingManipulator {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+TextEditingManipulator")]
impl crate::UnityEngine::UIElements::TextEditingManipulator {
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
    pub fn InitTextEditorEventHandler(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("InitTextEditorEventHandler", ())?;
        Ok(__cordl_ret)
    }
    pub fn New(
        textElement: *mut crate::UnityEngine::UIElements::TextElement,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (textElement))?;
        Ok(__cordl_object)
    }
    pub fn OnFocusInEvent(
        &mut self,
        _: *mut crate::UnityEngine::UIElements::FocusInEvent,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnFocusInEvent", (_))?;
        Ok(__cordl_ret)
    }
    pub fn OnFocusOutEvent(
        &mut self,
        _: *mut crate::UnityEngine::UIElements::FocusOutEvent,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnFocusOutEvent", (_))?;
        Ok(__cordl_ret)
    }
    pub fn _OnFocusInEvent_b__10_0(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("<OnFocusInEvent>b__10_0", ())?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
        textElement: *mut crate::UnityEngine::UIElements::TextElement,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (textElement))?;
        Ok(__cordl_ret)
    }
    pub fn get_touchScreenTextFieldChanged(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("get_touchScreenTextFieldChanged", ())?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "UnityEngine+UIElements+TextEditingManipulator")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::UIElements::TextEditingManipulator {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
