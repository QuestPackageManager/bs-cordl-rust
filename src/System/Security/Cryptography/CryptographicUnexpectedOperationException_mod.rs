#[cfg(
    feature = "System+Security+Cryptography+CryptographicUnexpectedOperationException"
)]
#[repr(C)]
#[derive(Debug)]
pub struct CryptographicUnexpectedOperationException {
    __cordl_parent: crate::System::Security::Cryptography::CryptographicException,
}
#[cfg(
    feature = "System+Security+Cryptography+CryptographicUnexpectedOperationException"
)]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::System::Security::Cryptography::CryptographicUnexpectedOperationException =>
    "System.Security.Cryptography"."CryptographicUnexpectedOperationException"
);
#[cfg(
    feature = "System+Security+Cryptography+CryptographicUnexpectedOperationException"
)]
impl std::ops::Deref
for crate::System::Security::Cryptography::CryptographicUnexpectedOperationException {
    type Target = crate::System::Security::Cryptography::CryptographicException;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(
    feature = "System+Security+Cryptography+CryptographicUnexpectedOperationException"
)]
impl std::ops::DerefMut
for crate::System::Security::Cryptography::CryptographicUnexpectedOperationException {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(
    feature = "System+Security+Cryptography+CryptographicUnexpectedOperationException"
)]
impl crate::System::Security::Cryptography::CryptographicUnexpectedOperationException {
    pub fn New_0() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
    pub fn New_SerializationInfo_StreamingContext2(
        info: *mut crate::System::Runtime::Serialization::SerializationInfo,
        context: crate::System::Runtime::Serialization::StreamingContext,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (info, context))?;
        Ok(__cordl_object)
    }
    pub fn New_String1(
        message: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (message))?;
        Ok(__cordl_object)
    }
    pub fn _ctor_0(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_SerializationInfo_StreamingContext2(
        &mut self,
        info: *mut crate::System::Runtime::Serialization::SerializationInfo,
        context: crate::System::Runtime::Serialization::StreamingContext,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (info, context))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_String1(
        &mut self,
        message: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (message))?;
        Ok(__cordl_ret)
    }
}
#[cfg(
    feature = "System+Security+Cryptography+CryptographicUnexpectedOperationException"
)]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Security::Cryptography::CryptographicUnexpectedOperationException {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
