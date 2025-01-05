#[cfg(feature = "Org+BouncyCastle+Cms+CmsEnvelopedDataGenerator")]
#[repr(C)]
#[derive(Debug)]
pub struct CmsEnvelopedDataGenerator {
    __cordl_parent: crate::Org::BouncyCastle::Cms::CmsEnvelopedGenerator,
}
#[cfg(feature = "Org+BouncyCastle+Cms+CmsEnvelopedDataGenerator")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::Org::BouncyCastle::Cms::CmsEnvelopedDataGenerator => "Org.BouncyCastle.Cms"
    ."CmsEnvelopedDataGenerator"
);
#[cfg(feature = "Org+BouncyCastle+Cms+CmsEnvelopedDataGenerator")]
impl std::ops::Deref for crate::Org::BouncyCastle::Cms::CmsEnvelopedDataGenerator {
    type Target = crate::Org::BouncyCastle::Cms::CmsEnvelopedGenerator;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Cms+CmsEnvelopedDataGenerator")]
impl std::ops::DerefMut for crate::Org::BouncyCastle::Cms::CmsEnvelopedDataGenerator {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Cms+CmsEnvelopedDataGenerator")]
impl crate::Org::BouncyCastle::Cms::CmsEnvelopedDataGenerator {
    pub fn Generate_ICipherBuilderWithKey2(
        &mut self,
        content: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Cms::CmsProcessable,
        >,
        cipherBuilder: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::ICipherBuilderWithKey,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Cms::CmsEnvelopedData>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Cms::CmsEnvelopedData,
        > = __cordl_object.invoke("Generate", (content, cipherBuilder))?;
        Ok(__cordl_ret.into())
    }
    pub fn Generate_Il2CppString1(
        &mut self,
        content: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Cms::CmsProcessable,
        >,
        encryptionOid: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Cms::CmsEnvelopedData>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Cms::CmsEnvelopedData,
        > = __cordl_object.invoke("Generate", (content, encryptionOid))?;
        Ok(__cordl_ret.into())
    }
    pub fn Generate_Il2CppString_CipherKeyGenerator0(
        &mut self,
        content: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Cms::CmsProcessable,
        >,
        encryptionOid: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        keyGen: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::CipherKeyGenerator,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Cms::CmsEnvelopedData>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Cms::CmsEnvelopedData,
        > = __cordl_object.invoke("Generate", (content, encryptionOid, keyGen))?;
        Ok(__cordl_ret.into())
    }
    pub fn Generate_Il2CppString_i32_3(
        &mut self,
        content: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Cms::CmsProcessable,
        >,
        encryptionOid: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        keySize: i32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Cms::CmsEnvelopedData>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Cms::CmsEnvelopedData,
        > = __cordl_object.invoke("Generate", (content, encryptionOid, keySize))?;
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
}
#[cfg(feature = "Org+BouncyCastle+Cms+CmsEnvelopedDataGenerator")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Cms::CmsEnvelopedDataGenerator {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
