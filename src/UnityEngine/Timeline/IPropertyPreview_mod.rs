#[cfg(feature = "UnityEngine+Timeline+IPropertyPreview")]
#[repr(C)]
#[derive(Debug)]
pub struct IPropertyPreview {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "UnityEngine+Timeline+IPropertyPreview")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::Timeline::IPropertyPreview =>
    "UnityEngine.Timeline"."IPropertyPreview"
);
#[cfg(feature = "UnityEngine+Timeline+IPropertyPreview")]
impl std::ops::Deref for crate::UnityEngine::Timeline::IPropertyPreview {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Timeline+IPropertyPreview")]
impl std::ops::DerefMut for crate::UnityEngine::Timeline::IPropertyPreview {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Timeline+IPropertyPreview")]
impl crate::UnityEngine::Timeline::IPropertyPreview {
    pub fn GatherProperties(
        &mut self,
        director: *mut crate::UnityEngine::Playables::PlayableDirector,
        driver: *mut crate::UnityEngine::Timeline::IPropertyCollector,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("GatherProperties", (director, driver))?;
        Ok(__cordl_ret)
    }
    pub fn from_object_mut(
        object_param: *mut quest_hook::libil2cpp::Il2CppObject,
    ) -> *mut Self {
        unsafe { (object_param as *mut Self) }
    }
}
#[cfg(feature = "UnityEngine+Timeline+IPropertyPreview")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::Timeline::IPropertyPreview {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
