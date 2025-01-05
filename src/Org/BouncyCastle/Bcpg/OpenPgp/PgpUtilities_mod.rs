#[cfg(feature = "Org+BouncyCastle+Bcpg+OpenPgp+PgpUtilities")]
#[repr(C)]
#[derive(Debug)]
pub struct PgpUtilities {
    __cordl_parent: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
}
#[cfg(feature = "Org+BouncyCastle+Bcpg+OpenPgp+PgpUtilities")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Org::BouncyCastle::Bcpg::OpenPgp::PgpUtilities
    => "Org.BouncyCastle.Bcpg.OpenPgp"."PgpUtilities"
);
#[cfg(feature = "Org+BouncyCastle+Bcpg+OpenPgp+PgpUtilities")]
impl std::ops::Deref for crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpUtilities {
    type Target = quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Bcpg+OpenPgp+PgpUtilities")]
impl std::ops::DerefMut for crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpUtilities {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Bcpg+OpenPgp+PgpUtilities")]
impl crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpUtilities {
    pub const ReadAhead: i32 = 60i32;
    pub fn CreateWrapper(
        encAlgorithm: crate::Org::BouncyCastle::Bcpg::SymmetricKeyAlgorithmTag,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Crypto::IWrapper>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::IWrapper,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CreateWrapper", (encAlgorithm))?;
        Ok(__cordl_ret.into())
    }
    pub fn DoMakeKeyFromPassPhrase(
        algorithm: crate::Org::BouncyCastle::Bcpg::SymmetricKeyAlgorithmTag,
        s2k: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Bcpg::S2k>,
        rawPassPhrase: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        clearPassPhrase: bool,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::Parameters::KeyParameter,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::Parameters::KeyParameter,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "DoMakeKeyFromPassPhrase",
                (algorithm, s2k, rawPassPhrase, clearPassPhrase),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn DsaSigToMpi(
        encoding: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Bcpg::MPInteger>,
            >,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Bcpg::MPInteger>,
            >,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("DsaSigToMpi", (encoding))?;
        Ok(__cordl_ret.into())
    }
    pub fn EncodePassPhrase(
        passPhrase: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<char>>,
        utf8: bool,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<u8>,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("EncodePassPhrase", (passPhrase, utf8))?;
        Ok(__cordl_ret.into())
    }
    pub fn GenerateIV(
        length: i32,
        random: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Security::SecureRandom,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<u8>,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GenerateIV", (length, random))?;
        Ok(__cordl_ret.into())
    }
    pub fn GenerateS2k(
        hashAlgorithm: crate::Org::BouncyCastle::Bcpg::HashAlgorithmTag,
        s2kCount: i32,
        random: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Security::SecureRandom,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Bcpg::S2k>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Bcpg::S2k,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GenerateS2k", (hashAlgorithm, s2kCount, random))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetDecoderStream(
        inputStream: quest_hook::libil2cpp::Gc<crate::System::IO::Stream>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::IO::Stream>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::IO::Stream> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetDecoderStream", (inputStream))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetDigestName(
        hashAlgorithm: crate::Org::BouncyCastle::Bcpg::HashAlgorithmTag,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetDigestName", (hashAlgorithm))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetKeySize(
        algorithm: crate::Org::BouncyCastle::Bcpg::SymmetricKeyAlgorithmTag,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetKeySize", (algorithm))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetSignatureName(
        keyAlgorithm: crate::Org::BouncyCastle::Bcpg::PublicKeyAlgorithmTag,
        hashAlgorithm: crate::Org::BouncyCastle::Bcpg::HashAlgorithmTag,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetSignatureName", (keyAlgorithm, hashAlgorithm))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetSymmetricCipherName(
        algorithm: crate::Org::BouncyCastle::Bcpg::SymmetricKeyAlgorithmTag,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetSymmetricCipherName", (algorithm))?;
        Ok(__cordl_ret.into())
    }
    pub fn IsPossiblyBase64(ch: i32) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("IsPossiblyBase64", (ch))?;
        Ok(__cordl_ret.into())
    }
    pub fn MakeKey(
        algorithm: crate::Org::BouncyCastle::Bcpg::SymmetricKeyAlgorithmTag,
        keyBytes: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::Parameters::KeyParameter,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::Parameters::KeyParameter,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("MakeKey", (algorithm, keyBytes))?;
        Ok(__cordl_ret.into())
    }
    pub fn MakeKeyFromPassPhrase(
        algorithm: crate::Org::BouncyCastle::Bcpg::SymmetricKeyAlgorithmTag,
        s2k: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Bcpg::S2k>,
        passPhrase: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<char>>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::Parameters::KeyParameter,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::Parameters::KeyParameter,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("MakeKeyFromPassPhrase", (algorithm, s2k, passPhrase))?;
        Ok(__cordl_ret.into())
    }
    pub fn MakeKeyFromPassPhraseRaw(
        algorithm: crate::Org::BouncyCastle::Bcpg::SymmetricKeyAlgorithmTag,
        s2k: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Bcpg::S2k>,
        rawPassPhrase: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::Parameters::KeyParameter,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::Parameters::KeyParameter,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("MakeKeyFromPassPhraseRaw", (algorithm, s2k, rawPassPhrase))?;
        Ok(__cordl_ret.into())
    }
    pub fn MakeKeyFromPassPhraseUtf8(
        algorithm: crate::Org::BouncyCastle::Bcpg::SymmetricKeyAlgorithmTag,
        s2k: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Bcpg::S2k>,
        passPhrase: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<char>>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::Parameters::KeyParameter,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::Parameters::KeyParameter,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("MakeKeyFromPassPhraseUtf8", (algorithm, s2k, passPhrase))?;
        Ok(__cordl_ret.into())
    }
    pub fn MakeRandomKey(
        algorithm: crate::Org::BouncyCastle::Bcpg::SymmetricKeyAlgorithmTag,
        random: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Security::SecureRandom,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::Parameters::KeyParameter,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::Parameters::KeyParameter,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("MakeRandomKey", (algorithm, random))?;
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn PipeFileContents(
        file: quest_hook::libil2cpp::Gc<crate::System::IO::FileInfo>,
        pOut: quest_hook::libil2cpp::Gc<crate::System::IO::Stream>,
        bufSize: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("PipeFileContents", (file, pOut, bufSize))?;
        Ok(__cordl_ret.into())
    }
    pub fn RsaSigToMpi(
        encoding: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Bcpg::MPInteger>,
            >,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Bcpg::MPInteger>,
            >,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("RsaSigToMpi", (encoding))?;
        Ok(__cordl_ret.into())
    }
    pub fn WriteFileToLiteralData_Gc1(
        output: quest_hook::libil2cpp::Gc<crate::System::IO::Stream>,
        fileType: char,
        file: quest_hook::libil2cpp::Gc<crate::System::IO::FileInfo>,
        buffer: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("WriteFileToLiteralData", (output, fileType, file, buffer))?;
        Ok(__cordl_ret.into())
    }
    pub fn WriteFileToLiteralData_Gc__cordl_char_Gc0(
        output: quest_hook::libil2cpp::Gc<crate::System::IO::Stream>,
        fileType: char,
        file: quest_hook::libil2cpp::Gc<crate::System::IO::FileInfo>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("WriteFileToLiteralData", (output, fileType, file))?;
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
#[cfg(feature = "Org+BouncyCastle+Bcpg+OpenPgp+PgpUtilities")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpUtilities {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
