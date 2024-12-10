#[cfg(feature = "UnityEngine+UIElements+PanelRaycaster")]
#[repr(C)]
#[derive(Debug)]
pub struct PanelRaycaster {
    __cordl_parent: crate::UnityEngine::EventSystems::BaseRaycaster,
    pub m_Panel: *mut crate::UnityEngine::UIElements::BaseRuntimePanel,
}
#[cfg(feature = "UnityEngine+UIElements+PanelRaycaster")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::UIElements::PanelRaycaster =>
    "UnityEngine.UIElements"."PanelRaycaster"
);
#[cfg(feature = "UnityEngine+UIElements+PanelRaycaster")]
impl std::ops::Deref for crate::UnityEngine::UIElements::PanelRaycaster {
    type Target = crate::UnityEngine::EventSystems::BaseRaycaster;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+PanelRaycaster")]
impl std::ops::DerefMut for crate::UnityEngine::UIElements::PanelRaycaster {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+PanelRaycaster")]
impl crate::UnityEngine::UIElements::PanelRaycaster {
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn OnPanelDestroyed(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnPanelDestroyed", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn Raycast(
        &mut self,
        eventData: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::EventSystems::PointerEventData,
        >,
        resultAppendList: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                crate::UnityEngine::EventSystems::RaycastResult,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Raycast", (eventData, resultAppendList))?;
        Ok(__cordl_ret.into())
    }
    pub fn RegisterCallbacks(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("RegisterCallbacks", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn UnregisterCallbacks(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UnregisterCallbacks", ())?;
        Ok(__cordl_ret.into())
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
    pub fn get_eventCamera(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::Camera>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::Camera> = __cordl_object
            .invoke("get_eventCamera", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_panel(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::IPanel>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::IPanel,
        > = __cordl_object.invoke("get_panel", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_renderOrderPriority(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_renderOrderPriority", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_selectableGameObject(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::GameObject>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::GameObject> = __cordl_object
            .invoke("get_selectableGameObject", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_sortOrderPriority(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_sortOrderPriority", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn set_panel(
        &mut self,
        value: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::IPanel>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_panel", (value))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+UIElements+PanelRaycaster")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::UIElements::PanelRaycaster {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "UnityEngine+UIElements+PanelRaycaster")]
impl AsRef<crate::UnityEngine::UIElements::IRuntimePanelComponent>
for crate::UnityEngine::UIElements::PanelRaycaster {
    fn as_ref(&self) -> &crate::UnityEngine::UIElements::IRuntimePanelComponent {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "UnityEngine+UIElements+PanelRaycaster")]
impl AsMut<crate::UnityEngine::UIElements::IRuntimePanelComponent>
for crate::UnityEngine::UIElements::PanelRaycaster {
    fn as_mut(&mut self) -> &mut crate::UnityEngine::UIElements::IRuntimePanelComponent {
        unsafe { std::mem::transmute(self) }
    }
}
