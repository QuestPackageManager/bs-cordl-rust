#[cfg(feature = "BeatSaber+TrackDefinitions+EventTrackDefinitionSO")]
#[repr(C)]
#[derive(Debug)]
pub struct EventTrackDefinitionSO {
    __cordl_parent: crate::GlobalNamespace::PersistentScriptableObject,
    pub _dataTransformationType: crate::BeatSaber::TrackDefinitions::EventTrackDefinitionSO_DataTransformationType,
    pub _markerType: crate::BeatSaber::TrackDefinitions::EventTrackDefinitionSO_MarkerType,
    pub _visible: bool,
    pub _needsFiltering: bool,
}
#[cfg(feature = "BeatSaber+TrackDefinitions+EventTrackDefinitionSO")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::BeatSaber::TrackDefinitions::EventTrackDefinitionSO => "BeatSaber.TrackDefinitions"
    ."EventTrackDefinitionSO"
);
#[cfg(feature = "BeatSaber+TrackDefinitions+EventTrackDefinitionSO")]
impl std::ops::Deref for crate::BeatSaber::TrackDefinitions::EventTrackDefinitionSO {
    type Target = crate::GlobalNamespace::PersistentScriptableObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BeatSaber+TrackDefinitions+EventTrackDefinitionSO")]
impl std::ops::DerefMut for crate::BeatSaber::TrackDefinitions::EventTrackDefinitionSO {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BeatSaber+TrackDefinitions+EventTrackDefinitionSO")]
impl crate::BeatSaber::TrackDefinitions::EventTrackDefinitionSO {
    #[cfg(
        feature = "BeatSaber+TrackDefinitions+EventTrackDefinitionSO+DataTransformationType"
    )]
    pub type DataTransformationType = crate::BeatSaber::TrackDefinitions::EventTrackDefinitionSO_DataTransformationType;
    #[cfg(feature = "BeatSaber+TrackDefinitions+EventTrackDefinitionSO+MarkerType")]
    pub type MarkerType = crate::BeatSaber::TrackDefinitions::EventTrackDefinitionSO_MarkerType;
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
    pub fn get_dataTransformation(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::BeatSaber::TrackDefinitions::EventTrackDefinitionSO_DataTransformationType,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::BeatSaber::TrackDefinitions::EventTrackDefinitionSO_DataTransformationType = __cordl_object
            .invoke("get_dataTransformation", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_markerType(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::BeatSaber::TrackDefinitions::EventTrackDefinitionSO_MarkerType,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::BeatSaber::TrackDefinitions::EventTrackDefinitionSO_MarkerType = __cordl_object
            .invoke("get_markerType", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_needsFiltering(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_needsFiltering", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_visible(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_visible", ())?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "BeatSaber+TrackDefinitions+EventTrackDefinitionSO")]
impl quest_hook::libil2cpp::ObjectType
for crate::BeatSaber::TrackDefinitions::EventTrackDefinitionSO {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(
    feature = "BeatSaber+TrackDefinitions+EventTrackDefinitionSO+DataTransformationType"
)]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EventTrackDefinitionSO_DataTransformationType {
    DeltaRotation = 2i32,
    Light = 1i32,
    NoTransformation = 0i32,
    Switch = 3i32,
    TurnOffValueDuration = 4i32,
    ValueDuration = 5i32,
}
#[cfg(
    feature = "BeatSaber+TrackDefinitions+EventTrackDefinitionSO+DataTransformationType"
)]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::BeatSaber::TrackDefinitions::EventTrackDefinitionSO_DataTransformationType =>
    "BeatSaber.TrackDefinitions"."EventTrackDefinitionSO/DataTransformationType"
);
#[cfg(feature = "BeatSaber+TrackDefinitions+EventTrackDefinitionSO+MarkerType")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EventTrackDefinitionSO_MarkerType {
    BasicMarker = 0i32,
    DurationMarker = 1i32,
    LightMarker = 2i32,
    TextMarker = 3i32,
    TooltipMarker = 4i32,
}
#[cfg(feature = "BeatSaber+TrackDefinitions+EventTrackDefinitionSO+MarkerType")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::BeatSaber::TrackDefinitions::EventTrackDefinitionSO_MarkerType =>
    "BeatSaber.TrackDefinitions"."EventTrackDefinitionSO/MarkerType"
);
