#[cfg(feature = "SpectrogramMultiplierFloatFxEffectTarget")]
#[repr(C)]
#[derive(Debug)]
pub struct SpectrogramMultiplierFloatFxEffectTarget {
    __cordl_parent: crate::GlobalNamespace::FloatFxGroupEffectTarget,
    pub _spectrogram: *mut crate::GlobalNamespace::SpectrogramRowPropertyAnimator,
}
#[cfg(feature = "SpectrogramMultiplierFloatFxEffectTarget")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::SpectrogramMultiplierFloatFxEffectTarget => ""
    ."SpectrogramMultiplierFloatFxEffectTarget"
);
#[cfg(feature = "SpectrogramMultiplierFloatFxEffectTarget")]
impl std::ops::Deref
for crate::GlobalNamespace::SpectrogramMultiplierFloatFxEffectTarget {
    type Target = crate::GlobalNamespace::FloatFxGroupEffectTarget;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "SpectrogramMultiplierFloatFxEffectTarget")]
impl std::ops::DerefMut
for crate::GlobalNamespace::SpectrogramMultiplierFloatFxEffectTarget {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "SpectrogramMultiplierFloatFxEffectTarget")]
impl crate::GlobalNamespace::SpectrogramMultiplierFloatFxEffectTarget {
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn SetValue(
        &mut self,
        groupId: i32,
        elementId: i32,
        value: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetValue", (groupId, elementId, value))?;
        Ok(__cordl_ret.into())
    }
    pub fn TriggerValue(
        &mut self,
        groupId: i32,
        elementId: i32,
        value: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("TriggerValue", (groupId, elementId, value))?;
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
#[cfg(feature = "SpectrogramMultiplierFloatFxEffectTarget")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::SpectrogramMultiplierFloatFxEffectTarget {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
