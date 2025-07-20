#[cfg(feature = "SpriteLightWithId")]
#[repr(C)]
#[derive(Debug)]
pub struct SpriteLightWithId {
    __cordl_parent: crate::GlobalNamespace::LightWithIdMonoBehaviour,
    pub _spriteRenderer: quest_hook::libil2cpp::Gc<crate::UnityEngine::SpriteRenderer>,
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
unsafe impl quest_hook::libil2cpp::Type for crate::GlobalNamespace::SpriteLightWithId {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "SpriteLightWithId";
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.class().is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_value_argument(_: &quest_hook::libil2cpp::Il2CppType) -> bool {
        false
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
    fn matches_value_parameter(_: &quest_hook::libil2cpp::Il2CppType) -> bool {
        false
    }
}
#[cfg(feature = "SpriteLightWithId")]
impl std::ops::Deref for crate::GlobalNamespace::SpriteLightWithId {
    type Target = crate::GlobalNamespace::LightWithIdMonoBehaviour;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "SpriteLightWithId")]
impl std::ops::DerefMut for crate::GlobalNamespace::SpriteLightWithId {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "SpriteLightWithId")]
impl crate::GlobalNamespace::SpriteLightWithId {
    #[cfg(feature = "SpriteLightWithId+MultiplyColorByAlphaType")]
    pub type MultiplyColorByAlphaType = crate::GlobalNamespace::SpriteLightWithId_MultiplyColorByAlphaType;
    pub fn ColorWasSet(
        &mut self,
        color: crate::UnityEngine::Color,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::GlobalNamespace::SpriteLightWithId as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (crate::UnityEngine::Color),
                quest_hook::libil2cpp::Void,
                1usize,
            >("ColorWasSet")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::GlobalNamespace::SpriteLightWithId as
                    quest_hook::libil2cpp::Type > ::class(), "ColorWasSet", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (color))?
        };
        Ok(__cordl_ret.into())
    }
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
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::GlobalNamespace::SpriteLightWithId as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), quest_hook::libil2cpp::Void, 0usize>(".ctor")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::GlobalNamespace::SpriteLightWithId as
                    quest_hook::libil2cpp::Type > ::class(), ".ctor", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_color(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Color> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::GlobalNamespace::SpriteLightWithId as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), crate::UnityEngine::Color, 0usize>("get_color")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::GlobalNamespace::SpriteLightWithId as
                    quest_hook::libil2cpp::Type > ::class(), "get_color", 0usize
                )
            });
        let __cordl_ret: crate::UnityEngine::Color = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "SpriteLightWithId")]
impl quest_hook::libil2cpp::ObjectType for crate::GlobalNamespace::SpriteLightWithId {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "SpriteLightWithId+MultiplyColorByAlphaType")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum SpriteLightWithId_MultiplyColorByAlphaType {
    #[default]
    AfterApplyingMinAlpha = 2i32,
    BeforeApplyingMinAlpha = 1i32,
    None = 0i32,
}
#[cfg(feature = "SpriteLightWithId+MultiplyColorByAlphaType")]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::SpriteLightWithId_MultiplyColorByAlphaType {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "SpriteLightWithId/MultiplyColorByAlphaType";
    fn matches_value_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        !ty.is_ref()
            && ty
                .class()
                .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref()
            && ty
                .class()
                .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_value_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        !ty.is_ref()
            && <Self as quest_hook::libil2cpp::Type>::class()
                .is_assignable_from(ty.class())
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref()
            && <Self as quest_hook::libil2cpp::Type>::class()
                .is_assignable_from(ty.class())
    }
}
#[cfg(feature = "SpriteLightWithId+MultiplyColorByAlphaType")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::GlobalNamespace::SpriteLightWithId_MultiplyColorByAlphaType {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "SpriteLightWithId+MultiplyColorByAlphaType")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::GlobalNamespace::SpriteLightWithId_MultiplyColorByAlphaType {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_parameter(ty)
    }
    fn from_actual(actual: Self::Actual) -> Self {
        actual
    }
    fn into_actual(self) -> Self::Actual {
        self
    }
}
#[cfg(feature = "SpriteLightWithId+MultiplyColorByAlphaType")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::GlobalNamespace::SpriteLightWithId_MultiplyColorByAlphaType {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_returned(ty)
    }
    fn from_object(object: Option<&mut quest_hook::libil2cpp::Il2CppObject>) -> Self {
        unsafe {
            quest_hook::libil2cpp::raw::unbox(
                quest_hook::libil2cpp::WrapRaw::raw(object.unwrap()),
            )
        }
    }
}
#[cfg(feature = "SpriteLightWithId+MultiplyColorByAlphaType")]
unsafe impl quest_hook::libil2cpp::Return
for crate::GlobalNamespace::SpriteLightWithId_MultiplyColorByAlphaType {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_return(ty)
    }
    fn into_actual(self) -> Self::Actual {
        self
    }
    fn from_actual(actual: Self::Actual) -> Self {
        actual
    }
}
