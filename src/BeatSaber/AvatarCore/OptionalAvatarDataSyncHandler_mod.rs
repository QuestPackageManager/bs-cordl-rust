#[cfg(feature = "BeatSaber+AvatarCore+OptionalAvatarDataSyncHandler")]
#[repr(C)]
#[derive(Debug)]
pub struct OptionalAvatarDataSyncHandler {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub didChangeOptionalAvatarDataEvent: quest_hook::libil2cpp::Gc<
        crate::System::Action_2<
            quest_hook::libil2cpp::Gc<crate::GlobalNamespace::IConnectedPlayer>,
            crate::BeatSaber::AvatarCore::OptionalAvatarData,
        >,
    >,
    pub _latestOptionalAvatarDataDictionary: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::Dictionary_2<
            quest_hook::libil2cpp::Gc<crate::GlobalNamespace::IConnectedPlayer>,
            quest_hook::libil2cpp::Gc<
                crate::System::Collections::Generic::Dictionary_2<
                    u32,
                    crate::BeatSaber::AvatarCore::OptionalAvatarData,
                >,
            >,
        >,
    >,
    pub _multiplayerSessionManager: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::IMultiplayerSessionManager,
    >,
}
#[cfg(feature = "BeatSaber+AvatarCore+OptionalAvatarDataSyncHandler")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::BeatSaber::AvatarCore::OptionalAvatarDataSyncHandler => "BeatSaber.AvatarCore"
    ."OptionalAvatarDataSyncHandler"
);
#[cfg(feature = "BeatSaber+AvatarCore+OptionalAvatarDataSyncHandler")]
impl std::ops::Deref for crate::BeatSaber::AvatarCore::OptionalAvatarDataSyncHandler {
    type Target = quest_hook::libil2cpp::Il2CppObject;
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
        Ok(__cordl_ret.into())
    }
    pub fn HandleOptionalAvatarDataChanged(
        &mut self,
        dataType: u32,
        data: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::ByteArrayNetSerializable,
        >,
        connectedPlayer: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::IConnectedPlayer,
        >,
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
        Ok(__cordl_ret.into())
    }
    pub fn HandleOptionalAvatarDataPacket(
        &mut self,
        packet: quest_hook::libil2cpp::Gc<
            crate::BeatSaber::AvatarCore::OptionalAvatarDataPacket,
        >,
        connectedPlayer: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::IConnectedPlayer,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleOptionalAvatarDataPacket", (packet, connectedPlayer))?;
        Ok(__cordl_ret.into())
    }
    pub fn New(
        multiplayerSessionManager: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::IMultiplayerSessionManager,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (multiplayerSessionManager))?;
        Ok(__cordl_object.into())
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
        Ok(__cordl_ret.into())
    }
    pub fn TryGetAllLatestOptionalAvatarData(
        &mut self,
        connectedPlayer: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::IConnectedPlayer,
        >,
        data: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<
                crate::System::Collections::Generic::Dictionary_2<
                    u32,
                    crate::BeatSaber::AvatarCore::OptionalAvatarData,
                >,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("TryGetAllLatestOptionalAvatarData", (connectedPlayer, data))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        multiplayerSessionManager: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::IMultiplayerSessionManager,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (multiplayerSessionManager))?;
        Ok(__cordl_ret.into())
    }
    pub fn add_didChangeOptionalAvatarDataEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::System::Action_2<
                quest_hook::libil2cpp::Gc<crate::GlobalNamespace::IConnectedPlayer>,
                crate::BeatSaber::AvatarCore::OptionalAvatarData,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_didChangeOptionalAvatarDataEvent", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn remove_didChangeOptionalAvatarDataEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::System::Action_2<
                quest_hook::libil2cpp::Gc<crate::GlobalNamespace::IConnectedPlayer>,
                crate::BeatSaber::AvatarCore::OptionalAvatarData,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_didChangeOptionalAvatarDataEvent", (value))?;
        Ok(__cordl_ret.into())
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
#[cfg(feature = "BeatSaber+AvatarCore+OptionalAvatarDataSyncHandler")]
impl AsRef<crate::System::IDisposable>
for crate::BeatSaber::AvatarCore::OptionalAvatarDataSyncHandler {
    fn as_ref(&self) -> &crate::System::IDisposable {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "BeatSaber+AvatarCore+OptionalAvatarDataSyncHandler")]
impl AsMut<crate::System::IDisposable>
for crate::BeatSaber::AvatarCore::OptionalAvatarDataSyncHandler {
    fn as_mut(&mut self) -> &mut crate::System::IDisposable {
        unsafe { std::mem::transmute(self) }
    }
}
