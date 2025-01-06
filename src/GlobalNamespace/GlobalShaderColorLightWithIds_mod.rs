#[cfg(feature = "GlobalShaderColorLightWithIds")]
#[repr(C)]
#[derive(Debug)]
pub struct GlobalShaderColorLightWithIds {
    __cordl_parent: crate::GlobalNamespace::LightWithIds,
    pub _lightIntensityData: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<
            quest_hook::libil2cpp::Gc<
                crate::GlobalNamespace::GlobalShaderColorLightWithIds_LightIntensitiesWithId,
            >,
        >,
    >,
    pub _overrideSaturation: bool,
    pub _saturation: f32,
}
#[cfg(feature = "GlobalShaderColorLightWithIds")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::GlobalShaderColorLightWithIds
    => ""."GlobalShaderColorLightWithIds"
);
#[cfg(feature = "GlobalShaderColorLightWithIds")]
impl std::ops::Deref for crate::GlobalNamespace::GlobalShaderColorLightWithIds {
    type Target = crate::GlobalNamespace::LightWithIds;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "GlobalShaderColorLightWithIds")]
impl std::ops::DerefMut for crate::GlobalNamespace::GlobalShaderColorLightWithIds {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "GlobalShaderColorLightWithIds")]
impl crate::GlobalNamespace::GlobalShaderColorLightWithIds {
    #[cfg(feature = "GlobalShaderColorLightWithIds+LightIntensitiesWithId")]
    pub type LightIntensitiesWithId = crate::GlobalNamespace::GlobalShaderColorLightWithIds_LightIntensitiesWithId;
    pub fn GetLightWithIds(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<
                quest_hook::libil2cpp::Gc<
                    crate::GlobalNamespace::LightWithIds_LightWithId,
                >,
            >,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<
                quest_hook::libil2cpp::Gc<
                    crate::GlobalNamespace::LightWithIds_LightWithId,
                >,
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
}
#[cfg(feature = "GlobalShaderColorLightWithIds")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::GlobalShaderColorLightWithIds {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "GlobalShaderColorLightWithIds+LightIntensitiesWithId")]
#[repr(C)]
#[derive(Debug)]
pub struct GlobalShaderColorLightWithIds_LightIntensitiesWithId {
    __cordl_parent: crate::GlobalNamespace::LightWithIds_LightWithId,
    pub _intensity: f32,
}
#[cfg(feature = "GlobalShaderColorLightWithIds+LightIntensitiesWithId")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::GlobalShaderColorLightWithIds_LightIntensitiesWithId => ""
    ."GlobalShaderColorLightWithIds/LightIntensitiesWithId"
);
#[cfg(feature = "GlobalShaderColorLightWithIds+LightIntensitiesWithId")]
impl std::ops::Deref
for crate::GlobalNamespace::GlobalShaderColorLightWithIds_LightIntensitiesWithId {
    type Target = crate::GlobalNamespace::LightWithIds_LightWithId;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "GlobalShaderColorLightWithIds+LightIntensitiesWithId")]
impl std::ops::DerefMut
for crate::GlobalNamespace::GlobalShaderColorLightWithIds_LightIntensitiesWithId {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "GlobalShaderColorLightWithIds+LightIntensitiesWithId")]
impl crate::GlobalNamespace::GlobalShaderColorLightWithIds_LightIntensitiesWithId {
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
}
#[cfg(feature = "GlobalShaderColorLightWithIds+LightIntensitiesWithId")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::GlobalShaderColorLightWithIds_LightIntensitiesWithId {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
