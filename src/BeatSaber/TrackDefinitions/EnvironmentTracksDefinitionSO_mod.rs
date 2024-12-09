#[cfg(feature = "BeatSaber+TrackDefinitions+EnvironmentTracksDefinitionSO")]
#[repr(C)]
#[derive(Debug)]
pub struct EnvironmentTracksDefinitionSO {
    __cordl_parent: crate::GlobalNamespace::PersistentScriptableObject,
    pub _environmentInfo: *mut crate::GlobalNamespace::EnvironmentInfoSO,
    pub _basicEventTrackInfos: *mut quest_hook::libil2cpp::Il2CppArray<
        *mut crate::BeatSaber::TrackDefinitions::EnvironmentTracksDefinitionSO_BasicEventTrackInfo,
    >,
    pub _eventBoxGroupPageInfos: *mut crate::System::Collections::Generic::List_1<
        *mut crate::BeatSaber::TrackDefinitions::EnvironmentTracksDefinitionSO_EventBoxGroupPageInfo,
    >,
}
#[cfg(feature = "BeatSaber+TrackDefinitions+EnvironmentTracksDefinitionSO")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::BeatSaber::TrackDefinitions::EnvironmentTracksDefinitionSO =>
    "BeatSaber.TrackDefinitions"."EnvironmentTracksDefinitionSO"
);
#[cfg(feature = "BeatSaber+TrackDefinitions+EnvironmentTracksDefinitionSO")]
impl std::ops::Deref
for crate::BeatSaber::TrackDefinitions::EnvironmentTracksDefinitionSO {
    type Target = crate::GlobalNamespace::PersistentScriptableObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BeatSaber+TrackDefinitions+EnvironmentTracksDefinitionSO")]
impl std::ops::DerefMut
for crate::BeatSaber::TrackDefinitions::EnvironmentTracksDefinitionSO {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BeatSaber+TrackDefinitions+EnvironmentTracksDefinitionSO")]
impl crate::BeatSaber::TrackDefinitions::EnvironmentTracksDefinitionSO {
    #[cfg(
        feature = "BeatSaber+TrackDefinitions+EnvironmentTracksDefinitionSO+BasicEventTrackInfo"
    )]
    pub type BasicEventTrackInfo = crate::BeatSaber::TrackDefinitions::EnvironmentTracksDefinitionSO_BasicEventTrackInfo;
    #[cfg(
        feature = "BeatSaber+TrackDefinitions+EnvironmentTracksDefinitionSO+BasicEventTrackPage"
    )]
    pub type BasicEventTrackPage = crate::BeatSaber::TrackDefinitions::EnvironmentTracksDefinitionSO_BasicEventTrackPage;
    #[cfg(
        feature = "BeatSaber+TrackDefinitions+EnvironmentTracksDefinitionSO+EventBoxGroupPageInfo"
    )]
    pub type EventBoxGroupPageInfo = crate::BeatSaber::TrackDefinitions::EnvironmentTracksDefinitionSO_EventBoxGroupPageInfo;
    #[cfg(
        feature = "BeatSaber+TrackDefinitions+EnvironmentTracksDefinitionSO+EventBoxGroupTrackInfo"
    )]
    pub type EventBoxGroupTrackInfo = crate::BeatSaber::TrackDefinitions::EnvironmentTracksDefinitionSO_EventBoxGroupTrackInfo;
    #[cfg(
        feature = "BeatSaber+TrackDefinitions+EnvironmentTracksDefinitionSO+OverrideDefaultLightAxis"
    )]
    pub type OverrideDefaultLightAxis = crate::BeatSaber::TrackDefinitions::EnvironmentTracksDefinitionSO_OverrideDefaultLightAxis;
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
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
    pub fn get_environmentInfo(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::GlobalNamespace::EnvironmentInfoSO> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::GlobalNamespace::EnvironmentInfoSO = __cordl_object
            .invoke("get_environmentInfo", ())?;
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
}
#[cfg(feature = "BeatSaber+TrackDefinitions+EnvironmentTracksDefinitionSO")]
impl quest_hook::libil2cpp::ObjectType
for crate::BeatSaber::TrackDefinitions::EnvironmentTracksDefinitionSO {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(
    feature = "BeatSaber+TrackDefinitions+EnvironmentTracksDefinitionSO+BasicEventTrackInfo"
)]
#[repr(C)]
#[derive(Debug)]
pub struct EnvironmentTracksDefinitionSO_BasicEventTrackInfo {
    __cordl_parent: crate::System::Object,
    pub _trackName: *mut crate::System::String,
    pub _beatmapEventType: crate::GlobalNamespace::BasicBeatmapEventType,
    pub _trackToolbarType: crate::BeatSaber::TrackDefinitions::DataModels::TrackToolbarType,
    pub _trackDefinition: *mut crate::BeatSaber::TrackDefinitions::EventTrackDefinitionSO,
    pub _basicEventTrackPage: crate::BeatSaber::TrackDefinitions::EnvironmentTracksDefinitionSO_BasicEventTrackPage,
}
#[cfg(
    feature = "BeatSaber+TrackDefinitions+EnvironmentTracksDefinitionSO+BasicEventTrackInfo"
)]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::BeatSaber::TrackDefinitions::EnvironmentTracksDefinitionSO_BasicEventTrackInfo =>
    "BeatSaber.TrackDefinitions"."EnvironmentTracksDefinitionSO/BasicEventTrackInfo"
);
#[cfg(
    feature = "BeatSaber+TrackDefinitions+EnvironmentTracksDefinitionSO+BasicEventTrackInfo"
)]
impl std::ops::Deref
for crate::BeatSaber::TrackDefinitions::EnvironmentTracksDefinitionSO_BasicEventTrackInfo {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(
    feature = "BeatSaber+TrackDefinitions+EnvironmentTracksDefinitionSO+BasicEventTrackInfo"
)]
impl std::ops::DerefMut
for crate::BeatSaber::TrackDefinitions::EnvironmentTracksDefinitionSO_BasicEventTrackInfo {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(
    feature = "BeatSaber+TrackDefinitions+EnvironmentTracksDefinitionSO+BasicEventTrackInfo"
)]
impl crate::BeatSaber::TrackDefinitions::EnvironmentTracksDefinitionSO_BasicEventTrackInfo {
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_basicBeatmapEventType(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::BasicBeatmapEventType> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::GlobalNamespace::BasicBeatmapEventType = __cordl_object
            .invoke("get_basicBeatmapEventType", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_basicEventTrackPage(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::BeatSaber::TrackDefinitions::EnvironmentTracksDefinitionSO_BasicEventTrackPage,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::BeatSaber::TrackDefinitions::EnvironmentTracksDefinitionSO_BasicEventTrackPage = __cordl_object
            .invoke("get_basicEventTrackPage", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_trackDefinition(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::BeatSaber::TrackDefinitions::EventTrackDefinitionSO,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::BeatSaber::TrackDefinitions::EventTrackDefinitionSO = __cordl_object
            .invoke("get_trackDefinition", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_trackName(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("get_trackName", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_trackToolbarType(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::BeatSaber::TrackDefinitions::DataModels::TrackToolbarType,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::BeatSaber::TrackDefinitions::DataModels::TrackToolbarType = __cordl_object
            .invoke("get_trackToolbarType", ())?;
        Ok(__cordl_ret)
    }
}
#[cfg(
    feature = "BeatSaber+TrackDefinitions+EnvironmentTracksDefinitionSO+BasicEventTrackInfo"
)]
impl quest_hook::libil2cpp::ObjectType
for crate::BeatSaber::TrackDefinitions::EnvironmentTracksDefinitionSO_BasicEventTrackInfo {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(
    feature = "BeatSaber+TrackDefinitions+EnvironmentTracksDefinitionSO+BasicEventTrackPage"
)]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EnvironmentTracksDefinitionSO_BasicEventTrackPage {
    Count = 2i32,
    Page1 = 0i32,
    Page2 = 1i32,
}
#[cfg(
    feature = "BeatSaber+TrackDefinitions+EnvironmentTracksDefinitionSO+BasicEventTrackPage"
)]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::BeatSaber::TrackDefinitions::EnvironmentTracksDefinitionSO_BasicEventTrackPage =>
    "BeatSaber.TrackDefinitions"."EnvironmentTracksDefinitionSO/BasicEventTrackPage"
);
#[cfg(
    feature = "BeatSaber+TrackDefinitions+EnvironmentTracksDefinitionSO+EventBoxGroupPageInfo"
)]
#[repr(C)]
#[derive(Debug)]
pub struct EnvironmentTracksDefinitionSO_EventBoxGroupPageInfo {
    __cordl_parent: crate::System::Object,
    pub _eventBoxGroupPageName: *mut crate::System::String,
    pub _eventBoxGroupTrackInfos: *mut crate::System::Collections::Generic::List_1<
        *mut crate::BeatSaber::TrackDefinitions::EnvironmentTracksDefinitionSO_EventBoxGroupTrackInfo,
    >,
}
#[cfg(
    feature = "BeatSaber+TrackDefinitions+EnvironmentTracksDefinitionSO+EventBoxGroupPageInfo"
)]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::BeatSaber::TrackDefinitions::EnvironmentTracksDefinitionSO_EventBoxGroupPageInfo =>
    "BeatSaber.TrackDefinitions"."EnvironmentTracksDefinitionSO/EventBoxGroupPageInfo"
);
#[cfg(
    feature = "BeatSaber+TrackDefinitions+EnvironmentTracksDefinitionSO+EventBoxGroupPageInfo"
)]
impl std::ops::Deref
for crate::BeatSaber::TrackDefinitions::EnvironmentTracksDefinitionSO_EventBoxGroupPageInfo {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(
    feature = "BeatSaber+TrackDefinitions+EnvironmentTracksDefinitionSO+EventBoxGroupPageInfo"
)]
impl std::ops::DerefMut
for crate::BeatSaber::TrackDefinitions::EnvironmentTracksDefinitionSO_EventBoxGroupPageInfo {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(
    feature = "BeatSaber+TrackDefinitions+EnvironmentTracksDefinitionSO+EventBoxGroupPageInfo"
)]
impl crate::BeatSaber::TrackDefinitions::EnvironmentTracksDefinitionSO_EventBoxGroupPageInfo {
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_eventBoxGroupPageName(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("get_eventBoxGroupPageName", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_eventBoxGroupTrackInfos(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Collections::Generic::List_1<
            *mut crate::BeatSaber::TrackDefinitions::EnvironmentTracksDefinitionSO_EventBoxGroupTrackInfo,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Collections::Generic::List_1<
            *mut crate::BeatSaber::TrackDefinitions::EnvironmentTracksDefinitionSO_EventBoxGroupTrackInfo,
        > = __cordl_object.invoke("get_eventBoxGroupTrackInfos", ())?;
        Ok(__cordl_ret)
    }
}
#[cfg(
    feature = "BeatSaber+TrackDefinitions+EnvironmentTracksDefinitionSO+EventBoxGroupPageInfo"
)]
impl quest_hook::libil2cpp::ObjectType
for crate::BeatSaber::TrackDefinitions::EnvironmentTracksDefinitionSO_EventBoxGroupPageInfo {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(
    feature = "BeatSaber+TrackDefinitions+EnvironmentTracksDefinitionSO+EventBoxGroupTrackInfo"
)]
#[repr(C)]
#[derive(Debug)]
pub struct EnvironmentTracksDefinitionSO_EventBoxGroupTrackInfo {
    __cordl_parent: crate::System::Object,
    pub _groupName: *mut crate::System::String,
    pub _lightGroup: *mut crate::GlobalNamespace::LightGroupSO,
    pub _showColorTrack: bool,
    pub _showRotationXTrack: bool,
    pub _showRotationYTrack: bool,
    pub _showRotationZTrack: bool,
    pub _overrideDefaultRotationAxis: crate::BeatSaber::TrackDefinitions::EnvironmentTracksDefinitionSO_OverrideDefaultLightAxis,
    pub _showTranslationXTrack: bool,
    pub _showTranslationYTrack: bool,
    pub _showTranslationZTrack: bool,
    pub _overrideDefaultTranslationAxis: crate::BeatSaber::TrackDefinitions::EnvironmentTracksDefinitionSO_OverrideDefaultLightAxis,
    pub _showFloatFxTrack: bool,
    pub _enableDuplicate: bool,
    pub _duplicationGroup: *mut crate::BeatSaber::TrackDefinitions::LightGroupDuplicationGroup,
    pub _targetLightGroups: *mut quest_hook::libil2cpp::Il2CppArray<
        *mut crate::GlobalNamespace::LightGroupSO,
    >,
}
#[cfg(
    feature = "BeatSaber+TrackDefinitions+EnvironmentTracksDefinitionSO+EventBoxGroupTrackInfo"
)]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::BeatSaber::TrackDefinitions::EnvironmentTracksDefinitionSO_EventBoxGroupTrackInfo
    => "BeatSaber.TrackDefinitions"
    ."EnvironmentTracksDefinitionSO/EventBoxGroupTrackInfo"
);
#[cfg(
    feature = "BeatSaber+TrackDefinitions+EnvironmentTracksDefinitionSO+EventBoxGroupTrackInfo"
)]
impl std::ops::Deref
for crate::BeatSaber::TrackDefinitions::EnvironmentTracksDefinitionSO_EventBoxGroupTrackInfo {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(
    feature = "BeatSaber+TrackDefinitions+EnvironmentTracksDefinitionSO+EventBoxGroupTrackInfo"
)]
impl std::ops::DerefMut
for crate::BeatSaber::TrackDefinitions::EnvironmentTracksDefinitionSO_EventBoxGroupTrackInfo {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(
    feature = "BeatSaber+TrackDefinitions+EnvironmentTracksDefinitionSO+EventBoxGroupTrackInfo"
)]
impl crate::BeatSaber::TrackDefinitions::EnvironmentTracksDefinitionSO_EventBoxGroupTrackInfo {
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_enableDuplicate(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_enableDuplicate", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_groupName(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("get_groupName", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_lightGroup(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::GlobalNamespace::LightGroupSO> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::GlobalNamespace::LightGroupSO = __cordl_object
            .invoke("get_lightGroup", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_overrideDefaultRotationAxis(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::BeatSaber::TrackDefinitions::EnvironmentTracksDefinitionSO_OverrideDefaultLightAxis,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::BeatSaber::TrackDefinitions::EnvironmentTracksDefinitionSO_OverrideDefaultLightAxis = __cordl_object
            .invoke("get_overrideDefaultRotationAxis", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_overrideDefaultTranslationAxis(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::BeatSaber::TrackDefinitions::EnvironmentTracksDefinitionSO_OverrideDefaultLightAxis,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::BeatSaber::TrackDefinitions::EnvironmentTracksDefinitionSO_OverrideDefaultLightAxis = __cordl_object
            .invoke("get_overrideDefaultTranslationAxis", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_showColorTrack(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_showColorTrack", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_showFloatFxTrack(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_showFloatFxTrack", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_showRotationTrack(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_showRotationTrack", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_showRotationXTrack(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_showRotationXTrack", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_showRotationYTrack(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_showRotationYTrack", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_showRotationZTrack(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_showRotationZTrack", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_showTranslationTrack(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_showTranslationTrack", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_showTranslationXTrack(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_showTranslationXTrack", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_showTranslationYTrack(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_showTranslationYTrack", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_showTranslationZTrack(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_showTranslationZTrack", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_targetLightGroups(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut quest_hook::libil2cpp::Il2CppArray<
            *mut crate::GlobalNamespace::LightGroupSO,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppArray<
            *mut crate::GlobalNamespace::LightGroupSO,
        > = __cordl_object.invoke("get_targetLightGroups", ())?;
        Ok(__cordl_ret)
    }
    pub fn set_groupName(
        &mut self,
        value: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_groupName", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_lightGroup(
        &mut self,
        value: *mut crate::GlobalNamespace::LightGroupSO,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_lightGroup", (value))?;
        Ok(__cordl_ret)
    }
}
#[cfg(
    feature = "BeatSaber+TrackDefinitions+EnvironmentTracksDefinitionSO+EventBoxGroupTrackInfo"
)]
impl quest_hook::libil2cpp::ObjectType
for crate::BeatSaber::TrackDefinitions::EnvironmentTracksDefinitionSO_EventBoxGroupTrackInfo {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(
    feature = "BeatSaber+TrackDefinitions+EnvironmentTracksDefinitionSO+OverrideDefaultLightAxis"
)]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EnvironmentTracksDefinitionSO_OverrideDefaultLightAxis {
    NoOverride = 0i32,
    X = 1i32,
    Y = 2i32,
    Z = 3i32,
}
#[cfg(
    feature = "BeatSaber+TrackDefinitions+EnvironmentTracksDefinitionSO+OverrideDefaultLightAxis"
)]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::BeatSaber::TrackDefinitions::EnvironmentTracksDefinitionSO_OverrideDefaultLightAxis
    => "BeatSaber.TrackDefinitions"
    ."EnvironmentTracksDefinitionSO/OverrideDefaultLightAxis"
);
