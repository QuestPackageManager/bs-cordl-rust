#[cfg(feature = "Org+BouncyCastle+Bcpg+OpenPgp+PgpSignatureSubpacketVector")]
#[repr(C)]
#[derive(Debug)]
pub struct PgpSignatureSubpacketVector {
    __cordl_parent: crate::System::Object,
    pub packets: *mut quest_hook::libil2cpp::Il2CppArray<
        *mut crate::Org::BouncyCastle::Bcpg::SignatureSubpacket,
    >,
}
#[cfg(feature = "Org+BouncyCastle+Bcpg+OpenPgp+PgpSignatureSubpacketVector")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::Org::BouncyCastle::Bcpg::OpenPgp::PgpSignatureSubpacketVector =>
    "Org.BouncyCastle.Bcpg.OpenPgp"."PgpSignatureSubpacketVector"
);
#[cfg(feature = "Org+BouncyCastle+Bcpg+OpenPgp+PgpSignatureSubpacketVector")]
impl std::ops::Deref
for crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpSignatureSubpacketVector {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Bcpg+OpenPgp+PgpSignatureSubpacketVector")]
impl std::ops::DerefMut
for crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpSignatureSubpacketVector {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Bcpg+OpenPgp+PgpSignatureSubpacketVector")]
impl crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpSignatureSubpacketVector {
    pub fn GetCriticalTags(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut quest_hook::libil2cpp::Il2CppArray<
            crate::Org::BouncyCastle::Bcpg::SignatureSubpacketTag,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppArray<
            crate::Org::BouncyCastle::Bcpg::SignatureSubpacketTag,
        > = __cordl_object.invoke("GetCriticalTags", ())?;
        Ok(__cordl_ret)
    }
    pub fn GetFeatures(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Bcpg::Sig::Features,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Bcpg::Sig::Features = __cordl_object
            .invoke("GetFeatures", ())?;
        Ok(__cordl_ret)
    }
    pub fn GetIssuerKeyId(&mut self) -> quest_hook::libil2cpp::Result<i64> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i64 = __cordl_object.invoke("GetIssuerKeyId", ())?;
        Ok(__cordl_ret)
    }
    pub fn GetKeyExpirationTime(&mut self) -> quest_hook::libil2cpp::Result<i64> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i64 = __cordl_object.invoke("GetKeyExpirationTime", ())?;
        Ok(__cordl_ret)
    }
    pub fn GetKeyFlags(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("GetKeyFlags", ())?;
        Ok(__cordl_ret)
    }
    pub fn GetNotationDataOccurences(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut quest_hook::libil2cpp::Il2CppArray<
            *mut crate::Org::BouncyCastle::Bcpg::Sig::NotationData,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppArray<
            *mut crate::Org::BouncyCastle::Bcpg::Sig::NotationData,
        > = __cordl_object.invoke("GetNotationDataOccurences", ())?;
        Ok(__cordl_ret)
    }
    pub fn GetNotationDataOccurrences(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut quest_hook::libil2cpp::Il2CppArray<
            *mut crate::Org::BouncyCastle::Bcpg::Sig::NotationData,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppArray<
            *mut crate::Org::BouncyCastle::Bcpg::Sig::NotationData,
        > = __cordl_object.invoke("GetNotationDataOccurrences", ())?;
        Ok(__cordl_ret)
    }
    pub fn GetPreferredCompressionAlgorithms(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut quest_hook::libil2cpp::Il2CppArray<i32>> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppArray<i32> = __cordl_object
            .invoke("GetPreferredCompressionAlgorithms", ())?;
        Ok(__cordl_ret)
    }
    pub fn GetPreferredHashAlgorithms(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut quest_hook::libil2cpp::Il2CppArray<i32>> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppArray<i32> = __cordl_object
            .invoke("GetPreferredHashAlgorithms", ())?;
        Ok(__cordl_ret)
    }
    pub fn GetPreferredSymmetricAlgorithms(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut quest_hook::libil2cpp::Il2CppArray<i32>> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppArray<i32> = __cordl_object
            .invoke("GetPreferredSymmetricAlgorithms", ())?;
        Ok(__cordl_ret)
    }
    pub fn GetSignatureCreationTime(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::System::DateTime> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::System::DateTime = __cordl_object
            .invoke("GetSignatureCreationTime", ())?;
        Ok(__cordl_ret)
    }
    pub fn GetSignatureExpirationTime(&mut self) -> quest_hook::libil2cpp::Result<i64> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i64 = __cordl_object.invoke("GetSignatureExpirationTime", ())?;
        Ok(__cordl_ret)
    }
    pub fn GetSignerUserId(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("GetSignerUserId", ())?;
        Ok(__cordl_ret)
    }
    pub fn GetSubpacket(
        &mut self,
        _cordl_type: crate::Org::BouncyCastle::Bcpg::SignatureSubpacketTag,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Bcpg::SignatureSubpacket,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Bcpg::SignatureSubpacket = __cordl_object
            .invoke("GetSubpacket", (_cordl_type))?;
        Ok(__cordl_ret)
    }
    pub fn GetSubpackets(
        &mut self,
        _cordl_type: crate::Org::BouncyCastle::Bcpg::SignatureSubpacketTag,
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
        > = __cordl_object.invoke("GetSubpackets", (_cordl_type))?;
        Ok(__cordl_ret)
    }
    pub fn HasSignatureCreationTime(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("HasSignatureCreationTime", ())?;
        Ok(__cordl_ret)
    }
    pub fn HasSubpacket(
        &mut self,
        _cordl_type: crate::Org::BouncyCastle::Bcpg::SignatureSubpacketTag,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("HasSubpacket", (_cordl_type))?;
        Ok(__cordl_ret)
    }
    pub fn IsPrimaryUserId(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("IsPrimaryUserId", ())?;
        Ok(__cordl_ret)
    }
    pub fn New(
        packets: *mut quest_hook::libil2cpp::Il2CppArray<
            *mut crate::Org::BouncyCastle::Bcpg::SignatureSubpacket,
        >,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (packets))?;
        Ok(__cordl_object)
    }
    pub fn ToSubpacketArray(
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
        > = __cordl_object.invoke("ToSubpacketArray", ())?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
        packets: *mut quest_hook::libil2cpp::Il2CppArray<
            *mut crate::Org::BouncyCastle::Bcpg::SignatureSubpacket,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (packets))?;
        Ok(__cordl_ret)
    }
    pub fn get_Count(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_Count", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_Size(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_Size", ())?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "Org+BouncyCastle+Bcpg+OpenPgp+PgpSignatureSubpacketVector")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpSignatureSubpacketVector {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}