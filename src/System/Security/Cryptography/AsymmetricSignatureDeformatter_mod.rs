#[cfg(feature = "System+Security+Cryptography+AsymmetricSignatureDeformatter")]
#[repr(C)]
#[derive(Debug)]
pub struct AsymmetricSignatureDeformatter {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "System+Security+Cryptography+AsymmetricSignatureDeformatter")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::System::Security::Cryptography::AsymmetricSignatureDeformatter =>
    "System.Security.Cryptography"."AsymmetricSignatureDeformatter"
);
#[cfg(feature = "System+Security+Cryptography+AsymmetricSignatureDeformatter")]
impl std::ops::Deref
for crate::System::Security::Cryptography::AsymmetricSignatureDeformatter {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Security+Cryptography+AsymmetricSignatureDeformatter")]
impl std::ops::DerefMut
for crate::System::Security::Cryptography::AsymmetricSignatureDeformatter {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Security+Cryptography+AsymmetricSignatureDeformatter")]
impl crate::System::Security::Cryptography::AsymmetricSignatureDeformatter {
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
    pub fn SetHashAlgorithm(
        &mut self,
        strName: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetHashAlgorithm", (strName))?;
        Ok(__cordl_ret)
    }
    pub fn SetKey(
        &mut self,
        key: *mut crate::System::Security::Cryptography::AsymmetricAlgorithm,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetKey", (key))?;
        Ok(__cordl_ret)
    }
    pub fn VerifySignature(
        &mut self,
        rgbHash: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
        rgbSignature: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("VerifySignature", (rgbHash, rgbSignature))?;
        Ok(__cordl_ret)
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
}
#[cfg(feature = "System+Security+Cryptography+AsymmetricSignatureDeformatter")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Security::Cryptography::AsymmetricSignatureDeformatter {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
