#[cfg(feature = "Org+BouncyCastle+Cms+CmsSignedGenerator")]
#[repr(C)]
#[derive(Debug)]
pub struct CmsSignedGenerator {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _certs: quest_hook::libil2cpp::Gc<crate::System::Collections::IList>,
    pub _crls: quest_hook::libil2cpp::Gc<crate::System::Collections::IList>,
    pub _signers: quest_hook::libil2cpp::Gc<crate::System::Collections::IList>,
    pub _digests: quest_hook::libil2cpp::Gc<crate::System::Collections::IDictionary>,
    pub _useDerForCerts: bool,
    pub _useDerForCrls: bool,
    pub _cordl_rand: quest_hook::libil2cpp::Gc<
        crate::Org::BouncyCastle::Security::SecureRandom,
    >,
}
#[cfg(feature = "Org+BouncyCastle+Cms+CmsSignedGenerator")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Org::BouncyCastle::Cms::CmsSignedGenerator =>
    "Org.BouncyCastle.Cms"."CmsSignedGenerator"
);
#[cfg(feature = "Org+BouncyCastle+Cms+CmsSignedGenerator")]
impl std::ops::Deref for crate::Org::BouncyCastle::Cms::CmsSignedGenerator {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Cms+CmsSignedGenerator")]
impl std::ops::DerefMut for crate::Org::BouncyCastle::Cms::CmsSignedGenerator {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Cms+CmsSignedGenerator")]
impl crate::Org::BouncyCastle::Cms::CmsSignedGenerator {
    pub fn AddAttributeCertificates(
        &mut self,
        store: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::X509::Store::IX509Store,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AddAttributeCertificates", (store))?;
        Ok(__cordl_ret.into())
    }
    pub fn AddCertificates(
        &mut self,
        certStore: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::X509::Store::IX509Store,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AddCertificates", (certStore))?;
        Ok(__cordl_ret.into())
    }
    pub fn AddCrls(
        &mut self,
        crlStore: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::X509::Store::IX509Store,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AddCrls", (crlStore))?;
        Ok(__cordl_ret.into())
    }
    pub fn AddSignerCallback(
        &mut self,
        si: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Cms::SignerInformation>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AddSignerCallback", (si))?;
        Ok(__cordl_ret.into())
    }
    pub fn AddSigners(
        &mut self,
        signerStore: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Cms::SignerInformationStore,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AddSigners", (signerStore))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetAttributeSet(
        &mut self,
        attr: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::Cms::AttributeTable,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Asn1::Asn1Set>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::Asn1Set,
        > = __cordl_object.invoke("GetAttributeSet", (attr))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetBaseParameters(
        &mut self,
        contentType: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::DerObjectIdentifier,
        >,
        digAlgId: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::X509::AlgorithmIdentifier,
        >,
        hash: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Collections::IDictionary>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::IDictionary,
        > = __cordl_object.invoke("GetBaseParameters", (contentType, digAlgId, hash))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetGeneratedDigests(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Collections::IDictionary>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::IDictionary,
        > = __cordl_object.invoke("GetGeneratedDigests", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetSignerIdentifier_Il2CppArray1(
        subjectKeyIdentifier: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<u8>,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Asn1::Cms::SignerIdentifier>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::Cms::SignerIdentifier,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetSignerIdentifier", (subjectKeyIdentifier))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetSignerIdentifier_X509Certificate0(
        cert: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::X509::X509Certificate>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Asn1::Cms::SignerIdentifier>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::Cms::SignerIdentifier,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetSignerIdentifier", (cert))?;
        Ok(__cordl_ret.into())
    }
    pub fn New_0() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn New_SecureRandom1(
        _cordl_rand: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Security::SecureRandom,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (_cordl_rand))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor_0(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_SecureRandom1(
        &mut self,
        _cordl_rand: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Security::SecureRandom,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (_cordl_rand))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_UseDerForCerts(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_UseDerForCerts", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_UseDerForCrls(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_UseDerForCrls", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn set_UseDerForCerts(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_UseDerForCerts", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_UseDerForCrls(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_UseDerForCrls", (value))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Org+BouncyCastle+Cms+CmsSignedGenerator")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Cms::CmsSignedGenerator {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
