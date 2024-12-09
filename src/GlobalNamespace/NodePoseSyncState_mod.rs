#[cfg(feature = "NodePoseSyncState")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct NodePoseSyncState {
    pub _head: crate::GlobalNamespace::PoseSerializable,
    pub _leftController: crate::GlobalNamespace::PoseSerializable,
    pub _rightController: crate::GlobalNamespace::PoseSerializable,
}
#[cfg(feature = "NodePoseSyncState")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::NodePoseSyncState => ""
    ."NodePoseSyncState"
);
#[cfg(feature = "NodePoseSyncState")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::GlobalNamespace::NodePoseSyncState {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "NodePoseSyncState")]
impl crate::GlobalNamespace::NodePoseSyncState {
    #[cfg(feature = "NodePoseSyncState+NodePose")]
    pub type NodePose = crate::GlobalNamespace::NodePoseSyncState_NodePose;
    pub fn ApplyDelta(
        &mut self,
        delta: quest_hook::libil2cpp::ByRefMut<crate::GlobalNamespace::NodePoseSyncState>,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::NodePoseSyncState> {
        let __cordl_ret: crate::GlobalNamespace::NodePoseSyncState = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "ApplyDelta",
            (delta),
        )?;
        Ok(__cordl_ret)
    }
    pub fn Deserialize(
        &mut self,
        reader: *mut crate::LiteNetLib::Utils::NetDataReader,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Deserialize",
            (reader),
        )?;
        Ok(__cordl_ret)
    }
    pub fn Equals(
        &mut self,
        other: quest_hook::libil2cpp::ByRefMut<crate::GlobalNamespace::NodePoseSyncState>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Equals",
            (other),
        )?;
        Ok(__cordl_ret)
    }
    pub fn GetDelta(
        &mut self,
        latest: quest_hook::libil2cpp::ByRefMut<
            crate::GlobalNamespace::NodePoseSyncState,
        >,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::NodePoseSyncState> {
        let __cordl_ret: crate::GlobalNamespace::NodePoseSyncState = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "GetDelta",
            (latest),
        )?;
        Ok(__cordl_ret)
    }
    pub fn GetSize(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "GetSize",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn GetState(
        &mut self,
        nodePose: crate::GlobalNamespace::NodePoseSyncState_NodePose,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::PoseSerializable> {
        let __cordl_ret: crate::GlobalNamespace::PoseSerializable = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "GetState",
            (nodePose),
        )?;
        Ok(__cordl_ret)
    }
    pub fn IEquatableByReference_NodePoseSyncState__Equals(
        &mut self,
        other: quest_hook::libil2cpp::ByRefMut<crate::GlobalNamespace::NodePoseSyncState>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "IEquatableByReference<NodePoseSyncState>.Equals",
            (other),
        )?;
        Ok(__cordl_ret)
    }
    pub fn IStateTable_NodePoseSyncState_NodePoseSyncState_NodePose_PoseSerializable__ApplyDelta(
        &mut self,
        delta: quest_hook::libil2cpp::ByRefMut<crate::GlobalNamespace::NodePoseSyncState>,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::NodePoseSyncState> {
        let __cordl_ret: crate::GlobalNamespace::NodePoseSyncState = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "IStateTable<NodePoseSyncState,NodePoseSyncState.NodePose,PoseSerializable>.ApplyDelta",
            (delta),
        )?;
        Ok(__cordl_ret)
    }
    pub fn IStateTable_NodePoseSyncState_NodePoseSyncState_NodePose_PoseSerializable__GetDelta(
        &mut self,
        stateTable: quest_hook::libil2cpp::ByRefMut<
            crate::GlobalNamespace::NodePoseSyncState,
        >,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::NodePoseSyncState> {
        let __cordl_ret: crate::GlobalNamespace::NodePoseSyncState = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "IStateTable<NodePoseSyncState,NodePoseSyncState.NodePose,PoseSerializable>.GetDelta",
            (stateTable),
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
    pub fn SetState(
        &mut self,
        nodePose: crate::GlobalNamespace::NodePoseSyncState_NodePose,
        pose: crate::GlobalNamespace::PoseSerializable,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "SetState",
            (nodePose, pose),
        )?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "NodePoseSyncState+NodePose")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NodePoseSyncState_NodePose {
    Count = 3i32,
    Head = 0i32,
    LeftController = 1i32,
    RightController = 2i32,
}
#[cfg(feature = "NodePoseSyncState+NodePose")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::NodePoseSyncState_NodePose =>
    ""."NodePoseSyncState/NodePose"
);
