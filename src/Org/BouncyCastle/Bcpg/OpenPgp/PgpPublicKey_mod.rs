#[cfg(feature = "Org+BouncyCastle+Bcpg+OpenPgp+PgpPublicKey")]
#[repr(C)]
#[derive(Debug)]
pub struct PgpPublicKey {
    __cordl_parent: crate::System::Object,
    pub keyId: i64,
    pub fingerprint: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
    pub keyStrength: i32,
    pub publicPk: *mut crate::Org::BouncyCastle::Bcpg::PublicKeyPacket,
    pub trustPk: *mut crate::Org::BouncyCastle::Bcpg::TrustPacket,
    pub keySigs: *mut crate::System::Collections::IList,
    pub ids: *mut crate::System::Collections::IList,
    pub idTrusts: *mut crate::System::Collections::IList,
    pub idSigs: *mut crate::System::Collections::IList,
    pub subSigs: *mut crate::System::Collections::IList,
}
#[cfg(feature = "Org+BouncyCastle+Bcpg+OpenPgp+PgpPublicKey")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Org::BouncyCastle::Bcpg::OpenPgp::PgpPublicKey
    => "Org.BouncyCastle.Bcpg.OpenPgp"."PgpPublicKey"
);
#[cfg(feature = "Org+BouncyCastle+Bcpg+OpenPgp+PgpPublicKey")]
impl std::ops::Deref for crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpPublicKey {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Bcpg+OpenPgp+PgpPublicKey")]
impl std::ops::DerefMut for crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpPublicKey {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Bcpg+OpenPgp+PgpPublicKey")]
impl crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpPublicKey {
    pub fn Encode(
        &mut self,
        outStr: *mut crate::System::IO::Stream,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Encode", (outStr))?;
        Ok(__cordl_ret)
    }
    pub fn GetECKey(
        &mut self,
        algorithm: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Crypto::Parameters::ECPublicKeyParameters,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Crypto::Parameters::ECPublicKeyParameters = __cordl_object
            .invoke("GetECKey", (algorithm))?;
        Ok(__cordl_ret)
    }
    pub fn GetEncoded(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut quest_hook::libil2cpp::Il2CppArray<u8>> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppArray<u8> = __cordl_object
            .invoke("GetEncoded", ())?;
        Ok(__cordl_ret)
    }
    pub fn GetExpirationTimeFromSig(
        &mut self,
        selfSigned: bool,
        signatureType: i32,
    ) -> quest_hook::libil2cpp::Result<i64> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i64 = __cordl_object
            .invoke("GetExpirationTimeFromSig", (selfSigned, signatureType))?;
        Ok(__cordl_ret)
    }
    pub fn GetFingerprint(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut quest_hook::libil2cpp::Il2CppArray<u8>> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppArray<u8> = __cordl_object
            .invoke("GetFingerprint", ())?;
        Ok(__cordl_ret)
    }
    pub fn GetKey(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Crypto::AsymmetricKeyParameter,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Crypto::AsymmetricKeyParameter = __cordl_object
            .invoke("GetKey", ())?;
        Ok(__cordl_ret)
    }
    pub fn GetKeySignatures(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Collections::IEnumerable> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Collections::IEnumerable = __cordl_object
            .invoke("GetKeySignatures", ())?;
        Ok(__cordl_ret)
    }
    pub fn GetSignatures(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Collections::IEnumerable> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Collections::IEnumerable = __cordl_object
            .invoke("GetSignatures", ())?;
        Ok(__cordl_ret)
    }
    pub fn GetSignaturesForId(
        &mut self,
        id: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Collections::IEnumerable> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Collections::IEnumerable = __cordl_object
            .invoke("GetSignaturesForId", (id))?;
        Ok(__cordl_ret)
    }
    pub fn GetSignaturesForUserAttribute(
        &mut self,
        userAttributes: *mut crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpUserAttributeSubpacketVector,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Collections::IEnumerable> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Collections::IEnumerable = __cordl_object
            .invoke("GetSignaturesForUserAttribute", (userAttributes))?;
        Ok(__cordl_ret)
    }
    pub fn GetSignaturesOfType(
        &mut self,
        signatureType: i32,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Collections::IEnumerable> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Collections::IEnumerable = __cordl_object
            .invoke("GetSignaturesOfType", (signatureType))?;
        Ok(__cordl_ret)
    }
    pub fn GetTrustData(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut quest_hook::libil2cpp::Il2CppArray<u8>> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppArray<u8> = __cordl_object
            .invoke("GetTrustData", ())?;
        Ok(__cordl_ret)
    }
    pub fn GetUserAttributes(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Collections::IEnumerable> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Collections::IEnumerable = __cordl_object
            .invoke("GetUserAttributes", ())?;
        Ok(__cordl_ret)
    }
    pub fn GetUserIds(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Collections::IEnumerable> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Collections::IEnumerable = __cordl_object
            .invoke("GetUserIds", ())?;
        Ok(__cordl_ret)
    }
    pub fn GetValidSeconds(&mut self) -> quest_hook::libil2cpp::Result<i64> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i64 = __cordl_object.invoke("GetValidSeconds", ())?;
        Ok(__cordl_ret)
    }
    pub fn Init(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Init", ())?;
        Ok(__cordl_ret)
    }
    pub fn IsRevoked(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("IsRevoked", ())?;
        Ok(__cordl_ret)
    }
    pub fn New_PgpPublicKey4(
        pubKey: *mut crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpPublicKey,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (pubKey))?;
        Ok(__cordl_object)
    }
    pub fn New_PgpPublicKey_TrustPacket_IList3(
        key: *mut crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpPublicKey,
        trust: *mut crate::Org::BouncyCastle::Bcpg::TrustPacket,
        subSigs: *mut crate::System::Collections::IList,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (key, trust, subSigs))?;
        Ok(__cordl_object)
    }
    pub fn New_PublicKeyAlgorithmTag_AsymmetricKeyParameter_DateTime0(
        algorithm: crate::Org::BouncyCastle::Bcpg::PublicKeyAlgorithmTag,
        pubKey: *mut crate::Org::BouncyCastle::Crypto::AsymmetricKeyParameter,
        _cordl_time: crate::System::DateTime,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (algorithm, pubKey, _cordl_time))?;
        Ok(__cordl_object)
    }
    pub fn New_PublicKeyPacket1(
        publicPk: *mut crate::Org::BouncyCastle::Bcpg::PublicKeyPacket,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (publicPk))?;
        Ok(__cordl_object)
    }
    pub fn New_PublicKeyPacket_IList_IList6(
        publicPk: *mut crate::Org::BouncyCastle::Bcpg::PublicKeyPacket,
        ids: *mut crate::System::Collections::IList,
        idSigs: *mut crate::System::Collections::IList,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (publicPk, ids, idSigs))?;
        Ok(__cordl_object)
    }
    pub fn New_PublicKeyPacket_TrustPacket_IList2(
        publicPk: *mut crate::Org::BouncyCastle::Bcpg::PublicKeyPacket,
        trustPk: *mut crate::Org::BouncyCastle::Bcpg::TrustPacket,
        sigs: *mut crate::System::Collections::IList,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (publicPk, trustPk, sigs))?;
        Ok(__cordl_object)
    }
    pub fn New_PublicKeyPacket_TrustPacket_IList_IList_IList_IList5(
        publicPk: *mut crate::Org::BouncyCastle::Bcpg::PublicKeyPacket,
        trustPk: *mut crate::Org::BouncyCastle::Bcpg::TrustPacket,
        keySigs: *mut crate::System::Collections::IList,
        ids: *mut crate::System::Collections::IList,
        idTrusts: *mut crate::System::Collections::IList,
        idSigs: *mut crate::System::Collections::IList,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (publicPk, trustPk, keySigs, ids, idTrusts, idSigs))?;
        Ok(__cordl_object)
    }
    pub fn _ctor_PgpPublicKey4(
        &mut self,
        pubKey: *mut crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpPublicKey,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (pubKey))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_PgpPublicKey_TrustPacket_IList3(
        &mut self,
        key: *mut crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpPublicKey,
        trust: *mut crate::Org::BouncyCastle::Bcpg::TrustPacket,
        subSigs: *mut crate::System::Collections::IList,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (key, trust, subSigs))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_PublicKeyAlgorithmTag_AsymmetricKeyParameter_DateTime0(
        &mut self,
        algorithm: crate::Org::BouncyCastle::Bcpg::PublicKeyAlgorithmTag,
        pubKey: *mut crate::Org::BouncyCastle::Crypto::AsymmetricKeyParameter,
        _cordl_time: crate::System::DateTime,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (algorithm, pubKey, _cordl_time))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_PublicKeyPacket1(
        &mut self,
        publicPk: *mut crate::Org::BouncyCastle::Bcpg::PublicKeyPacket,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (publicPk))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_PublicKeyPacket_IList_IList6(
        &mut self,
        publicPk: *mut crate::Org::BouncyCastle::Bcpg::PublicKeyPacket,
        ids: *mut crate::System::Collections::IList,
        idSigs: *mut crate::System::Collections::IList,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (publicPk, ids, idSigs))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_PublicKeyPacket_TrustPacket_IList2(
        &mut self,
        publicPk: *mut crate::Org::BouncyCastle::Bcpg::PublicKeyPacket,
        trustPk: *mut crate::Org::BouncyCastle::Bcpg::TrustPacket,
        sigs: *mut crate::System::Collections::IList,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (publicPk, trustPk, sigs))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_PublicKeyPacket_TrustPacket_IList_IList_IList_IList5(
        &mut self,
        publicPk: *mut crate::Org::BouncyCastle::Bcpg::PublicKeyPacket,
        trustPk: *mut crate::Org::BouncyCastle::Bcpg::TrustPacket,
        keySigs: *mut crate::System::Collections::IList,
        ids: *mut crate::System::Collections::IList,
        idTrusts: *mut crate::System::Collections::IList,
        idSigs: *mut crate::System::Collections::IList,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (publicPk, trustPk, keySigs, ids, idTrusts, idSigs))?;
        Ok(__cordl_ret)
    }
    pub fn get_Algorithm(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::Org::BouncyCastle::Bcpg::PublicKeyAlgorithmTag,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::Org::BouncyCastle::Bcpg::PublicKeyAlgorithmTag = __cordl_object
            .invoke("get_Algorithm", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_BitStrength(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_BitStrength", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_CreationTime(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::System::DateTime> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::System::DateTime = __cordl_object
            .invoke("get_CreationTime", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_IsEncryptionKey(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_IsEncryptionKey", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_IsMasterKey(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_IsMasterKey", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_KeyId(&mut self) -> quest_hook::libil2cpp::Result<i64> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i64 = __cordl_object.invoke("get_KeyId", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_PublicKeyPacket(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Bcpg::PublicKeyPacket,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Bcpg::PublicKeyPacket = __cordl_object
            .invoke("get_PublicKeyPacket", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_ValidDays(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_ValidDays", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_Version(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_Version", ())?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "Org+BouncyCastle+Bcpg+OpenPgp+PgpPublicKey")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpPublicKey {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}