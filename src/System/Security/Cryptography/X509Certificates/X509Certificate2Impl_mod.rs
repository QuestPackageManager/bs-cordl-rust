#[cfg(feature = "System+Security+Cryptography+X509Certificates+X509Certificate2Impl")]
#[repr(C)]
#[derive(Debug)]
pub struct X509Certificate2Impl {
    __cordl_parent: crate::System::Security::Cryptography::X509Certificates::X509CertificateImpl,
}
#[cfg(feature = "System+Security+Cryptography+X509Certificates+X509Certificate2Impl")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::System::Security::Cryptography::X509Certificates::X509Certificate2Impl =>
    "System.Security.Cryptography.X509Certificates"."X509Certificate2Impl"
);
#[cfg(feature = "System+Security+Cryptography+X509Certificates+X509Certificate2Impl")]
impl std::ops::Deref
for crate::System::Security::Cryptography::X509Certificates::X509Certificate2Impl {
    type Target = crate::System::Security::Cryptography::X509Certificates::X509CertificateImpl;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Security+Cryptography+X509Certificates+X509Certificate2Impl")]
impl std::ops::DerefMut
for crate::System::Security::Cryptography::X509Certificates::X509Certificate2Impl {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Security+Cryptography+X509Certificates+X509Certificate2Impl")]
impl crate::System::Security::Cryptography::X509Certificates::X509Certificate2Impl {
    pub fn set_PrivateKey(
        &mut self,
        value: *mut crate::System::Security::Cryptography::AsymmetricAlgorithm,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_PrivateKey", (value))?;
        Ok(__cordl_ret)
    }
    pub fn get_Version(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_Version", ())?;
        Ok(__cordl_ret)
    }
    pub fn GetNameInfo(
        &mut self,
        nameType: crate::System::Security::Cryptography::X509Certificates::X509NameType,
        forIssuer: bool,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("GetNameInfo", (nameType, forIssuer))?;
        Ok(__cordl_ret)
    }
    pub fn Verify(
        &mut self,
        thisCertificate: *mut crate::System::Security::Cryptography::X509Certificates::X509Certificate2,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("Verify", (thisCertificate))?;
        Ok(__cordl_ret)
    }
    pub fn get_SignatureAlgorithm(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("get_SignatureAlgorithm", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_Extensions(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Collections::Generic::IEnumerable_1<
            *mut crate::System::Security::Cryptography::X509Certificates::X509Extension,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Collections::Generic::IEnumerable_1<
            *mut crate::System::Security::Cryptography::X509Certificates::X509Extension,
        > = __cordl_object.invoke("get_Extensions", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_PrivateKey(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Security::Cryptography::AsymmetricAlgorithm,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Security::Cryptography::AsymmetricAlgorithm = __cordl_object
            .invoke("get_PrivateKey", ())?;
        Ok(__cordl_ret)
    }
    pub fn CopyWithPrivateKey(
        &mut self,
        privateKey: *mut crate::System::Security::Cryptography::RSA,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Security::Cryptography::X509Certificates::X509CertificateImpl,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Security::Cryptography::X509Certificates::X509CertificateImpl = __cordl_object
            .invoke("CopyWithPrivateKey", (privateKey))?;
        Ok(__cordl_ret)
    }
    pub fn AppendPrivateKeyInfo(
        &mut self,
        sb: *mut crate::System::Text::StringBuilder,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AppendPrivateKeyInfo", (sb))?;
        Ok(__cordl_ret)
    }
    pub fn get_SubjectName(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Security::Cryptography::X509Certificates::X500DistinguishedName,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Security::Cryptography::X509Certificates::X500DistinguishedName = __cordl_object
            .invoke("get_SubjectName", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_IssuerName(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Security::Cryptography::X509Certificates::X500DistinguishedName,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Security::Cryptography::X509Certificates::X500DistinguishedName = __cordl_object
            .invoke("get_IssuerName", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_IntermediateCertificates(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Security::Cryptography::X509Certificates::X509CertificateImplCollection,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Security::Cryptography::X509Certificates::X509CertificateImplCollection = __cordl_object
            .invoke("get_IntermediateCertificates", ())?;
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
    pub fn New() -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "System+Security+Cryptography+X509Certificates+X509Certificate2Impl")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Security::Cryptography::X509Certificates::X509Certificate2Impl {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
