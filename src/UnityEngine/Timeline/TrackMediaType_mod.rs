#[cfg(feature = "UnityEngine+Timeline+TrackMediaType")]
#[repr(C)]
#[derive(Debug)]
pub struct TrackMediaType {
    __cordl_parent: crate::System::Attribute,
    pub m_MediaType: crate::UnityEngine::Timeline::TimelineAsset_MediaType,
}
#[cfg(feature = "UnityEngine+Timeline+TrackMediaType")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::Timeline::TrackMediaType =>
    "UnityEngine.Timeline"."TrackMediaType"
);
#[cfg(feature = "UnityEngine+Timeline+TrackMediaType")]
impl std::ops::Deref for crate::UnityEngine::Timeline::TrackMediaType {
    type Target = crate::System::Attribute;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Timeline+TrackMediaType")]
impl std::ops::DerefMut for crate::UnityEngine::Timeline::TrackMediaType {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Timeline+TrackMediaType")]
impl crate::UnityEngine::Timeline::TrackMediaType {
    pub fn New(
        mt: crate::UnityEngine::Timeline::TimelineAsset_MediaType,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (mt))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        mt: crate::UnityEngine::Timeline::TimelineAsset_MediaType,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (mt))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+Timeline+TrackMediaType")]
impl quest_hook::libil2cpp::ObjectType for crate::UnityEngine::Timeline::TrackMediaType {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
