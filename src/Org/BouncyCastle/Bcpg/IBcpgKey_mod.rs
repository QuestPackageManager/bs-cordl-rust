#[cfg(feature = "Org+BouncyCastle+Bcpg+IBcpgKey")]
#[repr(C)]
#[derive(Debug)]
pub struct IBcpgKey {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "Org+BouncyCastle+Bcpg+IBcpgKey")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Org::BouncyCastle::Bcpg::IBcpgKey =>
    "Org.BouncyCastle.Bcpg"."IBcpgKey"
);
#[cfg(feature = "Org+BouncyCastle+Bcpg+IBcpgKey")]
impl std::ops::Deref for crate::Org::BouncyCastle::Bcpg::IBcpgKey {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Bcpg+IBcpgKey")]
impl std::ops::DerefMut for crate::Org::BouncyCastle::Bcpg::IBcpgKey {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Bcpg+IBcpgKey")]
impl crate::Org::BouncyCastle::Bcpg::IBcpgKey {
    pub fn get_Format(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("get_Format", ())?;
        Ok(__cordl_ret)
    }
    pub fn from_object_mut(
        object_param: *mut quest_hook::libil2cpp::Il2CppObject,
    ) -> *mut Self {
        unsafe { (object_param as *mut Self) }
    }
}
#[cfg(feature = "Org+BouncyCastle+Bcpg+IBcpgKey")]
impl quest_hook::libil2cpp::ObjectType for crate::Org::BouncyCastle::Bcpg::IBcpgKey {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
