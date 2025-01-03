#[cfg(feature = "Org+BouncyCastle+Bcpg+OpenPgp+PgpSignatureSubpacketGenerator")]
#[repr(C)]
#[derive(Debug)]
pub struct PgpSignatureSubpacketGenerator {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub list: quest_hook::libil2cpp::Gc<crate::System::Collections::IList>,
}
#[cfg(feature = "Org+BouncyCastle+Bcpg+OpenPgp+PgpSignatureSubpacketGenerator")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::Org::BouncyCastle::Bcpg::OpenPgp::PgpSignatureSubpacketGenerator =>
    "Org.BouncyCastle.Bcpg.OpenPgp"."PgpSignatureSubpacketGenerator"
);
#[cfg(feature = "Org+BouncyCastle+Bcpg+OpenPgp+PgpSignatureSubpacketGenerator")]
impl std::ops::Deref
for crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpSignatureSubpacketGenerator {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Bcpg+OpenPgp+PgpSignatureSubpacketGenerator")]
impl std::ops::DerefMut
for crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpSignatureSubpacketGenerator {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Bcpg+OpenPgp+PgpSignatureSubpacketGenerator")]
impl crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpSignatureSubpacketGenerator {
    pub fn Generate(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpSignatureSubpacketVector,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpSignatureSubpacketVector,
        > = __cordl_object.invoke("Generate", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn SetEmbeddedSignature(
        &mut self,
        isCritical: bool,
        pgpSignature: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpSignature,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetEmbeddedSignature", (isCritical, pgpSignature))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetExportable(
        &mut self,
        isCritical: bool,
        isExportable: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetExportable", (isCritical, isExportable))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetFeature(
        &mut self,
        isCritical: bool,
        feature: u8,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetFeature", (isCritical, feature))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetIssuerKeyID(
        &mut self,
        isCritical: bool,
        keyID: i64,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetIssuerKeyID", (isCritical, keyID))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetKeyExpirationTime(
        &mut self,
        isCritical: bool,
        seconds: i64,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetKeyExpirationTime", (isCritical, seconds))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetKeyFlags(
        &mut self,
        isCritical: bool,
        flags: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetKeyFlags", (isCritical, flags))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetNotationData(
        &mut self,
        isCritical: bool,
        isHumanReadable: bool,
        notationName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        notationValue: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "SetNotationData",
                (isCritical, isHumanReadable, notationName, notationValue),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn SetPreferredCompressionAlgorithms(
        &mut self,
        isCritical: bool,
        algorithms: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<i32>>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetPreferredCompressionAlgorithms", (isCritical, algorithms))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetPreferredHashAlgorithms(
        &mut self,
        isCritical: bool,
        algorithms: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<i32>>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetPreferredHashAlgorithms", (isCritical, algorithms))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetPreferredSymmetricAlgorithms(
        &mut self,
        isCritical: bool,
        algorithms: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<i32>>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetPreferredSymmetricAlgorithms", (isCritical, algorithms))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetPrimaryUserId(
        &mut self,
        isCritical: bool,
        isPrimaryUserId: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetPrimaryUserId", (isCritical, isPrimaryUserId))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetRevocable(
        &mut self,
        isCritical: bool,
        isRevocable: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetRevocable", (isCritical, isRevocable))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetRevocationKey(
        &mut self,
        isCritical: bool,
        keyAlgorithm: crate::Org::BouncyCastle::Bcpg::PublicKeyAlgorithmTag,
        fingerprint: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetRevocationKey", (isCritical, keyAlgorithm, fingerprint))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetRevocationReason(
        &mut self,
        isCritical: bool,
        reason: crate::Org::BouncyCastle::Bcpg::RevocationReasonTag,
        description: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetRevocationReason", (isCritical, reason, description))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetSignatureCreationTime(
        &mut self,
        isCritical: bool,
        date: crate::System::DateTime,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetSignatureCreationTime", (isCritical, date))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetSignatureExpirationTime(
        &mut self,
        isCritical: bool,
        seconds: i64,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetSignatureExpirationTime", (isCritical, seconds))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetSignerUserId_Il2CppArray1(
        &mut self,
        isCritical: bool,
        rawUserId: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetSignerUserId", (isCritical, rawUserId))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetSignerUserId_Il2CppString0(
        &mut self,
        isCritical: bool,
        userId: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetSignerUserId", (isCritical, userId))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetTrust(
        &mut self,
        isCritical: bool,
        depth: i32,
        trustAmount: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetTrust", (isCritical, depth, trustAmount))?;
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
#[cfg(feature = "Org+BouncyCastle+Bcpg+OpenPgp+PgpSignatureSubpacketGenerator")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpSignatureSubpacketGenerator {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
