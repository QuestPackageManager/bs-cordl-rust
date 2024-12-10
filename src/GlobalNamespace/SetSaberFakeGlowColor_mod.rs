#[cfg(feature = "SetSaberFakeGlowColor")]
#[repr(C)]
#[derive(Debug)]
pub struct SetSaberFakeGlowColor {
    __cordl_parent: crate::UnityEngine::MonoBehaviour,
    pub _tintColor: crate::UnityEngine::Color,
    pub _saberTypeObject: *mut crate::GlobalNamespace::SaberTypeObject,
    pub _parametric3SliceSprite: *mut crate::GlobalNamespace::Parametric3SliceSpriteController,
    pub _colorManager: *mut crate::GlobalNamespace::ColorManager,
    pub _saberType: crate::GlobalNamespace::SaberType,
}
#[cfg(feature = "SetSaberFakeGlowColor")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::SetSaberFakeGlowColor => ""
    ."SetSaberFakeGlowColor"
);
#[cfg(feature = "SetSaberFakeGlowColor")]
impl std::ops::Deref for crate::GlobalNamespace::SetSaberFakeGlowColor {
    type Target = crate::UnityEngine::MonoBehaviour;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "SetSaberFakeGlowColor")]
impl std::ops::DerefMut for crate::GlobalNamespace::SetSaberFakeGlowColor {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "SetSaberFakeGlowColor")]
impl crate::GlobalNamespace::SetSaberFakeGlowColor {
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
#[cfg(feature = "SetSaberFakeGlowColor")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::SetSaberFakeGlowColor {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
