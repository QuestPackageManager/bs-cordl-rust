#[cfg(feature = "BeatSaber+AvatarCore+ConnectedPlayerOptionalAvatarDataProvider")]
#[repr(C)]
#[derive(Debug)]
pub struct ConnectedPlayerOptionalAvatarDataProvider {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub dataDidChangeEvent: quest_hook::libil2cpp::Gc<
        crate::System::Action_1<crate::BeatSaber::AvatarCore::OptionalAvatarData>,
    >,
    pub _connectedPlayer: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::IConnectedPlayer,
    >,
    pub _multiplayerSessionManager: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::IMultiplayerSessionManager,
    >,
    pub _optionalAvatarDataSyncHandler: quest_hook::libil2cpp::Gc<
        crate::BeatSaber::AvatarCore::OptionalAvatarDataSyncHandler,
    >,
}
#[cfg(feature = "BeatSaber+AvatarCore+ConnectedPlayerOptionalAvatarDataProvider")]
unsafe impl quest_hook::libil2cpp::Type
for crate::BeatSaber::AvatarCore::ConnectedPlayerOptionalAvatarDataProvider {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "BeatSaber.AvatarCore";
    const CLASS_NAME: &'static str = "ConnectedPlayerOptionalAvatarDataProvider";
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.class().is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_value_argument(_: &quest_hook::libil2cpp::Il2CppType) -> bool {
        false
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
    fn matches_value_parameter(_: &quest_hook::libil2cpp::Il2CppType) -> bool {
        false
    }
}
#[cfg(feature = "BeatSaber+AvatarCore+ConnectedPlayerOptionalAvatarDataProvider")]
impl std::ops::Deref
for crate::BeatSaber::AvatarCore::ConnectedPlayerOptionalAvatarDataProvider {
    type Target = quest_hook::libil2cpp::Il2CppObject;
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
        Ok(__cordl_ret.into())
    }
    pub fn HandleOptionalAvatarDataSyncHandlerDidChangeOptionalAvatarDataSync(
        &mut self,
        connectedPlayer: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::IConnectedPlayer,
        >,
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
        Ok(__cordl_ret.into())
    }
    pub fn New(
        connectedPlayer: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::IConnectedPlayer,
        >,
        multiplayerSessionManager: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::IMultiplayerSessionManager,
        >,
        optionalAvatarDataSyncHandler: quest_hook::libil2cpp::Gc<
            crate::BeatSaber::AvatarCore::OptionalAvatarDataSyncHandler,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
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
        Ok(__cordl_object.into())
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
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        connectedPlayer: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::IConnectedPlayer,
        >,
        multiplayerSessionManager: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::IMultiplayerSessionManager,
        >,
        optionalAvatarDataSyncHandler: quest_hook::libil2cpp::Gc<
            crate::BeatSaber::AvatarCore::OptionalAvatarDataSyncHandler,
        >,
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
        Ok(__cordl_ret.into())
    }
    pub fn add_dataDidChangeEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::System::Action_1<crate::BeatSaber::AvatarCore::OptionalAvatarData>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_dataDidChangeEvent", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_currentData(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::Dictionary_2<
                u32,
                crate::BeatSaber::AvatarCore::OptionalAvatarData,
            >,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::Dictionary_2<
                u32,
                crate::BeatSaber::AvatarCore::OptionalAvatarData,
            >,
        > = __cordl_object.invoke("get_currentData", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_playbackDelaySeconds(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("get_playbackDelaySeconds", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn remove_dataDidChangeEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::System::Action_1<crate::BeatSaber::AvatarCore::OptionalAvatarData>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_dataDidChangeEvent", (value))?;
        Ok(__cordl_ret.into())
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
#[cfg(feature = "BeatSaber+AvatarCore+ConnectedPlayerOptionalAvatarDataProvider")]
impl AsRef<crate::BeatSaber::AvatarCore::IOptionalAvatarDataProvider>
for crate::BeatSaber::AvatarCore::ConnectedPlayerOptionalAvatarDataProvider {
    fn as_ref(&self) -> &crate::BeatSaber::AvatarCore::IOptionalAvatarDataProvider {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "BeatSaber+AvatarCore+ConnectedPlayerOptionalAvatarDataProvider")]
impl AsMut<crate::BeatSaber::AvatarCore::IOptionalAvatarDataProvider>
for crate::BeatSaber::AvatarCore::ConnectedPlayerOptionalAvatarDataProvider {
    fn as_mut(
        &mut self,
    ) -> &mut crate::BeatSaber::AvatarCore::IOptionalAvatarDataProvider {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "BeatSaber+AvatarCore+ConnectedPlayerOptionalAvatarDataProvider")]
impl AsRef<crate::System::IDisposable>
for crate::BeatSaber::AvatarCore::ConnectedPlayerOptionalAvatarDataProvider {
    fn as_ref(&self) -> &crate::System::IDisposable {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "BeatSaber+AvatarCore+ConnectedPlayerOptionalAvatarDataProvider")]
impl AsMut<crate::System::IDisposable>
for crate::BeatSaber::AvatarCore::ConnectedPlayerOptionalAvatarDataProvider {
    fn as_mut(&mut self) -> &mut crate::System::IDisposable {
        unsafe { std::mem::transmute(self) }
    }
}
