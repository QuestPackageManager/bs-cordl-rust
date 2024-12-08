#[cfg(feature = "System+Security+Cryptography+RC2CryptoServiceProvider")]
#[repr(C)]
#[derive(Debug)]
pub struct RC2CryptoServiceProvider {
    __cordl_parent: crate::System::Security::Cryptography::RC2,
    pub m_use40bitSalt: bool,
}
#[cfg(feature = "System+Security+Cryptography+RC2CryptoServiceProvider")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::System::Security::Cryptography::RC2CryptoServiceProvider =>
    "System.Security.Cryptography"."RC2CryptoServiceProvider"
);
#[cfg(feature = "System+Security+Cryptography+RC2CryptoServiceProvider")]
impl std::ops::Deref
for crate::System::Security::Cryptography::RC2CryptoServiceProvider {
    type Target = crate::System::Security::Cryptography::RC2;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Security+Cryptography+RC2CryptoServiceProvider")]
impl std::ops::DerefMut
for crate::System::Security::Cryptography::RC2CryptoServiceProvider {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Security+Cryptography+RC2CryptoServiceProvider")]
impl crate::System::Security::Cryptography::RC2CryptoServiceProvider {
    pub fn CreateDecryptor(
        &mut self,
        rgbKey: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
        rgbIV: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Security::Cryptography::ICryptoTransform,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Security::Cryptography::ICryptoTransform = __cordl_object
            .invoke("CreateDecryptor", (rgbKey, rgbIV))?;
        Ok(__cordl_ret)
    }
    pub fn CreateEncryptor(
        &mut self,
        rgbKey: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
        rgbIV: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Security::Cryptography::ICryptoTransform,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Security::Cryptography::ICryptoTransform = __cordl_object
            .invoke("CreateEncryptor", (rgbKey, rgbIV))?;
        Ok(__cordl_ret)
    }
    pub fn GenerateIV(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("GenerateIV", ())?;
        Ok(__cordl_ret)
    }
    pub fn GenerateKey(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("GenerateKey", ())?;
        Ok(__cordl_ret)
    }
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
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
    pub fn get_EffectiveKeySize(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_EffectiveKeySize", ())?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "System+Security+Cryptography+RC2CryptoServiceProvider")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Security::Cryptography::RC2CryptoServiceProvider {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
