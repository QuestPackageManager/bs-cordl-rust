#[cfg(feature = "BeatSaber+AvatarCore+ConnectedPlayerOptionalAvatarDataProvider")]
#[repr(C)]
#[derive(Debug)]
pub struct ConnectedPlayerOptionalAvatarDataProvider {
    __cordl_parent: crate::System::Object,
    pub dataDidChangeEvent: *mut crate::System::Action_1<
        crate::BeatSaber::AvatarCore::OptionalAvatarData,
    >,
    pub _connectedPlayer: *mut IConnectedPlayer,
    pub _multiplayerSessionManager: *mut IMultiplayerSessionManager,
    pub _optionalAvatarDataSyncHandler: *mut crate::BeatSaber::AvatarCore::OptionalAvatarDataSyncHandler,
}
#[cfg(feature = "BeatSaber+AvatarCore+ConnectedPlayerOptionalAvatarDataProvider")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::BeatSaber::AvatarCore::ConnectedPlayerOptionalAvatarDataProvider =>
    "BeatSaber.AvatarCore"."ConnectedPlayerOptionalAvatarDataProvider"
);
#[cfg(feature = "BeatSaber+AvatarCore+ConnectedPlayerOptionalAvatarDataProvider")]
impl std::ops::Deref
for crate::BeatSaber::AvatarCore::ConnectedPlayerOptionalAvatarDataProvider {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BeatSaber+AvatarCore+ConnectedPlayerOptionalAvatarDataProvider")]
impl std::ops::DerefMut
for crate::BeatSaber::AvatarCore::ConnectedPlayerOptionalAvatarDataProvider {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BeatSaber+AvatarCore+ConnectedPlayerOptionalAvatarDataProvider")]
impl crate::BeatSaber::AvatarCore::ConnectedPlayerOptionalAvatarDataProvider {
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
    pub fn HandleOptionalAvatarDataSyncHandlerDidChangeOptionalAvatarDataSync(
        &mut self,
        connectedPlayer: *mut IConnectedPlayer,
        optionalAvatarData: crate::BeatSaber::AvatarCore::OptionalAvatarData,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "HandleOptionalAvatarDataSyncHandlerDidChangeOptionalAvatarDataSync",
                (connectedPlayer, optionalAvatarData),
            )?;
        Ok(__cordl_ret)
    }
    pub fn New(
        connectedPlayer: *mut IConnectedPlayer,
        multiplayerSessionManager: *mut IMultiplayerSessionManager,
        optionalAvatarDataSyncHandler: *mut crate::BeatSaber::AvatarCore::OptionalAvatarDataSyncHandler,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(
                ".ctor",
                (
                    connectedPlayer,
                    multiplayerSessionManager,
                    optionalAvatarDataSyncHandler,
                ),
            )?;
        Ok(__cordl_object)
    }
    pub fn SetDisplayContext(
        &mut self,
        avatarDisplayContext: crate::BeatSaber::AvatarCore::AvatarDisplayContext,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetDisplayContext", (avatarDisplayContext))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
        connectedPlayer: *mut IConnectedPlayer,
        multiplayerSessionManager: *mut IMultiplayerSessionManager,
        optionalAvatarDataSyncHandler: *mut crate::BeatSaber::AvatarCore::OptionalAvatarDataSyncHandler,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                ".ctor",
                (
                    connectedPlayer,
                    multiplayerSessionManager,
                    optionalAvatarDataSyncHandler,
                ),
            )?;
        Ok(__cordl_ret)
    }
    pub fn add_dataDidChangeEvent(
        &mut self,
        value: *mut crate::System::Action_1<
            crate::BeatSaber::AvatarCore::OptionalAvatarData,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_dataDidChangeEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn get_currentData(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Collections::Generic::Dictionary_2<
            u32,
            crate::BeatSaber::AvatarCore::OptionalAvatarData,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Collections::Generic::Dictionary_2<
            u32,
            crate::BeatSaber::AvatarCore::OptionalAvatarData,
        > = __cordl_object.invoke("get_currentData", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_playbackDelaySeconds(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("get_playbackDelaySeconds", ())?;
        Ok(__cordl_ret)
    }
    pub fn remove_dataDidChangeEvent(
        &mut self,
        value: *mut crate::System::Action_1<
            crate::BeatSaber::AvatarCore::OptionalAvatarData,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_dataDidChangeEvent", (value))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "BeatSaber+AvatarCore+ConnectedPlayerOptionalAvatarDataProvider")]
impl quest_hook::libil2cpp::ObjectType
for crate::BeatSaber::AvatarCore::ConnectedPlayerOptionalAvatarDataProvider {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
