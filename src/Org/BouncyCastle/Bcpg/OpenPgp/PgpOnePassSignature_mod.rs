#[cfg(feature = "Org+BouncyCastle+Bcpg+OpenPgp+PgpOnePassSignature")]
#[repr(C)]
#[derive(Debug)]
pub struct PgpOnePassSignature {
    __cordl_parent: crate::System::Object,
    pub sigPack: *mut crate::Org::BouncyCastle::Bcpg::OnePassSignaturePacket,
    pub signatureType: i32,
    pub sig: *mut crate::Org::BouncyCastle::Crypto::ISigner,
    pub lastb: u8,
}
#[cfg(feature = "Org+BouncyCastle+Bcpg+OpenPgp+PgpOnePassSignature")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::Org::BouncyCastle::Bcpg::OpenPgp::PgpOnePassSignature =>
    "Org.BouncyCastle.Bcpg.OpenPgp"."PgpOnePassSignature"
);
#[cfg(feature = "Org+BouncyCastle+Bcpg+OpenPgp+PgpOnePassSignature")]
impl std::ops::Deref for crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpOnePassSignature {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Bcpg+OpenPgp+PgpOnePassSignature")]
impl std::ops::DerefMut
for crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpOnePassSignature {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Bcpg+OpenPgp+PgpOnePassSignature")]
impl crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpOnePassSignature {
    pub fn doUpdateCRLF(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("doUpdateCRLF", ())?;
        Ok(__cordl_ret)
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
        Ok(__cordl_ret)
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
        Ok(__cordl_ret)
    }
    pub fn Update_Il2CppArray1(
        &mut self,
        bytes: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Update", (bytes))?;
        Ok(__cordl_ret)
    }
    pub fn Update_Il2CppArray_i32_i32_2(
        &mut self,
        bytes: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
        off: i32,
        length: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Update", (bytes, off, length))?;
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
    pub fn get_KeyId(&mut self) -> quest_hook::libil2cpp::Result<i64> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i64 = __cordl_object.invoke("get_KeyId", ())?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_BcpgInputStream0(
        &mut self,
        bcpgInput: *mut crate::Org::BouncyCastle::Bcpg::BcpgInputStream,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (bcpgInput))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_OnePassSignaturePacket1(
        &mut self,
        sigPack: *mut crate::Org::BouncyCastle::Bcpg::OnePassSignaturePacket,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (sigPack))?;
        Ok(__cordl_ret)
    }
    pub fn InitVerify(
        &mut self,
        pubKey: *mut crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpPublicKey,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("InitVerify", (pubKey))?;
        Ok(__cordl_ret)
    }
    pub fn get_SignatureType(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_SignatureType", ())?;
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
    pub fn Verify(
        &mut self,
        pgpSig: *mut crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpSignature,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("Verify", (pgpSig))?;
        Ok(__cordl_ret)
    }
    pub fn New_BcpgInputStream0(
        bcpgInput: *mut crate::Org::BouncyCastle::Bcpg::BcpgInputStream,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (bcpgInput))?;
        Ok(__cordl_object)
    }
    pub fn New_OnePassSignaturePacket1(
        sigPack: *mut crate::Org::BouncyCastle::Bcpg::OnePassSignaturePacket,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (sigPack))?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "Org+BouncyCastle+Bcpg+OpenPgp+PgpOnePassSignature")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpOnePassSignature {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
