#[cfg(feature = "UnityEngine+Animations+IAnimationWindowPreview")]
#[repr(C)]
#[derive(Debug)]
pub struct IAnimationWindowPreview {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "UnityEngine+Animations+IAnimationWindowPreview")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::Animations::IAnimationWindowPreview
    => "UnityEngine.Animations"."IAnimationWindowPreview"
);
#[cfg(feature = "UnityEngine+Animations+IAnimationWindowPreview")]
impl std::ops::Deref for crate::UnityEngine::Animations::IAnimationWindowPreview {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Animations+IAnimationWindowPreview")]
impl std::ops::DerefMut for crate::UnityEngine::Animations::IAnimationWindowPreview {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Animations+IAnimationWindowPreview")]
impl crate::UnityEngine::Animations::IAnimationWindowPreview {
    pub fn UpdatePreviewGraph(
        &mut self,
        graph: crate::UnityEngine::Playables::PlayableGraph,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UpdatePreviewGraph", (graph))?;
        Ok(__cordl_ret)
    }
    pub fn from_object_mut(
        object_param: *mut quest_hook::libil2cpp::Il2CppObject,
    ) -> *mut Self {
        unsafe { (object_param as *mut Self) }
    }
}
#[cfg(feature = "UnityEngine+Animations+IAnimationWindowPreview")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::Animations::IAnimationWindowPreview {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
