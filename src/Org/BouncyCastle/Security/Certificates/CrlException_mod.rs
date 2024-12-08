#[cfg(feature = "Org+BouncyCastle+Security+Certificates+CrlException")]
#[repr(C)]
#[derive(Debug)]
pub struct CrlException {
    __cordl_parent: crate::Org::BouncyCastle::Security::GeneralSecurityException,
}
#[cfg(feature = "Org+BouncyCastle+Security+Certificates+CrlException")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::Org::BouncyCastle::Security::Certificates::CrlException =>
    "Org.BouncyCastle.Security.Certificates"."CrlException"
);
#[cfg(feature = "Org+BouncyCastle+Security+Certificates+CrlException")]
impl std::ops::Deref for crate::Org::BouncyCastle::Security::Certificates::CrlException {
    type Target = crate::Org::BouncyCastle::Security::GeneralSecurityException;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Security+Certificates+CrlException")]
impl std::ops::DerefMut
for crate::Org::BouncyCastle::Security::Certificates::CrlException {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Security+Certificates+CrlException")]
impl crate::Org::BouncyCastle::Security::Certificates::CrlException {
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
    pub fn _ctor_String1(
        &mut self,
        msg: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (msg))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_String_Exception2(
        &mut self,
        msg: *mut crate::System::String,
        e: *mut crate::System::Exception,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (msg, e))?;
        Ok(__cordl_ret)
    }
    pub fn New_0() -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
    pub fn New_String1(
        msg: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (msg))?;
        Ok(__cordl_object)
    }
    pub fn New_String_Exception2(
        msg: *mut crate::System::String,
        e: *mut crate::System::Exception,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (msg, e))?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "Org+BouncyCastle+Security+Certificates+CrlException")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Security::Certificates::CrlException {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
