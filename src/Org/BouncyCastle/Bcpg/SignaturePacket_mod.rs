#[cfg(feature = "Org+BouncyCastle+Bcpg+SignaturePacket")]
#[repr(C)]
#[derive(Debug)]
pub struct SignaturePacket {
    __cordl_parent: crate::Org::BouncyCastle::Bcpg::ContainedPacket,
    pub version: i32,
    pub signatureType: i32,
    pub creationTime: i64,
    pub keyId: i64,
    pub keyAlgorithm: crate::Org::BouncyCastle::Bcpg::PublicKeyAlgorithmTag,
    pub hashAlgorithm: crate::Org::BouncyCastle::Bcpg::HashAlgorithmTag,
    pub signature: *mut quest_hook::libil2cpp::Il2CppArray<
        *mut crate::Org::BouncyCastle::Bcpg::MPInteger,
    >,
    pub fingerprint: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
    pub hashedData: *mut quest_hook::libil2cpp::Il2CppArray<
        *mut crate::Org::BouncyCastle::Bcpg::SignatureSubpacket,
    >,
    pub unhashedData: *mut quest_hook::libil2cpp::Il2CppArray<
        *mut crate::Org::BouncyCastle::Bcpg::SignatureSubpacket,
    >,
    pub signatureEncoding: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
}
#[cfg(feature = "Org+BouncyCastle+Bcpg+SignaturePacket")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Org::BouncyCastle::Bcpg::SignaturePacket =>
    "Org.BouncyCastle.Bcpg"."SignaturePacket"
);
#[cfg(feature = "Org+BouncyCastle+Bcpg+SignaturePacket")]
impl std::ops::Deref for crate::Org::BouncyCastle::Bcpg::SignaturePacket {
    type Target = crate::Org::BouncyCastle::Bcpg::ContainedPacket;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Bcpg+SignaturePacket")]
impl std::ops::DerefMut for crate::Org::BouncyCastle::Bcpg::SignaturePacket {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Bcpg+SignaturePacket")]
impl crate::Org::BouncyCastle::Bcpg::SignaturePacket {
    pub fn get_CreationTime(&mut self) -> quest_hook::libil2cpp::Result<i64> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i64 = __cordl_object.invoke("get_CreationTime", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_Version(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_Version", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_KeyId(&mut self) -> quest_hook::libil2cpp::Result<i64> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i64 = __cordl_object.invoke("get_KeyId", ())?;
        Ok(__cordl_ret)
    }
    pub fn GetSignatureBytes(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut quest_hook::libil2cpp::Il2CppArray<u8>> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppArray<u8> = __cordl_object
            .invoke("GetSignatureBytes", ())?;
        Ok(__cordl_ret)
    }
    pub fn Encode(
        &mut self,
        bcpgOut: *mut crate::Org::BouncyCastle::Bcpg::BcpgOutputStream,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Encode", (bcpgOut))?;
        Ok(__cordl_ret)
    }
    pub fn GetSignature(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut quest_hook::libil2cpp::Il2CppArray<
            *mut crate::Org::BouncyCastle::Bcpg::MPInteger,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppArray<
            *mut crate::Org::BouncyCastle::Bcpg::MPInteger,
        > = __cordl_object.invoke("GetSignature", ())?;
        Ok(__cordl_ret)
    }
    pub fn GetSignatureTrailer(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut quest_hook::libil2cpp::Il2CppArray<u8>> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppArray<u8> = __cordl_object
            .invoke("GetSignatureTrailer", ())?;
        Ok(__cordl_ret)
    }
    pub fn GetUnhashedSubPackets(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut quest_hook::libil2cpp::Il2CppArray<
            *mut crate::Org::BouncyCastle::Bcpg::SignatureSubpacket,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppArray<
            *mut crate::Org::BouncyCastle::Bcpg::SignatureSubpacket,
        > = __cordl_object.invoke("GetUnhashedSubPackets", ())?;
        Ok(__cordl_ret)
    }
    pub fn GetHashedSubPackets(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut quest_hook::libil2cpp::Il2CppArray<
            *mut crate::Org::BouncyCastle::Bcpg::SignatureSubpacket,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppArray<
            *mut crate::Org::BouncyCastle::Bcpg::SignatureSubpacket,
        > = __cordl_object.invoke("GetHashedSubPackets", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_HashAlgorithm(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::Org::BouncyCastle::Bcpg::HashAlgorithmTag,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::Org::BouncyCastle::Bcpg::HashAlgorithmTag = __cordl_object
            .invoke("get_HashAlgorithm", ())?;
        Ok(__cordl_ret)
    }
    pub fn setCreationTime(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("setCreationTime", ())?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_BcpgInputStream0(
        &mut self,
        bcpgIn: *mut crate::Org::BouncyCastle::Bcpg::BcpgInputStream,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (bcpgIn))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_i32_i64_PublicKeyAlgorithmTag_HashAlgorithmTag_Il2CppArray_Il2CppArray_Il2CppArray_Il2CppArray1(
        &mut self,
        signatureType: i32,
        keyId: i64,
        keyAlgorithm: crate::Org::BouncyCastle::Bcpg::PublicKeyAlgorithmTag,
        hashAlgorithm: crate::Org::BouncyCastle::Bcpg::HashAlgorithmTag,
        hashedData: *mut quest_hook::libil2cpp::Il2CppArray<
            *mut crate::Org::BouncyCastle::Bcpg::SignatureSubpacket,
        >,
        unhashedData: *mut quest_hook::libil2cpp::Il2CppArray<
            *mut crate::Org::BouncyCastle::Bcpg::SignatureSubpacket,
        >,
        fingerprint: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
        signature: *mut quest_hook::libil2cpp::Il2CppArray<
            *mut crate::Org::BouncyCastle::Bcpg::MPInteger,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                ".ctor",
                (
                    signatureType,
                    keyId,
                    keyAlgorithm,
                    hashAlgorithm,
                    hashedData,
                    unhashedData,
                    fingerprint,
                    signature,
                ),
            )?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_i32_i32_i64_PublicKeyAlgorithmTag_HashAlgorithmTag_i64_Il2CppArray_Il2CppArray2(
        &mut self,
        version: i32,
        signatureType: i32,
        keyId: i64,
        keyAlgorithm: crate::Org::BouncyCastle::Bcpg::PublicKeyAlgorithmTag,
        hashAlgorithm: crate::Org::BouncyCastle::Bcpg::HashAlgorithmTag,
        creationTime: i64,
        fingerprint: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
        signature: *mut quest_hook::libil2cpp::Il2CppArray<
            *mut crate::Org::BouncyCastle::Bcpg::MPInteger,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                ".ctor",
                (
                    version,
                    signatureType,
                    keyId,
                    keyAlgorithm,
                    hashAlgorithm,
                    creationTime,
                    fingerprint,
                    signature,
                ),
            )?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_i32_i32_i64_PublicKeyAlgorithmTag_HashAlgorithmTag_Il2CppArray_Il2CppArray_Il2CppArray_Il2CppArray3(
        &mut self,
        version: i32,
        signatureType: i32,
        keyId: i64,
        keyAlgorithm: crate::Org::BouncyCastle::Bcpg::PublicKeyAlgorithmTag,
        hashAlgorithm: crate::Org::BouncyCastle::Bcpg::HashAlgorithmTag,
        hashedData: *mut quest_hook::libil2cpp::Il2CppArray<
            *mut crate::Org::BouncyCastle::Bcpg::SignatureSubpacket,
        >,
        unhashedData: *mut quest_hook::libil2cpp::Il2CppArray<
            *mut crate::Org::BouncyCastle::Bcpg::SignatureSubpacket,
        >,
        fingerprint: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
        signature: *mut quest_hook::libil2cpp::Il2CppArray<
            *mut crate::Org::BouncyCastle::Bcpg::MPInteger,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                ".ctor",
                (
                    version,
                    signatureType,
                    keyId,
                    keyAlgorithm,
                    hashAlgorithm,
                    hashedData,
                    unhashedData,
                    fingerprint,
                    signature,
                ),
            )?;
        Ok(__cordl_ret)
    }
    pub fn get_KeyAlgorithm(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::Org::BouncyCastle::Bcpg::PublicKeyAlgorithmTag,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::Org::BouncyCastle::Bcpg::PublicKeyAlgorithmTag = __cordl_object
            .invoke("get_KeyAlgorithm", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_SignatureType(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_SignatureType", ())?;
        Ok(__cordl_ret)
    }
    pub fn New_BcpgInputStream0(
        bcpgIn: *mut crate::Org::BouncyCastle::Bcpg::BcpgInputStream,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (bcpgIn))?;
        Ok(__cordl_object)
    }
    pub fn New_i32_i64_PublicKeyAlgorithmTag_HashAlgorithmTag_Il2CppArray_Il2CppArray_Il2CppArray_Il2CppArray1(
        signatureType: i32,
        keyId: i64,
        keyAlgorithm: crate::Org::BouncyCastle::Bcpg::PublicKeyAlgorithmTag,
        hashAlgorithm: crate::Org::BouncyCastle::Bcpg::HashAlgorithmTag,
        hashedData: *mut quest_hook::libil2cpp::Il2CppArray<
            *mut crate::Org::BouncyCastle::Bcpg::SignatureSubpacket,
        >,
        unhashedData: *mut quest_hook::libil2cpp::Il2CppArray<
            *mut crate::Org::BouncyCastle::Bcpg::SignatureSubpacket,
        >,
        fingerprint: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
        signature: *mut quest_hook::libil2cpp::Il2CppArray<
            *mut crate::Org::BouncyCastle::Bcpg::MPInteger,
        >,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(
                ".ctor",
                (
                    signatureType,
                    keyId,
                    keyAlgorithm,
                    hashAlgorithm,
                    hashedData,
                    unhashedData,
                    fingerprint,
                    signature,
                ),
            )?;
        Ok(__cordl_object)
    }
    pub fn New_i32_i32_i64_PublicKeyAlgorithmTag_HashAlgorithmTag_i64_Il2CppArray_Il2CppArray2(
        version: i32,
        signatureType: i32,
        keyId: i64,
        keyAlgorithm: crate::Org::BouncyCastle::Bcpg::PublicKeyAlgorithmTag,
        hashAlgorithm: crate::Org::BouncyCastle::Bcpg::HashAlgorithmTag,
        creationTime: i64,
        fingerprint: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
        signature: *mut quest_hook::libil2cpp::Il2CppArray<
            *mut crate::Org::BouncyCastle::Bcpg::MPInteger,
        >,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(
                ".ctor",
                (
                    version,
                    signatureType,
                    keyId,
                    keyAlgorithm,
                    hashAlgorithm,
                    creationTime,
                    fingerprint,
                    signature,
                ),
            )?;
        Ok(__cordl_object)
    }
    pub fn New_i32_i32_i64_PublicKeyAlgorithmTag_HashAlgorithmTag_Il2CppArray_Il2CppArray_Il2CppArray_Il2CppArray3(
        version: i32,
        signatureType: i32,
        keyId: i64,
        keyAlgorithm: crate::Org::BouncyCastle::Bcpg::PublicKeyAlgorithmTag,
        hashAlgorithm: crate::Org::BouncyCastle::Bcpg::HashAlgorithmTag,
        hashedData: *mut quest_hook::libil2cpp::Il2CppArray<
            *mut crate::Org::BouncyCastle::Bcpg::SignatureSubpacket,
        >,
        unhashedData: *mut quest_hook::libil2cpp::Il2CppArray<
            *mut crate::Org::BouncyCastle::Bcpg::SignatureSubpacket,
        >,
        fingerprint: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
        signature: *mut quest_hook::libil2cpp::Il2CppArray<
            *mut crate::Org::BouncyCastle::Bcpg::MPInteger,
        >,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(
                ".ctor",
                (
                    version,
                    signatureType,
                    keyId,
                    keyAlgorithm,
                    hashAlgorithm,
                    hashedData,
                    unhashedData,
                    fingerprint,
                    signature,
                ),
            )?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "Org+BouncyCastle+Bcpg+SignaturePacket")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Bcpg::SignaturePacket {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
