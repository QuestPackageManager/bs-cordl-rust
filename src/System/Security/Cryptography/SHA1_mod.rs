#[cfg(feature = "System+Security+Cryptography+SHA1")]
#[repr(C)]
#[derive(Debug)]
pub struct SHA1 {
    __cordl_parent: crate::System::Security::Cryptography::HashAlgorithm,
}
#[cfg(feature = "System+Security+Cryptography+SHA1")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Security::Cryptography::SHA1 =>
    "System.Security.Cryptography"."SHA1"
);
#[cfg(feature = "System+Security+Cryptography+SHA1")]
impl std::ops::Deref for crate::System::Security::Cryptography::SHA1 {
    type Target = crate::System::Security::Cryptography::HashAlgorithm;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Security+Cryptography+SHA1")]
impl std::ops::DerefMut for crate::System::Security::Cryptography::SHA1 {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Security+Cryptography+SHA1")]
impl crate::System::Security::Cryptography::SHA1 {
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret)
    }
    pub fn New() -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "System+Security+Cryptography+SHA1")]
impl quest_hook::libil2cpp::ObjectType for crate::System::Security::Cryptography::SHA1 {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
