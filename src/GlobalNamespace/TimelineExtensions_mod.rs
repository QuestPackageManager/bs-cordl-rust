#[cfg(feature = "TimelineExtensions")]
#[repr(C)]
#[derive(Debug)]
pub struct TimelineExtensions {
    __cordl_parent: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
}
#[cfg(feature = "TimelineExtensions")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::TimelineExtensions => ""
    ."TimelineExtensions"
);
#[cfg(feature = "TimelineExtensions")]
impl std::ops::Deref for crate::GlobalNamespace::TimelineExtensions {
    type Target = quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "TimelineExtensions")]
impl std::ops::DerefMut for crate::GlobalNamespace::TimelineExtensions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "TimelineExtensions")]
impl crate::GlobalNamespace::TimelineExtensions {
    pub fn JumpToTimelineMarker(
        playableDirector: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Playables::PlayableDirector,
        >,
        markerType: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("JumpToTimelineMarker", (playableDirector, markerType))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "TimelineExtensions")]
impl quest_hook::libil2cpp::ObjectType for crate::GlobalNamespace::TimelineExtensions {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
