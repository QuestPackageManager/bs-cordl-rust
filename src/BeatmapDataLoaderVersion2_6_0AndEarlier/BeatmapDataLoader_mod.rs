#[cfg(feature = "BeatmapDataLoaderVersion2_6_0AndEarlier+BeatmapDataLoader")]
#[repr(C)]
#[derive(Debug)]
pub struct BeatmapDataLoader {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "BeatmapDataLoaderVersion2_6_0AndEarlier+BeatmapDataLoader")]
unsafe impl quest_hook::libil2cpp::Type
for crate::BeatmapDataLoaderVersion2_6_0AndEarlier::BeatmapDataLoader {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "BeatmapDataLoaderVersion2_6_0AndEarlier";
    const CLASS_NAME: &'static str = "BeatmapDataLoader";
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
#[cfg(feature = "BeatmapDataLoaderVersion2_6_0AndEarlier+BeatmapDataLoader")]
impl std::ops::Deref
for crate::BeatmapDataLoaderVersion2_6_0AndEarlier::BeatmapDataLoader {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BeatmapDataLoaderVersion2_6_0AndEarlier+BeatmapDataLoader")]
impl std::ops::DerefMut
for crate::BeatmapDataLoaderVersion2_6_0AndEarlier::BeatmapDataLoader {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BeatmapDataLoaderVersion2_6_0AndEarlier+BeatmapDataLoader")]
impl crate::BeatmapDataLoaderVersion2_6_0AndEarlier::BeatmapDataLoader {
    pub const kDefaultNumberOfLines: i32 = 4i32;
    #[cfg(
        feature = "BeatmapDataLoaderVersion2_6_0AndEarlier+BeatmapDataLoader+BasicEventConverter"
    )]
    pub type BasicEventConverter = crate::BeatmapDataLoaderVersion2_6_0AndEarlier::BeatmapDataLoader_BasicEventConverter;
    #[cfg(
        feature = "BeatmapDataLoaderVersion2_6_0AndEarlier+BeatmapDataLoader+ColorNoteConverter"
    )]
    pub type ColorNoteConverter = crate::BeatmapDataLoaderVersion2_6_0AndEarlier::BeatmapDataLoader_ColorNoteConverter;
    #[cfg(
        feature = "BeatmapDataLoaderVersion2_6_0AndEarlier+BeatmapDataLoader+ObstacleConverter"
    )]
    pub type ObstacleConverter = crate::BeatmapDataLoaderVersion2_6_0AndEarlier::BeatmapDataLoader_ObstacleConverter;
    #[cfg(
        feature = "BeatmapDataLoaderVersion2_6_0AndEarlier+BeatmapDataLoader+SliderConverter"
    )]
    pub type SliderConverter = crate::BeatmapDataLoaderVersion2_6_0AndEarlier::BeatmapDataLoader_SliderConverter;
    #[cfg(
        feature = "BeatmapDataLoaderVersion2_6_0AndEarlier+BeatmapDataLoader+SpecialEventsFilter"
    )]
    pub type SpecialEventsFilter = crate::BeatmapDataLoaderVersion2_6_0AndEarlier::BeatmapDataLoader_SpecialEventsFilter;
    #[cfg(
        feature = "BeatmapDataLoaderVersion2_6_0AndEarlier+BeatmapDataLoader+WaypointConverter"
    )]
    pub type WaypointConverter = crate::BeatmapDataLoaderVersion2_6_0AndEarlier::BeatmapDataLoader_WaypointConverter;
    pub fn BeatmapSaveDataAreSorted(
        beatmapSaveData: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IReadOnlyList_1<
                quest_hook::libil2cpp::Gc<
                    crate::BeatmapSaveDataVersion2_6_0AndEarlier::BeatmapSaveDataItem,
                >,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::BeatmapDataLoaderVersion2_6_0AndEarlier::BeatmapDataLoader as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (quest_hook::libil2cpp::Gc<
                    crate::System::Collections::Generic::IReadOnlyList_1<
                        quest_hook::libil2cpp::Gc<
                            crate::BeatmapSaveDataVersion2_6_0AndEarlier::BeatmapSaveDataItem,
                        >,
                    >,
                >),
                bool,
                1usize,
            >("BeatmapSaveDataAreSorted")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::BeatmapDataLoaderVersion2_6_0AndEarlier::BeatmapDataLoader as
                    quest_hook::libil2cpp::Type > ::class(), "BeatmapSaveDataAreSorted",
                    1usize
                )
            });
        let __cordl_ret: bool = unsafe {
            method.invoke_unchecked((), (beatmapSaveData))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ConvertBeatmapSaveDataPreV2_5_0Inline(
        beatmapSaveData: quest_hook::libil2cpp::Gc<
            crate::BeatmapSaveDataVersion2_6_0AndEarlier::BeatmapSaveData,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::BeatmapDataLoaderVersion2_6_0AndEarlier::BeatmapDataLoader as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (quest_hook::libil2cpp::Gc<
                    crate::BeatmapSaveDataVersion2_6_0AndEarlier::BeatmapSaveData,
                >),
                quest_hook::libil2cpp::Void,
                1usize,
            >("ConvertBeatmapSaveDataPreV2_5_0Inline")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::BeatmapDataLoaderVersion2_6_0AndEarlier::BeatmapDataLoader as
                    quest_hook::libil2cpp::Type > ::class(),
                    "ConvertBeatmapSaveDataPreV2_5_0Inline", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (beatmapSaveData))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetBeatmapDataBasicInfoFromSaveDataJson(
        beatmapSaveDataJson: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::BeatmapDataBasicInfo>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::BeatmapDataLoaderVersion2_6_0AndEarlier::BeatmapDataLoader as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>),
                quest_hook::libil2cpp::Gc<crate::GlobalNamespace::BeatmapDataBasicInfo>,
                1usize,
            >("GetBeatmapDataBasicInfoFromSaveDataJson")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::BeatmapDataLoaderVersion2_6_0AndEarlier::BeatmapDataLoader as
                    quest_hook::libil2cpp::Type > ::class(),
                    "GetBeatmapDataBasicInfoFromSaveDataJson", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::BeatmapDataBasicInfo,
        > = unsafe { method.invoke_unchecked((), (beatmapSaveDataJson))? };
        Ok(__cordl_ret.into())
    }
    pub fn GetBeatmapDataBasicInfoFromSaveDataJsonAsync(
        beatmapJson: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task_1<
                quest_hook::libil2cpp::Gc<crate::GlobalNamespace::BeatmapDataBasicInfo>,
            >,
        >,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::BeatmapDataLoaderVersion2_6_0AndEarlier::BeatmapDataLoader as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>),
                quest_hook::libil2cpp::Gc<
                    crate::System::Threading::Tasks::Task_1<
                        quest_hook::libil2cpp::Gc<
                            crate::GlobalNamespace::BeatmapDataBasicInfo,
                        >,
                    >,
                >,
                1usize,
            >("GetBeatmapDataBasicInfoFromSaveDataJsonAsync")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::BeatmapDataLoaderVersion2_6_0AndEarlier::BeatmapDataLoader as
                    quest_hook::libil2cpp::Type > ::class(),
                    "GetBeatmapDataBasicInfoFromSaveDataJsonAsync", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task_1<
                quest_hook::libil2cpp::Gc<crate::GlobalNamespace::BeatmapDataBasicInfo>,
            >,
        > = unsafe { method.invoke_unchecked((), (beatmapJson))? };
        Ok(__cordl_ret.into())
    }
    pub fn GetBeatmapDataFromSaveData(
        beatmapSaveData: quest_hook::libil2cpp::Gc<
            crate::BeatmapSaveDataVersion2_6_0AndEarlier::BeatmapSaveData,
        >,
        defaultLightshowSaveData: quest_hook::libil2cpp::Gc<
            crate::BeatmapSaveDataVersion4::LightshowSaveData,
        >,
        beatmapDifficulty: crate::GlobalNamespace::BeatmapDifficulty,
        startBpm: f32,
        loadingForDesignatedEnvironment: bool,
        environmentKeywords: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::EnvironmentKeywords,
        >,
        environmentLightGroups: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::IEnvironmentLightGroups,
        >,
        playerSpecificSettings: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::PlayerSpecificSettings,
        >,
        lightEventConverter: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::IBeatmapLightEventConverter,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::BeatmapData>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::BeatmapDataLoaderVersion2_6_0AndEarlier::BeatmapDataLoader as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::Gc<
                        crate::BeatmapSaveDataVersion2_6_0AndEarlier::BeatmapSaveData,
                    >,
                    quest_hook::libil2cpp::Gc<
                        crate::BeatmapSaveDataVersion4::LightshowSaveData,
                    >,
                    crate::GlobalNamespace::BeatmapDifficulty,
                    f32,
                    bool,
                    quest_hook::libil2cpp::Gc<
                        crate::GlobalNamespace::EnvironmentKeywords,
                    >,
                    quest_hook::libil2cpp::Gc<
                        crate::GlobalNamespace::IEnvironmentLightGroups,
                    >,
                    quest_hook::libil2cpp::Gc<
                        crate::GlobalNamespace::PlayerSpecificSettings,
                    >,
                    quest_hook::libil2cpp::Gc<
                        crate::GlobalNamespace::IBeatmapLightEventConverter,
                    >,
                ),
                quest_hook::libil2cpp::Gc<crate::GlobalNamespace::BeatmapData>,
                9usize,
            >("GetBeatmapDataFromSaveData")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::BeatmapDataLoaderVersion2_6_0AndEarlier::BeatmapDataLoader as
                    quest_hook::libil2cpp::Type > ::class(),
                    "GetBeatmapDataFromSaveData", 9usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::BeatmapData,
        > = unsafe {
            method
                .invoke_unchecked(
                    (),
                    (
                        beatmapSaveData,
                        defaultLightshowSaveData,
                        beatmapDifficulty,
                        startBpm,
                        loadingForDesignatedEnvironment,
                        environmentKeywords,
                        environmentLightGroups,
                        playerSpecificSettings,
                        lightEventConverter,
                    ),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetBeatmapDataFromSaveDataJson(
        beatmapJson: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        defaultLightshowSaveDataJson: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        >,
        beatmapDifficulty: crate::GlobalNamespace::BeatmapDifficulty,
        startBpm: f32,
        loadingForDesignatedEnvironment: bool,
        environmentInfo: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::IEnvironmentInfo,
        >,
        beatmapLevelDataVersion: crate::GlobalNamespace::BeatmapLevelDataVersion,
        playerSpecificSettings: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::PlayerSpecificSettings,
        >,
        lightEventConverter: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::IBeatmapLightEventConverter,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::BeatmapData>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::BeatmapDataLoaderVersion2_6_0AndEarlier::BeatmapDataLoader as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                    crate::GlobalNamespace::BeatmapDifficulty,
                    f32,
                    bool,
                    quest_hook::libil2cpp::Gc<crate::GlobalNamespace::IEnvironmentInfo>,
                    crate::GlobalNamespace::BeatmapLevelDataVersion,
                    quest_hook::libil2cpp::Gc<
                        crate::GlobalNamespace::PlayerSpecificSettings,
                    >,
                    quest_hook::libil2cpp::Gc<
                        crate::GlobalNamespace::IBeatmapLightEventConverter,
                    >,
                ),
                quest_hook::libil2cpp::Gc<crate::GlobalNamespace::BeatmapData>,
                9usize,
            >("GetBeatmapDataFromSaveDataJson")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::BeatmapDataLoaderVersion2_6_0AndEarlier::BeatmapDataLoader as
                    quest_hook::libil2cpp::Type > ::class(),
                    "GetBeatmapDataFromSaveDataJson", 9usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::BeatmapData,
        > = unsafe {
            method
                .invoke_unchecked(
                    (),
                    (
                        beatmapJson,
                        defaultLightshowSaveDataJson,
                        beatmapDifficulty,
                        startBpm,
                        loadingForDesignatedEnvironment,
                        environmentInfo,
                        beatmapLevelDataVersion,
                        playerSpecificSettings,
                        lightEventConverter,
                    ),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetBeatmapDataFromSaveDataJsonAsync(
        beatmapJson: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        defaultLightshowSaveDataJson: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        >,
        beatmapDifficulty: crate::GlobalNamespace::BeatmapDifficulty,
        startBpm: f32,
        loadingForDesignatedEnvironment: bool,
        environmentInfo: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::IEnvironmentInfo,
        >,
        beatmapLevelDataVersion: crate::GlobalNamespace::BeatmapLevelDataVersion,
        playerSpecificSettings: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::PlayerSpecificSettings,
        >,
        lightEventConverter: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::IBeatmapLightEventConverter,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task_1<
                quest_hook::libil2cpp::Gc<crate::GlobalNamespace::BeatmapData>,
            >,
        >,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::BeatmapDataLoaderVersion2_6_0AndEarlier::BeatmapDataLoader as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                    crate::GlobalNamespace::BeatmapDifficulty,
                    f32,
                    bool,
                    quest_hook::libil2cpp::Gc<crate::GlobalNamespace::IEnvironmentInfo>,
                    crate::GlobalNamespace::BeatmapLevelDataVersion,
                    quest_hook::libil2cpp::Gc<
                        crate::GlobalNamespace::PlayerSpecificSettings,
                    >,
                    quest_hook::libil2cpp::Gc<
                        crate::GlobalNamespace::IBeatmapLightEventConverter,
                    >,
                ),
                quest_hook::libil2cpp::Gc<
                    crate::System::Threading::Tasks::Task_1<
                        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::BeatmapData>,
                    >,
                >,
                9usize,
            >("GetBeatmapDataFromSaveDataJsonAsync")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::BeatmapDataLoaderVersion2_6_0AndEarlier::BeatmapDataLoader as
                    quest_hook::libil2cpp::Type > ::class(),
                    "GetBeatmapDataFromSaveDataJsonAsync", 9usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task_1<
                quest_hook::libil2cpp::Gc<crate::GlobalNamespace::BeatmapData>,
            >,
        > = unsafe {
            method
                .invoke_unchecked(
                    (),
                    (
                        beatmapJson,
                        defaultLightshowSaveDataJson,
                        beatmapDifficulty,
                        startBpm,
                        loadingForDesignatedEnvironment,
                        environmentInfo,
                        beatmapLevelDataVersion,
                        playerSpecificSettings,
                        lightEventConverter,
                    ),
                )?
        };
        Ok(__cordl_ret.into())
    }
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
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::BeatmapDataLoaderVersion2_6_0AndEarlier::BeatmapDataLoader as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), quest_hook::libil2cpp::Void, 0usize>(".ctor")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::BeatmapDataLoaderVersion2_6_0AndEarlier::BeatmapDataLoader as
                    quest_hook::libil2cpp::Type > ::class(), ".ctor", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "BeatmapDataLoaderVersion2_6_0AndEarlier+BeatmapDataLoader")]
impl quest_hook::libil2cpp::ObjectType
for crate::BeatmapDataLoaderVersion2_6_0AndEarlier::BeatmapDataLoader {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(
    feature = "BeatmapDataLoaderVersion2_6_0AndEarlier+BeatmapDataLoader+BasicEventConverter"
)]
#[repr(C)]
#[derive(Debug)]
pub struct BeatmapDataLoader_BasicEventConverter {
    __cordl_parent: crate::GlobalNamespace::BeatToTimeConverter,
    pub _specialEventsFilter: quest_hook::libil2cpp::Gc<
        crate::BeatmapDataLoaderVersion2_6_0AndEarlier::BeatmapDataLoader_SpecialEventsFilter,
    >,
    pub _canUseEnvironmentEventsAndShouldLoadDynamicEvents: bool,
}
#[cfg(
    feature = "BeatmapDataLoaderVersion2_6_0AndEarlier+BeatmapDataLoader+BasicEventConverter"
)]
unsafe impl quest_hook::libil2cpp::Type
for crate::BeatmapDataLoaderVersion2_6_0AndEarlier::BeatmapDataLoader_BasicEventConverter {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "BeatmapDataLoaderVersion2_6_0AndEarlier";
    const CLASS_NAME: &'static str = "BeatmapDataLoader/BasicEventConverter";
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
#[cfg(
    feature = "BeatmapDataLoaderVersion2_6_0AndEarlier+BeatmapDataLoader+BasicEventConverter"
)]
impl std::ops::Deref
for crate::BeatmapDataLoaderVersion2_6_0AndEarlier::BeatmapDataLoader_BasicEventConverter {
    type Target = crate::GlobalNamespace::BeatToTimeConverter;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(
    feature = "BeatmapDataLoaderVersion2_6_0AndEarlier+BeatmapDataLoader+BasicEventConverter"
)]
impl std::ops::DerefMut
for crate::BeatmapDataLoaderVersion2_6_0AndEarlier::BeatmapDataLoader_BasicEventConverter {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(
    feature = "BeatmapDataLoaderVersion2_6_0AndEarlier+BeatmapDataLoader+BasicEventConverter"
)]
impl crate::BeatmapDataLoaderVersion2_6_0AndEarlier::BeatmapDataLoader_BasicEventConverter {
    pub fn Convert(
        &mut self,
        e: quest_hook::libil2cpp::Gc<
            crate::BeatmapSaveDataVersion2_6_0AndEarlier::EventData,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::BeatmapEventData>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::BeatmapDataLoaderVersion2_6_0AndEarlier::BeatmapDataLoader_BasicEventConverter as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<
                    crate::BeatmapSaveDataVersion2_6_0AndEarlier::EventData,
                >),
                quest_hook::libil2cpp::Gc<crate::GlobalNamespace::BeatmapEventData>,
                1usize,
            >("Convert")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate
                    ::BeatmapDataLoaderVersion2_6_0AndEarlier::BeatmapDataLoader_BasicEventConverter
                    as quest_hook::libil2cpp::Type > ::class(), "Convert", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::BeatmapEventData,
        > = unsafe { method.invoke_unchecked(self, (e))? };
        Ok(__cordl_ret.into())
    }
    pub fn New(
        bpmTimeProcessor: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::IBeatToTimeConverter,
        >,
        specialEventsFilter: quest_hook::libil2cpp::Gc<
            crate::BeatmapDataLoaderVersion2_6_0AndEarlier::BeatmapDataLoader_SpecialEventsFilter,
        >,
        canUseEnvironmentEventsAndShouldLoadDynamicEvents: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(
                ".ctor",
                (
                    bpmTimeProcessor,
                    specialEventsFilter,
                    canUseEnvironmentEventsAndShouldLoadDynamicEvents,
                ),
            )?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        bpmTimeProcessor: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::IBeatToTimeConverter,
        >,
        specialEventsFilter: quest_hook::libil2cpp::Gc<
            crate::BeatmapDataLoaderVersion2_6_0AndEarlier::BeatmapDataLoader_SpecialEventsFilter,
        >,
        canUseEnvironmentEventsAndShouldLoadDynamicEvents: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::BeatmapDataLoaderVersion2_6_0AndEarlier::BeatmapDataLoader_BasicEventConverter as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<
                        crate::GlobalNamespace::IBeatToTimeConverter,
                    >,
                    quest_hook::libil2cpp::Gc<
                        crate::BeatmapDataLoaderVersion2_6_0AndEarlier::BeatmapDataLoader_SpecialEventsFilter,
                    >,
                    bool,
                ),
                quest_hook::libil2cpp::Void,
                3usize,
            >(".ctor")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate
                    ::BeatmapDataLoaderVersion2_6_0AndEarlier::BeatmapDataLoader_BasicEventConverter
                    as quest_hook::libil2cpp::Type > ::class(), ".ctor", 3usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method
                .invoke_unchecked(
                    self,
                    (
                        bpmTimeProcessor,
                        specialEventsFilter,
                        canUseEnvironmentEventsAndShouldLoadDynamicEvents,
                    ),
                )?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(
    feature = "BeatmapDataLoaderVersion2_6_0AndEarlier+BeatmapDataLoader+BasicEventConverter"
)]
impl quest_hook::libil2cpp::ObjectType
for crate::BeatmapDataLoaderVersion2_6_0AndEarlier::BeatmapDataLoader_BasicEventConverter {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(
    feature = "BeatmapDataLoaderVersion2_6_0AndEarlier+BeatmapDataLoader+ColorNoteConverter"
)]
#[repr(C)]
#[derive(Debug)]
pub struct BeatmapDataLoader_ColorNoteConverter {
    __cordl_parent: crate::GlobalNamespace::BeatToTimeAndRotationConverter,
}
#[cfg(
    feature = "BeatmapDataLoaderVersion2_6_0AndEarlier+BeatmapDataLoader+ColorNoteConverter"
)]
unsafe impl quest_hook::libil2cpp::Type
for crate::BeatmapDataLoaderVersion2_6_0AndEarlier::BeatmapDataLoader_ColorNoteConverter {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "BeatmapDataLoaderVersion2_6_0AndEarlier";
    const CLASS_NAME: &'static str = "BeatmapDataLoader/ColorNoteConverter";
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
#[cfg(
    feature = "BeatmapDataLoaderVersion2_6_0AndEarlier+BeatmapDataLoader+ColorNoteConverter"
)]
impl std::ops::Deref
for crate::BeatmapDataLoaderVersion2_6_0AndEarlier::BeatmapDataLoader_ColorNoteConverter {
    type Target = crate::GlobalNamespace::BeatToTimeAndRotationConverter;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(
    feature = "BeatmapDataLoaderVersion2_6_0AndEarlier+BeatmapDataLoader+ColorNoteConverter"
)]
impl std::ops::DerefMut
for crate::BeatmapDataLoaderVersion2_6_0AndEarlier::BeatmapDataLoader_ColorNoteConverter {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(
    feature = "BeatmapDataLoaderVersion2_6_0AndEarlier+BeatmapDataLoader+ColorNoteConverter"
)]
impl crate::BeatmapDataLoaderVersion2_6_0AndEarlier::BeatmapDataLoader_ColorNoteConverter {
    pub fn Convert(
        &mut self,
        n: quest_hook::libil2cpp::Gc<
            crate::BeatmapSaveDataVersion2_6_0AndEarlier::NoteData,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::BeatmapObjectData>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::BeatmapDataLoaderVersion2_6_0AndEarlier::BeatmapDataLoader_ColorNoteConverter as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<
                    crate::BeatmapSaveDataVersion2_6_0AndEarlier::NoteData,
                >),
                quest_hook::libil2cpp::Gc<crate::GlobalNamespace::BeatmapObjectData>,
                1usize,
            >("Convert")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate
                    ::BeatmapDataLoaderVersion2_6_0AndEarlier::BeatmapDataLoader_ColorNoteConverter
                    as quest_hook::libil2cpp::Type > ::class(), "Convert", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::BeatmapObjectData,
        > = unsafe { method.invoke_unchecked(self, (n))? };
        Ok(__cordl_ret.into())
    }
    pub fn New(
        bpmTimeProcessor: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::IBeatToTimeConverter,
        >,
        rotationTimeProcessor: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::RotationTimeProcessor,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (bpmTimeProcessor, rotationTimeProcessor))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        bpmTimeProcessor: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::IBeatToTimeConverter,
        >,
        rotationTimeProcessor: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::RotationTimeProcessor,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::BeatmapDataLoaderVersion2_6_0AndEarlier::BeatmapDataLoader_ColorNoteConverter as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<
                        crate::GlobalNamespace::IBeatToTimeConverter,
                    >,
                    quest_hook::libil2cpp::Gc<
                        crate::GlobalNamespace::RotationTimeProcessor,
                    >,
                ),
                quest_hook::libil2cpp::Void,
                2usize,
            >(".ctor")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate
                    ::BeatmapDataLoaderVersion2_6_0AndEarlier::BeatmapDataLoader_ColorNoteConverter
                    as quest_hook::libil2cpp::Type > ::class(), ".ctor", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (bpmTimeProcessor, rotationTimeProcessor))?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(
    feature = "BeatmapDataLoaderVersion2_6_0AndEarlier+BeatmapDataLoader+ColorNoteConverter"
)]
impl quest_hook::libil2cpp::ObjectType
for crate::BeatmapDataLoaderVersion2_6_0AndEarlier::BeatmapDataLoader_ColorNoteConverter {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(
    feature = "BeatmapDataLoaderVersion2_6_0AndEarlier+BeatmapDataLoader+ObstacleConverter"
)]
#[repr(C)]
#[derive(Debug)]
pub struct BeatmapDataLoader_ObstacleConverter {
    __cordl_parent: crate::GlobalNamespace::BeatToTimeAndRotationConverter,
}
#[cfg(
    feature = "BeatmapDataLoaderVersion2_6_0AndEarlier+BeatmapDataLoader+ObstacleConverter"
)]
unsafe impl quest_hook::libil2cpp::Type
for crate::BeatmapDataLoaderVersion2_6_0AndEarlier::BeatmapDataLoader_ObstacleConverter {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "BeatmapDataLoaderVersion2_6_0AndEarlier";
    const CLASS_NAME: &'static str = "BeatmapDataLoader/ObstacleConverter";
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
#[cfg(
    feature = "BeatmapDataLoaderVersion2_6_0AndEarlier+BeatmapDataLoader+ObstacleConverter"
)]
impl std::ops::Deref
for crate::BeatmapDataLoaderVersion2_6_0AndEarlier::BeatmapDataLoader_ObstacleConverter {
    type Target = crate::GlobalNamespace::BeatToTimeAndRotationConverter;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(
    feature = "BeatmapDataLoaderVersion2_6_0AndEarlier+BeatmapDataLoader+ObstacleConverter"
)]
impl std::ops::DerefMut
for crate::BeatmapDataLoaderVersion2_6_0AndEarlier::BeatmapDataLoader_ObstacleConverter {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(
    feature = "BeatmapDataLoaderVersion2_6_0AndEarlier+BeatmapDataLoader+ObstacleConverter"
)]
impl crate::BeatmapDataLoaderVersion2_6_0AndEarlier::BeatmapDataLoader_ObstacleConverter {
    pub fn Convert(
        &mut self,
        o: quest_hook::libil2cpp::Gc<
            crate::BeatmapSaveDataVersion2_6_0AndEarlier::ObstacleData,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::ObstacleData>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::BeatmapDataLoaderVersion2_6_0AndEarlier::BeatmapDataLoader_ObstacleConverter as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<
                    crate::BeatmapSaveDataVersion2_6_0AndEarlier::ObstacleData,
                >),
                quest_hook::libil2cpp::Gc<crate::GlobalNamespace::ObstacleData>,
                1usize,
            >("Convert")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate
                    ::BeatmapDataLoaderVersion2_6_0AndEarlier::BeatmapDataLoader_ObstacleConverter
                    as quest_hook::libil2cpp::Type > ::class(), "Convert", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::ObstacleData,
        > = unsafe { method.invoke_unchecked(self, (o))? };
        Ok(__cordl_ret.into())
    }
    pub fn GetHeightForObstacleType(
        obstacleType: crate::BeatmapSaveDataVersion2_6_0AndEarlier::ObstacleType,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::BeatmapDataLoaderVersion2_6_0AndEarlier::BeatmapDataLoader_ObstacleConverter as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::BeatmapSaveDataVersion2_6_0AndEarlier::ObstacleType),
                i32,
                1usize,
            >("GetHeightForObstacleType")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate
                    ::BeatmapDataLoaderVersion2_6_0AndEarlier::BeatmapDataLoader_ObstacleConverter
                    as quest_hook::libil2cpp::Type > ::class(),
                    "GetHeightForObstacleType", 1usize
                )
            });
        let __cordl_ret: i32 = unsafe { method.invoke_unchecked((), (obstacleType))? };
        Ok(__cordl_ret.into())
    }
    pub fn GetLayerForObstacleType(
        obstacleType: crate::BeatmapSaveDataVersion2_6_0AndEarlier::ObstacleType,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::BeatmapDataLoaderVersion2_6_0AndEarlier::BeatmapDataLoader_ObstacleConverter as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::BeatmapSaveDataVersion2_6_0AndEarlier::ObstacleType),
                i32,
                1usize,
            >("GetLayerForObstacleType")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate
                    ::BeatmapDataLoaderVersion2_6_0AndEarlier::BeatmapDataLoader_ObstacleConverter
                    as quest_hook::libil2cpp::Type > ::class(),
                    "GetLayerForObstacleType", 1usize
                )
            });
        let __cordl_ret: i32 = unsafe { method.invoke_unchecked((), (obstacleType))? };
        Ok(__cordl_ret.into())
    }
    pub fn New(
        bpmTimeProcessor: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::IBeatToTimeConverter,
        >,
        rotationTimeProcessor: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::RotationTimeProcessor,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (bpmTimeProcessor, rotationTimeProcessor))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        bpmTimeProcessor: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::IBeatToTimeConverter,
        >,
        rotationTimeProcessor: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::RotationTimeProcessor,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::BeatmapDataLoaderVersion2_6_0AndEarlier::BeatmapDataLoader_ObstacleConverter as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<
                        crate::GlobalNamespace::IBeatToTimeConverter,
                    >,
                    quest_hook::libil2cpp::Gc<
                        crate::GlobalNamespace::RotationTimeProcessor,
                    >,
                ),
                quest_hook::libil2cpp::Void,
                2usize,
            >(".ctor")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate
                    ::BeatmapDataLoaderVersion2_6_0AndEarlier::BeatmapDataLoader_ObstacleConverter
                    as quest_hook::libil2cpp::Type > ::class(), ".ctor", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (bpmTimeProcessor, rotationTimeProcessor))?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(
    feature = "BeatmapDataLoaderVersion2_6_0AndEarlier+BeatmapDataLoader+ObstacleConverter"
)]
impl quest_hook::libil2cpp::ObjectType
for crate::BeatmapDataLoaderVersion2_6_0AndEarlier::BeatmapDataLoader_ObstacleConverter {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(
    feature = "BeatmapDataLoaderVersion2_6_0AndEarlier+BeatmapDataLoader+SliderConverter"
)]
#[repr(C)]
#[derive(Debug)]
pub struct BeatmapDataLoader_SliderConverter {
    __cordl_parent: crate::GlobalNamespace::BeatToTimeAndRotationConverter,
}
#[cfg(
    feature = "BeatmapDataLoaderVersion2_6_0AndEarlier+BeatmapDataLoader+SliderConverter"
)]
unsafe impl quest_hook::libil2cpp::Type
for crate::BeatmapDataLoaderVersion2_6_0AndEarlier::BeatmapDataLoader_SliderConverter {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "BeatmapDataLoaderVersion2_6_0AndEarlier";
    const CLASS_NAME: &'static str = "BeatmapDataLoader/SliderConverter";
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
#[cfg(
    feature = "BeatmapDataLoaderVersion2_6_0AndEarlier+BeatmapDataLoader+SliderConverter"
)]
impl std::ops::Deref
for crate::BeatmapDataLoaderVersion2_6_0AndEarlier::BeatmapDataLoader_SliderConverter {
    type Target = crate::GlobalNamespace::BeatToTimeAndRotationConverter;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(
    feature = "BeatmapDataLoaderVersion2_6_0AndEarlier+BeatmapDataLoader+SliderConverter"
)]
impl std::ops::DerefMut
for crate::BeatmapDataLoaderVersion2_6_0AndEarlier::BeatmapDataLoader_SliderConverter {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(
    feature = "BeatmapDataLoaderVersion2_6_0AndEarlier+BeatmapDataLoader+SliderConverter"
)]
impl crate::BeatmapDataLoaderVersion2_6_0AndEarlier::BeatmapDataLoader_SliderConverter {
    pub fn Convert(
        &mut self,
        s: quest_hook::libil2cpp::Gc<
            crate::BeatmapSaveDataVersion2_6_0AndEarlier::SliderData,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::SliderData>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::BeatmapDataLoaderVersion2_6_0AndEarlier::BeatmapDataLoader_SliderConverter as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<
                    crate::BeatmapSaveDataVersion2_6_0AndEarlier::SliderData,
                >),
                quest_hook::libil2cpp::Gc<crate::GlobalNamespace::SliderData>,
                1usize,
            >("Convert")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate
                    ::BeatmapDataLoaderVersion2_6_0AndEarlier::BeatmapDataLoader_SliderConverter
                    as quest_hook::libil2cpp::Type > ::class(), "Convert", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::SliderData> = unsafe {
            method.invoke_unchecked(self, (s))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn New(
        bpmTimeProcessor: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::IBeatToTimeConverter,
        >,
        rotationTimeProcessor: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::RotationTimeProcessor,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (bpmTimeProcessor, rotationTimeProcessor))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        bpmTimeProcessor: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::IBeatToTimeConverter,
        >,
        rotationTimeProcessor: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::RotationTimeProcessor,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::BeatmapDataLoaderVersion2_6_0AndEarlier::BeatmapDataLoader_SliderConverter as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<
                        crate::GlobalNamespace::IBeatToTimeConverter,
                    >,
                    quest_hook::libil2cpp::Gc<
                        crate::GlobalNamespace::RotationTimeProcessor,
                    >,
                ),
                quest_hook::libil2cpp::Void,
                2usize,
            >(".ctor")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate
                    ::BeatmapDataLoaderVersion2_6_0AndEarlier::BeatmapDataLoader_SliderConverter
                    as quest_hook::libil2cpp::Type > ::class(), ".ctor", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (bpmTimeProcessor, rotationTimeProcessor))?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(
    feature = "BeatmapDataLoaderVersion2_6_0AndEarlier+BeatmapDataLoader+SliderConverter"
)]
impl quest_hook::libil2cpp::ObjectType
for crate::BeatmapDataLoaderVersion2_6_0AndEarlier::BeatmapDataLoader_SliderConverter {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(
    feature = "BeatmapDataLoaderVersion2_6_0AndEarlier+BeatmapDataLoader+SpecialEventsFilter"
)]
#[repr(C)]
#[derive(Debug)]
pub struct BeatmapDataLoader_SpecialEventsFilter {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _eventTypesToFilter: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::HashSet_1<
            crate::BeatmapSaveDataCommon::BeatmapEventType,
        >,
    >,
}
#[cfg(
    feature = "BeatmapDataLoaderVersion2_6_0AndEarlier+BeatmapDataLoader+SpecialEventsFilter"
)]
unsafe impl quest_hook::libil2cpp::Type
for crate::BeatmapDataLoaderVersion2_6_0AndEarlier::BeatmapDataLoader_SpecialEventsFilter {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "BeatmapDataLoaderVersion2_6_0AndEarlier";
    const CLASS_NAME: &'static str = "BeatmapDataLoader/SpecialEventsFilter";
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
#[cfg(
    feature = "BeatmapDataLoaderVersion2_6_0AndEarlier+BeatmapDataLoader+SpecialEventsFilter"
)]
impl std::ops::Deref
for crate::BeatmapDataLoaderVersion2_6_0AndEarlier::BeatmapDataLoader_SpecialEventsFilter {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(
    feature = "BeatmapDataLoaderVersion2_6_0AndEarlier+BeatmapDataLoader+SpecialEventsFilter"
)]
impl std::ops::DerefMut
for crate::BeatmapDataLoaderVersion2_6_0AndEarlier::BeatmapDataLoader_SpecialEventsFilter {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(
    feature = "BeatmapDataLoaderVersion2_6_0AndEarlier+BeatmapDataLoader+SpecialEventsFilter"
)]
impl crate::BeatmapDataLoaderVersion2_6_0AndEarlier::BeatmapDataLoader_SpecialEventsFilter {
    pub fn IsEventValid(
        &mut self,
        basicBeatmapEventType: crate::BeatmapSaveDataCommon::BeatmapEventType,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::BeatmapDataLoaderVersion2_6_0AndEarlier::BeatmapDataLoader_SpecialEventsFilter as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (crate::BeatmapSaveDataCommon::BeatmapEventType),
                bool,
                1usize,
            >("IsEventValid")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate
                    ::BeatmapDataLoaderVersion2_6_0AndEarlier::BeatmapDataLoader_SpecialEventsFilter
                    as quest_hook::libil2cpp::Type > ::class(), "IsEventValid", 1usize
                )
            });
        let __cordl_ret: bool = unsafe {
            method.invoke_unchecked(self, (basicBeatmapEventType))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn New(
        basicEventTypesWithKeywords: quest_hook::libil2cpp::Gc<
            crate::BeatmapSaveDataVersion2_6_0AndEarlier::SpecialEventKeywordFiltersData,
        >,
        environmentKeywords: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::EnvironmentKeywords,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (basicEventTypesWithKeywords, environmentKeywords))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        basicEventTypesWithKeywords: quest_hook::libil2cpp::Gc<
            crate::BeatmapSaveDataVersion2_6_0AndEarlier::SpecialEventKeywordFiltersData,
        >,
        environmentKeywords: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::EnvironmentKeywords,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::BeatmapDataLoaderVersion2_6_0AndEarlier::BeatmapDataLoader_SpecialEventsFilter as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<
                        crate::BeatmapSaveDataVersion2_6_0AndEarlier::SpecialEventKeywordFiltersData,
                    >,
                    quest_hook::libil2cpp::Gc<
                        crate::GlobalNamespace::EnvironmentKeywords,
                    >,
                ),
                quest_hook::libil2cpp::Void,
                2usize,
            >(".ctor")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate
                    ::BeatmapDataLoaderVersion2_6_0AndEarlier::BeatmapDataLoader_SpecialEventsFilter
                    as quest_hook::libil2cpp::Type > ::class(), ".ctor", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method
                .invoke_unchecked(
                    self,
                    (basicEventTypesWithKeywords, environmentKeywords),
                )?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(
    feature = "BeatmapDataLoaderVersion2_6_0AndEarlier+BeatmapDataLoader+SpecialEventsFilter"
)]
impl quest_hook::libil2cpp::ObjectType
for crate::BeatmapDataLoaderVersion2_6_0AndEarlier::BeatmapDataLoader_SpecialEventsFilter {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(
    feature = "BeatmapDataLoaderVersion2_6_0AndEarlier+BeatmapDataLoader+WaypointConverter"
)]
#[repr(C)]
#[derive(Debug)]
pub struct BeatmapDataLoader_WaypointConverter {
    __cordl_parent: crate::GlobalNamespace::BeatToTimeAndRotationConverter,
}
#[cfg(
    feature = "BeatmapDataLoaderVersion2_6_0AndEarlier+BeatmapDataLoader+WaypointConverter"
)]
unsafe impl quest_hook::libil2cpp::Type
for crate::BeatmapDataLoaderVersion2_6_0AndEarlier::BeatmapDataLoader_WaypointConverter {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "BeatmapDataLoaderVersion2_6_0AndEarlier";
    const CLASS_NAME: &'static str = "BeatmapDataLoader/WaypointConverter";
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
#[cfg(
    feature = "BeatmapDataLoaderVersion2_6_0AndEarlier+BeatmapDataLoader+WaypointConverter"
)]
impl std::ops::Deref
for crate::BeatmapDataLoaderVersion2_6_0AndEarlier::BeatmapDataLoader_WaypointConverter {
    type Target = crate::GlobalNamespace::BeatToTimeAndRotationConverter;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(
    feature = "BeatmapDataLoaderVersion2_6_0AndEarlier+BeatmapDataLoader+WaypointConverter"
)]
impl std::ops::DerefMut
for crate::BeatmapDataLoaderVersion2_6_0AndEarlier::BeatmapDataLoader_WaypointConverter {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(
    feature = "BeatmapDataLoaderVersion2_6_0AndEarlier+BeatmapDataLoader+WaypointConverter"
)]
impl crate::BeatmapDataLoaderVersion2_6_0AndEarlier::BeatmapDataLoader_WaypointConverter {
    pub fn Convert(
        &mut self,
        waypointSaveData: quest_hook::libil2cpp::Gc<
            crate::BeatmapSaveDataVersion2_6_0AndEarlier::WaypointData,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::WaypointData>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::BeatmapDataLoaderVersion2_6_0AndEarlier::BeatmapDataLoader_WaypointConverter as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<
                    crate::BeatmapSaveDataVersion2_6_0AndEarlier::WaypointData,
                >),
                quest_hook::libil2cpp::Gc<crate::GlobalNamespace::WaypointData>,
                1usize,
            >("Convert")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate
                    ::BeatmapDataLoaderVersion2_6_0AndEarlier::BeatmapDataLoader_WaypointConverter
                    as quest_hook::libil2cpp::Type > ::class(), "Convert", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::WaypointData,
        > = unsafe { method.invoke_unchecked(self, (waypointSaveData))? };
        Ok(__cordl_ret.into())
    }
    pub fn New(
        bpmTimeProcessor: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::IBeatToTimeConverter,
        >,
        rotationTimeProcessor: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::RotationTimeProcessor,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (bpmTimeProcessor, rotationTimeProcessor))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        bpmTimeProcessor: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::IBeatToTimeConverter,
        >,
        rotationTimeProcessor: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::RotationTimeProcessor,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::BeatmapDataLoaderVersion2_6_0AndEarlier::BeatmapDataLoader_WaypointConverter as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<
                        crate::GlobalNamespace::IBeatToTimeConverter,
                    >,
                    quest_hook::libil2cpp::Gc<
                        crate::GlobalNamespace::RotationTimeProcessor,
                    >,
                ),
                quest_hook::libil2cpp::Void,
                2usize,
            >(".ctor")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate
                    ::BeatmapDataLoaderVersion2_6_0AndEarlier::BeatmapDataLoader_WaypointConverter
                    as quest_hook::libil2cpp::Type > ::class(), ".ctor", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (bpmTimeProcessor, rotationTimeProcessor))?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(
    feature = "BeatmapDataLoaderVersion2_6_0AndEarlier+BeatmapDataLoader+WaypointConverter"
)]
impl quest_hook::libil2cpp::ObjectType
for crate::BeatmapDataLoaderVersion2_6_0AndEarlier::BeatmapDataLoader_WaypointConverter {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
