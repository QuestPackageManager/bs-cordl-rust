#[cfg(feature = "UnityEngine+UIElements+TextEditingManipulator")]
#[repr(C)]
#[derive(Debug)]
pub struct TextEditingManipulator {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub m_TextElement: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::UIElements::TextElement,
    >,
    pub editingEventHandler: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::UIElements::TextEditorEventHandler,
    >,
    pub editingUtilities: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::TextEditingUtilities,
    >,
    pub m_TouchScreenTextFieldInitialized: bool,
    pub m_HardwareKeyboardPoller: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::UIElements::IVisualElementScheduledItem,
    >,
}
#[cfg(feature = "UnityEngine+UIElements+TextEditingManipulator")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::UIElements::TextEditingManipulator
    => "UnityEngine.UIElements"."TextEditingManipulator"
);
#[cfg(feature = "UnityEngine+UIElements+TextEditingManipulator")]
impl std::ops::Deref for crate::UnityEngine::UIElements::TextEditingManipulator {
    type Target = quest_hook::libil2cpp::Il2CppObject;
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
        evt: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::EventBase>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ExecuteDefaultActionAtTarget", (evt))?;
        Ok(__cordl_ret.into())
    }
    pub fn InitTextEditorEventHandler(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("InitTextEditorEventHandler", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn New(
        textElement: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::TextElement,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (textElement))?;
        Ok(__cordl_object.into())
    }
    pub fn OnFocusInEvent(
        &mut self,
        _cordl__: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::FocusInEvent>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnFocusInEvent", (_cordl__))?;
        Ok(__cordl_ret.into())
    }
    pub fn OnFocusOutEvent(
        &mut self,
        _cordl__: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::FocusOutEvent,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnFocusOutEvent", (_cordl__))?;
        Ok(__cordl_ret.into())
    }
    pub fn _OnFocusInEvent_b__10_0(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("<OnFocusInEvent>b__10_0", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        textElement: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::TextElement,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (textElement))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_touchScreenTextFieldChanged(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("get_touchScreenTextFieldChanged", ())?;
        Ok(__cordl_ret.into())
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
