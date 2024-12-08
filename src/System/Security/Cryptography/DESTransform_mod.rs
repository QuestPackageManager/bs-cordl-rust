#[cfg(feature = "System+Security+Cryptography+DESTransform")]
#[repr(C)]
#[derive(Debug)]
pub struct DESTransform {
    __cordl_parent: crate::Mono::Security::Cryptography::SymmetricTransform,
    pub keySchedule: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
    pub byteBuff: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
    pub dwordBuff: *mut quest_hook::libil2cpp::Il2CppArray<u32>,
}
#[cfg(feature = "System+Security+Cryptography+DESTransform")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Security::Cryptography::DESTransform =>
    "System.Security.Cryptography"."DESTransform"
);
#[cfg(feature = "System+Security+Cryptography+DESTransform")]
impl std::ops::Deref for crate::System::Security::Cryptography::DESTransform {
    type Target = crate::Mono::Security::Cryptography::SymmetricTransform;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Security+Cryptography+DESTransform")]
impl std::ops::DerefMut for crate::System::Security::Cryptography::DESTransform {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Security+Cryptography+DESTransform")]
impl crate::System::Security::Cryptography::DESTransform {
    pub fn SetKey(
        &mut self,
        key: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetKey", (key))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
        symmAlgo: *mut crate::System::Security::Cryptography::SymmetricAlgorithm,
        encryption: bool,
        key: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
        iv: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (symmAlgo, encryption, key, iv))?;
        Ok(__cordl_ret)
    }
    pub fn ECB(
        &mut self,
        input: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
        output: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ECB", (input, output))?;
        Ok(__cordl_ret)
    }
    pub fn CipherFunct(&mut self, r: u32, n: i32) -> quest_hook::libil2cpp::Result<u32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: u32 = __cordl_object.invoke("CipherFunct", (r, n))?;
        Ok(__cordl_ret)
    }
    pub fn ProcessBlock(
        &mut self,
        input: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
        output: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ProcessBlock", (input, output))?;
        Ok(__cordl_ret)
    }
    pub fn New(
        symmAlgo: *mut crate::System::Security::Cryptography::SymmetricAlgorithm,
        encryption: bool,
        key: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
        iv: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (symmAlgo, encryption, key, iv))?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "System+Security+Cryptography+DESTransform")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Security::Cryptography::DESTransform {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
