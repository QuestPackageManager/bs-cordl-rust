#[cfg(feature = "UnityEngine+UIElements+VisualElementPanelActivator")]
#[repr(C)]
#[derive(Debug)]
pub struct VisualElementPanelActivator {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub m_Activatable: *mut crate::UnityEngine::UIElements::IVisualElementPanelActivatable,
    pub _isActive_k__BackingField: bool,
    pub _isDetaching_k__BackingField: bool,
    pub m_OnAttachToPanelCallback: *mut crate::UnityEngine::UIElements::EventCallback_1<
        *mut crate::UnityEngine::UIElements::AttachToPanelEvent,
    >,
    pub m_OnDetachFromPanelCallback: *mut crate::UnityEngine::UIElements::EventCallback_1<
        *mut crate::UnityEngine::UIElements::DetachFromPanelEvent,
    >,
}
#[cfg(feature = "UnityEngine+UIElements+VisualElementPanelActivator")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::UIElements::VisualElementPanelActivator => "UnityEngine.UIElements"
    ."VisualElementPanelActivator"
);
#[cfg(feature = "UnityEngine+UIElements+VisualElementPanelActivator")]
impl std::ops::Deref for crate::UnityEngine::UIElements::VisualElementPanelActivator {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+VisualElementPanelActivator")]
impl std::ops::DerefMut for crate::UnityEngine::UIElements::VisualElementPanelActivator {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+VisualElementPanelActivator")]
impl crate::UnityEngine::UIElements::VisualElementPanelActivator {
    pub fn New(
        activatable: *mut crate::UnityEngine::UIElements::IVisualElementPanelActivatable,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (activatable))?;
        Ok(__cordl_object)
    }
    pub fn OnEnter(
        &mut self,
        evt: *mut crate::UnityEngine::UIElements::AttachToPanelEvent,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnEnter", (evt))?;
        Ok(__cordl_ret)
    }
    pub fn OnLeave(
        &mut self,
        evt: *mut crate::UnityEngine::UIElements::DetachFromPanelEvent,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnLeave", (evt))?;
        Ok(__cordl_ret)
    }
    pub fn SendActivation(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SendActivation", ())?;
        Ok(__cordl_ret)
    }
    pub fn SendDeactivation(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SendDeactivation", ())?;
        Ok(__cordl_ret)
    }
    pub fn SetActive(
        &mut self,
        action: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetActive", (action))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
        activatable: *mut crate::UnityEngine::UIElements::IVisualElementPanelActivatable,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (activatable))?;
        Ok(__cordl_ret)
    }
    pub fn get_isActive(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_isActive", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_isDetaching(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_isDetaching", ())?;
        Ok(__cordl_ret)
    }
    pub fn set_isActive(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_isActive", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_isDetaching(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_isDetaching", (value))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "UnityEngine+UIElements+VisualElementPanelActivator")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::UIElements::VisualElementPanelActivator {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
