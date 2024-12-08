#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+TlsSrpIdentityManager")]
#[repr(C)]
#[derive(Debug)]
pub struct TlsSrpIdentityManager {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+TlsSrpIdentityManager")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::Org::BouncyCastle::Crypto::Tls::TlsSrpIdentityManager =>
    "Org.BouncyCastle.Crypto.Tls"."TlsSrpIdentityManager"
);
#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+TlsSrpIdentityManager")]
impl std::ops::Deref for crate::Org::BouncyCastle::Crypto::Tls::TlsSrpIdentityManager {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+TlsSrpIdentityManager")]
impl std::ops::DerefMut
for crate::Org::BouncyCastle::Crypto::Tls::TlsSrpIdentityManager {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+TlsSrpIdentityManager")]
impl crate::Org::BouncyCastle::Crypto::Tls::TlsSrpIdentityManager {
    pub fn GetLoginParameters(
        &mut self,
        identity: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Crypto::Tls::TlsSrpLoginParameters,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Crypto::Tls::TlsSrpLoginParameters = __cordl_object
            .invoke("GetLoginParameters", (identity))?;
        Ok(__cordl_ret)
    }
    pub fn from_object_mut(
        object_param: *mut quest_hook::libil2cpp::Il2CppObject,
    ) -> *mut Self {
        unsafe { (object_param as *mut Self) }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+TlsSrpIdentityManager")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Crypto::Tls::TlsSrpIdentityManager {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
