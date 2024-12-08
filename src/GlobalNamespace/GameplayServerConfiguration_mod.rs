#[cfg(feature = "GameplayServerConfiguration")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct GameplayServerConfiguration {
    pub maxPlayerCount: i32,
    pub discoveryPolicy: DiscoveryPolicy,
    pub invitePolicy: InvitePolicy,
    pub gameplayServerMode: GameplayServerMode,
    pub songSelectionMode: SongSelectionMode,
    pub gameplayServerControlSettings: GameplayServerControlSettings,
}
#[cfg(feature = "GameplayServerConfiguration")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for GameplayServerConfiguration => ""
    ."GameplayServerConfiguration"
);
#[cfg(feature = "GameplayServerConfiguration")]
unsafe impl quest_hook::libil2cpp::ThisArgument for GameplayServerConfiguration {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "GameplayServerConfiguration")]
impl GameplayServerConfiguration {
    pub fn CreateFromSerializedData(
        &mut self,
        reader: *mut crate::LiteNetLib::Utils::NetDataReader,
    ) -> quest_hook::libil2cpp::Result<GameplayServerConfiguration> {
        let __cordl_ret: GameplayServerConfiguration = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "CreateFromSerializedData",
            (reader),
        )?;
        Ok(__cordl_ret)
    }
    pub fn Equals_ByRefMut0(
        &mut self,
        other: quest_hook::libil2cpp::ByRefMut<GameplayServerConfiguration>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Equals",
            (other),
        )?;
        Ok(__cordl_ret)
    }
    pub fn Equals_GameplayServerConfiguration1(
        &mut self,
        other: GameplayServerConfiguration,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Equals",
            (other),
        )?;
        Ok(__cordl_ret)
    }
    pub fn Equals_Object2(
        &mut self,
        obj: *mut crate::System::Object,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Equals",
            (obj),
        )?;
        Ok(__cordl_ret)
    }
    pub fn GetHashCode(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "GetHashCode",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn Serialize(
        &mut self,
        writer: *mut crate::LiteNetLib::Utils::NetDataWriter,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Serialize",
            (writer),
        )?;
        Ok(__cordl_ret)
    }
    pub fn WithMaxPlayerCount(
        &mut self,
        maxPlayerCount: i32,
    ) -> quest_hook::libil2cpp::Result<GameplayServerConfiguration> {
        let __cordl_ret: GameplayServerConfiguration = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "WithMaxPlayerCount",
            (maxPlayerCount),
        )?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
        maxPlayerCount: i32,
        discoveryPolicy: DiscoveryPolicy,
        invitePolicy: InvitePolicy,
        gameplayServerMode: GameplayServerMode,
        songSelectionMode: SongSelectionMode,
        gameplayServerControlSettings: GameplayServerControlSettings,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (
                maxPlayerCount,
                discoveryPolicy,
                invitePolicy,
                gameplayServerMode,
                songSelectionMode,
                gameplayServerControlSettings,
            ),
        )?;
        Ok(__cordl_ret)
    }
}
