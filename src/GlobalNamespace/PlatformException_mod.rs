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
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::PlatformException => ""
    ."PlatformException"
);
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
    pub fn New_String_Exception0(
        error: crate::GlobalNamespace::PlatformException_ErrorType,
        message: *mut crate::System::String,
        innerException: *mut crate::System::Exception,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (error, message, innerException))?;
        Ok(__cordl_object)
    }
    pub fn New_i32_i32_String_Exception1(
        error: crate::GlobalNamespace::PlatformException_ErrorType,
        code: i32,
        httpCode: i32,
        message: *mut crate::System::String,
        innerException: *mut crate::System::Exception,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (error, code, httpCode, message, innerException))?;
        Ok(__cordl_object)
    }
    pub fn _ctor_String_Exception0(
        &mut self,
        error: crate::GlobalNamespace::PlatformException_ErrorType,
        message: *mut crate::System::String,
        innerException: *mut crate::System::Exception,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (error, message, innerException))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_i32_i32_String_Exception1(
        &mut self,
        error: crate::GlobalNamespace::PlatformException_ErrorType,
        code: i32,
        httpCode: i32,
        message: *mut crate::System::String,
        innerException: *mut crate::System::Exception,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (error, code, httpCode, message, innerException))?;
        Ok(__cordl_ret)
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
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PlatformException_ErrorType {
    PlatformInitialization = 1i32,
    PlatformNotInstalled = 0i32,
    PlatformUserEntitlement = 2i32,
}
#[cfg(feature = "PlatformException+ErrorType")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::PlatformException_ErrorType =>
    ""."PlatformException/ErrorType"
);
