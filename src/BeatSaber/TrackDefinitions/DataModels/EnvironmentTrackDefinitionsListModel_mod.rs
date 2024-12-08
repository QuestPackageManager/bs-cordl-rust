#[cfg(
    feature = "BeatSaber+TrackDefinitions+DataModels+EnvironmentTrackDefinitionsListModel"
)]
#[repr(C)]
#[derive(Debug)]
pub struct EnvironmentTrackDefinitionsListModel {
    __cordl_parent: crate::System::Object,
    pub _environmentTracksTypeMap: *mut crate::System::Collections::Generic::Dictionary_2<
        *mut EnvironmentInfoSO,
        *mut crate::BeatSaber::TrackDefinitions::DataModels::EnvironmentTrackDefinitionModel,
    >,
}
#[cfg(
    feature = "BeatSaber+TrackDefinitions+DataModels+EnvironmentTrackDefinitionsListModel"
)]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::BeatSaber::TrackDefinitions::DataModels::EnvironmentTrackDefinitionsListModel =>
    "BeatSaber.TrackDefinitions.DataModels"."EnvironmentTrackDefinitionsListModel"
);
#[cfg(
    feature = "BeatSaber+TrackDefinitions+DataModels+EnvironmentTrackDefinitionsListModel"
)]
impl std::ops::Deref
for crate::BeatSaber::TrackDefinitions::DataModels::EnvironmentTrackDefinitionsListModel {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(
    feature = "BeatSaber+TrackDefinitions+DataModels+EnvironmentTrackDefinitionsListModel"
)]
impl std::ops::DerefMut
for crate::BeatSaber::TrackDefinitions::DataModels::EnvironmentTrackDefinitionsListModel {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(
    feature = "BeatSaber+TrackDefinitions+DataModels+EnvironmentTrackDefinitionsListModel"
)]
impl crate::BeatSaber::TrackDefinitions::DataModels::EnvironmentTrackDefinitionsListModel {
    #[cfg(
        feature = "BeatSaber+TrackDefinitions+DataModels+EnvironmentTrackDefinitionsListModel+__c"
    )]
    pub type __c = crate::BeatSaber::TrackDefinitions::DataModels::EnvironmentTrackDefinitionsListModel___c;
    pub fn New(
        environmentTrackDefinitions: *mut crate::System::Collections::Generic::IEnumerable_1<
            *mut crate::BeatSaber::TrackDefinitions::EnvironmentTracksDefinitionSO,
        >,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (environmentTrackDefinitions))?;
        Ok(__cordl_object)
    }
    pub fn _ctor(
        &mut self,
        environmentTrackDefinitions: *mut crate::System::Collections::Generic::IEnumerable_1<
            *mut crate::BeatSaber::TrackDefinitions::EnvironmentTracksDefinitionSO,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (environmentTrackDefinitions))?;
        Ok(__cordl_ret)
    }
    pub fn get_Item(
        &mut self,
        _cordl_type: *mut EnvironmentInfoSO,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::BeatSaber::TrackDefinitions::DataModels::EnvironmentTrackDefinitionModel,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::BeatSaber::TrackDefinitions::DataModels::EnvironmentTrackDefinitionModel = __cordl_object
            .invoke("get_Item", (_cordl_type))?;
        Ok(__cordl_ret)
    }
}
#[cfg(
    feature = "BeatSaber+TrackDefinitions+DataModels+EnvironmentTrackDefinitionsListModel"
)]
impl quest_hook::libil2cpp::ObjectType
for crate::BeatSaber::TrackDefinitions::DataModels::EnvironmentTrackDefinitionsListModel {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}