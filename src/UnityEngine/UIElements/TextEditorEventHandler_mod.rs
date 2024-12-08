#[cfg(feature = "UnityEngine+UIElements+TextEditorEventHandler")]
#[repr(C)]
#[derive(Debug)]
pub struct TextEditorEventHandler {
    __cordl_parent: crate::System::Object,
    pub textElement: *mut crate::UnityEngine::UIElements::TextElement,
    pub editingUtilities: *mut crate::UnityEngine::TextEditingUtilities,
}
#[cfg(feature = "UnityEngine+UIElements+TextEditorEventHandler")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::UIElements::TextEditorEventHandler
    => "UnityEngine.UIElements"."TextEditorEventHandler"
);
#[cfg(feature = "UnityEngine+UIElements+TextEditorEventHandler")]
impl std::ops::Deref for crate::UnityEngine::UIElements::TextEditorEventHandler {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+TextEditorEventHandler")]
impl std::ops::DerefMut for crate::UnityEngine::UIElements::TextEditorEventHandler {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+TextEditorEventHandler")]
impl crate::UnityEngine::UIElements::TextEditorEventHandler {
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
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (textElement, editingUtilities))?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "UnityEngine+UIElements+TextEditorEventHandler")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::UIElements::TextEditorEventHandler {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
