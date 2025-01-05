#[cfg(feature = "System+Security+Cryptography+TripleDES")]
#[repr(C)]
#[derive(Debug)]
pub struct TripleDES {
    __cordl_parent: crate::System::Security::Cryptography::SymmetricAlgorithm,
}
#[cfg(feature = "System+Security+Cryptography+TripleDES")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Security::Cryptography::TripleDES =>
    "System.Security.Cryptography"."TripleDES"
);
#[cfg(feature = "System+Security+Cryptography+TripleDES")]
impl std::ops::Deref for crate::System::Security::Cryptography::TripleDES {
    type Target = crate::System::Security::Cryptography::SymmetricAlgorithm;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Security+Cryptography+TripleDES")]
impl std::ops::DerefMut for crate::System::Security::Cryptography::TripleDES {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Security+Cryptography+TripleDES")]
impl crate::System::Security::Cryptography::TripleDES {
    pub fn Create() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Security::Cryptography::TripleDES>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Security::Cryptography::TripleDES,
        > = <Self as quest_hook::libil2cpp::Type>::class().invoke("Create", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn EqualBytes(
        rgbKey: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        start1: i32,
        start2: i32,
        count: i32,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("EqualBytes", (rgbKey, start1, start2, count))?;
        Ok(__cordl_ret.into())
    }
    pub fn IsLegalKeySize(
        rgbKey: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("IsLegalKeySize", (rgbKey))?;
        Ok(__cordl_ret.into())
    }
    pub fn IsWeakKey(
        rgbKey: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("IsWeakKey", (rgbKey))?;
        Ok(__cordl_ret.into())
    }
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
    pub fn get_Key(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<u8>,
        > = __cordl_object.invoke("get_Key", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn set_Key(
        &mut self,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_Key", (value))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Security+Cryptography+TripleDES")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Security::Cryptography::TripleDES {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
