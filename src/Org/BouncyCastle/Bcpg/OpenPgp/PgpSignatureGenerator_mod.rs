#[cfg(feature = "Org+BouncyCastle+Bcpg+OpenPgp+PgpSignatureGenerator")]
#[repr(C)]
#[derive(Debug)]
pub struct PgpSignatureGenerator {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub keyAlgorithm: crate::Org::BouncyCastle::Bcpg::PublicKeyAlgorithmTag,
    pub hashAlgorithm: crate::Org::BouncyCastle::Bcpg::HashAlgorithmTag,
    pub privKey: quest_hook::libil2cpp::Gc<
        crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpPrivateKey,
    >,
    pub sig: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Crypto::ISigner>,
    pub dig: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Crypto::IDigest>,
    pub signatureType: i32,
    pub lastb: u8,
    pub unhashed: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<
            *mut crate::Org::BouncyCastle::Bcpg::SignatureSubpacket,
        >,
    >,
    pub hashed: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<
            *mut crate::Org::BouncyCastle::Bcpg::SignatureSubpacket,
        >,
    >,
}
#[cfg(feature = "Org+BouncyCastle+Bcpg+OpenPgp+PgpSignatureGenerator")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::Org::BouncyCastle::Bcpg::OpenPgp::PgpSignatureGenerator =>
    "Org.BouncyCastle.Bcpg.OpenPgp"."PgpSignatureGenerator"
);
#[cfg(feature = "Org+BouncyCastle+Bcpg+OpenPgp+PgpSignatureGenerator")]
impl std::ops::Deref for crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpSignatureGenerator {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Bcpg+OpenPgp+PgpSignatureGenerator")]
impl std::ops::DerefMut
for crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpSignatureGenerator {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Bcpg+OpenPgp+PgpSignatureGenerator")]
impl crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpSignatureGenerator {
    pub fn Generate(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpSignature>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpSignature,
        > = __cordl_object.invoke("Generate", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GenerateCertification_Il2CppString_PgpPublicKey0(
        &mut self,
        id: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        pubKey: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpPublicKey,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpSignature>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpSignature,
        > = __cordl_object.invoke("GenerateCertification", (id, pubKey))?;
        Ok(__cordl_ret.into())
    }
    pub fn GenerateCertification_PgpPublicKey3(
        &mut self,
        pubKey: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpPublicKey,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpSignature>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpSignature,
        > = __cordl_object.invoke("GenerateCertification", (pubKey))?;
        Ok(__cordl_ret.into())
    }
    pub fn GenerateCertification_PgpPublicKey_PgpPublicKey2(
        &mut self,
        masterKey: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpPublicKey,
        >,
        pubKey: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpPublicKey,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpSignature>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpSignature,
        > = __cordl_object.invoke("GenerateCertification", (masterKey, pubKey))?;
        Ok(__cordl_ret.into())
    }
    pub fn GenerateCertification_PgpUserAttributeSubpacketVector_PgpPublicKey1(
        &mut self,
        userAttributes: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpUserAttributeSubpacketVector,
        >,
        pubKey: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpPublicKey,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpSignature>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpSignature,
        > = __cordl_object.invoke("GenerateCertification", (userAttributes, pubKey))?;
        Ok(__cordl_ret.into())
    }
    pub fn GenerateOnePassVersion(
        &mut self,
        isNested: bool,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpOnePassSignature,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpOnePassSignature,
        > = __cordl_object.invoke("GenerateOnePassVersion", (isNested))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetEncodedPublicKey(
        &mut self,
        pubKey: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpPublicKey,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<u8>,
        > = __cordl_object.invoke("GetEncodedPublicKey", (pubKey))?;
        Ok(__cordl_ret.into())
    }
    pub fn InitSign_SecureRandom1(
        &mut self,
        sigType: i32,
        key: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpPrivateKey,
        >,
        random: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Security::SecureRandom,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("InitSign", (sigType, key, random))?;
        Ok(__cordl_ret.into())
    }
    pub fn InitSign_i32_PgpPrivateKey0(
        &mut self,
        sigType: i32,
        key: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpPrivateKey,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("InitSign", (sigType, key))?;
        Ok(__cordl_ret.into())
    }
    pub fn New(
        keyAlgorithm: crate::Org::BouncyCastle::Bcpg::PublicKeyAlgorithmTag,
        hashAlgorithm: crate::Org::BouncyCastle::Bcpg::HashAlgorithmTag,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (keyAlgorithm, hashAlgorithm))?;
        Ok(__cordl_object.into())
    }
    pub fn SetHashedSubpackets(
        &mut self,
        hashedPackets: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpSignatureSubpacketVector,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetHashedSubpackets", (hashedPackets))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetUnhashedSubpackets(
        &mut self,
        unhashedPackets: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpSignatureSubpacketVector,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetUnhashedSubpackets", (unhashedPackets))?;
        Ok(__cordl_ret.into())
    }
    pub fn UpdateWithIdData(
        &mut self,
        header: i32,
        idBytes: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UpdateWithIdData", (header, idBytes))?;
        Ok(__cordl_ret.into())
    }
    pub fn UpdateWithPublicKey(
        &mut self,
        key: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpPublicKey,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UpdateWithPublicKey", (key))?;
        Ok(__cordl_ret.into())
    }
    pub fn Update_Il2CppArray1(
        &mut self,
        b: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Update", (b))?;
        Ok(__cordl_ret.into())
    }
    pub fn Update_Il2CppArray_i32_i32_2(
        &mut self,
        b: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        off: i32,
        len: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Update", (b, off, len))?;
        Ok(__cordl_ret.into())
    }
    pub fn Update_u8_0(
        &mut self,
        b: u8,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Update", (b))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        keyAlgorithm: crate::Org::BouncyCastle::Bcpg::PublicKeyAlgorithmTag,
        hashAlgorithm: crate::Org::BouncyCastle::Bcpg::HashAlgorithmTag,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (keyAlgorithm, hashAlgorithm))?;
        Ok(__cordl_ret.into())
    }
    pub fn doCanonicalUpdateByte(
        &mut self,
        b: u8,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("doCanonicalUpdateByte", (b))?;
        Ok(__cordl_ret.into())
    }
    pub fn doUpdateByte(
        &mut self,
        b: u8,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("doUpdateByte", (b))?;
        Ok(__cordl_ret.into())
    }
    pub fn doUpdateCRLF(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("doUpdateCRLF", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn insertSubpacket(
        &mut self,
        packets: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                *mut crate::Org::BouncyCastle::Bcpg::SignatureSubpacket,
            >,
        >,
        subpacket: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Bcpg::SignatureSubpacket,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                *mut crate::Org::BouncyCastle::Bcpg::SignatureSubpacket,
            >,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                *mut crate::Org::BouncyCastle::Bcpg::SignatureSubpacket,
            >,
        > = __cordl_object.invoke("insertSubpacket", (packets, subpacket))?;
        Ok(__cordl_ret.into())
    }
    pub fn packetPresent(
        &mut self,
        packets: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                *mut crate::Org::BouncyCastle::Bcpg::SignatureSubpacket,
            >,
        >,
        _cordl_type: crate::Org::BouncyCastle::Bcpg::SignatureSubpacketTag,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("packetPresent", (packets, _cordl_type))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Org+BouncyCastle+Bcpg+OpenPgp+PgpSignatureGenerator")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpSignatureGenerator {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
