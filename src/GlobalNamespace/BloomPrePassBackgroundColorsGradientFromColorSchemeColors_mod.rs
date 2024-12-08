#[cfg(feature = "BloomPrePassBackgroundColorsGradientFromColorSchemeColors")]
#[repr(C)]
#[derive(Debug)]
pub struct BloomPrePassBackgroundColorsGradientFromColorSchemeColors {
    __cordl_parent: crate::UnityEngine::MonoBehaviour,
    pub _bloomPrePassBackgroundColorsGradient: *mut BloomPrePassBackgroundColorsGradient,
    pub _elements: *mut quest_hook::libil2cpp::Il2CppArray<
        *mut crate::GlobalNamespace::BloomPrePassBackgroundColorsGradientFromColorSchemeColors_Element,
    >,
    pub _colorManager: *mut EnvironmentColorManager,
}
#[cfg(feature = "BloomPrePassBackgroundColorsGradientFromColorSchemeColors")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for
    BloomPrePassBackgroundColorsGradientFromColorSchemeColors => ""
    ."BloomPrePassBackgroundColorsGradientFromColorSchemeColors"
);
#[cfg(feature = "BloomPrePassBackgroundColorsGradientFromColorSchemeColors")]
impl std::ops::Deref for BloomPrePassBackgroundColorsGradientFromColorSchemeColors {
    type Target = crate::UnityEngine::MonoBehaviour;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BloomPrePassBackgroundColorsGradientFromColorSchemeColors")]
impl std::ops::DerefMut for BloomPrePassBackgroundColorsGradientFromColorSchemeColors {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BloomPrePassBackgroundColorsGradientFromColorSchemeColors")]
impl BloomPrePassBackgroundColorsGradientFromColorSchemeColors {
    #[cfg(feature = "BloomPrePassBackgroundColorsGradientFromColorSchemeColors+Element")]
    pub type Element = crate::GlobalNamespace::BloomPrePassBackgroundColorsGradientFromColorSchemeColors_Element;
    #[cfg(
        feature = "BloomPrePassBackgroundColorsGradientFromColorSchemeColors+EnvironmentColor"
    )]
    pub type EnvironmentColor = crate::GlobalNamespace::BloomPrePassBackgroundColorsGradientFromColorSchemeColors_EnvironmentColor;
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
    pub fn New() -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "BloomPrePassBackgroundColorsGradientFromColorSchemeColors")]
impl quest_hook::libil2cpp::ObjectType
for BloomPrePassBackgroundColorsGradientFromColorSchemeColors {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "BloomPrePassBackgroundColorsGradientFromColorSchemeColors+Element")]
#[repr(C)]
#[derive(Debug)]
pub struct BloomPrePassBackgroundColorsGradientFromColorSchemeColors_Element {
    __cordl_parent: crate::System::Object,
    pub loadFromColorScheme: bool,
    pub environmentColor: crate::GlobalNamespace::BloomPrePassBackgroundColorsGradientFromColorSchemeColors_EnvironmentColor,
    pub intensity: f32,
    pub color: crate::UnityEngine::Color,
}
#[cfg(feature = "BloomPrePassBackgroundColorsGradientFromColorSchemeColors+Element")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::BloomPrePassBackgroundColorsGradientFromColorSchemeColors_Element
    => ""."BloomPrePassBackgroundColorsGradientFromColorSchemeColors/Element"
);
#[cfg(feature = "BloomPrePassBackgroundColorsGradientFromColorSchemeColors+Element")]
impl std::ops::Deref
for crate::GlobalNamespace::BloomPrePassBackgroundColorsGradientFromColorSchemeColors_Element {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BloomPrePassBackgroundColorsGradientFromColorSchemeColors+Element")]
impl std::ops::DerefMut
for crate::GlobalNamespace::BloomPrePassBackgroundColorsGradientFromColorSchemeColors_Element {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BloomPrePassBackgroundColorsGradientFromColorSchemeColors+Element")]
impl crate::GlobalNamespace::BloomPrePassBackgroundColorsGradientFromColorSchemeColors_Element {
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
#[cfg(feature = "BloomPrePassBackgroundColorsGradientFromColorSchemeColors+Element")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::BloomPrePassBackgroundColorsGradientFromColorSchemeColors_Element {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(
    feature = "BloomPrePassBackgroundColorsGradientFromColorSchemeColors+EnvironmentColor"
)]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BloomPrePassBackgroundColorsGradientFromColorSchemeColors_EnvironmentColor {
    Color0 = 0i32,
    Color0Boost = 2i32,
    Color1 = 1i32,
    Color1Boost = 3i32,
}
#[cfg(
    feature = "BloomPrePassBackgroundColorsGradientFromColorSchemeColors+EnvironmentColor"
)]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::BloomPrePassBackgroundColorsGradientFromColorSchemeColors_EnvironmentColor
    => ""."BloomPrePassBackgroundColorsGradientFromColorSchemeColors/EnvironmentColor"
);
