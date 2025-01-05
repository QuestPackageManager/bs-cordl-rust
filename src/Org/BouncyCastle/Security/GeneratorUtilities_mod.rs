#[cfg(feature = "Org+BouncyCastle+Security+GeneratorUtilities")]
#[repr(C)]
#[derive(Debug)]
pub struct GeneratorUtilities {
    __cordl_parent: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
}
#[cfg(feature = "Org+BouncyCastle+Security+GeneratorUtilities")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Org::BouncyCastle::Security::GeneratorUtilities
    => "Org.BouncyCastle.Security"."GeneratorUtilities"
);
#[cfg(feature = "Org+BouncyCastle+Security+GeneratorUtilities")]
impl std::ops::Deref for crate::Org::BouncyCastle::Security::GeneratorUtilities {
    type Target = quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Security+GeneratorUtilities")]
impl std::ops::DerefMut for crate::Org::BouncyCastle::Security::GeneratorUtilities {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Security+GeneratorUtilities")]
impl crate::Org::BouncyCastle::Security::GeneratorUtilities {
    pub fn AddDefaultKeySizeEntries(
        _cordl_size: i32,
        algorithms: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("AddDefaultKeySizeEntries", (_cordl_size, algorithms))?;
        Ok(__cordl_ret.into())
    }
    pub fn AddHMacKeyGenerator(
        algorithm: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        aliases: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("AddHMacKeyGenerator", (algorithm, aliases))?;
        Ok(__cordl_ret.into())
    }
    pub fn AddKgAlgorithm(
        canonicalName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        aliases: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("AddKgAlgorithm", (canonicalName, aliases))?;
        Ok(__cordl_ret.into())
    }
    pub fn AddKpgAlgorithm(
        canonicalName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        aliases: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("AddKpgAlgorithm", (canonicalName, aliases))?;
        Ok(__cordl_ret.into())
    }
    pub fn FindDefaultKeySize(
        canonicalName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("FindDefaultKeySize", (canonicalName))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetCanonicalKeyGeneratorAlgorithm(
        algorithm: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetCanonicalKeyGeneratorAlgorithm", (algorithm))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetCanonicalKeyPairGeneratorAlgorithm(
        algorithm: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetCanonicalKeyPairGeneratorAlgorithm", (algorithm))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetDefaultKeySize_Gc0(
        oid: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::DerObjectIdentifier,
        >,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetDefaultKeySize", (oid))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetDefaultKeySize_Gc1(
        algorithm: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetDefaultKeySize", (algorithm))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetKeyGenerator_Gc0(
        oid: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::DerObjectIdentifier,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Crypto::CipherKeyGenerator>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::CipherKeyGenerator,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetKeyGenerator", (oid))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetKeyGenerator_Gc1(
        algorithm: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Crypto::CipherKeyGenerator>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::CipherKeyGenerator,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetKeyGenerator", (algorithm))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetKeyPairGenerator_Gc0(
        oid: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::DerObjectIdentifier,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::IAsymmetricCipherKeyPairGenerator,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::IAsymmetricCipherKeyPairGenerator,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetKeyPairGenerator", (oid))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetKeyPairGenerator_Gc1(
        algorithm: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::IAsymmetricCipherKeyPairGenerator,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::IAsymmetricCipherKeyPairGenerator,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetKeyPairGenerator", (algorithm))?;
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
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
#[cfg(feature = "Org+BouncyCastle+Security+GeneratorUtilities")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Security::GeneratorUtilities {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
