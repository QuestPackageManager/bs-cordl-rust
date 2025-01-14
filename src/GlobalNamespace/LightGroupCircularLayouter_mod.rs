#[cfg(feature = "LightGroupCircularLayouter")]
#[repr(C)]
#[derive(Debug)]
pub struct LightGroupCircularLayouter {
    __cordl_parent: crate::GlobalNamespace::LightGroupSubsystem,
    pub _radius: f32,
    pub _angle: f32,
    pub _startingAngle: f32,
    pub _rotationDirection: crate::GlobalNamespace::LightGroupCircularLayouter_RotationDirection,
    pub _staticRotation: bool,
    pub _additionalAngle: crate::UnityEngine::Vector3,
}
#[cfg(feature = "LightGroupCircularLayouter")]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::LightGroupCircularLayouter {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "LightGroupCircularLayouter";
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
#[cfg(feature = "LightGroupCircularLayouter")]
impl std::ops::Deref for crate::GlobalNamespace::LightGroupCircularLayouter {
    type Target = crate::GlobalNamespace::LightGroupSubsystem;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "LightGroupCircularLayouter")]
impl std::ops::DerefMut for crate::GlobalNamespace::LightGroupCircularLayouter {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "LightGroupCircularLayouter")]
impl crate::GlobalNamespace::LightGroupCircularLayouter {
    #[cfg(feature = "LightGroupCircularLayouter+RotationDirection")]
    pub type RotationDirection = crate::GlobalNamespace::LightGroupCircularLayouter_RotationDirection;
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
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), quest_hook::libil2cpp::Void, 0usize>(".ctor")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, ".ctor", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "LightGroupCircularLayouter")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::LightGroupCircularLayouter {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "LightGroupCircularLayouter+RotationDirection")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum LightGroupCircularLayouter_RotationDirection {
    #[default]
    Clockwise = 0i32,
    Counterclockwise = 1i32,
}
#[cfg(feature = "LightGroupCircularLayouter+RotationDirection")]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::LightGroupCircularLayouter_RotationDirection {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "LightGroupCircularLayouter/RotationDirection";
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
#[cfg(feature = "LightGroupCircularLayouter+RotationDirection")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::GlobalNamespace::LightGroupCircularLayouter_RotationDirection {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "LightGroupCircularLayouter+RotationDirection")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::GlobalNamespace::LightGroupCircularLayouter_RotationDirection {
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
#[cfg(feature = "LightGroupCircularLayouter+RotationDirection")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::GlobalNamespace::LightGroupCircularLayouter_RotationDirection {
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
#[cfg(feature = "LightGroupCircularLayouter+RotationDirection")]
unsafe impl quest_hook::libil2cpp::Return
for crate::GlobalNamespace::LightGroupCircularLayouter_RotationDirection {
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
