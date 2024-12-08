#[cfg(feature = "System+Security+Cryptography+X509Certificates+X509ChainPolicy")]
#[repr(C)]
#[derive(Debug)]
pub struct X509ChainPolicy {
    __cordl_parent: crate::System::Object,
    pub apps: *mut crate::System::Security::Cryptography::OidCollection,
    pub cert: *mut crate::System::Security::Cryptography::OidCollection,
    pub store: *mut crate::System::Security::Cryptography::X509Certificates::X509CertificateCollection,
    pub store2: *mut crate::System::Security::Cryptography::X509Certificates::X509Certificate2Collection,
    pub rflag: crate::System::Security::Cryptography::X509Certificates::X509RevocationFlag,
    pub mode: crate::System::Security::Cryptography::X509Certificates::X509RevocationMode,
    pub timeout: crate::System::TimeSpan,
    pub vflags: crate::System::Security::Cryptography::X509Certificates::X509VerificationFlags,
    pub vtime: crate::System::DateTime,
}
#[cfg(feature = "System+Security+Cryptography+X509Certificates+X509ChainPolicy")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::System::Security::Cryptography::X509Certificates::X509ChainPolicy =>
    "System.Security.Cryptography.X509Certificates"."X509ChainPolicy"
);
#[cfg(feature = "System+Security+Cryptography+X509Certificates+X509ChainPolicy")]
impl std::ops::Deref
for crate::System::Security::Cryptography::X509Certificates::X509ChainPolicy {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Security+Cryptography+X509Certificates+X509ChainPolicy")]
impl std::ops::DerefMut
for crate::System::Security::Cryptography::X509Certificates::X509ChainPolicy {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Security+Cryptography+X509Certificates+X509ChainPolicy")]
impl crate::System::Security::Cryptography::X509Certificates::X509ChainPolicy {
    pub fn get_ExtraStore(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Security::Cryptography::X509Certificates::X509Certificate2Collection,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Security::Cryptography::X509Certificates::X509Certificate2Collection = __cordl_object
            .invoke("get_ExtraStore", ())?;
        Ok(__cordl_ret)
    }
    pub fn set_VerificationFlags(
        &mut self,
        value: crate::System::Security::Cryptography::X509Certificates::X509VerificationFlags,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_VerificationFlags", (value))?;
        Ok(__cordl_ret)
    }
    pub fn get_VerificationTime(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::System::DateTime> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::System::DateTime = __cordl_object
            .invoke("get_VerificationTime", ())?;
        Ok(__cordl_ret)
    }
    pub fn set_UrlRetrievalTimeout(
        &mut self,
        value: crate::System::TimeSpan,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_UrlRetrievalTimeout", (value))?;
        Ok(__cordl_ret)
    }
    pub fn get_RevocationMode(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::System::Security::Cryptography::X509Certificates::X509RevocationMode,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::System::Security::Cryptography::X509Certificates::X509RevocationMode = __cordl_object
            .invoke("get_RevocationMode", ())?;
        Ok(__cordl_ret)
    }
    pub fn set_RevocationMode(
        &mut self,
        value: crate::System::Security::Cryptography::X509Certificates::X509RevocationMode,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_RevocationMode", (value))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_RevocationFlag(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::System::Security::Cryptography::X509Certificates::X509RevocationFlag,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::System::Security::Cryptography::X509Certificates::X509RevocationFlag = __cordl_object
            .invoke("get_RevocationFlag", ())?;
        Ok(__cordl_ret)
    }
    pub fn set_VerificationTime(
        &mut self,
        value: crate::System::DateTime,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_VerificationTime", (value))?;
        Ok(__cordl_ret)
    }
    pub fn Reset(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Reset", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_VerificationFlags(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::System::Security::Cryptography::X509Certificates::X509VerificationFlags,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::System::Security::Cryptography::X509Certificates::X509VerificationFlags = __cordl_object
            .invoke("get_VerificationFlags", ())?;
        Ok(__cordl_ret)
    }
    pub fn set_RevocationFlag(
        &mut self,
        value: crate::System::Security::Cryptography::X509Certificates::X509RevocationFlag,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_RevocationFlag", (value))?;
        Ok(__cordl_ret)
    }
    pub fn New() -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "System+Security+Cryptography+X509Certificates+X509ChainPolicy")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Security::Cryptography::X509Certificates::X509ChainPolicy {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
