#[cfg(feature = "GameLiftClientConnectionRequestHandler")]
#[repr(C)]
#[derive(Debug)]
pub struct GameLiftClientConnectionRequestHandler {
    __cordl_parent: crate::System::Object,
    pub _playerSessionId_k__BackingField: *mut crate::System::String,
}
#[cfg(feature = "GameLiftClientConnectionRequestHandler")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for GameLiftClientConnectionRequestHandler => ""
    ."GameLiftClientConnectionRequestHandler"
);
#[cfg(feature = "GameLiftClientConnectionRequestHandler")]
impl std::ops::Deref for GameLiftClientConnectionRequestHandler {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "GameLiftClientConnectionRequestHandler")]
impl std::ops::DerefMut for GameLiftClientConnectionRequestHandler {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "GameLiftClientConnectionRequestHandler")]
impl GameLiftClientConnectionRequestHandler {
    pub fn GetConnectionMessage(
        &mut self,
        writer: *mut crate::LiteNetLib::Utils::NetDataWriter,
        userId: *mut crate::System::String,
        userName: *mut crate::System::String,
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
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
    pub fn ValidateConnectionMessage(
        &mut self,
        reader: *mut crate::LiteNetLib::Utils::NetDataReader,
        userId: quest_hook::libil2cpp::ByRefMut<*mut crate::System::String>,
        userName: quest_hook::libil2cpp::ByRefMut<*mut crate::System::String>,
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
    pub fn get_playerSessionId(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("get_playerSessionId", ())?;
        Ok(__cordl_ret)
    }
    pub fn set_playerSessionId(
        &mut self,
        value: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_playerSessionId", (value))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "GameLiftClientConnectionRequestHandler")]
impl quest_hook::libil2cpp::ObjectType for GameLiftClientConnectionRequestHandler {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
