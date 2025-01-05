#[cfg(feature = "Org+BouncyCastle+Security+DigestUtilities")]
#[repr(C)]
#[derive(Debug)]
pub struct DigestUtilities {
    __cordl_parent: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
}
#[cfg(feature = "Org+BouncyCastle+Security+DigestUtilities")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Org::BouncyCastle::Security::DigestUtilities =>
    "Org.BouncyCastle.Security"."DigestUtilities"
);
#[cfg(feature = "Org+BouncyCastle+Security+DigestUtilities")]
impl std::ops::Deref for crate::Org::BouncyCastle::Security::DigestUtilities {
    type Target = quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Security+DigestUtilities")]
impl std::ops::DerefMut for crate::Org::BouncyCastle::Security::DigestUtilities {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Security+DigestUtilities")]
impl crate::Org::BouncyCastle::Security::DigestUtilities {
    #[cfg(feature = "Org+BouncyCastle+Security+DigestUtilities+DigestAlgorithm")]
    pub type DigestAlgorithm = crate::Org::BouncyCastle::Security::DigestUtilities_DigestAlgorithm;
    pub fn CalculateDigest_Gc_Gc0(
        id: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::DerObjectIdentifier,
        >,
        input: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<u8>,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CalculateDigest", (id, input))?;
        Ok(__cordl_ret.into())
    }
    pub fn CalculateDigest_Gc_Gc1(
        algorithm: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        input: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<u8>,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CalculateDigest", (algorithm, input))?;
        Ok(__cordl_ret.into())
    }
    pub fn DoFinal_Gc0(
        digest: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Crypto::IDigest>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<u8>,
        > = <Self as quest_hook::libil2cpp::Type>::class().invoke("DoFinal", (digest))?;
        Ok(__cordl_ret.into())
    }
    pub fn DoFinal_Gc1(
        digest: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Crypto::IDigest>,
        input: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<u8>,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("DoFinal", (digest, input))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetAlgorithmName(
        oid: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::DerObjectIdentifier,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetAlgorithmName", (oid))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetDigest_Gc0(
        id: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::DerObjectIdentifier,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Crypto::IDigest>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::IDigest,
        > = <Self as quest_hook::libil2cpp::Type>::class().invoke("GetDigest", (id))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetDigest_Gc1(
        algorithm: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Crypto::IDigest>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::IDigest,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetDigest", (algorithm))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetObjectIdentifier(
        mechanism: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Asn1::DerObjectIdentifier>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::DerObjectIdentifier,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetObjectIdentifier", (mechanism))?;
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
    pub fn get_Algorithms() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Collections::ICollection>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::ICollection,
        > = <Self as quest_hook::libil2cpp::Type>::class().invoke("get_Algorithms", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Org+BouncyCastle+Security+DigestUtilities")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Security::DigestUtilities {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "Org+BouncyCastle+Security+DigestUtilities+DigestAlgorithm")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum DigestUtilities_DigestAlgorithm {
    #[default]
    BLAKE2B_160 = 0i32,
    BLAKE2B_256 = 1i32,
    BLAKE2B_384 = 2i32,
    BLAKE2B_512 = 3i32,
    BLAKE2S_128 = 4i32,
    BLAKE2S_160 = 5i32,
    BLAKE2S_224 = 6i32,
    BLAKE2S_256 = 7i32,
    DSTU7564_256 = 8i32,
    DSTU7564_384 = 9i32,
    DSTU7564_512 = 10i32,
    GOST3411 = 11i32,
    GOST3411_2012_256 = 12i32,
    GOST3411_2012_512 = 13i32,
    KECCAK_224 = 14i32,
    KECCAK_256 = 15i32,
    KECCAK_288 = 16i32,
    KECCAK_384 = 17i32,
    KECCAK_512 = 18i32,
    MD2 = 19i32,
    MD4 = 20i32,
    MD5 = 21i32,
    NONE = 22i32,
    RIPEMD128 = 23i32,
    RIPEMD160 = 24i32,
    RIPEMD256 = 25i32,
    RIPEMD320 = 26i32,
    SHA3_224 = 34i32,
    SHA3_256 = 35i32,
    SHA3_384 = 36i32,
    SHA3_512 = 37i32,
    SHAKE128 = 38i32,
    SHAKE256 = 39i32,
    SHA_1 = 27i32,
    SHA_224 = 28i32,
    SHA_256 = 29i32,
    SHA_384 = 30i32,
    SHA_512 = 31i32,
    SHA_512_224 = 32i32,
    SHA_512_256 = 33i32,
    SM3 = 40i32,
    TIGER = 41i32,
    WHIRLPOOL = 42i32,
}
#[cfg(feature = "Org+BouncyCastle+Security+DigestUtilities+DigestAlgorithm")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::Org::BouncyCastle::Security::DigestUtilities_DigestAlgorithm =>
    "Org.BouncyCastle.Security"."DigestUtilities/DigestAlgorithm"
);
