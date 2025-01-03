#[cfg(feature = "UnityEngine+Timeline+TimelineClipExtensions")]
#[repr(C)]
#[derive(Debug)]
pub struct TimelineClipExtensions {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "UnityEngine+Timeline+TimelineClipExtensions")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::Timeline::TimelineClipExtensions =>
    "UnityEngine.Timeline"."TimelineClipExtensions"
);
#[cfg(feature = "UnityEngine+Timeline+TimelineClipExtensions")]
impl std::ops::Deref for crate::UnityEngine::Timeline::TimelineClipExtensions {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Timeline+TimelineClipExtensions")]
impl std::ops::DerefMut for crate::UnityEngine::Timeline::TimelineClipExtensions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Timeline+TimelineClipExtensions")]
impl crate::UnityEngine::Timeline::TimelineClipExtensions {
    pub fn MoveToTrack(
        clip: quest_hook::libil2cpp::Gc<crate::UnityEngine::Timeline::TimelineClip>,
        destinationTrack: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Timeline::TrackAsset,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("MoveToTrack", (clip, destinationTrack))?;
        Ok(__cordl_ret.into())
    }
    pub fn MoveToTrack_Impl(
        clip: quest_hook::libil2cpp::Gc<crate::UnityEngine::Timeline::TimelineClip>,
        destinationTrack: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Timeline::TrackAsset,
        >,
        asset: quest_hook::libil2cpp::Gc<crate::UnityEngine::Object>,
        parentTrack: quest_hook::libil2cpp::Gc<crate::UnityEngine::Timeline::TrackAsset>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("MoveToTrack_Impl", (clip, destinationTrack, asset, parentTrack))?;
        Ok(__cordl_ret.into())
    }
    pub fn TryMoveToTrack(
        clip: quest_hook::libil2cpp::Gc<crate::UnityEngine::Timeline::TimelineClip>,
        destinationTrack: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Timeline::TrackAsset,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("TryMoveToTrack", (clip, destinationTrack))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+Timeline+TimelineClipExtensions")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::Timeline::TimelineClipExtensions {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
