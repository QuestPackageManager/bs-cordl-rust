#[cfg(feature = "TubeLightIntensityAsset")]
#[repr(C)]
#[derive(Debug)]
pub struct TubeLightIntensityAsset {
    __cordl_parent: crate::UnityEngine::Playables::PlayableAsset,
    pub _template: *mut TubeLightIntensityBehaviour,
}
#[cfg(feature = "TubeLightIntensityAsset")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for TubeLightIntensityAsset => ""."TubeLightIntensityAsset"
);
#[cfg(feature = "TubeLightIntensityAsset")]
impl std::ops::Deref for TubeLightIntensityAsset {
    type Target = crate::UnityEngine::Playables::PlayableAsset;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "TubeLightIntensityAsset")]
impl std::ops::DerefMut for TubeLightIntensityAsset {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "TubeLightIntensityAsset")]
impl TubeLightIntensityAsset {
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_clipCaps(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Timeline::ClipCaps> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Timeline::ClipCaps = __cordl_object
            .invoke("get_clipCaps", ())?;
        Ok(__cordl_ret)
    }
    pub fn CreatePlayable(
        &mut self,
        graph: crate::UnityEngine::Playables::PlayableGraph,
        go: *mut crate::UnityEngine::GameObject,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Playables::Playable> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Playables::Playable = __cordl_object
            .invoke("CreatePlayable", (graph, go))?;
        Ok(__cordl_ret)
    }
    pub fn New() -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "TubeLightIntensityAsset")]
impl quest_hook::libil2cpp::ObjectType for TubeLightIntensityAsset {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
