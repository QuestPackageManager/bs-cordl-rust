#[cfg(feature = "Org+BouncyCastle+Bcpg+OpenPgp+PgpOnePassSignature")]
#[repr(C)]
#[derive(Debug)]
pub struct PgpOnePassSignature {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub sigPack: quest_hook::libil2cpp::Gc<
        crate::Org::BouncyCastle::Bcpg::OnePassSignaturePacket,
    >,
    pub signatureType: i32,
    pub sig: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Crypto::ISigner>,
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
    type Target = quest_hook::libil2cpp::Il2CppObject;
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
    pub fn Cast(
        packet: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Bcpg::Packet>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Bcpg::OnePassSignaturePacket>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Bcpg::OnePassSignaturePacket,
        > = <Self as quest_hook::libil2cpp::Type>::class().invoke("Cast", (packet))?;
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
    pub fn InitVerify(
        &mut self,
        pubKey: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpPublicKey,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("InitVerify", (pubKey))?;
        Ok(__cordl_ret.into())
    }
    pub fn New_BcpgInputStream0(
        bcpgInput: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Bcpg::BcpgInputStream,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (bcpgInput))?;
        Ok(__cordl_object.into())
    }
    pub fn New_OnePassSignaturePacket1(
        sigPack: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Bcpg::OnePassSignaturePacket,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (sigPack))?;
        Ok(__cordl_object.into())
    }
    pub fn Update_Il2CppArray1(
        &mut self,
        bytes: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Update", (bytes))?;
        Ok(__cordl_ret.into())
    }
    pub fn Update_Il2CppArray_i32_i32_2(
        &mut self,
        bytes: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        off: i32,
        length: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Update", (bytes, off, length))?;
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
    pub fn Verify(
        &mut self,
        pgpSig: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpSignature,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("Verify", (pgpSig))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_BcpgInputStream0(
        &mut self,
        bcpgInput: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Bcpg::BcpgInputStream,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (bcpgInput))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_OnePassSignaturePacket1(
        &mut self,
        sigPack: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Bcpg::OnePassSignaturePacket,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (sigPack))?;
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
        Ok(__cordl_ret.into())
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
        Ok(__cordl_ret.into())
    }
    pub fn get_KeyId(&mut self) -> quest_hook::libil2cpp::Result<i64> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i64 = __cordl_object.invoke("get_KeyId", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_SignatureType(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_SignatureType", ())?;
        Ok(__cordl_ret.into())
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
