#[cfg(feature = "SetSaberBladeParams+PropertyTintColorPair")]
#[repr(C)]
#[derive(Debug)]
pub struct SetSaberBladeParams_PropertyTintColorPair {
    __cordl_parent: crate::System::Object,
    pub tintColor: crate::UnityEngine::Color,
    pub property: *mut crate::System::String,
}
#[cfg(feature = "SetSaberBladeParams+PropertyTintColorPair")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::SetSaberBladeParams_PropertyTintColorPair => ""
    ."SetSaberBladeParams/PropertyTintColorPair"
);
#[cfg(feature = "SetSaberBladeParams+PropertyTintColorPair")]
impl std::ops::Deref
for crate::GlobalNamespace::SetSaberBladeParams_PropertyTintColorPair {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "SetSaberBladeParams+PropertyTintColorPair")]
impl std::ops::DerefMut
for crate::GlobalNamespace::SetSaberBladeParams_PropertyTintColorPair {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "SetSaberBladeParams+PropertyTintColorPair")]
impl crate::GlobalNamespace::SetSaberBladeParams_PropertyTintColorPair {
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
}
#[cfg(feature = "SetSaberBladeParams+PropertyTintColorPair")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::SetSaberBladeParams_PropertyTintColorPair {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "SetSaberBladeParams")]
#[repr(C)]
#[derive(Debug)]
pub struct SetSaberBladeParams {
    __cordl_parent: crate::UnityEngine::MonoBehaviour,
    pub _saber: *mut crate::GlobalNamespace::SaberTypeObject,
    pub _meshRenderer: *mut crate::UnityEngine::MeshRenderer,
    pub _propertyTintColorPairs: *mut quest_hook::libil2cpp::Il2CppArray<
        *mut crate::GlobalNamespace::SetSaberBladeParams_PropertyTintColorPair,
    >,
    pub _colorManager: *mut crate::GlobalNamespace::ColorManager,
}
#[cfg(feature = "SetSaberBladeParams")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::SetSaberBladeParams => ""
    ."SetSaberBladeParams"
);
#[cfg(feature = "SetSaberBladeParams")]
impl std::ops::Deref for crate::GlobalNamespace::SetSaberBladeParams {
    type Target = crate::UnityEngine::MonoBehaviour;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "SetSaberBladeParams")]
impl std::ops::DerefMut for crate::GlobalNamespace::SetSaberBladeParams {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "SetSaberBladeParams")]
impl crate::GlobalNamespace::SetSaberBladeParams {
    #[cfg(feature = "SetSaberBladeParams+PropertyTintColorPair")]
    pub type PropertyTintColorPair = crate::GlobalNamespace::SetSaberBladeParams_PropertyTintColorPair;
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
    pub fn Start(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Start", ())?;
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
#[cfg(feature = "SetSaberBladeParams")]
impl quest_hook::libil2cpp::ObjectType for crate::GlobalNamespace::SetSaberBladeParams {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
