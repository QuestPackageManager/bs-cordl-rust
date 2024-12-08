#[cfg(feature = "BloomPrePassBackgroundColorsGradientTintColorWithLightId")]
#[repr(C)]
#[derive(Debug)]
pub struct BloomPrePassBackgroundColorsGradientTintColorWithLightId {
    __cordl_parent: RuntimeLightWithIds,
    pub _bloomPrePassBackgroundColorsGradient: *mut BloomPrePassBackgroundColorsGradient,
    pub _useGrayscale: bool,
    pub grayscaleFactor: f32,
}
#[cfg(feature = "BloomPrePassBackgroundColorsGradientTintColorWithLightId")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for BloomPrePassBackgroundColorsGradientTintColorWithLightId
    => ""."BloomPrePassBackgroundColorsGradientTintColorWithLightId"
);
#[cfg(feature = "BloomPrePassBackgroundColorsGradientTintColorWithLightId")]
impl std::ops::Deref for BloomPrePassBackgroundColorsGradientTintColorWithLightId {
    type Target = RuntimeLightWithIds;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BloomPrePassBackgroundColorsGradientTintColorWithLightId")]
impl std::ops::DerefMut for BloomPrePassBackgroundColorsGradientTintColorWithLightId {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BloomPrePassBackgroundColorsGradientTintColorWithLightId")]
impl BloomPrePassBackgroundColorsGradientTintColorWithLightId {
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
    pub fn New() -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "BloomPrePassBackgroundColorsGradientTintColorWithLightId")]
impl quest_hook::libil2cpp::ObjectType
for BloomPrePassBackgroundColorsGradientTintColorWithLightId {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
