#[cfg(feature = "IDiffieHellmanKeyPair")]
#[repr(C)]
#[derive(Debug)]
pub struct IDiffieHellmanKeyPair {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "IDiffieHellmanKeyPair")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::IDiffieHellmanKeyPair => ""
    ."IDiffieHellmanKeyPair"
);
#[cfg(feature = "IDiffieHellmanKeyPair")]
impl std::ops::Deref for crate::GlobalNamespace::IDiffieHellmanKeyPair {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "IDiffieHellmanKeyPair")]
impl std::ops::DerefMut for crate::GlobalNamespace::IDiffieHellmanKeyPair {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "IDiffieHellmanKeyPair")]
impl crate::GlobalNamespace::IDiffieHellmanKeyPair {
    pub fn GetPreMasterSecret(
        &mut self,
        clientPublicKey: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<u8>,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<u8>,
        > = __cordl_object.invoke("GetPreMasterSecret", (clientPublicKey))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetPreMasterSecretAsync(
        &mut self,
        taskUtility: quest_hook::libil2cpp::Gc<crate::BGNet::Core::ITaskUtility>,
        clientPublicKey: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<u8>,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task_1<
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
            >,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task_1<
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
            >,
        > = __cordl_object
            .invoke("GetPreMasterSecretAsync", (taskUtility, clientPublicKey))?;
        Ok(__cordl_ret.into())
    }
    pub fn from_object_mut(
        object_param: *mut quest_hook::libil2cpp::Il2CppObject,
    ) -> *mut Self {
        unsafe { (object_param as *mut Self) }
    }
    pub fn get_publicKey(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<u8>,
        > = __cordl_object.invoke("get_publicKey", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "IDiffieHellmanKeyPair")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::IDiffieHellmanKeyPair {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
