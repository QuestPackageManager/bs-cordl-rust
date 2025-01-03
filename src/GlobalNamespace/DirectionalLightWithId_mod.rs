#[cfg(feature = "DirectionalLightWithId")]
#[repr(C)]
#[derive(Debug)]
pub struct DirectionalLightWithId {
    __cordl_parent: crate::GlobalNamespace::LightWithIdMonoBehaviour,
    pub _light: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::DirectionalLight>,
    pub _intensity: f32,
    pub _minIntensity: f32,
}
#[cfg(feature = "DirectionalLightWithId")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::DirectionalLightWithId => ""
    ."DirectionalLightWithId"
);
#[cfg(feature = "DirectionalLightWithId")]
impl std::ops::Deref for crate::GlobalNamespace::DirectionalLightWithId {
    type Target = crate::GlobalNamespace::LightWithIdMonoBehaviour;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "DirectionalLightWithId")]
impl std::ops::DerefMut for crate::GlobalNamespace::DirectionalLightWithId {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "DirectionalLightWithId")]
impl crate::GlobalNamespace::DirectionalLightWithId {
    pub fn ColorWasSet(
        &mut self,
        color: crate::UnityEngine::Color,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ColorWasSet", (color))?;
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
}
#[cfg(feature = "DirectionalLightWithId")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::DirectionalLightWithId {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
