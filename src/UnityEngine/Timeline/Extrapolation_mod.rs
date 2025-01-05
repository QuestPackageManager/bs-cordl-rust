#[cfg(feature = "UnityEngine+Timeline+Extrapolation")]
#[repr(C)]
#[derive(Debug)]
pub struct Extrapolation {
    __cordl_parent: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
}
#[cfg(feature = "UnityEngine+Timeline+Extrapolation")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::Timeline::Extrapolation =>
    "UnityEngine.Timeline"."Extrapolation"
);
#[cfg(feature = "UnityEngine+Timeline+Extrapolation")]
impl std::ops::Deref for crate::UnityEngine::Timeline::Extrapolation {
    type Target = quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Timeline+Extrapolation")]
impl std::ops::DerefMut for crate::UnityEngine::Timeline::Extrapolation {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Timeline+Extrapolation")]
impl crate::UnityEngine::Timeline::Extrapolation {
    pub fn CalculateExtrapolationTimes(
        asset: quest_hook::libil2cpp::Gc<crate::UnityEngine::Timeline::TrackAsset>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CalculateExtrapolationTimes", (asset))?;
        Ok(__cordl_ret.into())
    }
    pub fn SortClipsByStartTime(
        clips: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<crate::UnityEngine::Timeline::TimelineClip>,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<crate::UnityEngine::Timeline::TimelineClip>,
            >,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<crate::UnityEngine::Timeline::TimelineClip>,
            >,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("SortClipsByStartTime", (clips))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+Timeline+Extrapolation")]
impl quest_hook::libil2cpp::ObjectType for crate::UnityEngine::Timeline::Extrapolation {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
