#[cfg(feature = "DirectionalLightWithIdsGroupEntry")]
#[repr(C)]
#[derive(Debug)]
pub struct DirectionalLightWithIdsGroupEntry {
    __cordl_parent: crate::UnityEngine::MonoBehaviour,
    pub _directionalLightWithIds: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::DirectionalLightWithIds,
    >,
    pub _groupLightData: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::List_1<
            *mut crate::GlobalNamespace::DirectionalLightWithIdsGroupEntry_GroupLightData,
        >,
    >,
    pub _individualLightData: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::List_1<
            *mut crate::GlobalNamespace::DirectionalLightWithIdsGroupEntry_LightIntensitiesWithId,
        >,
    >,
    pub _excludedLightIds: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<i32>,
    >,
    pub _groupLightsWeighting: crate::GlobalNamespace::ColorMixAndWeightingApproach,
}
#[cfg(feature = "DirectionalLightWithIdsGroupEntry")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::DirectionalLightWithIdsGroupEntry => ""
    ."DirectionalLightWithIdsGroupEntry"
);
#[cfg(feature = "DirectionalLightWithIdsGroupEntry")]
impl std::ops::Deref for crate::GlobalNamespace::DirectionalLightWithIdsGroupEntry {
    type Target = crate::UnityEngine::MonoBehaviour;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "DirectionalLightWithIdsGroupEntry")]
impl std::ops::DerefMut for crate::GlobalNamespace::DirectionalLightWithIdsGroupEntry {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "DirectionalLightWithIdsGroupEntry")]
impl crate::GlobalNamespace::DirectionalLightWithIdsGroupEntry {
    #[cfg(feature = "DirectionalLightWithIdsGroupEntry+GroupLightData")]
    pub type GroupLightData = crate::GlobalNamespace::DirectionalLightWithIdsGroupEntry_GroupLightData;
    #[cfg(feature = "DirectionalLightWithIdsGroupEntry+LightIntensitiesWithId")]
    pub type LightIntensitiesWithId = crate::GlobalNamespace::DirectionalLightWithIdsGroupEntry_LightIntensitiesWithId;
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
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_groupLightData(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                *mut crate::GlobalNamespace::DirectionalLightWithIdsGroupEntry_GroupLightData,
            >,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                *mut crate::GlobalNamespace::DirectionalLightWithIdsGroupEntry_GroupLightData,
            >,
        > = __cordl_object.invoke("get_groupLightData", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_groupLightsWeighting(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::GlobalNamespace::ColorMixAndWeightingApproach,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::GlobalNamespace::ColorMixAndWeightingApproach = __cordl_object
            .invoke("get_groupLightsWeighting", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_individualLightData(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                *mut crate::GlobalNamespace::DirectionalLightWithIdsGroupEntry_LightIntensitiesWithId,
            >,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                *mut crate::GlobalNamespace::DirectionalLightWithIdsGroupEntry_LightIntensitiesWithId,
            >,
        > = __cordl_object.invoke("get_individualLightData", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "DirectionalLightWithIdsGroupEntry")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::DirectionalLightWithIdsGroupEntry {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "DirectionalLightWithIdsGroupEntry+GroupLightData")]
#[repr(C)]
#[derive(Debug)]
pub struct DirectionalLightWithIdsGroupEntry_GroupLightData {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _lightGroup: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::LightGroupSO>,
    pub _groupIntensity: f32,
}
#[cfg(feature = "DirectionalLightWithIdsGroupEntry+GroupLightData")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::DirectionalLightWithIdsGroupEntry_GroupLightData => ""
    ."DirectionalLightWithIdsGroupEntry/GroupLightData"
);
#[cfg(feature = "DirectionalLightWithIdsGroupEntry+GroupLightData")]
impl std::ops::Deref
for crate::GlobalNamespace::DirectionalLightWithIdsGroupEntry_GroupLightData {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "DirectionalLightWithIdsGroupEntry+GroupLightData")]
impl std::ops::DerefMut
for crate::GlobalNamespace::DirectionalLightWithIdsGroupEntry_GroupLightData {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "DirectionalLightWithIdsGroupEntry+GroupLightData")]
impl crate::GlobalNamespace::DirectionalLightWithIdsGroupEntry_GroupLightData {
    pub fn New(
        lightGroup: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::LightGroupSO>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (lightGroup))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        lightGroup: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::LightGroupSO>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (lightGroup))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_groupIntensity(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("get_groupIntensity", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_lightGroup(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::LightGroupSO>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::LightGroupSO,
        > = __cordl_object.invoke("get_lightGroup", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "DirectionalLightWithIdsGroupEntry+GroupLightData")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::DirectionalLightWithIdsGroupEntry_GroupLightData {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "DirectionalLightWithIdsGroupEntry+LightIntensitiesWithId")]
#[repr(C)]
#[derive(Debug)]
pub struct DirectionalLightWithIdsGroupEntry_LightIntensitiesWithId {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _lightId: i32,
    pub _intensity: f32,
}
#[cfg(feature = "DirectionalLightWithIdsGroupEntry+LightIntensitiesWithId")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::DirectionalLightWithIdsGroupEntry_LightIntensitiesWithId => ""
    ."DirectionalLightWithIdsGroupEntry/LightIntensitiesWithId"
);
#[cfg(feature = "DirectionalLightWithIdsGroupEntry+LightIntensitiesWithId")]
impl std::ops::Deref
for crate::GlobalNamespace::DirectionalLightWithIdsGroupEntry_LightIntensitiesWithId {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "DirectionalLightWithIdsGroupEntry+LightIntensitiesWithId")]
impl std::ops::DerefMut
for crate::GlobalNamespace::DirectionalLightWithIdsGroupEntry_LightIntensitiesWithId {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "DirectionalLightWithIdsGroupEntry+LightIntensitiesWithId")]
impl crate::GlobalNamespace::DirectionalLightWithIdsGroupEntry_LightIntensitiesWithId {
    pub fn New(
        lightId: i32,
        lightIntensity: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (lightId, lightIntensity))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        lightId: i32,
        lightIntensity: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (lightId, lightIntensity))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_intensity(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("get_intensity", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_lightId(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_lightId", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "DirectionalLightWithIdsGroupEntry+LightIntensitiesWithId")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::DirectionalLightWithIdsGroupEntry_LightIntensitiesWithId {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
