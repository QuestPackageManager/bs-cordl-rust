#[cfg(
    feature = "BeatSaber+TrackDefinitions+DataModels+EnvironmentTrackDefinitionsListModel"
)]
#[repr(C)]
#[derive(Debug)]
pub struct EnvironmentTrackDefinitionsListModel {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _environmentTracksTypeMap: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::Dictionary_2<
            quest_hook::libil2cpp::Gc<crate::GlobalNamespace::EnvironmentInfoSO>,
            quest_hook::libil2cpp::Gc<
                crate::BeatSaber::TrackDefinitions::DataModels::EnvironmentTrackDefinitionModel,
            >,
        >,
    >,
}
#[cfg(
    feature = "BeatSaber+TrackDefinitions+DataModels+EnvironmentTrackDefinitionsListModel"
)]
unsafe impl quest_hook::libil2cpp::Type
for crate::BeatSaber::TrackDefinitions::DataModels::EnvironmentTrackDefinitionsListModel {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "BeatSaber.TrackDefinitions.DataModels";
    const CLASS_NAME: &'static str = "EnvironmentTrackDefinitionsListModel";
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
    feature = "BeatSaber+TrackDefinitions+DataModels+EnvironmentTrackDefinitionsListModel"
)]
impl std::ops::Deref
for crate::BeatSaber::TrackDefinitions::DataModels::EnvironmentTrackDefinitionsListModel {
    type Target = quest_hook::libil2cpp::Il2CppObject;
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
    pub fn New(
        environmentTrackDefinitions: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<
                quest_hook::libil2cpp::Gc<
                    crate::BeatSaber::TrackDefinitions::EnvironmentTracksDefinitionSO,
                >,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (environmentTrackDefinitions))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        environmentTrackDefinitions: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<
                quest_hook::libil2cpp::Gc<
                    crate::BeatSaber::TrackDefinitions::EnvironmentTracksDefinitionSO,
                >,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (environmentTrackDefinitions))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Item(
        &mut self,
        _cordl_type: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::EnvironmentInfoSO>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::BeatSaber::TrackDefinitions::DataModels::EnvironmentTrackDefinitionModel,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::BeatSaber::TrackDefinitions::DataModels::EnvironmentTrackDefinitionModel,
        > = __cordl_object.invoke("get_Item", (_cordl_type))?;
        Ok(__cordl_ret.into())
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
