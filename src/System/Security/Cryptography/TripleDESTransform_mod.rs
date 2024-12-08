#[cfg(feature = "System+Security+Cryptography+TripleDESTransform")]
#[repr(C)]
#[derive(Debug)]
pub struct TripleDESTransform {
    __cordl_parent: crate::Mono::Security::Cryptography::SymmetricTransform,
    pub E1: *mut crate::System::Security::Cryptography::DESTransform,
    pub D2: *mut crate::System::Security::Cryptography::DESTransform,
    pub E3: *mut crate::System::Security::Cryptography::DESTransform,
    pub D1: *mut crate::System::Security::Cryptography::DESTransform,
    pub E2: *mut crate::System::Security::Cryptography::DESTransform,
    pub D3: *mut crate::System::Security::Cryptography::DESTransform,
}
#[cfg(feature = "System+Security+Cryptography+TripleDESTransform")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::System::Security::Cryptography::TripleDESTransform =>
    "System.Security.Cryptography"."TripleDESTransform"
);
#[cfg(feature = "System+Security+Cryptography+TripleDESTransform")]
impl std::ops::Deref for crate::System::Security::Cryptography::TripleDESTransform {
    type Target = crate::Mono::Security::Cryptography::SymmetricTransform;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Security+Cryptography+TripleDESTransform")]
impl std::ops::DerefMut for crate::System::Security::Cryptography::TripleDESTransform {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Security+Cryptography+TripleDESTransform")]
impl crate::System::Security::Cryptography::TripleDESTransform {
    pub fn _ctor(
        &mut self,
        algo: *mut crate::System::Security::Cryptography::TripleDES,
        encryption: bool,
        key: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
        iv: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (algo, encryption, key, iv))?;
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
    pub fn New(
        algo: *mut crate::System::Security::Cryptography::TripleDES,
        encryption: bool,
        key: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
        iv: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (algo, encryption, key, iv))?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "System+Security+Cryptography+TripleDESTransform")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Security::Cryptography::TripleDESTransform {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
