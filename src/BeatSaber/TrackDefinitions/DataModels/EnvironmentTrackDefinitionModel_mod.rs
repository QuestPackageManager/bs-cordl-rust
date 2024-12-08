#[cfg(feature = "BeatSaber+TrackDefinitions+DataModels+EnvironmentTrackDefinitionModel")]
#[repr(C)]
#[derive(Debug)]
pub struct EnvironmentTrackDefinitionModel {
    __cordl_parent: crate::System::Object,
    pub _trackDefinition: *mut crate::BeatSaber::TrackDefinitions::EnvironmentTracksDefinitionSO,
    pub _beatmapTypeToTrackInfoMap: *mut crate::System::Collections::Generic::Dictionary_2<
        BasicBeatmapEventType,
        *mut crate::BeatSaber::TrackDefinitions::EnvironmentTracksDefinitionSO_BasicEventTrackInfo,
    >,
    pub _trackDefinitionToTrackInfoListMap: *mut crate::System::Collections::Generic::Dictionary_2<
        *mut crate::BeatSaber::TrackDefinitions::EventTrackDefinitionSO,
        *mut quest_hook::libil2cpp::Il2CppArray<
            *mut crate::BeatSaber::TrackDefinitions::EnvironmentTracksDefinitionSO_BasicEventTrackInfo,
        >,
    >,
    pub _trackPageToTrackInfoListMap: *mut quest_hook::libil2cpp::Il2CppArray<
        *mut crate::System::Collections::Generic::List_1<
            *mut crate::BeatSaber::TrackDefinitions::EnvironmentTracksDefinitionSO_BasicEventTrackInfo,
        >,
    >,
    pub _trackPageToTrackToolbarTypeMap: *mut quest_hook::libil2cpp::Il2CppArray<
        *mut crate::System::Collections::Generic::List_1<
            crate::BeatSaber::TrackDefinitions::DataModels::TrackToolbarType,
        >,
    >,
    pub _visibleTrackInfos: *mut quest_hook::libil2cpp::Il2CppArray<
        *mut crate::BeatSaber::TrackDefinitions::EnvironmentTracksDefinitionSO_BasicEventTrackInfo,
    >,
    pub _groupIdToPageMap: *mut crate::System::Collections::Generic::Dictionary_2<
        i32,
        i32,
    >,
    pub _groupIdToTrackMap: *mut crate::System::Collections::Generic::Dictionary_2<
        i32,
        *mut crate::BeatSaber::TrackDefinitions::EnvironmentTracksDefinitionSO_EventBoxGroupTrackInfo,
    >,
    pub _spawnableEventBoxGroupTracksByPageMap: *mut crate::System::Collections::Generic::Dictionary_2<
        i32,
        *mut crate::System::Collections::Generic::List_1<
            *mut crate::BeatSaber::TrackDefinitions::DataModels::EnvironmentTrackDefinitionModel_SpawnableEventBoxGroupTrack,
        >,
    >,
}
#[cfg(feature = "BeatSaber+TrackDefinitions+DataModels+EnvironmentTrackDefinitionModel")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::BeatSaber::TrackDefinitions::DataModels::EnvironmentTrackDefinitionModel =>
    "BeatSaber.TrackDefinitions.DataModels"."EnvironmentTrackDefinitionModel"
);
#[cfg(feature = "BeatSaber+TrackDefinitions+DataModels+EnvironmentTrackDefinitionModel")]
impl std::ops::Deref
for crate::BeatSaber::TrackDefinitions::DataModels::EnvironmentTrackDefinitionModel {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BeatSaber+TrackDefinitions+DataModels+EnvironmentTrackDefinitionModel")]
impl std::ops::DerefMut
for crate::BeatSaber::TrackDefinitions::DataModels::EnvironmentTrackDefinitionModel {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BeatSaber+TrackDefinitions+DataModels+EnvironmentTrackDefinitionModel")]
impl crate::BeatSaber::TrackDefinitions::DataModels::EnvironmentTrackDefinitionModel {
    #[cfg(
        feature = "BeatSaber+TrackDefinitions+DataModels+EnvironmentTrackDefinitionModel+__c"
    )]
    pub type __c = crate::BeatSaber::TrackDefinitions::DataModels::EnvironmentTrackDefinitionModel___c;
    #[cfg(
        feature = "BeatSaber+TrackDefinitions+DataModels+EnvironmentTrackDefinitionModel+SpawnableEventBoxGroupTypeTrack"
    )]
    pub type SpawnableEventBoxGroupTypeTrack = crate::BeatSaber::TrackDefinitions::DataModels::EnvironmentTrackDefinitionModel_SpawnableEventBoxGroupTypeTrack;
    #[cfg(
        feature = "BeatSaber+TrackDefinitions+DataModels+EnvironmentTrackDefinitionModel+SpawnableEventBoxGroupTrack"
    )]
    pub type SpawnableEventBoxGroupTrack = crate::BeatSaber::TrackDefinitions::DataModels::EnvironmentTrackDefinitionModel_SpawnableEventBoxGroupTrack;
    pub fn GetSpawnableEventBoxGroupTracks(
        &mut self,
        pageId: i32,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Collections::Generic::List_1<
            *mut crate::BeatSaber::TrackDefinitions::DataModels::EnvironmentTrackDefinitionModel_SpawnableEventBoxGroupTrack,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Collections::Generic::List_1<
            *mut crate::BeatSaber::TrackDefinitions::DataModels::EnvironmentTrackDefinitionModel_SpawnableEventBoxGroupTrack,
        > = __cordl_object.invoke("GetSpawnableEventBoxGroupTracks", (pageId))?;
        Ok(__cordl_ret)
    }
    pub fn GetToolbarTypesOnPage(
        &mut self,
        page: crate::BeatSaber::TrackDefinitions::EnvironmentTracksDefinitionSO_BasicEventTrackPage,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Collections::Generic::List_1<
            crate::BeatSaber::TrackDefinitions::DataModels::TrackToolbarType,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Collections::Generic::List_1<
            crate::BeatSaber::TrackDefinitions::DataModels::TrackToolbarType,
        > = __cordl_object.invoke("GetToolbarTypesOnPage", (page))?;
        Ok(__cordl_ret)
    }
    pub fn New(
        trackDefinition: *mut crate::BeatSaber::TrackDefinitions::EnvironmentTracksDefinitionSO,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (trackDefinition))?;
        Ok(__cordl_object)
    }
    pub fn _ctor(
        &mut self,
        trackDefinition: *mut crate::BeatSaber::TrackDefinitions::EnvironmentTracksDefinitionSO,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (trackDefinition))?;
        Ok(__cordl_ret)
    }
    pub fn get_Item_BasicBeatmapEventType0(
        &mut self,
        _cordl_type: BasicBeatmapEventType,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::BeatSaber::TrackDefinitions::EnvironmentTracksDefinitionSO_BasicEventTrackInfo,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::BeatSaber::TrackDefinitions::EnvironmentTracksDefinitionSO_BasicEventTrackInfo = __cordl_object
            .invoke("get_Item", (_cordl_type))?;
        Ok(__cordl_ret)
    }
    pub fn get_Item_EnvironmentTracksDefinitionSO_BasicEventTrackPage2(
        &mut self,
        page: crate::BeatSaber::TrackDefinitions::EnvironmentTracksDefinitionSO_BasicEventTrackPage,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Collections::Generic::List_1<
            *mut crate::BeatSaber::TrackDefinitions::EnvironmentTracksDefinitionSO_BasicEventTrackInfo,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Collections::Generic::List_1<
            *mut crate::BeatSaber::TrackDefinitions::EnvironmentTracksDefinitionSO_BasicEventTrackInfo,
        > = __cordl_object.invoke("get_Item", (page))?;
        Ok(__cordl_ret)
    }
    pub fn get_Item_EventTrackDefinitionSO1(
        &mut self,
        trackDefinition: *mut crate::BeatSaber::TrackDefinitions::EventTrackDefinitionSO,
    ) -> quest_hook::libil2cpp::Result<
        *mut quest_hook::libil2cpp::Il2CppArray<
            *mut crate::BeatSaber::TrackDefinitions::EnvironmentTracksDefinitionSO_BasicEventTrackInfo,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppArray<
            *mut crate::BeatSaber::TrackDefinitions::EnvironmentTracksDefinitionSO_BasicEventTrackInfo,
        > = __cordl_object.invoke("get_Item", (trackDefinition))?;
        Ok(__cordl_ret)
    }
    pub fn get_basicEventTrackInfos(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut quest_hook::libil2cpp::Il2CppArray<
            *mut crate::BeatSaber::TrackDefinitions::EnvironmentTracksDefinitionSO_BasicEventTrackInfo,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppArray<
            *mut crate::BeatSaber::TrackDefinitions::EnvironmentTracksDefinitionSO_BasicEventTrackInfo,
        > = __cordl_object.invoke("get_basicEventTrackInfos", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_beatmapTypeToTrackInfoMap(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut quest_hook::libil2cpp::Il2CppArray<
            *mut crate::System::Collections::Generic::List_1<
                *mut crate::BeatSaber::TrackDefinitions::EnvironmentTracksDefinitionSO_BasicEventTrackInfo,
            >,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppArray<
            *mut crate::System::Collections::Generic::List_1<
                *mut crate::BeatSaber::TrackDefinitions::EnvironmentTracksDefinitionSO_BasicEventTrackInfo,
            >,
        > = __cordl_object.invoke("get_beatmapTypeToTrackInfoMap", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_eventBoxGroupPageInfos(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Collections::Generic::List_1<
            *mut crate::BeatSaber::TrackDefinitions::EnvironmentTracksDefinitionSO_EventBoxGroupPageInfo,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Collections::Generic::List_1<
            *mut crate::BeatSaber::TrackDefinitions::EnvironmentTracksDefinitionSO_EventBoxGroupPageInfo,
        > = __cordl_object.invoke("get_eventBoxGroupPageInfos", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_groupIdToPageMap(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Collections::Generic::IReadOnlyDictionary_2<i32, i32>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Collections::Generic::IReadOnlyDictionary_2<
            i32,
            i32,
        > = __cordl_object.invoke("get_groupIdToPageMap", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_groupIdToTrackInfo(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Collections::Generic::IReadOnlyDictionary_2<
            i32,
            *mut crate::BeatSaber::TrackDefinitions::EnvironmentTracksDefinitionSO_EventBoxGroupTrackInfo,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Collections::Generic::IReadOnlyDictionary_2<
            i32,
            *mut crate::BeatSaber::TrackDefinitions::EnvironmentTracksDefinitionSO_EventBoxGroupTrackInfo,
        > = __cordl_object.invoke("get_groupIdToTrackInfo", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_pageCount(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_pageCount", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_visibleTrackInfos(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut quest_hook::libil2cpp::Il2CppArray<
            *mut crate::BeatSaber::TrackDefinitions::EnvironmentTracksDefinitionSO_BasicEventTrackInfo,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppArray<
            *mut crate::BeatSaber::TrackDefinitions::EnvironmentTracksDefinitionSO_BasicEventTrackInfo,
        > = __cordl_object.invoke("get_visibleTrackInfos", ())?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "BeatSaber+TrackDefinitions+DataModels+EnvironmentTrackDefinitionModel")]
impl quest_hook::libil2cpp::ObjectType
for crate::BeatSaber::TrackDefinitions::DataModels::EnvironmentTrackDefinitionModel {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(
    feature = "BeatSaber+TrackDefinitions+DataModels+EnvironmentTrackDefinitionModel+SpawnableEventBoxGroupTrack"
)]
#[repr(C)]
#[derive(Debug)]
pub struct EnvironmentTrackDefinitionModel_SpawnableEventBoxGroupTrack {
    __cordl_parent: crate::System::Object,
    pub lightGroup: *mut LightGroupSO,
    pub groupName: *mut crate::System::String,
    pub tracksCount: i32,
    pub eventBoxGroupTracks: *mut crate::System::Collections::Generic::IReadOnlyList_1<
        *mut crate::BeatSaber::TrackDefinitions::DataModels::EnvironmentTrackDefinitionModel_SpawnableEventBoxGroupTypeTrack,
    >,
}
#[cfg(
    feature = "BeatSaber+TrackDefinitions+DataModels+EnvironmentTrackDefinitionModel+SpawnableEventBoxGroupTrack"
)]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::BeatSaber::TrackDefinitions::DataModels::EnvironmentTrackDefinitionModel_SpawnableEventBoxGroupTrack
    => "BeatSaber.TrackDefinitions.DataModels"
    ."EnvironmentTrackDefinitionModel/SpawnableEventBoxGroupTrack"
);
#[cfg(
    feature = "BeatSaber+TrackDefinitions+DataModels+EnvironmentTrackDefinitionModel+SpawnableEventBoxGroupTrack"
)]
impl std::ops::Deref
for crate::BeatSaber::TrackDefinitions::DataModels::EnvironmentTrackDefinitionModel_SpawnableEventBoxGroupTrack {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(
    feature = "BeatSaber+TrackDefinitions+DataModels+EnvironmentTrackDefinitionModel+SpawnableEventBoxGroupTrack"
)]
impl std::ops::DerefMut
for crate::BeatSaber::TrackDefinitions::DataModels::EnvironmentTrackDefinitionModel_SpawnableEventBoxGroupTrack {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(
    feature = "BeatSaber+TrackDefinitions+DataModels+EnvironmentTrackDefinitionModel+SpawnableEventBoxGroupTrack"
)]
impl crate::BeatSaber::TrackDefinitions::DataModels::EnvironmentTrackDefinitionModel_SpawnableEventBoxGroupTrack {
    pub fn New(
        lightGroup: *mut LightGroupSO,
        groupName: *mut crate::System::String,
        tracksCount: i32,
        eventBoxGroupTracks: *mut crate::System::Collections::Generic::IReadOnlyList_1<
            *mut crate::BeatSaber::TrackDefinitions::DataModels::EnvironmentTrackDefinitionModel_SpawnableEventBoxGroupTypeTrack,
        >,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(
                ".ctor",
                (lightGroup, groupName, tracksCount, eventBoxGroupTracks),
            )?;
        Ok(__cordl_object)
    }
    pub fn _ctor(
        &mut self,
        lightGroup: *mut LightGroupSO,
        groupName: *mut crate::System::String,
        tracksCount: i32,
        eventBoxGroupTracks: *mut crate::System::Collections::Generic::IReadOnlyList_1<
            *mut crate::BeatSaber::TrackDefinitions::DataModels::EnvironmentTrackDefinitionModel_SpawnableEventBoxGroupTypeTrack,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (lightGroup, groupName, tracksCount, eventBoxGroupTracks))?;
        Ok(__cordl_ret)
    }
}
#[cfg(
    feature = "BeatSaber+TrackDefinitions+DataModels+EnvironmentTrackDefinitionModel+SpawnableEventBoxGroupTrack"
)]
impl quest_hook::libil2cpp::ObjectType
for crate::BeatSaber::TrackDefinitions::DataModels::EnvironmentTrackDefinitionModel_SpawnableEventBoxGroupTrack {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(
    feature = "BeatSaber+TrackDefinitions+DataModels+EnvironmentTrackDefinitionModel+SpawnableEventBoxGroupTypeTrack"
)]
#[repr(C)]
#[derive(Debug)]
pub struct EnvironmentTrackDefinitionModel_SpawnableEventBoxGroupTypeTrack {
    __cordl_parent: crate::System::Object,
    pub lightGroup: *mut LightGroupSO,
    pub groupName: *mut crate::System::String,
    pub trackType: crate::BeatSaber::TrackDefinitions::DataModels::EventBoxGroupType,
}
#[cfg(
    feature = "BeatSaber+TrackDefinitions+DataModels+EnvironmentTrackDefinitionModel+SpawnableEventBoxGroupTypeTrack"
)]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::BeatSaber::TrackDefinitions::DataModels::EnvironmentTrackDefinitionModel_SpawnableEventBoxGroupTypeTrack
    => "BeatSaber.TrackDefinitions.DataModels"
    ."EnvironmentTrackDefinitionModel/SpawnableEventBoxGroupTypeTrack"
);
#[cfg(
    feature = "BeatSaber+TrackDefinitions+DataModels+EnvironmentTrackDefinitionModel+SpawnableEventBoxGroupTypeTrack"
)]
impl std::ops::Deref
for crate::BeatSaber::TrackDefinitions::DataModels::EnvironmentTrackDefinitionModel_SpawnableEventBoxGroupTypeTrack {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(
    feature = "BeatSaber+TrackDefinitions+DataModels+EnvironmentTrackDefinitionModel+SpawnableEventBoxGroupTypeTrack"
)]
impl std::ops::DerefMut
for crate::BeatSaber::TrackDefinitions::DataModels::EnvironmentTrackDefinitionModel_SpawnableEventBoxGroupTypeTrack {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(
    feature = "BeatSaber+TrackDefinitions+DataModels+EnvironmentTrackDefinitionModel+SpawnableEventBoxGroupTypeTrack"
)]
impl crate::BeatSaber::TrackDefinitions::DataModels::EnvironmentTrackDefinitionModel_SpawnableEventBoxGroupTypeTrack {
    pub fn New(
        lightGroup: *mut LightGroupSO,
        groupName: *mut crate::System::String,
        trackType: crate::BeatSaber::TrackDefinitions::DataModels::EventBoxGroupType,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (lightGroup, groupName, trackType))?;
        Ok(__cordl_object)
    }
    pub fn _ctor(
        &mut self,
        lightGroup: *mut LightGroupSO,
        groupName: *mut crate::System::String,
        trackType: crate::BeatSaber::TrackDefinitions::DataModels::EventBoxGroupType,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (lightGroup, groupName, trackType))?;
        Ok(__cordl_ret)
    }
}
#[cfg(
    feature = "BeatSaber+TrackDefinitions+DataModels+EnvironmentTrackDefinitionModel+SpawnableEventBoxGroupTypeTrack"
)]
impl quest_hook::libil2cpp::ObjectType
for crate::BeatSaber::TrackDefinitions::DataModels::EnvironmentTrackDefinitionModel_SpawnableEventBoxGroupTypeTrack {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}