#[cfg(feature = "UnityEngine+Timeline+TimelineCreateUtilities")]
#[repr(C)]
#[derive(Debug)]
pub struct TimelineCreateUtilities {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "UnityEngine+Timeline+TimelineCreateUtilities")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::Timeline::TimelineCreateUtilities
    => "UnityEngine.Timeline"."TimelineCreateUtilities"
);
#[cfg(feature = "UnityEngine+Timeline+TimelineCreateUtilities")]
impl std::ops::Deref for crate::UnityEngine::Timeline::TimelineCreateUtilities {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Timeline+TimelineCreateUtilities")]
impl std::ops::DerefMut for crate::UnityEngine::Timeline::TimelineCreateUtilities {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Timeline+TimelineCreateUtilities")]
impl crate::UnityEngine::Timeline::TimelineCreateUtilities {
    pub fn CreateAnimationClipForTrack(
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        track: quest_hook::libil2cpp::Gc<crate::UnityEngine::Timeline::TrackAsset>,
        isLegacy: bool,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::AnimationClip>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::AnimationClip> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CreateAnimationClipForTrack", (name, track, isLegacy))?;
        Ok(__cordl_ret.into())
    }
    pub fn GenerateUniqueActorName(
        tracks: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                *mut crate::UnityEngine::ScriptableObject,
            >,
        >,
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GenerateUniqueActorName", (tracks, name))?;
        Ok(__cordl_ret.into())
    }
    pub fn RemoveAssetFromObject(
        childAsset: quest_hook::libil2cpp::Gc<crate::UnityEngine::Object>,
        masterAsset: quest_hook::libil2cpp::Gc<crate::UnityEngine::Object>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("RemoveAssetFromObject", (childAsset, masterAsset))?;
        Ok(__cordl_ret.into())
    }
    pub fn SaveAssetIntoObject(
        childAsset: quest_hook::libil2cpp::Gc<crate::UnityEngine::Object>,
        masterAsset: quest_hook::libil2cpp::Gc<crate::UnityEngine::Object>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("SaveAssetIntoObject", (childAsset, masterAsset))?;
        Ok(__cordl_ret.into())
    }
    pub fn ValidateParentTrack(
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::Timeline::TrackAsset>,
        childType: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ValidateParentTrack", (parent, childType))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+Timeline+TimelineCreateUtilities")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::Timeline::TimelineCreateUtilities {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
