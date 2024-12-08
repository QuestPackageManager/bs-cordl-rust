#[cfg(feature = "SpriteLightWithId+MultiplyColorByAlphaType")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SpriteLightWithId_MultiplyColorByAlphaType {
    AfterApplyingMinAlpha = 2i32,
    BeforeApplyingMinAlpha = 1i32,
    None = 0i32,
}
#[cfg(feature = "SpriteLightWithId+MultiplyColorByAlphaType")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::SpriteLightWithId_MultiplyColorByAlphaType => ""
    ."SpriteLightWithId/MultiplyColorByAlphaType"
);
#[cfg(feature = "SpriteLightWithId")]
#[repr(C)]
#[derive(Debug)]
pub struct SpriteLightWithId {
    __cordl_parent: LightWithIdMonoBehaviour,
    pub _spriteRenderer: *mut crate::UnityEngine::SpriteRenderer,
    pub _hideIfAlphaOutOfRange: bool,
    pub _hideAlphaRangeMin: f32,
    pub _hideAlphaRangeMax: f32,
    pub _intensity: f32,
    pub _minAlpha: f32,
    pub _multiplyColorByAlpha: crate::GlobalNamespace::SpriteLightWithId_MultiplyColorByAlphaType,
    pub _setColorOnly: bool,
    pub _setAlphaOnly: bool,
    pub _setOnlyOnce: bool,
}
#[cfg(feature = "SpriteLightWithId")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for SpriteLightWithId => ""."SpriteLightWithId"
);
#[cfg(feature = "SpriteLightWithId")]
impl std::ops::Deref for SpriteLightWithId {
    type Target = LightWithIdMonoBehaviour;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "SpriteLightWithId")]
impl std::ops::DerefMut for SpriteLightWithId {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "SpriteLightWithId")]
impl SpriteLightWithId {
    #[cfg(feature = "SpriteLightWithId+MultiplyColorByAlphaType")]
    pub type MultiplyColorByAlphaType = crate::GlobalNamespace::SpriteLightWithId_MultiplyColorByAlphaType;
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
    pub fn get_color(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Color> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Color = __cordl_object
            .invoke("get_color", ())?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "SpriteLightWithId")]
impl quest_hook::libil2cpp::ObjectType for SpriteLightWithId {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
