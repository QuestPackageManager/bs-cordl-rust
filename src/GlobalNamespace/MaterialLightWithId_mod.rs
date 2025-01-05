#[cfg(feature = "MaterialLightWithId")]
#[repr(C)]
#[derive(Debug)]
pub struct MaterialLightWithId {
    __cordl_parent: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::LightWithIdMonoBehaviour,
    >,
    pub _meshRenderer: quest_hook::libil2cpp::Gc<crate::UnityEngine::MeshRenderer>,
    pub _setAlphaOnly: bool,
    pub _alphaIntoColor: bool,
    pub _setColorOnly: bool,
    pub _colorProperty: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub _alphaIntensity: f32,
    pub _multiplyColorWithAlpha: bool,
    pub _multiplyColor: bool,
    pub _colorMultiplier: f32,
    pub _color: crate::UnityEngine::Color,
    pub _alpha: f32,
    pub _propertyId: i32,
}
#[cfg(feature = "MaterialLightWithId")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::MaterialLightWithId => ""
    ."MaterialLightWithId"
);
#[cfg(feature = "MaterialLightWithId")]
impl std::ops::Deref for crate::GlobalNamespace::MaterialLightWithId {
    type Target = quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::LightWithIdMonoBehaviour,
    >;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "MaterialLightWithId")]
impl std::ops::DerefMut for crate::GlobalNamespace::MaterialLightWithId {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "MaterialLightWithId")]
impl crate::GlobalNamespace::MaterialLightWithId {
    pub fn Awake(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Awake", ())?;
        Ok(__cordl_ret.into())
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
    pub fn get_color(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Color> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Color = __cordl_object
            .invoke("get_color", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "MaterialLightWithId")]
impl quest_hook::libil2cpp::ObjectType for crate::GlobalNamespace::MaterialLightWithId {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
