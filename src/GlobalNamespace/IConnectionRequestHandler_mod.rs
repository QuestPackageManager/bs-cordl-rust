#[cfg(feature = "IConnectionRequestHandler")]
#[repr(C)]
#[derive(Debug)]
pub struct IConnectionRequestHandler {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "IConnectionRequestHandler")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::IConnectionRequestHandler => ""
    ."IConnectionRequestHandler"
);
#[cfg(feature = "IConnectionRequestHandler")]
impl std::ops::Deref for crate::GlobalNamespace::IConnectionRequestHandler {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "IConnectionRequestHandler")]
impl std::ops::DerefMut for crate::GlobalNamespace::IConnectionRequestHandler {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "IConnectionRequestHandler")]
impl crate::GlobalNamespace::IConnectionRequestHandler {
    pub fn GetConnectionMessage(
        &mut self,
        writer: *mut crate::LiteNetLib::Utils::NetDataWriter,
        userId: *mut quest_hook::libil2cpp::Il2CppString,
        userName: *mut quest_hook::libil2cpp::Il2CppString,
        isConnectionOwner: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "GetConnectionMessage",
                (writer, userId, userName, isConnectionOwner),
            )?;
        Ok(__cordl_ret)
    }
    pub fn ValidateConnectionMessage(
        &mut self,
        reader: *mut crate::LiteNetLib::Utils::NetDataReader,
        userId: quest_hook::libil2cpp::ByRefMut<
            *mut quest_hook::libil2cpp::Il2CppString,
        >,
        userName: quest_hook::libil2cpp::ByRefMut<
            *mut quest_hook::libil2cpp::Il2CppString,
        >,
        isConnectionOwner: quest_hook::libil2cpp::ByRefMut<bool>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke(
                "ValidateConnectionMessage",
                (reader, userId, userName, isConnectionOwner),
            )?;
        Ok(__cordl_ret)
    }
    pub fn from_object_mut(
        object_param: *mut quest_hook::libil2cpp::Il2CppObject,
    ) -> *mut Self {
        unsafe { (object_param as *mut Self) }
    }
}
#[cfg(feature = "IConnectionRequestHandler")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::IConnectionRequestHandler {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
