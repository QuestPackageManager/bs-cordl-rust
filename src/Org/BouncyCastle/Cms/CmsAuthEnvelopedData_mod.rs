#[cfg(feature = "Org+BouncyCastle+Cms+CmsAuthEnvelopedData")]
#[repr(C)]
#[derive(Debug)]
pub struct CmsAuthEnvelopedData {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub recipientInfoStore: *mut crate::Org::BouncyCastle::Cms::RecipientInformationStore,
    pub contentInfo: *mut crate::Org::BouncyCastle::Asn1::Cms::ContentInfo,
    pub originator: *mut crate::Org::BouncyCastle::Asn1::Cms::OriginatorInfo,
    pub authEncAlg: *mut crate::Org::BouncyCastle::Asn1::X509::AlgorithmIdentifier,
    pub authAttrs: *mut crate::Org::BouncyCastle::Asn1::Asn1Set,
    pub mac: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
    pub unauthAttrs: *mut crate::Org::BouncyCastle::Asn1::Asn1Set,
}
#[cfg(feature = "Org+BouncyCastle+Cms+CmsAuthEnvelopedData")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Org::BouncyCastle::Cms::CmsAuthEnvelopedData =>
    "Org.BouncyCastle.Cms"."CmsAuthEnvelopedData"
);
#[cfg(feature = "Org+BouncyCastle+Cms+CmsAuthEnvelopedData")]
impl std::ops::Deref for crate::Org::BouncyCastle::Cms::CmsAuthEnvelopedData {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Cms+CmsAuthEnvelopedData")]
impl std::ops::DerefMut for crate::Org::BouncyCastle::Cms::CmsAuthEnvelopedData {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Cms+CmsAuthEnvelopedData")]
impl crate::Org::BouncyCastle::Cms::CmsAuthEnvelopedData {
    #[cfg(
        feature = "Org+BouncyCastle+Cms+CmsAuthEnvelopedData+AuthEnvelopedSecureReadable"
    )]
    pub type AuthEnvelopedSecureReadable = crate::Org::BouncyCastle::Cms::CmsAuthEnvelopedData_AuthEnvelopedSecureReadable;
    pub fn New_ContentInfo2(
        contentInfo: *mut crate::Org::BouncyCastle::Asn1::Cms::ContentInfo,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (contentInfo))?;
        Ok(__cordl_object)
    }
    pub fn New_Il2CppArray0(
        authEnvData: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (authEnvData))?;
        Ok(__cordl_object)
    }
    pub fn New_Stream1(
        authEnvData: *mut crate::System::IO::Stream,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (authEnvData))?;
        Ok(__cordl_object)
    }
    pub fn _ctor_ContentInfo2(
        &mut self,
        contentInfo: *mut crate::Org::BouncyCastle::Asn1::Cms::ContentInfo,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (contentInfo))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_Il2CppArray0(
        &mut self,
        authEnvData: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (authEnvData))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_Stream1(
        &mut self,
        authEnvData: *mut crate::System::IO::Stream,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (authEnvData))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "Org+BouncyCastle+Cms+CmsAuthEnvelopedData")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Cms::CmsAuthEnvelopedData {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "Org+BouncyCastle+Cms+CmsAuthEnvelopedData+AuthEnvelopedSecureReadable")]
#[repr(C)]
#[derive(Debug)]
pub struct CmsAuthEnvelopedData_AuthEnvelopedSecureReadable {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub parent: *mut crate::Org::BouncyCastle::Cms::CmsAuthEnvelopedData,
}
#[cfg(feature = "Org+BouncyCastle+Cms+CmsAuthEnvelopedData+AuthEnvelopedSecureReadable")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::Org::BouncyCastle::Cms::CmsAuthEnvelopedData_AuthEnvelopedSecureReadable =>
    "Org.BouncyCastle.Cms"."CmsAuthEnvelopedData/AuthEnvelopedSecureReadable"
);
#[cfg(feature = "Org+BouncyCastle+Cms+CmsAuthEnvelopedData+AuthEnvelopedSecureReadable")]
impl std::ops::Deref
for crate::Org::BouncyCastle::Cms::CmsAuthEnvelopedData_AuthEnvelopedSecureReadable {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Cms+CmsAuthEnvelopedData+AuthEnvelopedSecureReadable")]
impl std::ops::DerefMut
for crate::Org::BouncyCastle::Cms::CmsAuthEnvelopedData_AuthEnvelopedSecureReadable {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Cms+CmsAuthEnvelopedData+AuthEnvelopedSecureReadable")]
impl crate::Org::BouncyCastle::Cms::CmsAuthEnvelopedData_AuthEnvelopedSecureReadable {
    pub fn GetReadable(
        &mut self,
        key: *mut crate::Org::BouncyCastle::Crypto::Parameters::KeyParameter,
    ) -> quest_hook::libil2cpp::Result<*mut crate::Org::BouncyCastle::Cms::CmsReadable> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Cms::CmsReadable = __cordl_object
            .invoke("GetReadable", (key))?;
        Ok(__cordl_ret)
    }
    pub fn New(
        parent: *mut crate::Org::BouncyCastle::Cms::CmsAuthEnvelopedData,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (parent))?;
        Ok(__cordl_object)
    }
    pub fn _ctor(
        &mut self,
        parent: *mut crate::Org::BouncyCastle::Cms::CmsAuthEnvelopedData,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (parent))?;
        Ok(__cordl_ret)
    }
    pub fn get_Algorithm(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Asn1::X509::AlgorithmIdentifier,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Asn1::X509::AlgorithmIdentifier = __cordl_object
            .invoke("get_Algorithm", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_CryptoObject(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut quest_hook::libil2cpp::Il2CppObject> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppObject = __cordl_object
            .invoke("get_CryptoObject", ())?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "Org+BouncyCastle+Cms+CmsAuthEnvelopedData+AuthEnvelopedSecureReadable")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Cms::CmsAuthEnvelopedData_AuthEnvelopedSecureReadable {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
