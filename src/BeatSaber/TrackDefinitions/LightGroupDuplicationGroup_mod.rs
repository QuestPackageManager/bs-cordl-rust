#[cfg(feature = "BeatSaber+TrackDefinitions+LightGroupDuplicationGroup")]
#[repr(C)]
#[derive(Debug)]
pub struct LightGroupDuplicationGroup {
    __cordl_parent: crate::GlobalNamespace::PersistentScriptableObject,
    pub _duplicationGroupName: *mut crate::System::String,
    pub _lightGroups: *mut quest_hook::libil2cpp::Il2CppArray<
        *mut crate::GlobalNamespace::LightGroupSO,
    >,
}
#[cfg(feature = "BeatSaber+TrackDefinitions+LightGroupDuplicationGroup")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::BeatSaber::TrackDefinitions::LightGroupDuplicationGroup =>
    "BeatSaber.TrackDefinitions"."LightGroupDuplicationGroup"
);
#[cfg(feature = "BeatSaber+TrackDefinitions+LightGroupDuplicationGroup")]
impl std::ops::Deref for crate::BeatSaber::TrackDefinitions::LightGroupDuplicationGroup {
    type Target = crate::GlobalNamespace::PersistentScriptableObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BeatSaber+TrackDefinitions+LightGroupDuplicationGroup")]
impl std::ops::DerefMut
for crate::BeatSaber::TrackDefinitions::LightGroupDuplicationGroup {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BeatSaber+TrackDefinitions+LightGroupDuplicationGroup")]
impl crate::BeatSaber::TrackDefinitions::LightGroupDuplicationGroup {
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
    pub fn get_duplicationGroupName(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("get_duplicationGroupName", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_lightGroups(
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
        > = __cordl_object.invoke("get_lightGroups", ())?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "BeatSaber+TrackDefinitions+LightGroupDuplicationGroup")]
impl quest_hook::libil2cpp::ObjectType
for crate::BeatSaber::TrackDefinitions::LightGroupDuplicationGroup {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
