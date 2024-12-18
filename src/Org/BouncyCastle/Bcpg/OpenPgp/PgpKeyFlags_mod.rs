#[cfg(feature = "Org+BouncyCastle+Bcpg+OpenPgp+PgpKeyFlags")]
#[repr(C)]
#[derive(Debug)]
pub struct PgpKeyFlags {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "Org+BouncyCastle+Bcpg+OpenPgp+PgpKeyFlags")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Org::BouncyCastle::Bcpg::OpenPgp::PgpKeyFlags =>
    "Org.BouncyCastle.Bcpg.OpenPgp"."PgpKeyFlags"
);
#[cfg(feature = "Org+BouncyCastle+Bcpg+OpenPgp+PgpKeyFlags")]
impl std::ops::Deref for crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpKeyFlags {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Bcpg+OpenPgp+PgpKeyFlags")]
impl std::ops::DerefMut for crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpKeyFlags {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Bcpg+OpenPgp+PgpKeyFlags")]
impl crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpKeyFlags {
    pub const CanCertify: i32 = 1i32;
    pub const CanEncryptCommunications: i32 = 4i32;
    pub const CanEncryptStorage: i32 = 8i32;
    pub const CanSign: i32 = 2i32;
    pub const MaybeShared: i32 = 128i32;
    pub const MaybeSplit: i32 = 16i32;
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
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
#[cfg(feature = "Org+BouncyCastle+Bcpg+OpenPgp+PgpKeyFlags")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpKeyFlags {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
