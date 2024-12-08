#[cfg(feature = "RuntimeLightWithLightGroupIds+LightIntensitiesWithId")]
#[repr(C)]
#[derive(Debug)]
pub struct RuntimeLightWithLightGroupIds_LightIntensitiesWithId {
    __cordl_parent: crate::GlobalNamespace::LightWithIds_LightWithId,
    pub _intensity: f32,
}
#[cfg(feature = "RuntimeLightWithLightGroupIds+LightIntensitiesWithId")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::RuntimeLightWithLightGroupIds_LightIntensitiesWithId => ""
    ."RuntimeLightWithLightGroupIds/LightIntensitiesWithId"
);
#[cfg(feature = "RuntimeLightWithLightGroupIds+LightIntensitiesWithId")]
impl std::ops::Deref
for crate::GlobalNamespace::RuntimeLightWithLightGroupIds_LightIntensitiesWithId {
    type Target = crate::GlobalNamespace::LightWithIds_LightWithId;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "RuntimeLightWithLightGroupIds+LightIntensitiesWithId")]
impl std::ops::DerefMut
for crate::GlobalNamespace::RuntimeLightWithLightGroupIds_LightIntensitiesWithId {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "RuntimeLightWithLightGroupIds+LightIntensitiesWithId")]
impl crate::GlobalNamespace::RuntimeLightWithLightGroupIds_LightIntensitiesWithId {
    pub fn New(
        lightId: i32,
        intensity: f32,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (lightId, intensity))?;
        Ok(__cordl_object)
    }
    pub fn _ctor(
        &mut self,
        lightId: i32,
        intensity: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (lightId, intensity))?;
        Ok(__cordl_ret)
    }
    pub fn get_intensity(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("get_intensity", ())?;
        Ok(__cordl_ret)
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
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "RuntimeLightWithLightGroupIds+LightIntensitiesWithId")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::RuntimeLightWithLightGroupIds_LightIntensitiesWithId {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "RuntimeLightWithLightGroupIds")]
#[repr(C)]
#[derive(Debug)]
pub struct RuntimeLightWithLightGroupIds {
    __cordl_parent: crate::GlobalNamespace::LightWithIds,
    pub _lightGroupList: *mut quest_hook::libil2cpp::Il2CppArray<
        *mut crate::GlobalNamespace::LightGroup,
    >,
    pub _intensity: f32,
    pub _maxIntensity: f32,
    pub _multiplyColorByAlpha: bool,
    pub _lightIntensityData: *mut quest_hook::libil2cpp::Il2CppArray<
        *mut crate::GlobalNamespace::RuntimeLightWithLightGroupIds_LightIntensitiesWithId,
    >,
}
#[cfg(feature = "RuntimeLightWithLightGroupIds")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::RuntimeLightWithLightGroupIds
    => ""."RuntimeLightWithLightGroupIds"
);
#[cfg(feature = "RuntimeLightWithLightGroupIds")]
impl std::ops::Deref for crate::GlobalNamespace::RuntimeLightWithLightGroupIds {
    type Target = crate::GlobalNamespace::LightWithIds;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "RuntimeLightWithLightGroupIds")]
impl std::ops::DerefMut for crate::GlobalNamespace::RuntimeLightWithLightGroupIds {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "RuntimeLightWithLightGroupIds")]
impl crate::GlobalNamespace::RuntimeLightWithLightGroupIds {
    #[cfg(feature = "RuntimeLightWithLightGroupIds+LightIntensitiesWithId")]
    pub type LightIntensitiesWithId = crate::GlobalNamespace::RuntimeLightWithLightGroupIds_LightIntensitiesWithId;
    pub fn Awake(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Awake", ())?;
        Ok(__cordl_ret)
    }
    pub fn ColorWasSet(
        &mut self,
        color: crate::UnityEngine::Color,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ColorWasSet", (color))?;
        Ok(__cordl_ret)
    }
    pub fn GetLightWithIds(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Collections::Generic::IEnumerable_1<
            *mut crate::GlobalNamespace::LightWithIds_LightWithId,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Collections::Generic::IEnumerable_1<
            *mut crate::GlobalNamespace::LightWithIds_LightWithId,
        > = __cordl_object.invoke("GetLightWithIds", ())?;
        Ok(__cordl_ret)
    }
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
    pub fn ProcessColor(
        &mut self,
        color: crate::UnityEngine::Color,
        intensity: f32,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Color> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Color = __cordl_object
            .invoke("ProcessColor", (color, intensity))?;
        Ok(__cordl_ret)
    }
    pub fn ProcessNewColorData(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ProcessNewColorData", ())?;
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
}
#[cfg(feature = "RuntimeLightWithLightGroupIds")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::RuntimeLightWithLightGroupIds {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
