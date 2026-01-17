#[cfg(feature = "cordl_class_FloatTextureProcessor3DParameterEffectTarget")]
#[repr(C)]
#[cfg_attr(feature = "derive_Debug", derive(Debug))]
pub struct FloatTextureProcessor3DParameterEffectTarget {
    __cordl_parent: crate::GlobalNamespace::FloatFxGroupEffectTarget,
    pub _textureProcessor3D: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::TextureProcessor3D,
    >,
    pub _parameter: crate::GlobalNamespace::FloatTextureProcessor3DParameterEffectTarget_TextureProcessor3DParameter,
    pub _channel: crate::GlobalNamespace::FloatTextureProcessor3DParameterEffectTarget_TextureProcessor3DChannel,
    pub _valueBounds: crate::UnityEngine::Vector2,
}
#[cfg(feature = "cordl_class_FloatTextureProcessor3DParameterEffectTarget")]
unsafe impl quest_hook::libil2cpp::Type
    for crate::GlobalNamespace::FloatTextureProcessor3DParameterEffectTarget
{
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "FloatTextureProcessor3DParameterEffectTarget";
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.class()
            .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
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
#[cfg(feature = "FloatTextureProcessor3DParameterEffectTarget")]
impl std::ops::Deref for crate::GlobalNamespace::FloatTextureProcessor3DParameterEffectTarget {
    type Target = crate::GlobalNamespace::FloatFxGroupEffectTarget;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "FloatTextureProcessor3DParameterEffectTarget")]
impl std::ops::DerefMut for crate::GlobalNamespace::FloatTextureProcessor3DParameterEffectTarget {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "FloatTextureProcessor3DParameterEffectTarget")]
impl crate::GlobalNamespace::FloatTextureProcessor3DParameterEffectTarget {
    #[cfg(feature = "FloatTextureProcessor3DParameterEffectTarget+TextureProcessor3DChannel")]
    pub type TextureProcessor3DChannel = crate::GlobalNamespace::FloatTextureProcessor3DParameterEffectTarget_TextureProcessor3DChannel;
    #[cfg(feature = "FloatTextureProcessor3DParameterEffectTarget+TextureProcessor3DParameter")]
    pub type TextureProcessor3DParameter = crate::GlobalNamespace::FloatTextureProcessor3DParameterEffectTarget_TextureProcessor3DParameter;
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self =
            <Self as quest_hook::libil2cpp::Type>::class().instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn SetFloat(
        &mut self,
        value: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(f32), quest_hook::libil2cpp::Void, 1usize>("SetFloat")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "SetFloat",
                            1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (value))? };
        Ok(__cordl_ret.into())
    }
    pub fn SetValue(
        &mut self,
        groupId: i32,
        elementId: i32,
        value: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(i32, i32, f32), quest_hook::libil2cpp::Void, 3usize>("SetValue")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "SetValue",
                            3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (groupId, elementId, value))? };
        Ok(__cordl_ret.into())
    }
    pub fn TriggerValue(
        &mut self,
        groupId: i32,
        elementId: i32,
        value: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(i32, i32, f32), quest_hook::libil2cpp::Void, 3usize>(
                        "TriggerValue",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "TriggerValue",
                            3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (groupId, elementId, value))? };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(&mut self) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), quest_hook::libil2cpp::Void, 0usize>(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            ".ctor",
                            0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "cordl_class_FloatTextureProcessor3DParameterEffectTarget")]
impl quest_hook::libil2cpp::ObjectType
    for crate::GlobalNamespace::FloatTextureProcessor3DParameterEffectTarget
{
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(
    feature = "cordl_class_FloatTextureProcessor3DParameterEffectTarget+TextureProcessor3DChannel"
)]
#[cfg_attr(feature = "derive_Debug", derive(Debug))]
#[cfg_attr(feature = "derive_Clone", derive(Clone))]
#[cfg_attr(feature = "derive_Copy", derive(Copy))]
#[cfg_attr(feature = "derive_PartialEq", derive(PartialEq))]
#[cfg_attr(feature = "derive_Eq", derive(Eq))]
#[cfg_attr(feature = "derive_Default", derive(Default))]
#[repr(i32)]
pub enum FloatTextureProcessor3DParameterEffectTarget_TextureProcessor3DChannel {
    #[cfg_attr(feature = "derive_Default", default)]
    A = 0i32,
    B = 1i32,
    C = 2i32,
    D = 3i32,
}
#[cfg(
    feature = "cordl_class_FloatTextureProcessor3DParameterEffectTarget+TextureProcessor3DChannel"
)]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::FloatTextureProcessor3DParameterEffectTarget_TextureProcessor3DChannel {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "FloatTextureProcessor3DParameterEffectTarget/TextureProcessor3DChannel";
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
#[cfg(
    feature = "cordl_class_FloatTextureProcessor3DParameterEffectTarget+TextureProcessor3DChannel"
)]
unsafe impl quest_hook::libil2cpp::Argument
for crate::GlobalNamespace::FloatTextureProcessor3DParameterEffectTarget_TextureProcessor3DChannel {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(
    feature = "cordl_class_FloatTextureProcessor3DParameterEffectTarget+TextureProcessor3DChannel"
)]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::GlobalNamespace::FloatTextureProcessor3DParameterEffectTarget_TextureProcessor3DChannel {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_parameter(ty)
    }
    fn from_actual(actual: <Self as quest_hook::libil2cpp::Parameter>::Actual) -> Self {
        actual
    }
    fn into_actual(self) -> <Self as quest_hook::libil2cpp::Parameter>::Actual {
        self
    }
}
#[cfg(
    feature = "cordl_class_FloatTextureProcessor3DParameterEffectTarget+TextureProcessor3DChannel"
)]
unsafe impl quest_hook::libil2cpp::Returned
for crate::GlobalNamespace::FloatTextureProcessor3DParameterEffectTarget_TextureProcessor3DChannel {
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
#[cfg(
    feature = "cordl_class_FloatTextureProcessor3DParameterEffectTarget+TextureProcessor3DChannel"
)]
unsafe impl quest_hook::libil2cpp::Return
for crate::GlobalNamespace::FloatTextureProcessor3DParameterEffectTarget_TextureProcessor3DChannel {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_return(ty)
    }
    fn into_actual(self) -> <Self as quest_hook::libil2cpp::Return>::Actual {
        self
    }
    fn from_actual(actual: <Self as quest_hook::libil2cpp::Return>::Actual) -> Self {
        actual
    }
}
#[cfg(
    feature = "cordl_class_FloatTextureProcessor3DParameterEffectTarget+TextureProcessor3DParameter"
)]
#[cfg_attr(feature = "derive_Debug", derive(Debug))]
#[cfg_attr(feature = "derive_Clone", derive(Clone))]
#[cfg_attr(feature = "derive_Copy", derive(Copy))]
#[cfg_attr(feature = "derive_PartialEq", derive(PartialEq))]
#[cfg_attr(feature = "derive_Eq", derive(Eq))]
#[cfg_attr(feature = "derive_Default", derive(Default))]
#[repr(i32)]
pub enum FloatTextureProcessor3DParameterEffectTarget_TextureProcessor3DParameter {
    #[cfg_attr(feature = "derive_Default", default)]
    ComputeKernel = 0i32,
    OutputOffset = 7i32,
    Param1 = 5i32,
    Param2 = 6i32,
    Phase = 4i32,
    SpatialScale = 3i32,
    Speed = 2i32,
    TextureIndex = 1i32,
}
#[cfg(
    feature = "cordl_class_FloatTextureProcessor3DParameterEffectTarget+TextureProcessor3DParameter"
)]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::FloatTextureProcessor3DParameterEffectTarget_TextureProcessor3DParameter {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "FloatTextureProcessor3DParameterEffectTarget/TextureProcessor3DParameter";
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
#[cfg(
    feature = "cordl_class_FloatTextureProcessor3DParameterEffectTarget+TextureProcessor3DParameter"
)]
unsafe impl quest_hook::libil2cpp::Argument
for crate::GlobalNamespace::FloatTextureProcessor3DParameterEffectTarget_TextureProcessor3DParameter {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(
    feature = "cordl_class_FloatTextureProcessor3DParameterEffectTarget+TextureProcessor3DParameter"
)]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::GlobalNamespace::FloatTextureProcessor3DParameterEffectTarget_TextureProcessor3DParameter {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_parameter(ty)
    }
    fn from_actual(actual: <Self as quest_hook::libil2cpp::Parameter>::Actual) -> Self {
        actual
    }
    fn into_actual(self) -> <Self as quest_hook::libil2cpp::Parameter>::Actual {
        self
    }
}
#[cfg(
    feature = "cordl_class_FloatTextureProcessor3DParameterEffectTarget+TextureProcessor3DParameter"
)]
unsafe impl quest_hook::libil2cpp::Returned
for crate::GlobalNamespace::FloatTextureProcessor3DParameterEffectTarget_TextureProcessor3DParameter {
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
#[cfg(
    feature = "cordl_class_FloatTextureProcessor3DParameterEffectTarget+TextureProcessor3DParameter"
)]
unsafe impl quest_hook::libil2cpp::Return
for crate::GlobalNamespace::FloatTextureProcessor3DParameterEffectTarget_TextureProcessor3DParameter {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_return(ty)
    }
    fn into_actual(self) -> <Self as quest_hook::libil2cpp::Return>::Actual {
        self
    }
    fn from_actual(actual: <Self as quest_hook::libil2cpp::Return>::Actual) -> Self {
        actual
    }
}
