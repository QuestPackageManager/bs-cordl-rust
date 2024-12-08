#[cfg(feature = "Mono+Security+Authenticode+PrivateKey")]
#[repr(C)]
#[derive(Debug)]
pub struct PrivateKey {
    __cordl_parent: crate::System::Object,
    pub encrypted: bool,
    pub rsa: *mut crate::System::Security::Cryptography::RSA,
    pub weak: bool,
    pub keyType: i32,
}
#[cfg(feature = "Mono+Security+Authenticode+PrivateKey")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Mono::Security::Authenticode::PrivateKey =>
    "Mono.Security.Authenticode"."PrivateKey"
);
#[cfg(feature = "Mono+Security+Authenticode+PrivateKey")]
impl std::ops::Deref for crate::Mono::Security::Authenticode::PrivateKey {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Mono+Security+Authenticode+PrivateKey")]
impl std::ops::DerefMut for crate::Mono::Security::Authenticode::PrivateKey {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Mono+Security+Authenticode+PrivateKey")]
impl crate::Mono::Security::Authenticode::PrivateKey {
    pub fn Decode(
        &mut self,
        pvk: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
        password: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("Decode", (pvk, password))?;
        Ok(__cordl_ret)
    }
    pub fn DeriveKey(
        &mut self,
        salt: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
        password: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<*mut quest_hook::libil2cpp::Il2CppArray<u8>> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppArray<u8> = __cordl_object
            .invoke("DeriveKey", (salt, password))?;
        Ok(__cordl_ret)
    }
    pub fn New(
        data: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
        password: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (data, password))?;
        Ok(__cordl_object)
    }
    pub fn _ctor(
        &mut self,
        data: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
        password: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (data, password))?;
        Ok(__cordl_ret)
    }
    pub fn get_RSA(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Security::Cryptography::RSA> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Security::Cryptography::RSA = __cordl_object
            .invoke("get_RSA", ())?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "Mono+Security+Authenticode+PrivateKey")]
impl quest_hook::libil2cpp::ObjectType
for crate::Mono::Security::Authenticode::PrivateKey {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
