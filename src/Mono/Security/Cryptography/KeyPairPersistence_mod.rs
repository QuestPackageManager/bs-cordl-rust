#[cfg(feature = "Mono+Security+Cryptography+KeyPairPersistence")]
#[repr(C)]
#[derive(Debug)]
pub struct KeyPairPersistence {
    __cordl_parent: crate::System::Object,
    pub _params: *mut crate::System::Security::Cryptography::CspParameters,
    pub _keyvalue: *mut crate::System::String,
    pub _filename: *mut crate::System::String,
    pub _container: *mut crate::System::String,
}
#[cfg(feature = "Mono+Security+Cryptography+KeyPairPersistence")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Mono::Security::Cryptography::KeyPairPersistence
    => "Mono.Security.Cryptography"."KeyPairPersistence"
);
#[cfg(feature = "Mono+Security+Cryptography+KeyPairPersistence")]
impl std::ops::Deref for crate::Mono::Security::Cryptography::KeyPairPersistence {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Mono+Security+Cryptography+KeyPairPersistence")]
impl std::ops::DerefMut for crate::Mono::Security::Cryptography::KeyPairPersistence {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Mono+Security+Cryptography+KeyPairPersistence")]
impl crate::Mono::Security::Cryptography::KeyPairPersistence {
    pub fn Copy(
        &mut self,
        p: *mut crate::System::Security::Cryptography::CspParameters,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Security::Cryptography::CspParameters,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Security::Cryptography::CspParameters = __cordl_object
            .invoke("Copy", (p))?;
        Ok(__cordl_ret)
    }
    pub fn FromXml(
        &mut self,
        xml: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("FromXml", (xml))?;
        Ok(__cordl_ret)
    }
    pub fn Load(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("Load", ())?;
        Ok(__cordl_ret)
    }
    pub fn New_CspParameters0(
        parameters: *mut crate::System::Security::Cryptography::CspParameters,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (parameters))?;
        Ok(__cordl_object)
    }
    pub fn New_String1(
        parameters: *mut crate::System::Security::Cryptography::CspParameters,
        keyPair: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (parameters, keyPair))?;
        Ok(__cordl_object)
    }
    pub fn Remove(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Remove", ())?;
        Ok(__cordl_ret)
    }
    pub fn Save(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Save", ())?;
        Ok(__cordl_ret)
    }
    pub fn ToXml(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("ToXml", ())?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_CspParameters0(
        &mut self,
        parameters: *mut crate::System::Security::Cryptography::CspParameters,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (parameters))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_String1(
        &mut self,
        parameters: *mut crate::System::Security::Cryptography::CspParameters,
        keyPair: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (parameters, keyPair))?;
        Ok(__cordl_ret)
    }
    pub fn get_CanChange(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_CanChange", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_ContainerName(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("get_ContainerName", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_Filename(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("get_Filename", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_KeyValue(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("get_KeyValue", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_UseDefaultKeyContainer(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_UseDefaultKeyContainer", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_UseMachineKeyStore(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_UseMachineKeyStore", ())?;
        Ok(__cordl_ret)
    }
    pub fn set_KeyValue(
        &mut self,
        value: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_KeyValue", (value))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "Mono+Security+Cryptography+KeyPairPersistence")]
impl quest_hook::libil2cpp::ObjectType
for crate::Mono::Security::Cryptography::KeyPairPersistence {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
