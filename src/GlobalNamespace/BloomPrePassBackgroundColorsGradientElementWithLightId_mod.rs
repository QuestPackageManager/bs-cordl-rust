#[cfg(feature = "BloomPrePassBackgroundColorsGradientElementWithLightId")]
#[repr(C)]
#[derive(Debug)]
pub struct BloomPrePassBackgroundColorsGradientElementWithLightId {
    __cordl_parent: LightWithIdMonoBehaviour,
    pub _bloomPrePassBackgroundColorsGradient: *mut BloomPrePassBackgroundColorsGradient,
    pub _elements: *mut quest_hook::libil2cpp::Il2CppArray<
        *mut crate::GlobalNamespace::BloomPrePassBackgroundColorsGradientElementWithLightId_Elements,
    >,
}
#[cfg(feature = "BloomPrePassBackgroundColorsGradientElementWithLightId")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for BloomPrePassBackgroundColorsGradientElementWithLightId
    => ""."BloomPrePassBackgroundColorsGradientElementWithLightId"
);
#[cfg(feature = "BloomPrePassBackgroundColorsGradientElementWithLightId")]
impl std::ops::Deref for BloomPrePassBackgroundColorsGradientElementWithLightId {
    type Target = LightWithIdMonoBehaviour;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BloomPrePassBackgroundColorsGradientElementWithLightId")]
impl std::ops::DerefMut for BloomPrePassBackgroundColorsGradientElementWithLightId {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BloomPrePassBackgroundColorsGradientElementWithLightId")]
impl BloomPrePassBackgroundColorsGradientElementWithLightId {
    #[cfg(feature = "BloomPrePassBackgroundColorsGradientElementWithLightId+Elements")]
    pub type Elements = crate::GlobalNamespace::BloomPrePassBackgroundColorsGradientElementWithLightId_Elements;
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
#[cfg(feature = "BloomPrePassBackgroundColorsGradientElementWithLightId")]
impl quest_hook::libil2cpp::ObjectType
for BloomPrePassBackgroundColorsGradientElementWithLightId {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "BloomPrePassBackgroundColorsGradientElementWithLightId+Elements")]
#[repr(C)]
#[derive(Debug)]
pub struct BloomPrePassBackgroundColorsGradientElementWithLightId_Elements {
    __cordl_parent: crate::System::Object,
    pub elementNumber: i32,
    pub intensity: f32,
    pub minIntensity: f32,
}
#[cfg(feature = "BloomPrePassBackgroundColorsGradientElementWithLightId+Elements")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::BloomPrePassBackgroundColorsGradientElementWithLightId_Elements =>
    ""."BloomPrePassBackgroundColorsGradientElementWithLightId/Elements"
);
#[cfg(feature = "BloomPrePassBackgroundColorsGradientElementWithLightId+Elements")]
impl std::ops::Deref
for crate::GlobalNamespace::BloomPrePassBackgroundColorsGradientElementWithLightId_Elements {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BloomPrePassBackgroundColorsGradientElementWithLightId+Elements")]
impl std::ops::DerefMut
for crate::GlobalNamespace::BloomPrePassBackgroundColorsGradientElementWithLightId_Elements {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BloomPrePassBackgroundColorsGradientElementWithLightId+Elements")]
impl crate::GlobalNamespace::BloomPrePassBackgroundColorsGradientElementWithLightId_Elements {
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
#[cfg(feature = "BloomPrePassBackgroundColorsGradientElementWithLightId+Elements")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::BloomPrePassBackgroundColorsGradientElementWithLightId_Elements {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
