#[cfg(feature = "System+Security+Cryptography+RC2Transform")]
#[repr(C)]
#[derive(Debug)]
pub struct RC2Transform {
    __cordl_parent: quest_hook::libil2cpp::Gc<
        crate::Mono::Security::Cryptography::SymmetricTransform,
    >,
    pub R0: u16,
    pub R1: u16,
    pub R2: u16,
    pub R3: u16,
    pub K: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u16>>,
    pub j: i32,
}
#[cfg(feature = "System+Security+Cryptography+RC2Transform")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Security::Cryptography::RC2Transform =>
    "System.Security.Cryptography"."RC2Transform"
);
#[cfg(feature = "System+Security+Cryptography+RC2Transform")]
impl std::ops::Deref for crate::System::Security::Cryptography::RC2Transform {
    type Target = quest_hook::libil2cpp::Gc<
        crate::Mono::Security::Cryptography::SymmetricTransform,
    >;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Security+Cryptography+RC2Transform")]
impl std::ops::DerefMut for crate::System::Security::Cryptography::RC2Transform {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Security+Cryptography+RC2Transform")]
impl crate::System::Security::Cryptography::RC2Transform {
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
    pub fn New(
        rc2Algo: quest_hook::libil2cpp::Gc<crate::System::Security::Cryptography::RC2>,
        encryption: bool,
        key: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        iv: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (rc2Algo, encryption, key, iv))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        rc2Algo: quest_hook::libil2cpp::Gc<crate::System::Security::Cryptography::RC2>,
        encryption: bool,
        key: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        iv: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (rc2Algo, encryption, key, iv))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Security+Cryptography+RC2Transform")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Security::Cryptography::RC2Transform {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
