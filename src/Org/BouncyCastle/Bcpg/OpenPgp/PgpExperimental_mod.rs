#[cfg(feature = "Org+BouncyCastle+Bcpg+OpenPgp+PgpExperimental")]
#[repr(C)]
#[derive(Debug)]
pub struct PgpExperimental {
    __cordl_parent: crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpObject,
    pub p: *mut crate::Org::BouncyCastle::Bcpg::ExperimentalPacket,
}
#[cfg(feature = "Org+BouncyCastle+Bcpg+OpenPgp+PgpExperimental")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::Org::BouncyCastle::Bcpg::OpenPgp::PgpExperimental =>
    "Org.BouncyCastle.Bcpg.OpenPgp"."PgpExperimental"
);
#[cfg(feature = "Org+BouncyCastle+Bcpg+OpenPgp+PgpExperimental")]
impl std::ops::Deref for crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpExperimental {
    type Target = crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Bcpg+OpenPgp+PgpExperimental")]
impl std::ops::DerefMut for crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpExperimental {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Bcpg+OpenPgp+PgpExperimental")]
impl crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpExperimental {
    pub fn _ctor(
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
    pub fn New(
        bcpgIn: *mut crate::Org::BouncyCastle::Bcpg::BcpgInputStream,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (bcpgIn))?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "Org+BouncyCastle+Bcpg+OpenPgp+PgpExperimental")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpExperimental {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
