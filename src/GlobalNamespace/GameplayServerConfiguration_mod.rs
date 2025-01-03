#[cfg(feature = "GameplayServerConfiguration")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct GameplayServerConfiguration {
    pub maxPlayerCount: i32,
    pub discoveryPolicy: crate::GlobalNamespace::DiscoveryPolicy,
    pub invitePolicy: crate::GlobalNamespace::InvitePolicy,
    pub gameplayServerMode: crate::GlobalNamespace::GameplayServerMode,
    pub songSelectionMode: crate::GlobalNamespace::SongSelectionMode,
    pub gameplayServerControlSettings: crate::GlobalNamespace::GameplayServerControlSettings,
}
#[cfg(feature = "GameplayServerConfiguration")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::GameplayServerConfiguration =>
    ""."GameplayServerConfiguration"
);
#[cfg(feature = "GameplayServerConfiguration")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::GlobalNamespace::GameplayServerConfiguration {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "GameplayServerConfiguration")]
impl crate::GlobalNamespace::GameplayServerConfiguration {
    pub fn CreateFromSerializedData(
        &mut self,
        reader: quest_hook::libil2cpp::Gc<crate::LiteNetLib::Utils::NetDataReader>,
    ) -> quest_hook::libil2cpp::Result<
        crate::GlobalNamespace::GameplayServerConfiguration,
    > {
        let __cordl_ret: crate::GlobalNamespace::GameplayServerConfiguration = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "CreateFromSerializedData",
            (reader),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn Deserialize(
        reader: quest_hook::libil2cpp::Gc<crate::LiteNetLib::Utils::NetDataReader>,
    ) -> quest_hook::libil2cpp::Result<
        crate::GlobalNamespace::GameplayServerConfiguration,
    > {
        let __cordl_ret: crate::GlobalNamespace::GameplayServerConfiguration = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Deserialize", (reader))?;
        Ok(__cordl_ret.into())
    }
    pub fn Equals_ByRefMut0(
        &mut self,
        other: quest_hook::libil2cpp::ByRefMut<
            crate::GlobalNamespace::GameplayServerConfiguration,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Equals",
            (other),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn Equals_GameplayServerConfiguration1(
        &mut self,
        other: crate::GlobalNamespace::GameplayServerConfiguration,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Equals",
            (other),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn Equals_Il2CppObject2(
        &mut self,
        obj: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Equals",
            (obj),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn GetHashCode(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "GetHashCode",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn Serialize(
        &mut self,
        writer: quest_hook::libil2cpp::Gc<crate::LiteNetLib::Utils::NetDataWriter>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Serialize",
            (writer),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn WithMaxPlayerCount(
        &mut self,
        maxPlayerCount: i32,
    ) -> quest_hook::libil2cpp::Result<
        crate::GlobalNamespace::GameplayServerConfiguration,
    > {
        let __cordl_ret: crate::GlobalNamespace::GameplayServerConfiguration = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "WithMaxPlayerCount",
            (maxPlayerCount),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        maxPlayerCount: i32,
        discoveryPolicy: crate::GlobalNamespace::DiscoveryPolicy,
        invitePolicy: crate::GlobalNamespace::InvitePolicy,
        gameplayServerMode: crate::GlobalNamespace::GameplayServerMode,
        songSelectionMode: crate::GlobalNamespace::SongSelectionMode,
        gameplayServerControlSettings: crate::GlobalNamespace::GameplayServerControlSettings,
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
        Ok(__cordl_ret.into())
    }
    pub fn op_Equality(
        a: quest_hook::libil2cpp::ByRefMut<
            crate::GlobalNamespace::GameplayServerConfiguration,
        >,
        b: quest_hook::libil2cpp::ByRefMut<
            crate::GlobalNamespace::GameplayServerConfiguration,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Equality", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Inequality(
        a: quest_hook::libil2cpp::ByRefMut<
            crate::GlobalNamespace::GameplayServerConfiguration,
        >,
        b: quest_hook::libil2cpp::ByRefMut<
            crate::GlobalNamespace::GameplayServerConfiguration,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Inequality", (a, b))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "GameplayServerConfiguration")]
impl AsRef<
    crate::LiteNetLib::Utils::INetImmutableSerializable_1<
        crate::GlobalNamespace::GameplayServerConfiguration,
    >,
> for crate::GlobalNamespace::GameplayServerConfiguration {
    fn as_ref(
        &self,
    ) -> &crate::LiteNetLib::Utils::INetImmutableSerializable_1<
        crate::GlobalNamespace::GameplayServerConfiguration,
    > {
        todo!()
    }
}
#[cfg(feature = "GameplayServerConfiguration")]
impl AsMut<
    crate::LiteNetLib::Utils::INetImmutableSerializable_1<
        crate::GlobalNamespace::GameplayServerConfiguration,
    >,
> for crate::GlobalNamespace::GameplayServerConfiguration {
    fn as_mut(
        &mut self,
    ) -> &mut crate::LiteNetLib::Utils::INetImmutableSerializable_1<
        crate::GlobalNamespace::GameplayServerConfiguration,
    > {
        todo!()
    }
}
#[cfg(feature = "GameplayServerConfiguration")]
impl AsRef<
    crate::System::IEquatable_1<crate::GlobalNamespace::GameplayServerConfiguration>,
> for crate::GlobalNamespace::GameplayServerConfiguration {
    fn as_ref(
        &self,
    ) -> &crate::System::IEquatable_1<
        crate::GlobalNamespace::GameplayServerConfiguration,
    > {
        todo!()
    }
}
#[cfg(feature = "GameplayServerConfiguration")]
impl AsMut<
    crate::System::IEquatable_1<crate::GlobalNamespace::GameplayServerConfiguration>,
> for crate::GlobalNamespace::GameplayServerConfiguration {
    fn as_mut(
        &mut self,
    ) -> &mut crate::System::IEquatable_1<
        crate::GlobalNamespace::GameplayServerConfiguration,
    > {
        todo!()
    }
}
