#[cfg(feature = "Org+BouncyCastle+Crmf+PkiArchiveControlBuilder")]
#[repr(C)]
#[derive(Debug)]
pub struct PkiArchiveControlBuilder {
    __cordl_parent: crate::System::Object,
    pub envGen: *mut crate::Org::BouncyCastle::Cms::CmsEnvelopedDataGenerator,
    pub keyContent: *mut crate::Org::BouncyCastle::Cms::CmsProcessableByteArray,
}
#[cfg(feature = "Org+BouncyCastle+Crmf+PkiArchiveControlBuilder")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::Org::BouncyCastle::Crmf::PkiArchiveControlBuilder => "Org.BouncyCastle.Crmf"
    ."PkiArchiveControlBuilder"
);
#[cfg(feature = "Org+BouncyCastle+Crmf+PkiArchiveControlBuilder")]
impl std::ops::Deref for crate::Org::BouncyCastle::Crmf::PkiArchiveControlBuilder {
    type Target = crate::System::Object;
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
        recipientGen: *mut crate::Org::BouncyCastle::Cms::RecipientInfoGenerator,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Crmf::PkiArchiveControlBuilder,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Crmf::PkiArchiveControlBuilder = __cordl_object
            .invoke("AddRecipientGenerator", (recipientGen))?;
        Ok(__cordl_ret)
    }
    pub fn Build(
        &mut self,
        contentEncryptor: *mut crate::Org::BouncyCastle::Crypto::ICipherBuilderWithKey,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Crmf::PkiArchiveControl,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Crmf::PkiArchiveControl = __cordl_object
            .invoke("Build", (contentEncryptor))?;
        Ok(__cordl_ret)
    }
    pub fn New(
        privateKeyInfo: *mut crate::Org::BouncyCastle::Asn1::Pkcs::PrivateKeyInfo,
        generalName: *mut crate::Org::BouncyCastle::Asn1::X509::GeneralName,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (privateKeyInfo, generalName))?;
        Ok(__cordl_object)
    }
    pub fn _ctor(
        &mut self,
        privateKeyInfo: *mut crate::Org::BouncyCastle::Asn1::Pkcs::PrivateKeyInfo,
        generalName: *mut crate::Org::BouncyCastle::Asn1::X509::GeneralName,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (privateKeyInfo, generalName))?;
        Ok(__cordl_ret)
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
