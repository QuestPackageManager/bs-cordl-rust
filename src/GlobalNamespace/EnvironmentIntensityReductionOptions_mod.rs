#[cfg(feature = "EnvironmentIntensityReductionOptions")]
#[repr(C)]
#[derive(Debug)]
pub struct EnvironmentIntensityReductionOptions {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _compressExpand: crate::GlobalNamespace::EnvironmentIntensityReductionOptions_CompressExpandReductionType,
    pub _rotateRings: crate::GlobalNamespace::EnvironmentIntensityReductionOptions_RotateRingsReductionType,
}
#[cfg(feature = "EnvironmentIntensityReductionOptions")]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::EnvironmentIntensityReductionOptions {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "EnvironmentIntensityReductionOptions";
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
#[cfg(feature = "EnvironmentIntensityReductionOptions")]
impl std::ops::Deref for crate::GlobalNamespace::EnvironmentIntensityReductionOptions {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "EnvironmentIntensityReductionOptions")]
impl std::ops::DerefMut
for crate::GlobalNamespace::EnvironmentIntensityReductionOptions {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "EnvironmentIntensityReductionOptions")]
impl crate::GlobalNamespace::EnvironmentIntensityReductionOptions {
    #[cfg(feature = "EnvironmentIntensityReductionOptions+CompressExpandReductionType")]
    pub type CompressExpandReductionType = crate::GlobalNamespace::EnvironmentIntensityReductionOptions_CompressExpandReductionType;
    #[cfg(feature = "EnvironmentIntensityReductionOptions+RotateRingsReductionType")]
    pub type RotateRingsReductionType = crate::GlobalNamespace::EnvironmentIntensityReductionOptions_RotateRingsReductionType;
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
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<(), quest_hook::libil2cpp::Void, 0usize>(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), ".ctor", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_compressExpand(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::GlobalNamespace::EnvironmentIntensityReductionOptions_CompressExpandReductionType,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (),
                        crate::GlobalNamespace::EnvironmentIntensityReductionOptions_CompressExpandReductionType,
                        0usize,
                    >("get_compressExpand")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "get_compressExpand", 0usize
                        )
                    })
            });
        let __cordl_ret: crate::GlobalNamespace::EnvironmentIntensityReductionOptions_CompressExpandReductionType = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_rotateRings(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::GlobalNamespace::EnvironmentIntensityReductionOptions_RotateRingsReductionType,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (),
                        crate::GlobalNamespace::EnvironmentIntensityReductionOptions_RotateRingsReductionType,
                        0usize,
                    >("get_rotateRings")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "get_rotateRings", 0usize
                        )
                    })
            });
        let __cordl_ret: crate::GlobalNamespace::EnvironmentIntensityReductionOptions_RotateRingsReductionType = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "EnvironmentIntensityReductionOptions")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::EnvironmentIntensityReductionOptions {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "EnvironmentIntensityReductionOptions+CompressExpandReductionType")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum EnvironmentIntensityReductionOptions_CompressExpandReductionType {
    #[default]
    Keep = 0i32,
    RemoveWithStrobeFilter = 1i32,
}
#[cfg(feature = "EnvironmentIntensityReductionOptions+CompressExpandReductionType")]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::EnvironmentIntensityReductionOptions_CompressExpandReductionType {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "EnvironmentIntensityReductionOptions/CompressExpandReductionType";
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
#[cfg(feature = "EnvironmentIntensityReductionOptions+CompressExpandReductionType")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::GlobalNamespace::EnvironmentIntensityReductionOptions_CompressExpandReductionType {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "EnvironmentIntensityReductionOptions+CompressExpandReductionType")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::GlobalNamespace::EnvironmentIntensityReductionOptions_CompressExpandReductionType {
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
#[cfg(feature = "EnvironmentIntensityReductionOptions+CompressExpandReductionType")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::GlobalNamespace::EnvironmentIntensityReductionOptions_CompressExpandReductionType {
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
#[cfg(feature = "EnvironmentIntensityReductionOptions+CompressExpandReductionType")]
unsafe impl quest_hook::libil2cpp::Return
for crate::GlobalNamespace::EnvironmentIntensityReductionOptions_CompressExpandReductionType {
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
#[cfg(feature = "EnvironmentIntensityReductionOptions+RotateRingsReductionType")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum EnvironmentIntensityReductionOptions_RotateRingsReductionType {
    #[default]
    Keep = 0i32,
    RemoveWithStrobeFilter = 1i32,
}
#[cfg(feature = "EnvironmentIntensityReductionOptions+RotateRingsReductionType")]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::EnvironmentIntensityReductionOptions_RotateRingsReductionType {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "EnvironmentIntensityReductionOptions/RotateRingsReductionType";
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
#[cfg(feature = "EnvironmentIntensityReductionOptions+RotateRingsReductionType")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::GlobalNamespace::EnvironmentIntensityReductionOptions_RotateRingsReductionType {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "EnvironmentIntensityReductionOptions+RotateRingsReductionType")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::GlobalNamespace::EnvironmentIntensityReductionOptions_RotateRingsReductionType {
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
#[cfg(feature = "EnvironmentIntensityReductionOptions+RotateRingsReductionType")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::GlobalNamespace::EnvironmentIntensityReductionOptions_RotateRingsReductionType {
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
#[cfg(feature = "EnvironmentIntensityReductionOptions+RotateRingsReductionType")]
unsafe impl quest_hook::libil2cpp::Return
for crate::GlobalNamespace::EnvironmentIntensityReductionOptions_RotateRingsReductionType {
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
