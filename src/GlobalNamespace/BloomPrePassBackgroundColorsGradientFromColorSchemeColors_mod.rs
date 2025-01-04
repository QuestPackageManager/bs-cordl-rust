#[cfg(feature = "BloomPrePassBackgroundColorsGradientFromColorSchemeColors")]
#[repr(C)]
#[derive(Debug)]
pub struct BloomPrePassBackgroundColorsGradientFromColorSchemeColors {
    __cordl_parent: crate::UnityEngine::MonoBehaviour,
    pub _bloomPrePassBackgroundColorsGradient: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::BloomPrePassBackgroundColorsGradient,
    >,
    pub _elements: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<
            *mut crate::GlobalNamespace::BloomPrePassBackgroundColorsGradientFromColorSchemeColors_Element,
        >,
    >,
    pub _colorProvider: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::IEnvironmentColorProvider,
    >,
}
#[cfg(feature = "BloomPrePassBackgroundColorsGradientFromColorSchemeColors")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::BloomPrePassBackgroundColorsGradientFromColorSchemeColors => ""
    ."BloomPrePassBackgroundColorsGradientFromColorSchemeColors"
);
#[cfg(feature = "BloomPrePassBackgroundColorsGradientFromColorSchemeColors")]
impl std::ops::Deref
for crate::GlobalNamespace::BloomPrePassBackgroundColorsGradientFromColorSchemeColors {
    type Target = crate::UnityEngine::MonoBehaviour;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BloomPrePassBackgroundColorsGradientFromColorSchemeColors")]
impl std::ops::DerefMut
for crate::GlobalNamespace::BloomPrePassBackgroundColorsGradientFromColorSchemeColors {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BloomPrePassBackgroundColorsGradientFromColorSchemeColors")]
impl crate::GlobalNamespace::BloomPrePassBackgroundColorsGradientFromColorSchemeColors {
    #[cfg(feature = "BloomPrePassBackgroundColorsGradientFromColorSchemeColors+Element")]
    pub type Element = crate::GlobalNamespace::BloomPrePassBackgroundColorsGradientFromColorSchemeColors_Element;
    #[cfg(
        feature = "BloomPrePassBackgroundColorsGradientFromColorSchemeColors+EnvironmentColor"
    )]
    pub type EnvironmentColor = crate::GlobalNamespace::BloomPrePassBackgroundColorsGradientFromColorSchemeColors_EnvironmentColor;
    pub fn HandleColorProviderDidChangeColor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleColorProviderDidChangeColor", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn OnDestroy(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnDestroy", ())?;
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
}
#[cfg(feature = "BloomPrePassBackgroundColorsGradientFromColorSchemeColors")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::BloomPrePassBackgroundColorsGradientFromColorSchemeColors {
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
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
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
    type Target = quest_hook::libil2cpp::Il2CppObject;
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
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum BloomPrePassBackgroundColorsGradientFromColorSchemeColors_EnvironmentColor {
    #[default]
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
