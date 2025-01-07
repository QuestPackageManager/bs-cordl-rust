#[cfg(feature = "TimelineUtils")]
#[repr(C)]
#[derive(Debug)]
pub struct TimelineUtils {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "TimelineUtils")]
unsafe impl quest_hook::libil2cpp::Type for crate::GlobalNamespace::TimelineUtils {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "TimelineUtils";
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
#[cfg(feature = "TimelineUtils")]
impl std::ops::Deref for crate::GlobalNamespace::TimelineUtils {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "TimelineUtils")]
impl std::ops::DerefMut for crate::GlobalNamespace::TimelineUtils {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "TimelineUtils")]
impl crate::GlobalNamespace::TimelineUtils {
    pub fn FindTrackAssetByName(
        timeline: quest_hook::libil2cpp::Gc<crate::UnityEngine::Timeline::TimelineAsset>,
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::Timeline::TrackAsset>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Timeline::TrackAsset,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("FindTrackAssetByName", (timeline, name))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "TimelineUtils")]
impl quest_hook::libil2cpp::ObjectType for crate::GlobalNamespace::TimelineUtils {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
