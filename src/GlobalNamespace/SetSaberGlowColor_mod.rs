#[cfg(feature = "SetSaberGlowColor+PropertyTintColorPair")]
#[repr(C)]
#[derive(Debug)]
pub struct SetSaberGlowColor_PropertyTintColorPair {
    __cordl_parent: crate::System::Object,
    pub tintColor: crate::UnityEngine::Color,
    pub property: *mut crate::System::String,
}
#[cfg(feature = "SetSaberGlowColor+PropertyTintColorPair")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::SetSaberGlowColor_PropertyTintColorPair => ""
    ."SetSaberGlowColor/PropertyTintColorPair"
);
#[cfg(feature = "SetSaberGlowColor+PropertyTintColorPair")]
impl std::ops::Deref
for crate::GlobalNamespace::SetSaberGlowColor_PropertyTintColorPair {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "SetSaberGlowColor+PropertyTintColorPair")]
impl std::ops::DerefMut
for crate::GlobalNamespace::SetSaberGlowColor_PropertyTintColorPair {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "SetSaberGlowColor+PropertyTintColorPair")]
impl crate::GlobalNamespace::SetSaberGlowColor_PropertyTintColorPair {
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
#[cfg(feature = "SetSaberGlowColor+PropertyTintColorPair")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::SetSaberGlowColor_PropertyTintColorPair {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "SetSaberGlowColor")]
#[repr(C)]
#[derive(Debug)]
pub struct SetSaberGlowColor {
    __cordl_parent: crate::UnityEngine::MonoBehaviour,
    pub _saberTypeObject: *mut SaberTypeObject,
    pub _meshRenderer: *mut crate::UnityEngine::MeshRenderer,
    pub _propertyTintColorPairs: *mut quest_hook::libil2cpp::Il2CppArray<
        *mut crate::GlobalNamespace::SetSaberGlowColor_PropertyTintColorPair,
    >,
    pub _colorManager: *mut ColorManager,
    pub _materialPropertyBlock: *mut crate::UnityEngine::MaterialPropertyBlock,
    pub _saberType: SaberType,
}
#[cfg(feature = "SetSaberGlowColor")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for SetSaberGlowColor => ""."SetSaberGlowColor"
);
#[cfg(feature = "SetSaberGlowColor")]
impl std::ops::Deref for SetSaberGlowColor {
    type Target = crate::UnityEngine::MonoBehaviour;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "SetSaberGlowColor")]
impl std::ops::DerefMut for SetSaberGlowColor {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "SetSaberGlowColor")]
impl SetSaberGlowColor {
    #[cfg(feature = "SetSaberGlowColor+PropertyTintColorPair")]
    pub type PropertyTintColorPair = crate::GlobalNamespace::SetSaberGlowColor_PropertyTintColorPair;
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
    pub fn SetColors(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetColors", ())?;
        Ok(__cordl_ret)
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
    pub fn set_saberType(
        &mut self,
        value: SaberType,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_saberType", (value))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "SetSaberGlowColor")]
impl quest_hook::libil2cpp::ObjectType for SetSaberGlowColor {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
