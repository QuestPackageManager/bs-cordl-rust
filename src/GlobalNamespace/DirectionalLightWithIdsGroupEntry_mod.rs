#[cfg(feature = "DirectionalLightWithIdsGroupEntry")]
#[repr(C)]
#[derive(Debug)]
pub struct DirectionalLightWithIdsGroupEntry {
    __cordl_parent: crate::UnityEngine::MonoBehaviour,
    pub _directionalLightWithIds: *mut DirectionalLightWithIds,
    pub _groupLightData: *mut crate::System::Collections::Generic::List_1<
        *mut crate::GlobalNamespace::DirectionalLightWithIdsGroupEntry_GroupLightData,
    >,
    pub _individualLightData: *mut crate::System::Collections::Generic::List_1<
        *mut crate::GlobalNamespace::DirectionalLightWithIdsGroupEntry_LightIntensitiesWithId,
    >,
    pub _excludedLightIds: *mut quest_hook::libil2cpp::Il2CppArray<i32>,
    pub _groupLightsWeighting: ColorMixAndWeightingApproach,
}
#[cfg(feature = "DirectionalLightWithIdsGroupEntry")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for DirectionalLightWithIdsGroupEntry => ""
    ."DirectionalLightWithIdsGroupEntry"
);
#[cfg(feature = "DirectionalLightWithIdsGroupEntry")]
impl std::ops::Deref for DirectionalLightWithIdsGroupEntry {
    type Target = crate::UnityEngine::MonoBehaviour;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "DirectionalLightWithIdsGroupEntry")]
impl std::ops::DerefMut for DirectionalLightWithIdsGroupEntry {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "DirectionalLightWithIdsGroupEntry")]
impl DirectionalLightWithIdsGroupEntry {
    #[cfg(feature = "DirectionalLightWithIdsGroupEntry+LightIntensitiesWithId")]
    pub type LightIntensitiesWithId = crate::GlobalNamespace::DirectionalLightWithIdsGroupEntry_LightIntensitiesWithId;
    #[cfg(feature = "DirectionalLightWithIdsGroupEntry+GroupLightData")]
    pub type GroupLightData = crate::GlobalNamespace::DirectionalLightWithIdsGroupEntry_GroupLightData;
    pub fn get_groupLightData(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Collections::Generic::List_1<
            *mut crate::GlobalNamespace::DirectionalLightWithIdsGroupEntry_GroupLightData,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Collections::Generic::List_1<
            *mut crate::GlobalNamespace::DirectionalLightWithIdsGroupEntry_GroupLightData,
        > = __cordl_object.invoke("get_groupLightData", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_individualLightData(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Collections::Generic::List_1<
            *mut crate::GlobalNamespace::DirectionalLightWithIdsGroupEntry_LightIntensitiesWithId,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Collections::Generic::List_1<
            *mut crate::GlobalNamespace::DirectionalLightWithIdsGroupEntry_LightIntensitiesWithId,
        > = __cordl_object.invoke("get_individualLightData", ())?;
        Ok(__cordl_ret)
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
    pub fn get_groupLightsWeighting(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<ColorMixAndWeightingApproach> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: ColorMixAndWeightingApproach = __cordl_object
            .invoke("get_groupLightsWeighting", ())?;
        Ok(__cordl_ret)
    }
    pub fn New() -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "DirectionalLightWithIdsGroupEntry")]
impl quest_hook::libil2cpp::ObjectType for DirectionalLightWithIdsGroupEntry {
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
    __cordl_parent: crate::System::Object,
    pub _lightGroup: *mut LightGroupSO,
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
    type Target = crate::System::Object;
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
    pub fn get_groupIntensity(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("get_groupIntensity", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_lightGroup(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut LightGroupSO> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut LightGroupSO = __cordl_object
            .invoke("get_lightGroup", ())?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
        lightGroup: *mut LightGroupSO,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (lightGroup))?;
        Ok(__cordl_ret)
    }
    pub fn New(
        lightGroup: *mut LightGroupSO,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (lightGroup))?;
        Ok(__cordl_object)
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
    __cordl_parent: crate::System::Object,
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
    type Target = crate::System::Object;
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
    pub fn get_lightId(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_lightId", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_intensity(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("get_intensity", ())?;
        Ok(__cordl_ret)
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
        Ok(__cordl_ret)
    }
    pub fn New(
        lightId: i32,
        lightIntensity: f32,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (lightId, lightIntensity))?;
        Ok(__cordl_object)
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
