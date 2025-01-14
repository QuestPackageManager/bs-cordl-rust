#[cfg(feature = "PlatformException")]
#[repr(C)]
#[derive(Debug)]
pub struct PlatformException {
    __cordl_parent: crate::System::Exception,
    pub error: crate::GlobalNamespace::PlatformException_ErrorType,
    pub code: i32,
    pub httpCode: i32,
}
#[cfg(feature = "PlatformException")]
unsafe impl quest_hook::libil2cpp::Type for crate::GlobalNamespace::PlatformException {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "PlatformException";
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
#[cfg(feature = "PlatformException")]
impl std::ops::Deref for crate::GlobalNamespace::PlatformException {
    type Target = crate::System::Exception;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "PlatformException")]
impl std::ops::DerefMut for crate::GlobalNamespace::PlatformException {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "PlatformException")]
impl crate::GlobalNamespace::PlatformException {
    #[cfg(feature = "PlatformException+ErrorType")]
    pub type ErrorType = crate::GlobalNamespace::PlatformException_ErrorType;
    pub fn New_Il2CppString_Exception0(
        error: crate::GlobalNamespace::PlatformException_ErrorType,
        message: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        innerException: quest_hook::libil2cpp::Gc<crate::System::Exception>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (error, message, innerException))?;
        Ok(__cordl_object.into())
    }
    pub fn New_i32_i32_Il2CppString_Exception1(
        error: crate::GlobalNamespace::PlatformException_ErrorType,
        code: i32,
        httpCode: i32,
        message: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        innerException: quest_hook::libil2cpp::Gc<crate::System::Exception>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (error, code, httpCode, message, innerException))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor_Il2CppString_Exception0(
        &mut self,
        error: crate::GlobalNamespace::PlatformException_ErrorType,
        message: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        innerException: quest_hook::libil2cpp::Gc<crate::System::Exception>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    crate::GlobalNamespace::PlatformException_ErrorType,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                    quest_hook::libil2cpp::Gc<crate::System::Exception>,
                ),
                quest_hook::libil2cpp::Void,
                3usize,
            >(".ctor")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, ".ctor", 3usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (error, message, innerException))
        };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_i32_i32_Il2CppString_Exception1(
        &mut self,
        error: crate::GlobalNamespace::PlatformException_ErrorType,
        code: i32,
        httpCode: i32,
        message: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        innerException: quest_hook::libil2cpp::Gc<crate::System::Exception>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    crate::GlobalNamespace::PlatformException_ErrorType,
                    i32,
                    i32,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                    quest_hook::libil2cpp::Gc<crate::System::Exception>,
                ),
                quest_hook::libil2cpp::Void,
                5usize,
            >(".ctor")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, ".ctor", 5usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method
                .invoke_unchecked(self, (error, code, httpCode, message, innerException))
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "PlatformException")]
impl quest_hook::libil2cpp::ObjectType for crate::GlobalNamespace::PlatformException {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "PlatformException+ErrorType")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum PlatformException_ErrorType {
    #[default]
    InvalidPlatformVersion = 3i32,
    PlatformInitialization = 1i32,
    PlatformNotInstalled = 0i32,
    PlatformUserEntitlement = 2i32,
}
#[cfg(feature = "PlatformException+ErrorType")]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::PlatformException_ErrorType {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "PlatformException/ErrorType";
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
#[cfg(feature = "PlatformException+ErrorType")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::GlobalNamespace::PlatformException_ErrorType {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "PlatformException+ErrorType")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::GlobalNamespace::PlatformException_ErrorType {
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
#[cfg(feature = "PlatformException+ErrorType")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::GlobalNamespace::PlatformException_ErrorType {
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
#[cfg(feature = "PlatformException+ErrorType")]
unsafe impl quest_hook::libil2cpp::Return
for crate::GlobalNamespace::PlatformException_ErrorType {
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
