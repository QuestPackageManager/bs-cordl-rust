#[cfg(feature = "NetworkConstants")]
#[repr(C)]
#[derive(Debug)]
pub struct NetworkConstants {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "NetworkConstants")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for NetworkConstants => ""."NetworkConstants"
);
#[cfg(feature = "NetworkConstants")]
impl std::ops::Deref for NetworkConstants {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "NetworkConstants")]
impl std::ops::DerefMut for NetworkConstants {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "NetworkConstants")]
impl NetworkConstants {
    pub const backgroundedState: &'static str = "backgrounded";
    pub const dedicatedServerState: &'static str = "dedicated_server";
    pub const finishedLevel: &'static str = "finished_level";
    pub const isActive: &'static str = "is_active";
    pub const kDedicatedServerMasterServerMessageType: u32 = 1679819522u32;
    pub const kGameLiftMessageType: u32 = 1701060611u32;
    pub const kHandshakeMessageType: u32 = 1197129456u32;
    pub const kProtocolVersion: u32 = 1522462729u32;
    pub const kUserMasterServerMessageType: u32 = 537068033u32;
    pub const playerState: &'static str = "player";
    pub const spectatingState: &'static str = "spectating";
    pub const terminatingState: &'static str = "terminating";
    pub const wantsToPlayNextLevel: &'static str = "wants_to_play_next_level";
    pub const wasActiveAtLevelStart: &'static str = "was_active_at_level_start";
}
#[cfg(feature = "NetworkConstants")]
impl quest_hook::libil2cpp::ObjectType for NetworkConstants {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
