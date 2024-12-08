#[cfg(feature = "Org+BouncyCastle+Cmp+RevocationDetailsBuilder")]
#[repr(C)]
#[derive(Debug)]
pub struct RevocationDetailsBuilder {
    __cordl_parent: crate::System::Object,
    pub _templateBuilder: *mut crate::Org::BouncyCastle::Asn1::Crmf::CertTemplateBuilder,
}
#[cfg(feature = "Org+BouncyCastle+Cmp+RevocationDetailsBuilder")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Org::BouncyCastle::Cmp::RevocationDetailsBuilder
    => "Org.BouncyCastle.Cmp"."RevocationDetailsBuilder"
);
#[cfg(feature = "Org+BouncyCastle+Cmp+RevocationDetailsBuilder")]
impl std::ops::Deref for crate::Org::BouncyCastle::Cmp::RevocationDetailsBuilder {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Cmp+RevocationDetailsBuilder")]
impl std::ops::DerefMut for crate::Org::BouncyCastle::Cmp::RevocationDetailsBuilder {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Cmp+RevocationDetailsBuilder")]
impl crate::Org::BouncyCastle::Cmp::RevocationDetailsBuilder {
    pub fn Build(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Cmp::RevocationDetails,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Cmp::RevocationDetails = __cordl_object
            .invoke("Build", ())?;
        Ok(__cordl_ret)
    }
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
    pub fn SetIssuer(
        &mut self,
        issuer: *mut crate::Org::BouncyCastle::Asn1::X509::X509Name,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Cmp::RevocationDetailsBuilder,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Cmp::RevocationDetailsBuilder = __cordl_object
            .invoke("SetIssuer", (issuer))?;
        Ok(__cordl_ret)
    }
    pub fn SetPublicKey(
        &mut self,
        publicKey: *mut crate::Org::BouncyCastle::Asn1::X509::SubjectPublicKeyInfo,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Cmp::RevocationDetailsBuilder,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Cmp::RevocationDetailsBuilder = __cordl_object
            .invoke("SetPublicKey", (publicKey))?;
        Ok(__cordl_ret)
    }
    pub fn SetSerialNumber(
        &mut self,
        serialNumber: *mut crate::Org::BouncyCastle::Math::BigInteger,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Cmp::RevocationDetailsBuilder,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Cmp::RevocationDetailsBuilder = __cordl_object
            .invoke("SetSerialNumber", (serialNumber))?;
        Ok(__cordl_ret)
    }
    pub fn SetSubject(
        &mut self,
        subject: *mut crate::Org::BouncyCastle::Asn1::X509::X509Name,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Cmp::RevocationDetailsBuilder,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Cmp::RevocationDetailsBuilder = __cordl_object
            .invoke("SetSubject", (subject))?;
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
}
#[cfg(feature = "Org+BouncyCastle+Cmp+RevocationDetailsBuilder")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Cmp::RevocationDetailsBuilder {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
