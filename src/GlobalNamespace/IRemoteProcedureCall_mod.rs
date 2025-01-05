#[cfg(feature = "IRemoteProcedureCall")]
#[repr(C)]
#[derive(Debug)]
pub struct IRemoteProcedureCall {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "IRemoteProcedureCall")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::IRemoteProcedureCall => ""
    ."IRemoteProcedureCall"
);
#[cfg(feature = "IRemoteProcedureCall")]
impl std::ops::Deref for crate::GlobalNamespace::IRemoteProcedureCall {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "IRemoteProcedureCall")]
impl std::ops::DerefMut for crate::GlobalNamespace::IRemoteProcedureCall {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "IRemoteProcedureCall")]
impl crate::GlobalNamespace::IRemoteProcedureCall {
    pub fn from_object_mut(
        object_param: *mut quest_hook::libil2cpp::Il2CppObject,
    ) -> *mut Self {
        unsafe { (object_param as *mut Self) }
    }
    pub fn get_syncTime(&mut self) -> quest_hook::libil2cpp::Result<i64> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i64 = __cordl_object.invoke("get_syncTime", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "IRemoteProcedureCall")]
impl quest_hook::libil2cpp::ObjectType for crate::GlobalNamespace::IRemoteProcedureCall {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "IRemoteProcedureCall")]
impl AsRef<quest_hook::libil2cpp::Gc<crate::GlobalNamespace::IPoolablePacket>>
for crate::GlobalNamespace::IRemoteProcedureCall {
    fn as_ref(
        &self,
    ) -> &quest_hook::libil2cpp::Gc<crate::GlobalNamespace::IPoolablePacket> {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "IRemoteProcedureCall")]
impl AsMut<quest_hook::libil2cpp::Gc<crate::GlobalNamespace::IPoolablePacket>>
for crate::GlobalNamespace::IRemoteProcedureCall {
    fn as_mut(
        &mut self,
    ) -> &mut quest_hook::libil2cpp::Gc<crate::GlobalNamespace::IPoolablePacket> {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "IRemoteProcedureCall")]
impl AsRef<quest_hook::libil2cpp::Gc<crate::LiteNetLib::Utils::INetSerializable>>
for crate::GlobalNamespace::IRemoteProcedureCall {
    fn as_ref(
        &self,
    ) -> &quest_hook::libil2cpp::Gc<crate::LiteNetLib::Utils::INetSerializable> {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "IRemoteProcedureCall")]
impl AsMut<quest_hook::libil2cpp::Gc<crate::LiteNetLib::Utils::INetSerializable>>
for crate::GlobalNamespace::IRemoteProcedureCall {
    fn as_mut(
        &mut self,
    ) -> &mut quest_hook::libil2cpp::Gc<crate::LiteNetLib::Utils::INetSerializable> {
        unsafe { std::mem::transmute(self) }
    }
}
