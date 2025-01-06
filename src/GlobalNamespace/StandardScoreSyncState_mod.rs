#[cfg(feature = "StandardScoreSyncState")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct StandardScoreSyncState {
    pub _modifiedScore: i32,
    pub _multipliedScore: i32,
    pub _immediateMaxPossibleMultipliedScore: i32,
    pub _combo: i32,
    pub _multiplier: i32,
}
#[cfg(feature = "StandardScoreSyncState")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::StandardScoreSyncState => ""
    ."StandardScoreSyncState"
);
#[cfg(feature = "StandardScoreSyncState")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::GlobalNamespace::StandardScoreSyncState {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "StandardScoreSyncState")]
impl crate::GlobalNamespace::StandardScoreSyncState {
    #[cfg(feature = "StandardScoreSyncState+Score")]
    pub type Score = crate::GlobalNamespace::StandardScoreSyncState_Score;
    pub fn ApplyDelta(
        &mut self,
        delta: quest_hook::libil2cpp::ByRefMut<
            crate::GlobalNamespace::StandardScoreSyncState,
        >,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::StandardScoreSyncState> {
        let __cordl_ret: crate::GlobalNamespace::StandardScoreSyncState = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "ApplyDelta",
            (delta),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn Deserialize(
        &mut self,
        reader: quest_hook::libil2cpp::Gc<crate::LiteNetLib::Utils::NetDataReader>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Deserialize",
            (reader),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn Equals(
        &mut self,
        other: quest_hook::libil2cpp::ByRefMut<
            crate::GlobalNamespace::StandardScoreSyncState,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Equals",
            (other),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn GetDelta(
        &mut self,
        stateTable: quest_hook::libil2cpp::ByRefMut<
            crate::GlobalNamespace::StandardScoreSyncState,
        >,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::StandardScoreSyncState> {
        let __cordl_ret: crate::GlobalNamespace::StandardScoreSyncState = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "GetDelta",
            (stateTable),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn GetSize(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "GetSize",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn GetState(
        &mut self,
        s: crate::GlobalNamespace::StandardScoreSyncState_Score,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "GetState",
            (s),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn IEquatableByReference_StandardScoreSyncState__Equals(
        &mut self,
        other: quest_hook::libil2cpp::ByRefMut<
            crate::GlobalNamespace::StandardScoreSyncState,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "IEquatableByReference<StandardScoreSyncState>.Equals",
            (other),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn IStateTable_StandardScoreSyncState_StandardScoreSyncState_Score_System_Int32__ApplyDelta(
        &mut self,
        delta: quest_hook::libil2cpp::ByRefMut<
            crate::GlobalNamespace::StandardScoreSyncState,
        >,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::StandardScoreSyncState> {
        let __cordl_ret: crate::GlobalNamespace::StandardScoreSyncState = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "IStateTable<StandardScoreSyncState,StandardScoreSyncState.Score,System.Int32>.ApplyDelta",
            (delta),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn IStateTable_StandardScoreSyncState_StandardScoreSyncState_Score_System_Int32__GetDelta(
        &mut self,
        stateTable: quest_hook::libil2cpp::ByRefMut<
            crate::GlobalNamespace::StandardScoreSyncState,
        >,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::StandardScoreSyncState> {
        let __cordl_ret: crate::GlobalNamespace::StandardScoreSyncState = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "IStateTable<StandardScoreSyncState,StandardScoreSyncState.Score,System.Int32>.GetDelta",
            (stateTable),
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
    pub fn SetState(
        &mut self,
        s: crate::GlobalNamespace::StandardScoreSyncState_Score,
        value: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "SetState",
            (s, value),
        )?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "StandardScoreSyncState")]
impl AsRef<
    crate::GlobalNamespace::IEquatableByReference_1<
        crate::GlobalNamespace::StandardScoreSyncState,
    >,
> for crate::GlobalNamespace::StandardScoreSyncState {
    fn as_ref(
        &self,
    ) -> &crate::GlobalNamespace::IEquatableByReference_1<
        crate::GlobalNamespace::StandardScoreSyncState,
    > {
        todo!()
    }
}
#[cfg(feature = "StandardScoreSyncState")]
impl AsMut<
    crate::GlobalNamespace::IEquatableByReference_1<
        crate::GlobalNamespace::StandardScoreSyncState,
    >,
> for crate::GlobalNamespace::StandardScoreSyncState {
    fn as_mut(
        &mut self,
    ) -> &mut crate::GlobalNamespace::IEquatableByReference_1<
        crate::GlobalNamespace::StandardScoreSyncState,
    > {
        todo!()
    }
}
#[cfg(feature = "StandardScoreSyncState")]
impl AsRef<
    crate::GlobalNamespace::IStateTable_3<
        crate::GlobalNamespace::StandardScoreSyncState,
        crate::GlobalNamespace::StandardScoreSyncState_Score,
        i32,
    >,
> for crate::GlobalNamespace::StandardScoreSyncState {
    fn as_ref(
        &self,
    ) -> &crate::GlobalNamespace::IStateTable_3<
        crate::GlobalNamespace::StandardScoreSyncState,
        crate::GlobalNamespace::StandardScoreSyncState_Score,
        i32,
    > {
        todo!()
    }
}
#[cfg(feature = "StandardScoreSyncState")]
impl AsMut<
    crate::GlobalNamespace::IStateTable_3<
        crate::GlobalNamespace::StandardScoreSyncState,
        crate::GlobalNamespace::StandardScoreSyncState_Score,
        i32,
    >,
> for crate::GlobalNamespace::StandardScoreSyncState {
    fn as_mut(
        &mut self,
    ) -> &mut crate::GlobalNamespace::IStateTable_3<
        crate::GlobalNamespace::StandardScoreSyncState,
        crate::GlobalNamespace::StandardScoreSyncState_Score,
        i32,
    > {
        todo!()
    }
}
#[cfg(feature = "StandardScoreSyncState")]
impl AsRef<crate::LiteNetLib::Utils::INetSerializable>
for crate::GlobalNamespace::StandardScoreSyncState {
    fn as_ref(&self) -> &crate::LiteNetLib::Utils::INetSerializable {
        todo!()
    }
}
#[cfg(feature = "StandardScoreSyncState")]
impl AsMut<crate::LiteNetLib::Utils::INetSerializable>
for crate::GlobalNamespace::StandardScoreSyncState {
    fn as_mut(&mut self) -> &mut crate::LiteNetLib::Utils::INetSerializable {
        todo!()
    }
}
#[cfg(feature = "StandardScoreSyncState+Score")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum StandardScoreSyncState_Score {
    #[default]
    Combo = 3i32,
    Count = 5i32,
    ImmediateMaxPossibleMultipliedScore = 2i32,
    ModifiedScore = 0i32,
    MultipliedScore = 1i32,
    Multiplier = 4i32,
}
#[cfg(feature = "StandardScoreSyncState+Score")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::StandardScoreSyncState_Score =>
    ""."StandardScoreSyncState/Score"
);
