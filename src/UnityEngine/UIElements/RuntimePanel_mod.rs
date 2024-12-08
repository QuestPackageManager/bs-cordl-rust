#[cfg(feature = "UnityEngine+UIElements+RuntimePanel")]
#[repr(C)]
#[derive(Debug)]
pub struct RuntimePanel {
    __cordl_parent: crate::UnityEngine::UIElements::BaseRuntimePanel,
    pub m_PanelSettings: *mut crate::UnityEngine::UIElements::PanelSettings,
}
#[cfg(feature = "UnityEngine+UIElements+RuntimePanel")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::UIElements::RuntimePanel =>
    "UnityEngine.UIElements"."RuntimePanel"
);
#[cfg(feature = "UnityEngine+UIElements+RuntimePanel")]
impl std::ops::Deref for crate::UnityEngine::UIElements::RuntimePanel {
    type Target = crate::UnityEngine::UIElements::BaseRuntimePanel;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+RuntimePanel")]
impl std::ops::DerefMut for crate::UnityEngine::UIElements::RuntimePanel {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+RuntimePanel")]
impl crate::UnityEngine::UIElements::RuntimePanel {
    #[cfg(feature = "UnityEngine+UIElements+RuntimePanel+__c")]
    pub type __c = crate::UnityEngine::UIElements::RuntimePanel___c;
    pub fn New(
        ownerObject: *mut crate::UnityEngine::ScriptableObject,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (ownerObject))?;
        Ok(__cordl_object)
    }
    pub fn OnElementFocus(
        &mut self,
        evt: *mut crate::UnityEngine::UIElements::FocusEvent,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnElementFocus", (evt))?;
        Ok(__cordl_ret)
    }
    pub fn Update(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Update", ())?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
        ownerObject: *mut crate::UnityEngine::ScriptableObject,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (ownerObject))?;
        Ok(__cordl_ret)
    }
    pub fn get_panelSettings(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::UnityEngine::UIElements::PanelSettings,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::UIElements::PanelSettings = __cordl_object
            .invoke("get_panelSettings", ())?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "UnityEngine+UIElements+RuntimePanel")]
impl quest_hook::libil2cpp::ObjectType for crate::UnityEngine::UIElements::RuntimePanel {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
