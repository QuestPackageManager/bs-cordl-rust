#[cfg(feature = "UnityEngine+UIElements+IRuntimePanelComponent")]
#[repr(C)]
#[derive(Debug)]
pub struct IRuntimePanelComponent {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "UnityEngine+UIElements+IRuntimePanelComponent")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::UIElements::IRuntimePanelComponent
    => "UnityEngine.UIElements"."IRuntimePanelComponent"
);
#[cfg(feature = "UnityEngine+UIElements+IRuntimePanelComponent")]
impl std::ops::Deref for crate::UnityEngine::UIElements::IRuntimePanelComponent {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+IRuntimePanelComponent")]
impl std::ops::DerefMut for crate::UnityEngine::UIElements::IRuntimePanelComponent {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+IRuntimePanelComponent")]
impl crate::UnityEngine::UIElements::IRuntimePanelComponent {
    pub fn from_object_mut(
        object_param: *mut quest_hook::libil2cpp::Il2CppObject,
    ) -> *mut Self {
        unsafe { (object_param as *mut Self) }
    }
    pub fn set_panel(
        &mut self,
        value: *mut crate::UnityEngine::UIElements::IPanel,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_panel", (value))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "UnityEngine+UIElements+IRuntimePanelComponent")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::UIElements::IRuntimePanelComponent {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
