#[cfg(feature = "System+Security+Cryptography+Rijndael")]
#[repr(C)]
#[derive(Debug)]
pub struct Rijndael {
    __cordl_parent: crate::System::Security::Cryptography::SymmetricAlgorithm,
}
#[cfg(feature = "System+Security+Cryptography+Rijndael")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Security::Cryptography::Rijndael =>
    "System.Security.Cryptography"."Rijndael"
);
#[cfg(feature = "System+Security+Cryptography+Rijndael")]
impl std::ops::Deref for crate::System::Security::Cryptography::Rijndael {
    type Target = crate::System::Security::Cryptography::SymmetricAlgorithm;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Security+Cryptography+Rijndael")]
impl std::ops::DerefMut for crate::System::Security::Cryptography::Rijndael {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Security+Cryptography+Rijndael")]
impl crate::System::Security::Cryptography::Rijndael {
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Security+Cryptography+Rijndael")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Security::Cryptography::Rijndael {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
