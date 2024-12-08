#[cfg(feature = "Org+BouncyCastle+Security+EncryptionException")]
#[repr(C)]
#[derive(Debug)]
pub struct EncryptionException {
    __cordl_parent: crate::System::IO::IOException,
}
#[cfg(feature = "Org+BouncyCastle+Security+EncryptionException")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Org::BouncyCastle::Security::EncryptionException
    => "Org.BouncyCastle.Security"."EncryptionException"
);
#[cfg(feature = "Org+BouncyCastle+Security+EncryptionException")]
impl std::ops::Deref for crate::Org::BouncyCastle::Security::EncryptionException {
    type Target = crate::System::IO::IOException;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Security+EncryptionException")]
impl std::ops::DerefMut for crate::Org::BouncyCastle::Security::EncryptionException {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Security+EncryptionException")]
impl crate::Org::BouncyCastle::Security::EncryptionException {
    pub fn _ctor_String0(
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
    pub fn _ctor_Exception1(
        &mut self,
        message: *mut crate::System::String,
        exception: *mut crate::System::Exception,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (message, exception))?;
        Ok(__cordl_ret)
    }
    pub fn New_String0(
        message: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (message))?;
        Ok(__cordl_object)
    }
    pub fn New_Exception1(
        message: *mut crate::System::String,
        exception: *mut crate::System::Exception,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (message, exception))?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "Org+BouncyCastle+Security+EncryptionException")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Security::EncryptionException {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
