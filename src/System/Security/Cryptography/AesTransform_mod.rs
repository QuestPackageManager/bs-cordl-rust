#[cfg(feature = "System+Security+Cryptography+AesTransform")]
#[repr(C)]
#[derive(Debug)]
pub struct AesTransform {
    __cordl_parent: crate::Mono::Security::Cryptography::SymmetricTransform,
    pub expandedKey: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
    pub Nk: i32,
    pub Nr: i32,
}
#[cfg(feature = "System+Security+Cryptography+AesTransform")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Security::Cryptography::AesTransform =>
    "System.Security.Cryptography"."AesTransform"
);
#[cfg(feature = "System+Security+Cryptography+AesTransform")]
impl std::ops::Deref for crate::System::Security::Cryptography::AesTransform {
    type Target = crate::Mono::Security::Cryptography::SymmetricTransform;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Security+Cryptography+AesTransform")]
impl std::ops::DerefMut for crate::System::Security::Cryptography::AesTransform {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Security+Cryptography+AesTransform")]
impl crate::System::Security::Cryptography::AesTransform {
    pub fn Decrypt128(
        &mut self,
        indata: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        outdata: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        ekey: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Decrypt128", (indata, outdata, ekey))?;
        Ok(__cordl_ret.into())
    }
    pub fn ECB(
        &mut self,
        input: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        output: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ECB", (input, output))?;
        Ok(__cordl_ret.into())
    }
    pub fn Encrypt128(
        &mut self,
        indata: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        outdata: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        ekey: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Encrypt128", (indata, outdata, ekey))?;
        Ok(__cordl_ret.into())
    }
    pub fn New(
        algo: quest_hook::libil2cpp::Gc<crate::System::Security::Cryptography::Aes>,
        encryption: bool,
        key: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        iv: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (algo, encryption, key, iv))?;
        Ok(__cordl_object.into())
    }
    pub fn SubByte(&mut self, a: u32) -> quest_hook::libil2cpp::Result<u32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: u32 = __cordl_object.invoke("SubByte", (a))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        algo: quest_hook::libil2cpp::Gc<crate::System::Security::Cryptography::Aes>,
        encryption: bool,
        key: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        iv: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (algo, encryption, key, iv))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Security+Cryptography+AesTransform")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Security::Cryptography::AesTransform {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
