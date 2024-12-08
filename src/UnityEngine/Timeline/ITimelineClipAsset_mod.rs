#[cfg(feature = "UnityEngine+Timeline+ITimelineClipAsset")]
#[repr(C)]
#[derive(Debug)]
pub struct ITimelineClipAsset {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "UnityEngine+Timeline+ITimelineClipAsset")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::Timeline::ITimelineClipAsset =>
    "UnityEngine.Timeline"."ITimelineClipAsset"
);
#[cfg(feature = "UnityEngine+Timeline+ITimelineClipAsset")]
impl std::ops::Deref for crate::UnityEngine::Timeline::ITimelineClipAsset {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Timeline+ITimelineClipAsset")]
impl std::ops::DerefMut for crate::UnityEngine::Timeline::ITimelineClipAsset {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Timeline+ITimelineClipAsset")]
impl crate::UnityEngine::Timeline::ITimelineClipAsset {
    pub fn from_object_mut(
        object_param: *mut quest_hook::libil2cpp::Il2CppObject,
    ) -> *mut Self {
        unsafe { (object_param as *mut Self) }
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
}
#[cfg(feature = "UnityEngine+Timeline+ITimelineClipAsset")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::Timeline::ITimelineClipAsset {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
