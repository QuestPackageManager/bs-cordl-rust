#[cfg(feature = "UnityEngine+Timeline+TrackAssetExtensions")]
#[repr(C)]
#[derive(Debug)]
pub struct TrackAssetExtensions {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "UnityEngine+Timeline+TrackAssetExtensions")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::Timeline::TrackAssetExtensions =>
    "UnityEngine.Timeline"."TrackAssetExtensions"
);
#[cfg(feature = "UnityEngine+Timeline+TrackAssetExtensions")]
impl std::ops::Deref for crate::UnityEngine::Timeline::TrackAssetExtensions {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Timeline+TrackAssetExtensions")]
impl std::ops::DerefMut for crate::UnityEngine::Timeline::TrackAssetExtensions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Timeline+TrackAssetExtensions")]
impl crate::UnityEngine::Timeline::TrackAssetExtensions {
    pub fn GetGroup(
        asset: quest_hook::libil2cpp::Gc<crate::UnityEngine::Timeline::TrackAsset>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::Timeline::GroupTrack>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Timeline::GroupTrack,
        > = <Self as quest_hook::libil2cpp::Type>::class().invoke("GetGroup", (asset))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetGroup(
        asset: quest_hook::libil2cpp::Gc<crate::UnityEngine::Timeline::TrackAsset>,
        group: quest_hook::libil2cpp::Gc<crate::UnityEngine::Timeline::GroupTrack>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("SetGroup", (asset, group))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+Timeline+TrackAssetExtensions")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::Timeline::TrackAssetExtensions {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
