#[cfg(feature = "SetSaberGlowColor")]
#[repr(C)]
#[derive(Debug)]
pub struct SetSaberGlowColor {
    __cordl_parent: crate::UnityEngine::MonoBehaviour,
    pub _saberTypeObject: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::SaberTypeObject,
    >,
    pub _meshRenderer: quest_hook::libil2cpp::Gc<crate::UnityEngine::MeshRenderer>,
    pub _propertyTintColorPairs: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<
            quest_hook::libil2cpp::Gc<
                crate::GlobalNamespace::SetSaberGlowColor_PropertyTintColorPair,
            >,
        >,
    >,
    pub _colorManager: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::ColorManager>,
    pub _materialPropertyBlock: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::MaterialPropertyBlock,
    >,
    pub _saberType: crate::GlobalNamespace::SaberType,
}
#[cfg(feature = "SetSaberGlowColor")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::SetSaberGlowColor => ""
    ."SetSaberGlowColor"
);
#[cfg(feature = "SetSaberGlowColor")]
impl std::ops::Deref for crate::GlobalNamespace::SetSaberGlowColor {
    type Target = crate::UnityEngine::MonoBehaviour;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "SetSaberGlowColor")]
impl std::ops::DerefMut for crate::GlobalNamespace::SetSaberGlowColor {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "SetSaberGlowColor")]
impl crate::GlobalNamespace::SetSaberGlowColor {
    #[cfg(feature = "SetSaberGlowColor+PropertyTintColorPair")]
    pub type PropertyTintColorPair = crate::GlobalNamespace::SetSaberGlowColor_PropertyTintColorPair;
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn SetColors(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetColors", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn Start(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Start", ())?;
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
    pub fn set_saberType(
        &mut self,
        value: crate::GlobalNamespace::SaberType,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_saberType", (value))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "SetSaberGlowColor")]
impl quest_hook::libil2cpp::ObjectType for crate::GlobalNamespace::SetSaberGlowColor {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "SetSaberGlowColor+PropertyTintColorPair")]
#[repr(C)]
#[derive(Debug)]
pub struct SetSaberGlowColor_PropertyTintColorPair {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub tintColor: crate::UnityEngine::Color,
    pub property: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
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
    type Target = quest_hook::libil2cpp::Il2CppObject;
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
