#[cfg(feature = "cordl_class_FloatTextureProcessor3DMappingVectorEffectTarget")]
#[repr(C)]
#[derive(Debug)]
pub struct FloatTextureProcessor3DMappingVectorEffectTarget {
    __cordl_parent: crate::GlobalNamespace::FloatFxGroupEffectTarget,
    pub _material: quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
    pub _useSlave: bool,
    pub _slaveMaterial: quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
    pub _mapping: crate::GlobalNamespace::FloatTextureProcessor3DMappingVectorEffectTarget_TextureProcessor3DMapping,
    pub _channel: crate::GlobalNamespace::FloatTextureProcessor3DMappingVectorEffectTarget_TextureProcessor3DChannel,
    pub _valueBounds: crate::UnityEngine::Vector2,
    pub _invertAxis: bool,
    pub _invertAxisSlave: bool,
    pub _fullVector4: crate::UnityEngine::Vector4,
    pub _fullVector4Slave: crate::UnityEngine::Vector4,
}
#[cfg(feature = "cordl_class_FloatTextureProcessor3DMappingVectorEffectTarget")]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::FloatTextureProcessor3DMappingVectorEffectTarget {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "FloatTextureProcessor3DMappingVectorEffectTarget";
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
#[cfg(feature = "FloatTextureProcessor3DMappingVectorEffectTarget")]
impl std::ops::Deref
for crate::GlobalNamespace::FloatTextureProcessor3DMappingVectorEffectTarget {
    type Target = crate::GlobalNamespace::FloatFxGroupEffectTarget;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "FloatTextureProcessor3DMappingVectorEffectTarget")]
impl std::ops::DerefMut
for crate::GlobalNamespace::FloatTextureProcessor3DMappingVectorEffectTarget {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "FloatTextureProcessor3DMappingVectorEffectTarget")]
impl crate::GlobalNamespace::FloatTextureProcessor3DMappingVectorEffectTarget {
    #[cfg(
        feature = "FloatTextureProcessor3DMappingVectorEffectTarget+TextureProcessor3DChannel"
    )]
    pub type TextureProcessor3DChannel = crate::GlobalNamespace::FloatTextureProcessor3DMappingVectorEffectTarget_TextureProcessor3DChannel;
    #[cfg(
        feature = "FloatTextureProcessor3DMappingVectorEffectTarget+TextureProcessor3DMapping"
    )]
    pub type TextureProcessor3DMapping = crate::GlobalNamespace::FloatTextureProcessor3DMappingVectorEffectTarget_TextureProcessor3DMapping;
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn SetFloat(
        &mut self,
        value: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (f32),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("SetFloat")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "SetFloat", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (value))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn SetValue(
        &mut self,
        groupId: i32,
        elementId: i32,
        value: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (i32, i32, f32),
                        quest_hook::libil2cpp::Void,
                        3usize,
                    >("SetValue")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "SetValue", 3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (groupId, elementId, value))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn TriggerValue(
        &mut self,
        groupId: i32,
        elementId: i32,
        value: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (i32, i32, f32),
                        quest_hook::libil2cpp::Void,
                        3usize,
                    >("TriggerValue")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "TriggerValue", 3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (groupId, elementId, value))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), quest_hook::libil2cpp::Void, 0usize>(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), ".ctor",
                            0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "cordl_class_FloatTextureProcessor3DMappingVectorEffectTarget")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::FloatTextureProcessor3DMappingVectorEffectTarget {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(
    feature = "cordl_class_FloatTextureProcessor3DMappingVectorEffectTarget+TextureProcessor3DChannel"
)]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum FloatTextureProcessor3DMappingVectorEffectTarget_TextureProcessor3DChannel {
    #[default]
    A = 0i32,
    B = 1i32,
    C = 2i32,
    D = 3i32,
}
#[cfg(
    feature = "cordl_class_FloatTextureProcessor3DMappingVectorEffectTarget+TextureProcessor3DChannel"
)]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::FloatTextureProcessor3DMappingVectorEffectTarget_TextureProcessor3DChannel {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "FloatTextureProcessor3DMappingVectorEffectTarget/TextureProcessor3DChannel";
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
    feature = "cordl_class_FloatTextureProcessor3DMappingVectorEffectTarget+TextureProcessor3DChannel"
)]
unsafe impl quest_hook::libil2cpp::Argument
for crate::GlobalNamespace::FloatTextureProcessor3DMappingVectorEffectTarget_TextureProcessor3DChannel {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(
    feature = "cordl_class_FloatTextureProcessor3DMappingVectorEffectTarget+TextureProcessor3DChannel"
)]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::GlobalNamespace::FloatTextureProcessor3DMappingVectorEffectTarget_TextureProcessor3DChannel {
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
    feature = "cordl_class_FloatTextureProcessor3DMappingVectorEffectTarget+TextureProcessor3DChannel"
)]
unsafe impl quest_hook::libil2cpp::Returned
for crate::GlobalNamespace::FloatTextureProcessor3DMappingVectorEffectTarget_TextureProcessor3DChannel {
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
    feature = "cordl_class_FloatTextureProcessor3DMappingVectorEffectTarget+TextureProcessor3DChannel"
)]
unsafe impl quest_hook::libil2cpp::Return
for crate::GlobalNamespace::FloatTextureProcessor3DMappingVectorEffectTarget_TextureProcessor3DChannel {
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
    feature = "cordl_class_FloatTextureProcessor3DMappingVectorEffectTarget+TextureProcessor3DMapping"
)]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum FloatTextureProcessor3DMappingVectorEffectTarget_TextureProcessor3DMapping {
    #[default]
    Emissive = 6i32,
    RadialDisplacement = 3i32,
    Rotation = 5i32,
    Scale = 4i32,
    XDisplacement = 0i32,
    YDisplacement = 1i32,
    ZDisplacement = 2i32,
}
#[cfg(
    feature = "cordl_class_FloatTextureProcessor3DMappingVectorEffectTarget+TextureProcessor3DMapping"
)]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::FloatTextureProcessor3DMappingVectorEffectTarget_TextureProcessor3DMapping {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "FloatTextureProcessor3DMappingVectorEffectTarget/TextureProcessor3DMapping";
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
    feature = "cordl_class_FloatTextureProcessor3DMappingVectorEffectTarget+TextureProcessor3DMapping"
)]
unsafe impl quest_hook::libil2cpp::Argument
for crate::GlobalNamespace::FloatTextureProcessor3DMappingVectorEffectTarget_TextureProcessor3DMapping {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(
    feature = "cordl_class_FloatTextureProcessor3DMappingVectorEffectTarget+TextureProcessor3DMapping"
)]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::GlobalNamespace::FloatTextureProcessor3DMappingVectorEffectTarget_TextureProcessor3DMapping {
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
    feature = "cordl_class_FloatTextureProcessor3DMappingVectorEffectTarget+TextureProcessor3DMapping"
)]
unsafe impl quest_hook::libil2cpp::Returned
for crate::GlobalNamespace::FloatTextureProcessor3DMappingVectorEffectTarget_TextureProcessor3DMapping {
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
    feature = "cordl_class_FloatTextureProcessor3DMappingVectorEffectTarget+TextureProcessor3DMapping"
)]
unsafe impl quest_hook::libil2cpp::Return
for crate::GlobalNamespace::FloatTextureProcessor3DMappingVectorEffectTarget_TextureProcessor3DMapping {
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
