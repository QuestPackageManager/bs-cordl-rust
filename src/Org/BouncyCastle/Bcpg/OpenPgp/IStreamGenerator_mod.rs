#[cfg(feature = "Org+BouncyCastle+Bcpg+OpenPgp+IStreamGenerator")]
#[repr(C)]
#[derive(Debug)]
pub struct IStreamGenerator {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "Org+BouncyCastle+Bcpg+OpenPgp+IStreamGenerator")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::Org::BouncyCastle::Bcpg::OpenPgp::IStreamGenerator =>
    "Org.BouncyCastle.Bcpg.OpenPgp"."IStreamGenerator"
);
#[cfg(feature = "Org+BouncyCastle+Bcpg+OpenPgp+IStreamGenerator")]
impl std::ops::Deref for crate::Org::BouncyCastle::Bcpg::OpenPgp::IStreamGenerator {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Bcpg+OpenPgp+IStreamGenerator")]
impl std::ops::DerefMut for crate::Org::BouncyCastle::Bcpg::OpenPgp::IStreamGenerator {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Bcpg+OpenPgp+IStreamGenerator")]
impl crate::Org::BouncyCastle::Bcpg::OpenPgp::IStreamGenerator {
    pub fn Close(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Close", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn from_object_mut(
        object_param: *mut quest_hook::libil2cpp::Il2CppObject,
    ) -> *mut Self {
        unsafe { (object_param as *mut Self) }
    }
}
#[cfg(feature = "Org+BouncyCastle+Bcpg+OpenPgp+IStreamGenerator")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Bcpg::OpenPgp::IStreamGenerator {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
