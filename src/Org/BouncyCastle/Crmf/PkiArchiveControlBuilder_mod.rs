#[cfg(feature = "Org+BouncyCastle+Crmf+PkiArchiveControlBuilder")]
#[repr(C)]
#[derive(Debug)]
pub struct PkiArchiveControlBuilder {
    __cordl_parent: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    pub envGen: quest_hook::libil2cpp::Gc<
        crate::Org::BouncyCastle::Cms::CmsEnvelopedDataGenerator,
    >,
    pub keyContent: quest_hook::libil2cpp::Gc<
        crate::Org::BouncyCastle::Cms::CmsProcessableByteArray,
    >,
}
#[cfg(feature = "Org+BouncyCastle+Crmf+PkiArchiveControlBuilder")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::Org::BouncyCastle::Crmf::PkiArchiveControlBuilder => "Org.BouncyCastle.Crmf"
    ."PkiArchiveControlBuilder"
);
#[cfg(feature = "Org+BouncyCastle+Crmf+PkiArchiveControlBuilder")]
impl std::ops::Deref for crate::Org::BouncyCastle::Crmf::PkiArchiveControlBuilder {
    type Target = quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crmf+PkiArchiveControlBuilder")]
impl std::ops::DerefMut for crate::Org::BouncyCastle::Crmf::PkiArchiveControlBuilder {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crmf+PkiArchiveControlBuilder")]
impl crate::Org::BouncyCastle::Crmf::PkiArchiveControlBuilder {
    pub fn AddRecipientGenerator(
        &mut self,
        recipientGen: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Cms::RecipientInfoGenerator,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crmf::PkiArchiveControlBuilder,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crmf::PkiArchiveControlBuilder,
        > = __cordl_object.invoke("AddRecipientGenerator", (recipientGen))?;
        Ok(__cordl_ret.into())
    }
    pub fn Build(
        &mut self,
        contentEncryptor: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::ICipherBuilderWithKey,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Crmf::PkiArchiveControl>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crmf::PkiArchiveControl,
        > = __cordl_object.invoke("Build", (contentEncryptor))?;
        Ok(__cordl_ret.into())
    }
    pub fn New(
        privateKeyInfo: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::Pkcs::PrivateKeyInfo,
        >,
        generalName: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::X509::GeneralName,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (privateKeyInfo, generalName))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        privateKeyInfo: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::Pkcs::PrivateKeyInfo,
        >,
        generalName: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::X509::GeneralName,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (privateKeyInfo, generalName))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Org+BouncyCastle+Crmf+PkiArchiveControlBuilder")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Crmf::PkiArchiveControlBuilder {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
