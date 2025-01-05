#[cfg(feature = "Org+BouncyCastle+Cmp+RevocationDetailsBuilder")]
#[repr(C)]
#[derive(Debug)]
pub struct RevocationDetailsBuilder {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _templateBuilder: quest_hook::libil2cpp::Gc<
        crate::Org::BouncyCastle::Asn1::Crmf::CertTemplateBuilder,
    >,
}
#[cfg(feature = "Org+BouncyCastle+Cmp+RevocationDetailsBuilder")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Org::BouncyCastle::Cmp::RevocationDetailsBuilder
    => "Org.BouncyCastle.Cmp"."RevocationDetailsBuilder"
);
#[cfg(feature = "Org+BouncyCastle+Cmp+RevocationDetailsBuilder")]
impl std::ops::Deref for crate::Org::BouncyCastle::Cmp::RevocationDetailsBuilder {
    type Target = quest_hook::libil2cpp::Il2CppObject;
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
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Cmp::RevocationDetails>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Cmp::RevocationDetails,
        > = __cordl_object.invoke("Build", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn SetIssuer(
        &mut self,
        issuer: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Asn1::X509::X509Name>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Cmp::RevocationDetailsBuilder,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Cmp::RevocationDetailsBuilder,
        > = __cordl_object.invoke("SetIssuer", (issuer))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetPublicKey(
        &mut self,
        publicKey: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::X509::SubjectPublicKeyInfo,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Cmp::RevocationDetailsBuilder,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Cmp::RevocationDetailsBuilder,
        > = __cordl_object.invoke("SetPublicKey", (publicKey))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetSerialNumber(
        &mut self,
        serialNumber: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Math::BigInteger,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Cmp::RevocationDetailsBuilder,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Cmp::RevocationDetailsBuilder,
        > = __cordl_object.invoke("SetSerialNumber", (serialNumber))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetSubject(
        &mut self,
        subject: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::X509::X509Name,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Cmp::RevocationDetailsBuilder,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Cmp::RevocationDetailsBuilder,
        > = __cordl_object.invoke("SetSubject", (subject))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret.into())
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
