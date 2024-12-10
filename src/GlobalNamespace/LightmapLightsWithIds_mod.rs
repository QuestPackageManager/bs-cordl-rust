#[cfg(feature = "LightmapLightsWithIds")]
#[repr(C)]
#[derive(Debug)]
pub struct LightmapLightsWithIds {
    __cordl_parent: crate::GlobalNamespace::LightWithIds,
    pub _maxTotalIntensity: f32,
    pub _lightIntensityData: *mut quest_hook::libil2cpp::Il2CppArray<
        *mut crate::GlobalNamespace::LightmapLightsWithIds_LightIntensitiesWithId,
    >,
}
#[cfg(feature = "LightmapLightsWithIds")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::LightmapLightsWithIds => ""
    ."LightmapLightsWithIds"
);
#[cfg(feature = "LightmapLightsWithIds")]
impl std::ops::Deref for crate::GlobalNamespace::LightmapLightsWithIds {
    type Target = crate::GlobalNamespace::LightWithIds;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "LightmapLightsWithIds")]
impl std::ops::DerefMut for crate::GlobalNamespace::LightmapLightsWithIds {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "LightmapLightsWithIds")]
impl crate::GlobalNamespace::LightmapLightsWithIds {
    #[cfg(feature = "LightmapLightsWithIds+LightIntensitiesWithId")]
    pub type LightIntensitiesWithId = crate::GlobalNamespace::LightmapLightsWithIds_LightIntensitiesWithId;
    pub fn GetLightWithIds(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<
                *mut crate::GlobalNamespace::LightWithIds_LightWithId,
            >,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<
                *mut crate::GlobalNamespace::LightWithIds_LightWithId,
            >,
        > = __cordl_object.invoke("GetLightWithIds", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn ProcessNewColorData(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ProcessNewColorData", ())?;
        Ok(__cordl_ret.into())
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
    pub fn get_maxTotalIntensity(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("get_maxTotalIntensity", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn set_maxTotalIntensity(
        &mut self,
        value: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_maxTotalIntensity", (value))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "LightmapLightsWithIds")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::LightmapLightsWithIds {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "LightmapLightsWithIds+LightIntensitiesWithId")]
#[repr(C)]
#[derive(Debug)]
pub struct LightmapLightsWithIds_LightIntensitiesWithId {
    __cordl_parent: crate::GlobalNamespace::LightWithIds_LightWithId,
    pub _bakeId: crate::GlobalNamespace::LightConstants_BakeId,
    pub _intensity: f32,
    pub _weight: f32,
    pub _lightmapLightIdColorPropertyId: i32,
    pub _lightProbeLightIdColorPropertyId: i32,
    pub _initializedPropertyIds: bool,
}
#[cfg(feature = "LightmapLightsWithIds+LightIntensitiesWithId")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::LightmapLightsWithIds_LightIntensitiesWithId => ""
    ."LightmapLightsWithIds/LightIntensitiesWithId"
);
#[cfg(feature = "LightmapLightsWithIds+LightIntensitiesWithId")]
impl std::ops::Deref
for crate::GlobalNamespace::LightmapLightsWithIds_LightIntensitiesWithId {
    type Target = crate::GlobalNamespace::LightWithIds_LightWithId;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "LightmapLightsWithIds+LightIntensitiesWithId")]
impl std::ops::DerefMut
for crate::GlobalNamespace::LightmapLightsWithIds_LightIntensitiesWithId {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "LightmapLightsWithIds+LightIntensitiesWithId")]
impl crate::GlobalNamespace::LightmapLightsWithIds_LightIntensitiesWithId {
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn SetDataToShaders(
        &mut self,
        lightmapColor: crate::UnityEngine::Color,
        probeColor: crate::UnityEngine::Color,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetDataToShaders", (lightmapColor, probeColor))?;
        Ok(__cordl_ret.into())
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
    pub fn get_bakeId(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::LightConstants_BakeId> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::GlobalNamespace::LightConstants_BakeId = __cordl_object
            .invoke("get_bakeId", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_intensity(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("get_intensity", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_weight(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("get_weight", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn set_intensity(
        &mut self,
        value: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_intensity", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_weight(
        &mut self,
        value: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_weight", (value))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "LightmapLightsWithIds+LightIntensitiesWithId")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::LightmapLightsWithIds_LightIntensitiesWithId {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
