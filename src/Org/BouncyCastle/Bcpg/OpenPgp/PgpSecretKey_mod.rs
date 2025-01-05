#[cfg(feature = "Org+BouncyCastle+Bcpg+OpenPgp+PgpSecretKey")]
#[repr(C)]
#[derive(Debug)]
pub struct PgpSecretKey {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub secret: quest_hook::libil2cpp::Gc<
        crate::Org::BouncyCastle::Bcpg::SecretKeyPacket,
    >,
    pub _cordl_pub: quest_hook::libil2cpp::Gc<
        crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpPublicKey,
    >,
}
#[cfg(feature = "Org+BouncyCastle+Bcpg+OpenPgp+PgpSecretKey")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Org::BouncyCastle::Bcpg::OpenPgp::PgpSecretKey
    => "Org.BouncyCastle.Bcpg.OpenPgp"."PgpSecretKey"
);
#[cfg(feature = "Org+BouncyCastle+Bcpg+OpenPgp+PgpSecretKey")]
impl std::ops::Deref for crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpSecretKey {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Bcpg+OpenPgp+PgpSecretKey")]
impl std::ops::DerefMut for crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpSecretKey {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Bcpg+OpenPgp+PgpSecretKey")]
impl crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpSecretKey {
    pub fn CertifiedPublicKey_HashAlgorithmTag1(
        certificationLevel: i32,
        keyPair: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpKeyPair,
        >,
        id: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        hashedPackets: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpSignatureSubpacketVector,
        >,
        unhashedPackets: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpSignatureSubpacketVector,
        >,
        hashAlgorithm: crate::Org::BouncyCastle::Bcpg::HashAlgorithmTag,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpPublicKey>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpPublicKey,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "CertifiedPublicKey",
                (
                    certificationLevel,
                    keyPair,
                    id,
                    hashedPackets,
                    unhashedPackets,
                    hashAlgorithm,
                ),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn CertifiedPublicKey_i32_PgpKeyPair_Il2CppString_PgpSignatureSubpacketVector_PgpSignatureSubpacketVector0(
        certificationLevel: i32,
        keyPair: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpKeyPair,
        >,
        id: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        hashedPackets: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpSignatureSubpacketVector,
        >,
        unhashedPackets: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpSignatureSubpacketVector,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpPublicKey>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpPublicKey,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "CertifiedPublicKey",
                (certificationLevel, keyPair, id, hashedPackets, unhashedPackets),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn Checksum(
        useSha1: bool,
        bytes: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        length: i32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<u8>,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Checksum", (useSha1, bytes, length))?;
        Ok(__cordl_ret.into())
    }
    pub fn CopyWithNewPassword(
        key: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpSecretKey,
        >,
        oldPassPhrase: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<char>,
        >,
        newPassPhrase: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<char>,
        >,
        newEncAlgorithm: crate::Org::BouncyCastle::Bcpg::SymmetricKeyAlgorithmTag,
        _cordl_rand: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Security::SecureRandom,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpSecretKey>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpSecretKey,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "CopyWithNewPassword",
                (key, oldPassPhrase, newPassPhrase, newEncAlgorithm, _cordl_rand),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn CopyWithNewPasswordRaw(
        key: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpSecretKey,
        >,
        rawOldPassPhrase: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<u8>,
        >,
        rawNewPassPhrase: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<u8>,
        >,
        newEncAlgorithm: crate::Org::BouncyCastle::Bcpg::SymmetricKeyAlgorithmTag,
        _cordl_rand: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Security::SecureRandom,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpSecretKey>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpSecretKey,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "CopyWithNewPasswordRaw",
                (key, rawOldPassPhrase, rawNewPassPhrase, newEncAlgorithm, _cordl_rand),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn CopyWithNewPasswordUtf8(
        key: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpSecretKey,
        >,
        oldPassPhrase: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<char>,
        >,
        newPassPhrase: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<char>,
        >,
        newEncAlgorithm: crate::Org::BouncyCastle::Bcpg::SymmetricKeyAlgorithmTag,
        _cordl_rand: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Security::SecureRandom,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpSecretKey>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpSecretKey,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "CopyWithNewPasswordUtf8",
                (key, oldPassPhrase, newPassPhrase, newEncAlgorithm, _cordl_rand),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn DoCopyWithNewPassword(
        key: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpSecretKey,
        >,
        rawOldPassPhrase: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<u8>,
        >,
        rawNewPassPhrase: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<u8>,
        >,
        clearPassPhrase: bool,
        newEncAlgorithm: crate::Org::BouncyCastle::Bcpg::SymmetricKeyAlgorithmTag,
        _cordl_rand: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Security::SecureRandom,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpSecretKey>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpSecretKey,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "DoCopyWithNewPassword",
                (
                    key,
                    rawOldPassPhrase,
                    rawNewPassPhrase,
                    clearPassPhrase,
                    newEncAlgorithm,
                    _cordl_rand,
                ),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn DoExtractPrivateKey(
        &mut self,
        rawPassPhrase: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        clearPassPhrase: bool,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpPrivateKey>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpPrivateKey,
        > = __cordl_object
            .invoke("DoExtractPrivateKey", (rawPassPhrase, clearPassPhrase))?;
        Ok(__cordl_ret.into())
    }
    pub fn DoParseSecretKeyFromSExpr_PgpPublicKey0(
        inputStream: quest_hook::libil2cpp::Gc<crate::System::IO::Stream>,
        rawPassPhrase: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        clearPassPhrase: bool,
        pubKey: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpPublicKey,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpSecretKey>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpSecretKey,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "DoParseSecretKeyFromSExpr",
                (inputStream, rawPassPhrase, clearPassPhrase, pubKey),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn DoParseSecretKeyFromSExpr_Stream_Il2CppArray__cordl_bool1(
        inputStream: quest_hook::libil2cpp::Gc<crate::System::IO::Stream>,
        rawPassPhrase: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        clearPassPhrase: bool,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpSecretKey>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpSecretKey,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "DoParseSecretKeyFromSExpr",
                (inputStream, rawPassPhrase, clearPassPhrase),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn Encode(
        &mut self,
        outStr: quest_hook::libil2cpp::Gc<crate::System::IO::Stream>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Encode", (outStr))?;
        Ok(__cordl_ret.into())
    }
    pub fn EncryptData(
        encAlgorithm: crate::Org::BouncyCastle::Bcpg::SymmetricKeyAlgorithmTag,
        key: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::Parameters::KeyParameter,
        >,
        data: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        dataOff: i32,
        dataLen: i32,
        random: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Security::SecureRandom,
        >,
        iv: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<u8>,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "EncryptData",
                (encAlgorithm, key, data, dataOff, dataLen, random, iv),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn EncryptKeyDataV3(
        rawKeyData: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        encAlgorithm: crate::Org::BouncyCastle::Bcpg::SymmetricKeyAlgorithmTag,
        rawPassPhrase: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        clearPassPhrase: bool,
        random: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Security::SecureRandom,
        >,
        s2k: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Bcpg::S2k>,
        >,
        iv: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<u8>,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "EncryptKeyDataV3",
                (
                    rawKeyData,
                    encAlgorithm,
                    rawPassPhrase,
                    clearPassPhrase,
                    random,
                    s2k,
                    iv,
                ),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn EncryptKeyDataV4(
        rawKeyData: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        encAlgorithm: crate::Org::BouncyCastle::Bcpg::SymmetricKeyAlgorithmTag,
        hashAlgorithm: crate::Org::BouncyCastle::Bcpg::HashAlgorithmTag,
        rawPassPhrase: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        clearPassPhrase: bool,
        random: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Security::SecureRandom,
        >,
        s2k: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Bcpg::S2k>,
        >,
        iv: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<u8>,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "EncryptKeyDataV4",
                (
                    rawKeyData,
                    encAlgorithm,
                    hashAlgorithm,
                    rawPassPhrase,
                    clearPassPhrase,
                    random,
                    s2k,
                    iv,
                ),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn ExtractKeyData(
        &mut self,
        rawPassPhrase: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        clearPassPhrase: bool,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<u8>,
        > = __cordl_object.invoke("ExtractKeyData", (rawPassPhrase, clearPassPhrase))?;
        Ok(__cordl_ret.into())
    }
    pub fn ExtractPrivateKey(
        &mut self,
        passPhrase: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<char>>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpPrivateKey>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpPrivateKey,
        > = __cordl_object.invoke("ExtractPrivateKey", (passPhrase))?;
        Ok(__cordl_ret.into())
    }
    pub fn ExtractPrivateKeyRaw(
        &mut self,
        rawPassPhrase: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpPrivateKey>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpPrivateKey,
        > = __cordl_object.invoke("ExtractPrivateKeyRaw", (rawPassPhrase))?;
        Ok(__cordl_ret.into())
    }
    pub fn ExtractPrivateKeyUtf8(
        &mut self,
        passPhrase: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<char>>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpPrivateKey>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpPrivateKey,
        > = __cordl_object.invoke("ExtractPrivateKeyUtf8", (passPhrase))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetDValue(
        inputStream: quest_hook::libil2cpp::Gc<crate::System::IO::Stream>,
        rawPassPhrase: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        clearPassPhrase: bool,
        curveName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<u8>,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "GetDValue",
                (inputStream, rawPassPhrase, clearPassPhrase, curveName),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn GetECKey(
        &mut self,
        algorithm: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        bcpgIn: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Bcpg::BcpgInputStream,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::Parameters::ECPrivateKeyParameters,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::Parameters::ECPrivateKeyParameters,
        > = __cordl_object.invoke("GetECKey", (algorithm, bcpgIn))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetEncoded(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<u8>,
        > = __cordl_object.invoke("GetEncoded", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn New_PgpPrivateKey_PgpPublicKey_SymmetricKeyAlgorithmTag_Il2CppArray__cordl_bool__cordl_bool_SecureRandom__cordl_bool1(
        privKey: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpPrivateKey,
        >,
        pubKey: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpPublicKey,
        >,
        encAlgorithm: crate::Org::BouncyCastle::Bcpg::SymmetricKeyAlgorithmTag,
        rawPassPhrase: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        clearPassPhrase: bool,
        useSha1: bool,
        _cordl_rand: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Security::SecureRandom,
        >,
        isMasterKey: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(
                ".ctor",
                (
                    privKey,
                    pubKey,
                    encAlgorithm,
                    rawPassPhrase,
                    clearPassPhrase,
                    useSha1,
                    _cordl_rand,
                    isMasterKey,
                ),
            )?;
        Ok(__cordl_object.into())
    }
    pub fn New_SecretKeyPacket_PgpPublicKey0(
        secret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Bcpg::SecretKeyPacket,
        >,
        _cordl_pub: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpPublicKey,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (secret, _cordl_pub))?;
        Ok(__cordl_object.into())
    }
    pub fn New_i32_PgpKeyPair_Il2CppString_SymmetricKeyAlgorithmTag_HashAlgorithmTag_Il2CppArray__cordl_bool_PgpSignatureSubpacketVector_PgpSignatureSubpacketVector_SecureRandom7(
        certificationLevel: i32,
        keyPair: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpKeyPair,
        >,
        id: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        encAlgorithm: crate::Org::BouncyCastle::Bcpg::SymmetricKeyAlgorithmTag,
        hashAlgorithm: crate::Org::BouncyCastle::Bcpg::HashAlgorithmTag,
        passPhrase: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<char>>,
        useSha1: bool,
        hashedPackets: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpSignatureSubpacketVector,
        >,
        unhashedPackets: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpSignatureSubpacketVector,
        >,
        _cordl_rand: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Security::SecureRandom,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(
                ".ctor",
                (
                    certificationLevel,
                    keyPair,
                    id,
                    encAlgorithm,
                    hashAlgorithm,
                    passPhrase,
                    useSha1,
                    hashedPackets,
                    unhashedPackets,
                    _cordl_rand,
                ),
            )?;
        Ok(__cordl_object.into())
    }
    pub fn New_i32_PgpKeyPair_Il2CppString_SymmetricKeyAlgorithmTag_HashAlgorithmTag_Il2CppArray__cordl_bool_PgpSignatureSubpacketVector_PgpSignatureSubpacketVector_SecureRandom9(
        certificationLevel: i32,
        keyPair: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpKeyPair,
        >,
        id: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        encAlgorithm: crate::Org::BouncyCastle::Bcpg::SymmetricKeyAlgorithmTag,
        hashAlgorithm: crate::Org::BouncyCastle::Bcpg::HashAlgorithmTag,
        rawPassPhrase: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        useSha1: bool,
        hashedPackets: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpSignatureSubpacketVector,
        >,
        unhashedPackets: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpSignatureSubpacketVector,
        >,
        _cordl_rand: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Security::SecureRandom,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(
                ".ctor",
                (
                    certificationLevel,
                    keyPair,
                    id,
                    encAlgorithm,
                    hashAlgorithm,
                    rawPassPhrase,
                    useSha1,
                    hashedPackets,
                    unhashedPackets,
                    _cordl_rand,
                ),
            )?;
        Ok(__cordl_object.into())
    }
    pub fn New_i32_PgpKeyPair_Il2CppString_SymmetricKeyAlgorithmTag_HashAlgorithmTag_Il2CppArray__cordl_bool__cordl_bool_PgpSignatureSubpacketVector_PgpSignatureSubpacketVector_SecureRandom10(
        certificationLevel: i32,
        keyPair: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpKeyPair,
        >,
        id: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        encAlgorithm: crate::Org::BouncyCastle::Bcpg::SymmetricKeyAlgorithmTag,
        hashAlgorithm: crate::Org::BouncyCastle::Bcpg::HashAlgorithmTag,
        rawPassPhrase: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        clearPassPhrase: bool,
        useSha1: bool,
        hashedPackets: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpSignatureSubpacketVector,
        >,
        unhashedPackets: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpSignatureSubpacketVector,
        >,
        _cordl_rand: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Security::SecureRandom,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(
                ".ctor",
                (
                    certificationLevel,
                    keyPair,
                    id,
                    encAlgorithm,
                    hashAlgorithm,
                    rawPassPhrase,
                    clearPassPhrase,
                    useSha1,
                    hashedPackets,
                    unhashedPackets,
                    _cordl_rand,
                ),
            )?;
        Ok(__cordl_object.into())
    }
    pub fn New_i32_PgpKeyPair_Il2CppString_SymmetricKeyAlgorithmTag_HashAlgorithmTag__cordl_bool_Il2CppArray__cordl_bool_PgpSignatureSubpacketVector_PgpSignatureSubpacketVector_SecureRandom8(
        certificationLevel: i32,
        keyPair: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpKeyPair,
        >,
        id: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        encAlgorithm: crate::Org::BouncyCastle::Bcpg::SymmetricKeyAlgorithmTag,
        hashAlgorithm: crate::Org::BouncyCastle::Bcpg::HashAlgorithmTag,
        utf8PassPhrase: bool,
        passPhrase: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<char>>,
        useSha1: bool,
        hashedPackets: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpSignatureSubpacketVector,
        >,
        unhashedPackets: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpSignatureSubpacketVector,
        >,
        _cordl_rand: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Security::SecureRandom,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(
                ".ctor",
                (
                    certificationLevel,
                    keyPair,
                    id,
                    encAlgorithm,
                    hashAlgorithm,
                    utf8PassPhrase,
                    passPhrase,
                    useSha1,
                    hashedPackets,
                    unhashedPackets,
                    _cordl_rand,
                ),
            )?;
        Ok(__cordl_object.into())
    }
    pub fn New_i32_PgpKeyPair_Il2CppString_SymmetricKeyAlgorithmTag_Il2CppArray_PgpSignatureSubpacketVector_PgpSignatureSubpacketVector_SecureRandom2(
        certificationLevel: i32,
        keyPair: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpKeyPair,
        >,
        id: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        encAlgorithm: crate::Org::BouncyCastle::Bcpg::SymmetricKeyAlgorithmTag,
        passPhrase: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<char>>,
        hashedPackets: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpSignatureSubpacketVector,
        >,
        unhashedPackets: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpSignatureSubpacketVector,
        >,
        _cordl_rand: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Security::SecureRandom,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(
                ".ctor",
                (
                    certificationLevel,
                    keyPair,
                    id,
                    encAlgorithm,
                    passPhrase,
                    hashedPackets,
                    unhashedPackets,
                    _cordl_rand,
                ),
            )?;
        Ok(__cordl_object.into())
    }
    pub fn New_i32_PgpKeyPair_Il2CppString_SymmetricKeyAlgorithmTag_Il2CppArray__cordl_bool_PgpSignatureSubpacketVector_PgpSignatureSubpacketVector_SecureRandom3(
        certificationLevel: i32,
        keyPair: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpKeyPair,
        >,
        id: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        encAlgorithm: crate::Org::BouncyCastle::Bcpg::SymmetricKeyAlgorithmTag,
        passPhrase: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<char>>,
        useSha1: bool,
        hashedPackets: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpSignatureSubpacketVector,
        >,
        unhashedPackets: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpSignatureSubpacketVector,
        >,
        _cordl_rand: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Security::SecureRandom,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(
                ".ctor",
                (
                    certificationLevel,
                    keyPair,
                    id,
                    encAlgorithm,
                    passPhrase,
                    useSha1,
                    hashedPackets,
                    unhashedPackets,
                    _cordl_rand,
                ),
            )?;
        Ok(__cordl_object.into())
    }
    pub fn New_i32_PgpKeyPair_Il2CppString_SymmetricKeyAlgorithmTag_Il2CppArray__cordl_bool_PgpSignatureSubpacketVector_PgpSignatureSubpacketVector_SecureRandom5(
        certificationLevel: i32,
        keyPair: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpKeyPair,
        >,
        id: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        encAlgorithm: crate::Org::BouncyCastle::Bcpg::SymmetricKeyAlgorithmTag,
        rawPassPhrase: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        useSha1: bool,
        hashedPackets: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpSignatureSubpacketVector,
        >,
        unhashedPackets: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpSignatureSubpacketVector,
        >,
        _cordl_rand: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Security::SecureRandom,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(
                ".ctor",
                (
                    certificationLevel,
                    keyPair,
                    id,
                    encAlgorithm,
                    rawPassPhrase,
                    useSha1,
                    hashedPackets,
                    unhashedPackets,
                    _cordl_rand,
                ),
            )?;
        Ok(__cordl_object.into())
    }
    pub fn New_i32_PgpKeyPair_Il2CppString_SymmetricKeyAlgorithmTag_Il2CppArray__cordl_bool__cordl_bool_PgpSignatureSubpacketVector_PgpSignatureSubpacketVector_SecureRandom6(
        certificationLevel: i32,
        keyPair: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpKeyPair,
        >,
        id: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        encAlgorithm: crate::Org::BouncyCastle::Bcpg::SymmetricKeyAlgorithmTag,
        rawPassPhrase: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        clearPassPhrase: bool,
        useSha1: bool,
        hashedPackets: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpSignatureSubpacketVector,
        >,
        unhashedPackets: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpSignatureSubpacketVector,
        >,
        _cordl_rand: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Security::SecureRandom,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(
                ".ctor",
                (
                    certificationLevel,
                    keyPair,
                    id,
                    encAlgorithm,
                    rawPassPhrase,
                    clearPassPhrase,
                    useSha1,
                    hashedPackets,
                    unhashedPackets,
                    _cordl_rand,
                ),
            )?;
        Ok(__cordl_object.into())
    }
    pub fn New_i32_PgpKeyPair_Il2CppString_SymmetricKeyAlgorithmTag__cordl_bool_Il2CppArray__cordl_bool_PgpSignatureSubpacketVector_PgpSignatureSubpacketVector_SecureRandom4(
        certificationLevel: i32,
        keyPair: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpKeyPair,
        >,
        id: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        encAlgorithm: crate::Org::BouncyCastle::Bcpg::SymmetricKeyAlgorithmTag,
        utf8PassPhrase: bool,
        passPhrase: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<char>>,
        useSha1: bool,
        hashedPackets: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpSignatureSubpacketVector,
        >,
        unhashedPackets: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpSignatureSubpacketVector,
        >,
        _cordl_rand: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Security::SecureRandom,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(
                ".ctor",
                (
                    certificationLevel,
                    keyPair,
                    id,
                    encAlgorithm,
                    utf8PassPhrase,
                    passPhrase,
                    useSha1,
                    hashedPackets,
                    unhashedPackets,
                    _cordl_rand,
                ),
            )?;
        Ok(__cordl_object.into())
    }
    pub fn New_i32_PublicKeyAlgorithmTag_AsymmetricKeyParameter_AsymmetricKeyParameter_DateTime_Il2CppString_SymmetricKeyAlgorithmTag_Il2CppArray_PgpSignatureSubpacketVector_PgpSignatureSubpacketVector_SecureRandom11(
        certificationLevel: i32,
        algorithm: crate::Org::BouncyCastle::Bcpg::PublicKeyAlgorithmTag,
        pubKey: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::AsymmetricKeyParameter,
        >,
        privKey: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::AsymmetricKeyParameter,
        >,
        _cordl_time: crate::System::DateTime,
        id: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        encAlgorithm: crate::Org::BouncyCastle::Bcpg::SymmetricKeyAlgorithmTag,
        passPhrase: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<char>>,
        hashedPackets: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpSignatureSubpacketVector,
        >,
        unhashedPackets: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpSignatureSubpacketVector,
        >,
        _cordl_rand: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Security::SecureRandom,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(
                ".ctor",
                (
                    certificationLevel,
                    algorithm,
                    pubKey,
                    privKey,
                    _cordl_time,
                    id,
                    encAlgorithm,
                    passPhrase,
                    hashedPackets,
                    unhashedPackets,
                    _cordl_rand,
                ),
            )?;
        Ok(__cordl_object.into())
    }
    pub fn New_i32_PublicKeyAlgorithmTag_AsymmetricKeyParameter_AsymmetricKeyParameter_DateTime_Il2CppString_SymmetricKeyAlgorithmTag_Il2CppArray__cordl_bool_PgpSignatureSubpacketVector_PgpSignatureSubpacketVector_SecureRandom12(
        certificationLevel: i32,
        algorithm: crate::Org::BouncyCastle::Bcpg::PublicKeyAlgorithmTag,
        pubKey: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::AsymmetricKeyParameter,
        >,
        privKey: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::AsymmetricKeyParameter,
        >,
        _cordl_time: crate::System::DateTime,
        id: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        encAlgorithm: crate::Org::BouncyCastle::Bcpg::SymmetricKeyAlgorithmTag,
        passPhrase: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<char>>,
        useSha1: bool,
        hashedPackets: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpSignatureSubpacketVector,
        >,
        unhashedPackets: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpSignatureSubpacketVector,
        >,
        _cordl_rand: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Security::SecureRandom,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(
                ".ctor",
                (
                    certificationLevel,
                    algorithm,
                    pubKey,
                    privKey,
                    _cordl_time,
                    id,
                    encAlgorithm,
                    passPhrase,
                    useSha1,
                    hashedPackets,
                    unhashedPackets,
                    _cordl_rand,
                ),
            )?;
        Ok(__cordl_object.into())
    }
    pub fn ParseSecretKeyFromSExprRaw_PgpPublicKey0(
        inputStream: quest_hook::libil2cpp::Gc<crate::System::IO::Stream>,
        rawPassPhrase: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        pubKey: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpPublicKey,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpSecretKey>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpSecretKey,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ParseSecretKeyFromSExprRaw", (inputStream, rawPassPhrase, pubKey))?;
        Ok(__cordl_ret.into())
    }
    pub fn ParseSecretKeyFromSExprRaw_Stream_Il2CppArray1(
        inputStream: quest_hook::libil2cpp::Gc<crate::System::IO::Stream>,
        rawPassPhrase: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpSecretKey>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpSecretKey,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ParseSecretKeyFromSExprRaw", (inputStream, rawPassPhrase))?;
        Ok(__cordl_ret.into())
    }
    pub fn ParseSecretKeyFromSExprUtf8_PgpPublicKey0(
        inputStream: quest_hook::libil2cpp::Gc<crate::System::IO::Stream>,
        passPhrase: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<char>>,
        pubKey: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpPublicKey,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpSecretKey>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpSecretKey,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ParseSecretKeyFromSExprUtf8", (inputStream, passPhrase, pubKey))?;
        Ok(__cordl_ret.into())
    }
    pub fn ParseSecretKeyFromSExprUtf8_Stream_Il2CppArray1(
        inputStream: quest_hook::libil2cpp::Gc<crate::System::IO::Stream>,
        passPhrase: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<char>>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpSecretKey>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpSecretKey,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ParseSecretKeyFromSExprUtf8", (inputStream, passPhrase))?;
        Ok(__cordl_ret.into())
    }
    pub fn ParseSecretKeyFromSExpr_PgpPublicKey0(
        inputStream: quest_hook::libil2cpp::Gc<crate::System::IO::Stream>,
        passPhrase: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<char>>,
        pubKey: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpPublicKey,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpSecretKey>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpSecretKey,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ParseSecretKeyFromSExpr", (inputStream, passPhrase, pubKey))?;
        Ok(__cordl_ret.into())
    }
    pub fn ParseSecretKeyFromSExpr_Stream_Il2CppArray1(
        inputStream: quest_hook::libil2cpp::Gc<crate::System::IO::Stream>,
        passPhrase: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<char>>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpSecretKey>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpSecretKey,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ParseSecretKeyFromSExpr", (inputStream, passPhrase))?;
        Ok(__cordl_ret.into())
    }
    pub fn RecoverKeyData(
        encAlgorithm: crate::Org::BouncyCastle::Bcpg::SymmetricKeyAlgorithmTag,
        modeAndPadding: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        key: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::Parameters::KeyParameter,
        >,
        iv: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        keyData: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        keyOff: i32,
        keyLen: i32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<u8>,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "RecoverKeyData",
                (encAlgorithm, modeAndPadding, key, iv, keyData, keyOff, keyLen),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn ReplacePublicKey(
        secretKey: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpSecretKey,
        >,
        publicKey: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpPublicKey,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpSecretKey>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpSecretKey,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ReplacePublicKey", (secretKey, publicKey))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_PgpPrivateKey_PgpPublicKey_SymmetricKeyAlgorithmTag_Il2CppArray__cordl_bool__cordl_bool_SecureRandom__cordl_bool1(
        &mut self,
        privKey: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpPrivateKey,
        >,
        pubKey: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpPublicKey,
        >,
        encAlgorithm: crate::Org::BouncyCastle::Bcpg::SymmetricKeyAlgorithmTag,
        rawPassPhrase: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        clearPassPhrase: bool,
        useSha1: bool,
        _cordl_rand: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Security::SecureRandom,
        >,
        isMasterKey: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                ".ctor",
                (
                    privKey,
                    pubKey,
                    encAlgorithm,
                    rawPassPhrase,
                    clearPassPhrase,
                    useSha1,
                    _cordl_rand,
                    isMasterKey,
                ),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_SecretKeyPacket_PgpPublicKey0(
        &mut self,
        secret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Bcpg::SecretKeyPacket,
        >,
        _cordl_pub: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpPublicKey,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (secret, _cordl_pub))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_i32_PgpKeyPair_Il2CppString_SymmetricKeyAlgorithmTag_HashAlgorithmTag_Il2CppArray__cordl_bool_PgpSignatureSubpacketVector_PgpSignatureSubpacketVector_SecureRandom7(
        &mut self,
        certificationLevel: i32,
        keyPair: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpKeyPair,
        >,
        id: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        encAlgorithm: crate::Org::BouncyCastle::Bcpg::SymmetricKeyAlgorithmTag,
        hashAlgorithm: crate::Org::BouncyCastle::Bcpg::HashAlgorithmTag,
        passPhrase: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<char>>,
        useSha1: bool,
        hashedPackets: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpSignatureSubpacketVector,
        >,
        unhashedPackets: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpSignatureSubpacketVector,
        >,
        _cordl_rand: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Security::SecureRandom,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                ".ctor",
                (
                    certificationLevel,
                    keyPair,
                    id,
                    encAlgorithm,
                    hashAlgorithm,
                    passPhrase,
                    useSha1,
                    hashedPackets,
                    unhashedPackets,
                    _cordl_rand,
                ),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_i32_PgpKeyPair_Il2CppString_SymmetricKeyAlgorithmTag_HashAlgorithmTag_Il2CppArray__cordl_bool_PgpSignatureSubpacketVector_PgpSignatureSubpacketVector_SecureRandom9(
        &mut self,
        certificationLevel: i32,
        keyPair: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpKeyPair,
        >,
        id: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        encAlgorithm: crate::Org::BouncyCastle::Bcpg::SymmetricKeyAlgorithmTag,
        hashAlgorithm: crate::Org::BouncyCastle::Bcpg::HashAlgorithmTag,
        rawPassPhrase: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        useSha1: bool,
        hashedPackets: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpSignatureSubpacketVector,
        >,
        unhashedPackets: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpSignatureSubpacketVector,
        >,
        _cordl_rand: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Security::SecureRandom,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                ".ctor",
                (
                    certificationLevel,
                    keyPair,
                    id,
                    encAlgorithm,
                    hashAlgorithm,
                    rawPassPhrase,
                    useSha1,
                    hashedPackets,
                    unhashedPackets,
                    _cordl_rand,
                ),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_i32_PgpKeyPair_Il2CppString_SymmetricKeyAlgorithmTag_HashAlgorithmTag_Il2CppArray__cordl_bool__cordl_bool_PgpSignatureSubpacketVector_PgpSignatureSubpacketVector_SecureRandom10(
        &mut self,
        certificationLevel: i32,
        keyPair: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpKeyPair,
        >,
        id: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        encAlgorithm: crate::Org::BouncyCastle::Bcpg::SymmetricKeyAlgorithmTag,
        hashAlgorithm: crate::Org::BouncyCastle::Bcpg::HashAlgorithmTag,
        rawPassPhrase: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        clearPassPhrase: bool,
        useSha1: bool,
        hashedPackets: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpSignatureSubpacketVector,
        >,
        unhashedPackets: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpSignatureSubpacketVector,
        >,
        _cordl_rand: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Security::SecureRandom,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                ".ctor",
                (
                    certificationLevel,
                    keyPair,
                    id,
                    encAlgorithm,
                    hashAlgorithm,
                    rawPassPhrase,
                    clearPassPhrase,
                    useSha1,
                    hashedPackets,
                    unhashedPackets,
                    _cordl_rand,
                ),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_i32_PgpKeyPair_Il2CppString_SymmetricKeyAlgorithmTag_HashAlgorithmTag__cordl_bool_Il2CppArray__cordl_bool_PgpSignatureSubpacketVector_PgpSignatureSubpacketVector_SecureRandom8(
        &mut self,
        certificationLevel: i32,
        keyPair: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpKeyPair,
        >,
        id: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        encAlgorithm: crate::Org::BouncyCastle::Bcpg::SymmetricKeyAlgorithmTag,
        hashAlgorithm: crate::Org::BouncyCastle::Bcpg::HashAlgorithmTag,
        utf8PassPhrase: bool,
        passPhrase: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<char>>,
        useSha1: bool,
        hashedPackets: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpSignatureSubpacketVector,
        >,
        unhashedPackets: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpSignatureSubpacketVector,
        >,
        _cordl_rand: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Security::SecureRandom,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                ".ctor",
                (
                    certificationLevel,
                    keyPair,
                    id,
                    encAlgorithm,
                    hashAlgorithm,
                    utf8PassPhrase,
                    passPhrase,
                    useSha1,
                    hashedPackets,
                    unhashedPackets,
                    _cordl_rand,
                ),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_i32_PgpKeyPair_Il2CppString_SymmetricKeyAlgorithmTag_Il2CppArray_PgpSignatureSubpacketVector_PgpSignatureSubpacketVector_SecureRandom2(
        &mut self,
        certificationLevel: i32,
        keyPair: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpKeyPair,
        >,
        id: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        encAlgorithm: crate::Org::BouncyCastle::Bcpg::SymmetricKeyAlgorithmTag,
        passPhrase: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<char>>,
        hashedPackets: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpSignatureSubpacketVector,
        >,
        unhashedPackets: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpSignatureSubpacketVector,
        >,
        _cordl_rand: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Security::SecureRandom,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                ".ctor",
                (
                    certificationLevel,
                    keyPair,
                    id,
                    encAlgorithm,
                    passPhrase,
                    hashedPackets,
                    unhashedPackets,
                    _cordl_rand,
                ),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_i32_PgpKeyPair_Il2CppString_SymmetricKeyAlgorithmTag_Il2CppArray__cordl_bool_PgpSignatureSubpacketVector_PgpSignatureSubpacketVector_SecureRandom3(
        &mut self,
        certificationLevel: i32,
        keyPair: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpKeyPair,
        >,
        id: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        encAlgorithm: crate::Org::BouncyCastle::Bcpg::SymmetricKeyAlgorithmTag,
        passPhrase: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<char>>,
        useSha1: bool,
        hashedPackets: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpSignatureSubpacketVector,
        >,
        unhashedPackets: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpSignatureSubpacketVector,
        >,
        _cordl_rand: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Security::SecureRandom,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                ".ctor",
                (
                    certificationLevel,
                    keyPair,
                    id,
                    encAlgorithm,
                    passPhrase,
                    useSha1,
                    hashedPackets,
                    unhashedPackets,
                    _cordl_rand,
                ),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_i32_PgpKeyPair_Il2CppString_SymmetricKeyAlgorithmTag_Il2CppArray__cordl_bool_PgpSignatureSubpacketVector_PgpSignatureSubpacketVector_SecureRandom5(
        &mut self,
        certificationLevel: i32,
        keyPair: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpKeyPair,
        >,
        id: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        encAlgorithm: crate::Org::BouncyCastle::Bcpg::SymmetricKeyAlgorithmTag,
        rawPassPhrase: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        useSha1: bool,
        hashedPackets: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpSignatureSubpacketVector,
        >,
        unhashedPackets: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpSignatureSubpacketVector,
        >,
        _cordl_rand: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Security::SecureRandom,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                ".ctor",
                (
                    certificationLevel,
                    keyPair,
                    id,
                    encAlgorithm,
                    rawPassPhrase,
                    useSha1,
                    hashedPackets,
                    unhashedPackets,
                    _cordl_rand,
                ),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_i32_PgpKeyPair_Il2CppString_SymmetricKeyAlgorithmTag_Il2CppArray__cordl_bool__cordl_bool_PgpSignatureSubpacketVector_PgpSignatureSubpacketVector_SecureRandom6(
        &mut self,
        certificationLevel: i32,
        keyPair: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpKeyPair,
        >,
        id: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        encAlgorithm: crate::Org::BouncyCastle::Bcpg::SymmetricKeyAlgorithmTag,
        rawPassPhrase: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        clearPassPhrase: bool,
        useSha1: bool,
        hashedPackets: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpSignatureSubpacketVector,
        >,
        unhashedPackets: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpSignatureSubpacketVector,
        >,
        _cordl_rand: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Security::SecureRandom,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                ".ctor",
                (
                    certificationLevel,
                    keyPair,
                    id,
                    encAlgorithm,
                    rawPassPhrase,
                    clearPassPhrase,
                    useSha1,
                    hashedPackets,
                    unhashedPackets,
                    _cordl_rand,
                ),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_i32_PgpKeyPair_Il2CppString_SymmetricKeyAlgorithmTag__cordl_bool_Il2CppArray__cordl_bool_PgpSignatureSubpacketVector_PgpSignatureSubpacketVector_SecureRandom4(
        &mut self,
        certificationLevel: i32,
        keyPair: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpKeyPair,
        >,
        id: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        encAlgorithm: crate::Org::BouncyCastle::Bcpg::SymmetricKeyAlgorithmTag,
        utf8PassPhrase: bool,
        passPhrase: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<char>>,
        useSha1: bool,
        hashedPackets: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpSignatureSubpacketVector,
        >,
        unhashedPackets: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpSignatureSubpacketVector,
        >,
        _cordl_rand: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Security::SecureRandom,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                ".ctor",
                (
                    certificationLevel,
                    keyPair,
                    id,
                    encAlgorithm,
                    utf8PassPhrase,
                    passPhrase,
                    useSha1,
                    hashedPackets,
                    unhashedPackets,
                    _cordl_rand,
                ),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_i32_PublicKeyAlgorithmTag_AsymmetricKeyParameter_AsymmetricKeyParameter_DateTime_Il2CppString_SymmetricKeyAlgorithmTag_Il2CppArray_PgpSignatureSubpacketVector_PgpSignatureSubpacketVector_SecureRandom11(
        &mut self,
        certificationLevel: i32,
        algorithm: crate::Org::BouncyCastle::Bcpg::PublicKeyAlgorithmTag,
        pubKey: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::AsymmetricKeyParameter,
        >,
        privKey: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::AsymmetricKeyParameter,
        >,
        _cordl_time: crate::System::DateTime,
        id: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        encAlgorithm: crate::Org::BouncyCastle::Bcpg::SymmetricKeyAlgorithmTag,
        passPhrase: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<char>>,
        hashedPackets: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpSignatureSubpacketVector,
        >,
        unhashedPackets: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpSignatureSubpacketVector,
        >,
        _cordl_rand: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Security::SecureRandom,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                ".ctor",
                (
                    certificationLevel,
                    algorithm,
                    pubKey,
                    privKey,
                    _cordl_time,
                    id,
                    encAlgorithm,
                    passPhrase,
                    hashedPackets,
                    unhashedPackets,
                    _cordl_rand,
                ),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_i32_PublicKeyAlgorithmTag_AsymmetricKeyParameter_AsymmetricKeyParameter_DateTime_Il2CppString_SymmetricKeyAlgorithmTag_Il2CppArray__cordl_bool_PgpSignatureSubpacketVector_PgpSignatureSubpacketVector_SecureRandom12(
        &mut self,
        certificationLevel: i32,
        algorithm: crate::Org::BouncyCastle::Bcpg::PublicKeyAlgorithmTag,
        pubKey: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::AsymmetricKeyParameter,
        >,
        privKey: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::AsymmetricKeyParameter,
        >,
        _cordl_time: crate::System::DateTime,
        id: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        encAlgorithm: crate::Org::BouncyCastle::Bcpg::SymmetricKeyAlgorithmTag,
        passPhrase: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<char>>,
        useSha1: bool,
        hashedPackets: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpSignatureSubpacketVector,
        >,
        unhashedPackets: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpSignatureSubpacketVector,
        >,
        _cordl_rand: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Security::SecureRandom,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                ".ctor",
                (
                    certificationLevel,
                    algorithm,
                    pubKey,
                    privKey,
                    _cordl_time,
                    id,
                    encAlgorithm,
                    passPhrase,
                    useSha1,
                    hashedPackets,
                    unhashedPackets,
                    _cordl_rand,
                ),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_IsMasterKey(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_IsMasterKey", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_IsPrivateKeyEmpty(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_IsPrivateKeyEmpty", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_IsSigningKey(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_IsSigningKey", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_KeyEncryptionAlgorithm(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::Org::BouncyCastle::Bcpg::SymmetricKeyAlgorithmTag,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::Org::BouncyCastle::Bcpg::SymmetricKeyAlgorithmTag = __cordl_object
            .invoke("get_KeyEncryptionAlgorithm", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_KeyId(&mut self) -> quest_hook::libil2cpp::Result<i64> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i64 = __cordl_object.invoke("get_KeyId", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_PublicKey(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpPublicKey>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpPublicKey,
        > = __cordl_object.invoke("get_PublicKey", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_S2k(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Bcpg::S2k>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Bcpg::S2k,
        > = __cordl_object.invoke("get_S2k", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_S2kUsage(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_S2kUsage", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_UserAttributes(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Collections::IEnumerable>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::IEnumerable,
        > = __cordl_object.invoke("get_UserAttributes", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_UserIds(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Collections::IEnumerable>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::IEnumerable,
        > = __cordl_object.invoke("get_UserIds", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Org+BouncyCastle+Bcpg+OpenPgp+PgpSecretKey")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpSecretKey {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
