#[cfg(feature = "BeatSaber+AvatarCore+OptionalAvatarDataSyncHandler")]
#[repr(C)]
#[derive(Debug)]
pub struct OptionalAvatarDataSyncHandler {
    __cordl_parent: crate::System::Object,
    pub didChangeOptionalAvatarDataEvent: *mut crate::System::Action_2<
        *mut IConnectedPlayer,
        crate::BeatSaber::AvatarCore::OptionalAvatarData,
    >,
    pub _latestOptionalAvatarDataDictionary: *mut crate::System::Collections::Generic::Dictionary_2<
        *mut IConnectedPlayer,
        *mut crate::System::Collections::Generic::Dictionary_2<
            u32,
            crate::BeatSaber::AvatarCore::OptionalAvatarData,
        >,
    >,
    pub _multiplayerSessionManager: *mut IMultiplayerSessionManager,
}
#[cfg(feature = "BeatSaber+AvatarCore+OptionalAvatarDataSyncHandler")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::BeatSaber::AvatarCore::OptionalAvatarDataSyncHandler => "BeatSaber.AvatarCore"
    ."OptionalAvatarDataSyncHandler"
);
#[cfg(feature = "BeatSaber+AvatarCore+OptionalAvatarDataSyncHandler")]
impl std::ops::Deref for crate::BeatSaber::AvatarCore::OptionalAvatarDataSyncHandler {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BeatSaber+AvatarCore+OptionalAvatarDataSyncHandler")]
impl std::ops::DerefMut for crate::BeatSaber::AvatarCore::OptionalAvatarDataSyncHandler {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BeatSaber+AvatarCore+OptionalAvatarDataSyncHandler")]
impl crate::BeatSaber::AvatarCore::OptionalAvatarDataSyncHandler {
    pub fn Dispose(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Dispose", ())?;
        Ok(__cordl_ret)
    }
    pub fn HandleOptionalAvatarDataChanged(
        &mut self,
        dataType: u32,
        data: *mut ByteArrayNetSerializable,
        connectedPlayer: *mut IConnectedPlayer,
    ) -> quest_hook::libil2cpp::Result<
        crate::BeatSaber::AvatarCore::OptionalAvatarData,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::BeatSaber::AvatarCore::OptionalAvatarData = __cordl_object
            .invoke(
                "HandleOptionalAvatarDataChanged",
                (dataType, data, connectedPlayer),
            )?;
        Ok(__cordl_ret)
    }
    pub fn HandleOptionalAvatarDataPacket(
        &mut self,
        packet: *mut crate::BeatSaber::AvatarCore::OptionalAvatarDataPacket,
        connectedPlayer: *mut IConnectedPlayer,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleOptionalAvatarDataPacket", (packet, connectedPlayer))?;
        Ok(__cordl_ret)
    }
    pub fn New(
        multiplayerSessionManager: *mut IMultiplayerSessionManager,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (multiplayerSessionManager))?;
        Ok(__cordl_object)
    }
    pub fn SendOptionalAvatarData(
        &mut self,
        data: crate::BeatSaber::AvatarCore::OptionalAvatarData,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SendOptionalAvatarData", (data))?;
        Ok(__cordl_ret)
    }
    pub fn TryGetAllLatestOptionalAvatarData(
        &mut self,
        connectedPlayer: *mut IConnectedPlayer,
        data: quest_hook::libil2cpp::ByRefMut<
            *mut crate::System::Collections::Generic::Dictionary_2<
                u32,
                crate::BeatSaber::AvatarCore::OptionalAvatarData,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("TryGetAllLatestOptionalAvatarData", (connectedPlayer, data))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
        multiplayerSessionManager: *mut IMultiplayerSessionManager,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (multiplayerSessionManager))?;
        Ok(__cordl_ret)
    }
    pub fn add_didChangeOptionalAvatarDataEvent(
        &mut self,
        value: *mut crate::System::Action_2<
            *mut IConnectedPlayer,
            crate::BeatSaber::AvatarCore::OptionalAvatarData,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_didChangeOptionalAvatarDataEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn remove_didChangeOptionalAvatarDataEvent(
        &mut self,
        value: *mut crate::System::Action_2<
            *mut IConnectedPlayer,
            crate::BeatSaber::AvatarCore::OptionalAvatarData,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_didChangeOptionalAvatarDataEvent", (value))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "BeatSaber+AvatarCore+OptionalAvatarDataSyncHandler")]
impl quest_hook::libil2cpp::ObjectType
for crate::BeatSaber::AvatarCore::OptionalAvatarDataSyncHandler {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
