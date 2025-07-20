#[cfg(feature = "BeatmapSaveDataVersion4+LightshowSaveData")]
#[repr(C)]
#[derive(Debug)]
pub struct LightshowSaveData {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub version: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub waypoints: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<
            quest_hook::libil2cpp::Gc<crate::BeatmapSaveDataVersion4::BeatmapBeatIndex>,
        >,
    >,
    pub waypointsData: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<crate::BeatmapSaveDataVersion4::Waypoint>,
    >,
    pub basicEvents: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<
            quest_hook::libil2cpp::Gc<crate::BeatmapSaveDataVersion4::BeatIndex>,
        >,
    >,
    pub basicEventsData: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<crate::BeatmapSaveDataVersion4::BasicEvent>,
    >,
    pub colorBoostEvents: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<
            quest_hook::libil2cpp::Gc<crate::BeatmapSaveDataVersion4::BeatIndex>,
        >,
    >,
    pub colorBoostEventsData: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<
            crate::BeatmapSaveDataVersion4::ColorBoostEvent,
        >,
    >,
    pub eventBoxGroups: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<
            quest_hook::libil2cpp::Gc<crate::BeatmapSaveDataVersion4::EventBoxGroup>,
        >,
    >,
    pub indexFilters: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<crate::BeatmapSaveDataVersion4::IndexFilter>,
    >,
    pub lightColorEventBoxes: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<
            crate::BeatmapSaveDataVersion4::LightColorEventBox,
        >,
    >,
    pub lightColorEvents: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<
            crate::BeatmapSaveDataVersion4::LightColorEvent,
        >,
    >,
    pub lightRotationEventBoxes: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<
            crate::BeatmapSaveDataVersion4::LightRotationEventBox,
        >,
    >,
    pub lightRotationEvents: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<
            crate::BeatmapSaveDataVersion4::LightRotationEvent,
        >,
    >,
    pub lightTranslationEventBoxes: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<
            crate::BeatmapSaveDataVersion4::LightTranslationEventBox,
        >,
    >,
    pub lightTranslationEvents: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<
            crate::BeatmapSaveDataVersion4::LightTranslationEvent,
        >,
    >,
    pub fxEventBoxes: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<crate::BeatmapSaveDataVersion4::FxEventBox>,
    >,
    pub floatFxEvents: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<crate::BeatmapSaveDataVersion4::FloatFxEvent>,
    >,
    pub basicEventTypesWithKeywords: quest_hook::libil2cpp::Gc<
        crate::BeatmapSaveDataCommon::BasicEventTypesWithKeywords,
    >,
    pub useNormalEventsAsCompatibleEvents: bool,
}
#[cfg(feature = "BeatmapSaveDataVersion4+LightshowSaveData")]
unsafe impl quest_hook::libil2cpp::Type
for crate::BeatmapSaveDataVersion4::LightshowSaveData {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "BeatmapSaveDataVersion4";
    const CLASS_NAME: &'static str = "LightshowSaveData";
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
#[cfg(feature = "BeatmapSaveDataVersion4+LightshowSaveData")]
impl std::ops::Deref for crate::BeatmapSaveDataVersion4::LightshowSaveData {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BeatmapSaveDataVersion4+LightshowSaveData")]
impl std::ops::DerefMut for crate::BeatmapSaveDataVersion4::LightshowSaveData {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BeatmapSaveDataVersion4+LightshowSaveData")]
impl crate::BeatmapSaveDataVersion4::LightshowSaveData {
    pub const kCurrentVersion: &'static str = "4.0.0";
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), quest_hook::libil2cpp::Void, 0usize>(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), ".ctor",
                            0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "BeatmapSaveDataVersion4+LightshowSaveData")]
impl quest_hook::libil2cpp::ObjectType
for crate::BeatmapSaveDataVersion4::LightshowSaveData {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
