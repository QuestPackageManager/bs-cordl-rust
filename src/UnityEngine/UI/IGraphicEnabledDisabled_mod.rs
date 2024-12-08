#[cfg(feature = "UnityEngine+UI+IGraphicEnabledDisabled")]
#[repr(C)]
#[derive(Debug)]
pub struct IGraphicEnabledDisabled {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "UnityEngine+UI+IGraphicEnabledDisabled")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::UI::IGraphicEnabledDisabled =>
    "UnityEngine.UI"."IGraphicEnabledDisabled"
);
#[cfg(feature = "UnityEngine+UI+IGraphicEnabledDisabled")]
impl std::ops::Deref for crate::UnityEngine::UI::IGraphicEnabledDisabled {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UI+IGraphicEnabledDisabled")]
impl std::ops::DerefMut for crate::UnityEngine::UI::IGraphicEnabledDisabled {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UI+IGraphicEnabledDisabled")]
impl crate::UnityEngine::UI::IGraphicEnabledDisabled {
    pub fn OnSiblingGraphicEnabledDisabled(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnSiblingGraphicEnabledDisabled", ())?;
        Ok(__cordl_ret)
    }
    pub fn from_object_mut(
        object_param: *mut quest_hook::libil2cpp::Il2CppObject,
    ) -> *mut Self {
        unsafe { (object_param as *mut Self) }
    }
}
#[cfg(feature = "UnityEngine+UI+IGraphicEnabledDisabled")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::UI::IGraphicEnabledDisabled {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
