#[cfg(feature = "Org+BouncyCastle+Bcpg+OpenPgp+PgpKeyRing")]
#[repr(C)]
#[derive(Debug)]
pub struct PgpKeyRing {
    __cordl_parent: crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpObject,
}
#[cfg(feature = "Org+BouncyCastle+Bcpg+OpenPgp+PgpKeyRing")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Org::BouncyCastle::Bcpg::OpenPgp::PgpKeyRing =>
    "Org.BouncyCastle.Bcpg.OpenPgp"."PgpKeyRing"
);
#[cfg(feature = "Org+BouncyCastle+Bcpg+OpenPgp+PgpKeyRing")]
impl std::ops::Deref for crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpKeyRing {
    type Target = crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Bcpg+OpenPgp+PgpKeyRing")]
impl std::ops::DerefMut for crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpKeyRing {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Bcpg+OpenPgp+PgpKeyRing")]
impl crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpKeyRing {
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn ReadOptionalTrustPacket(
        bcpgInput: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Bcpg::BcpgInputStream,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Bcpg::TrustPacket>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Bcpg::TrustPacket,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ReadOptionalTrustPacket", (bcpgInput))?;
        Ok(__cordl_ret.into())
    }
    pub fn ReadSignaturesAndTrust(
        bcpgInput: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Bcpg::BcpgInputStream,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Collections::IList>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Collections::IList> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ReadSignaturesAndTrust", (bcpgInput))?;
        Ok(__cordl_ret.into())
    }
    pub fn ReadUserIDs(
        bcpgInput: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Bcpg::BcpgInputStream,
        >,
        ids: quest_hook::libil2cpp::ByRefMut<*mut crate::System::Collections::IList>,
        idTrusts: quest_hook::libil2cpp::ByRefMut<
            *mut crate::System::Collections::IList,
        >,
        idSigs: quest_hook::libil2cpp::ByRefMut<*mut crate::System::Collections::IList>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ReadUserIDs", (bcpgInput, ids, idTrusts, idSigs))?;
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
#[cfg(feature = "Org+BouncyCastle+Bcpg+OpenPgp+PgpKeyRing")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpKeyRing {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
