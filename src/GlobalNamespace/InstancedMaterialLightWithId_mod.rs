#[cfg(feature = "InstancedMaterialLightWithId")]
#[repr(C)]
#[derive(Debug)]
pub struct InstancedMaterialLightWithId {
    __cordl_parent: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::LightWithIdMonoBehaviour,
    >,
    pub _materialPropertyBlockColorSetter: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::MaterialPropertyBlockColorSetter,
    >,
    pub _setColorOnly: bool,
    pub _intensity: f32,
    pub _minAlpha: f32,
    pub _saturateIntensity: bool,
    pub _hdr: bool,
    pub _originalColor: crate::UnityEngine::Color,
    pub _color: crate::UnityEngine::Color,
    pub _startColorWasSet: bool,
}
#[cfg(feature = "InstancedMaterialLightWithId")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::InstancedMaterialLightWithId =>
    ""."InstancedMaterialLightWithId"
);
#[cfg(feature = "InstancedMaterialLightWithId")]
impl std::ops::Deref for crate::GlobalNamespace::InstancedMaterialLightWithId {
    type Target = quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::LightWithIdMonoBehaviour,
    >;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "InstancedMaterialLightWithId")]
impl std::ops::DerefMut for crate::GlobalNamespace::InstancedMaterialLightWithId {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "InstancedMaterialLightWithId")]
impl crate::GlobalNamespace::InstancedMaterialLightWithId {
    pub fn AddNecessaryComponents(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AddNecessaryComponents", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn ColorWasSet(
        &mut self,
        newColor: crate::UnityEngine::Color,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ColorWasSet", (newColor))?;
        Ok(__cordl_ret.into())
    }
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
#[cfg(feature = "InstancedMaterialLightWithId")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::InstancedMaterialLightWithId {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
