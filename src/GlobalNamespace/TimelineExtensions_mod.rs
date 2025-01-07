#[cfg(feature = "TimelineExtensions")]
#[repr(C)]
#[derive(Debug)]
pub struct TimelineExtensions {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "TimelineExtensions")]
unsafe impl quest_hook::libil2cpp::Type for crate::GlobalNamespace::TimelineExtensions {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "TimelineExtensions";
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.class().is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_value_argument(_: &quest_hook::libil2cpp::Il2CppType) -> bool {
        false
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
    fn matches_value_parameter(_: &quest_hook::libil2cpp::Il2CppType) -> bool {
        false
    }
}
#[cfg(feature = "TimelineExtensions")]
impl std::ops::Deref for crate::GlobalNamespace::TimelineExtensions {
    type Target = quest_hook::libil2cpp::Il2CppObject;
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
