#[cfg(feature = "UnityEngine+UIElements+ContextualMenuManager")]
#[repr(C)]
#[derive(Debug)]
pub struct ContextualMenuManager {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _displayMenuHandledOSX_k__BackingField: bool,
}
#[cfg(feature = "UnityEngine+UIElements+ContextualMenuManager")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::UIElements::ContextualMenuManager
    => "UnityEngine.UIElements"."ContextualMenuManager"
);
#[cfg(feature = "UnityEngine+UIElements+ContextualMenuManager")]
impl std::ops::Deref for crate::UnityEngine::UIElements::ContextualMenuManager {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+ContextualMenuManager")]
impl std::ops::DerefMut for crate::UnityEngine::UIElements::ContextualMenuManager {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+ContextualMenuManager")]
impl crate::UnityEngine::UIElements::ContextualMenuManager {
    pub fn DisplayMenu(
        &mut self,
        triggerEvent: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::EventBase,
        >,
        target: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::IEventHandler>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("DisplayMenu", (triggerEvent, target))?;
        Ok(__cordl_ret.into())
    }
    pub fn DisplayMenuIfEventMatches(
        &mut self,
        evt: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::EventBase>,
        eventHandler: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::IEventHandler,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("DisplayMenuIfEventMatches", (evt, eventHandler))?;
        Ok(__cordl_ret.into())
    }
    pub fn DoDisplayMenu(
        &mut self,
        menu: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::DropdownMenu>,
        triggerEvent: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::EventBase,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("DoDisplayMenu", (menu, triggerEvent))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_displayMenuHandledOSX(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_displayMenuHandledOSX", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn set_displayMenuHandledOSX(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_displayMenuHandledOSX", (value))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+UIElements+ContextualMenuManager")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::UIElements::ContextualMenuManager {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
