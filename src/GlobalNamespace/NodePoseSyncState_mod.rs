#[cfg(feature = "NodePoseSyncState")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct NodePoseSyncState {
    pub _head: crate::GlobalNamespace::PoseSerializable,
    pub _leftController: crate::GlobalNamespace::PoseSerializable,
    pub _rightController: crate::GlobalNamespace::PoseSerializable,
}
#[cfg(feature = "NodePoseSyncState")]
unsafe impl quest_hook::libil2cpp::Type for crate::GlobalNamespace::NodePoseSyncState {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "NodePoseSyncState";
    fn matches_value_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        !ty.is_ref()
            && ty
                .class()
                .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref()
            && ty
                .class()
                .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_value_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        !ty.is_ref()
            && <Self as quest_hook::libil2cpp::Type>::class()
                .is_assignable_from(ty.class())
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref()
            && <Self as quest_hook::libil2cpp::Type>::class()
                .is_assignable_from(ty.class())
    }
}
#[cfg(feature = "NodePoseSyncState")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::GlobalNamespace::NodePoseSyncState {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "NodePoseSyncState")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::GlobalNamespace::NodePoseSyncState {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_parameter(ty)
    }
    fn from_actual(actual: Self::Actual) -> Self {
        actual
    }
    fn into_actual(self) -> Self::Actual {
        self
    }
}
#[cfg(feature = "NodePoseSyncState")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::GlobalNamespace::NodePoseSyncState {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_returned(ty)
    }
    fn from_object(object: Option<&mut quest_hook::libil2cpp::Il2CppObject>) -> Self {
        unsafe {
            quest_hook::libil2cpp::raw::unbox(
                quest_hook::libil2cpp::WrapRaw::raw(object.unwrap()),
            )
        }
    }
}
#[cfg(feature = "NodePoseSyncState")]
unsafe impl quest_hook::libil2cpp::Return for crate::GlobalNamespace::NodePoseSyncState {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_return(ty)
    }
    fn into_actual(self) -> Self::Actual {
        self
    }
    fn from_actual(actual: Self::Actual) -> Self {
        actual
    }
}
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
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::ByRefMut<
                    crate::GlobalNamespace::NodePoseSyncState,
                >),
                crate::GlobalNamespace::NodePoseSyncState,
                1usize,
            >("ApplyDelta")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ApplyDelta", 1usize
                )
            });
        let __cordl_ret: crate::GlobalNamespace::NodePoseSyncState = unsafe {
            method.invoke_unchecked(self, (delta))
        };
        Ok(__cordl_ret.into())
    }
    pub fn Deserialize(
        &mut self,
        reader: quest_hook::libil2cpp::Gc<crate::LiteNetLib::Utils::NetDataReader>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<crate::LiteNetLib::Utils::NetDataReader>),
                quest_hook::libil2cpp::Void,
                1usize,
            >("Deserialize")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "Deserialize", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (reader))
        };
        Ok(__cordl_ret.into())
    }
    pub fn Equals(
        &mut self,
        other: quest_hook::libil2cpp::ByRefMut<crate::GlobalNamespace::NodePoseSyncState>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::ByRefMut<
                    crate::GlobalNamespace::NodePoseSyncState,
                >),
                bool,
                1usize,
            >("Equals")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "Equals", 1usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked(self, (other)) };
        Ok(__cordl_ret.into())
    }
    pub fn GetDelta(
        &mut self,
        latest: quest_hook::libil2cpp::ByRefMut<
            crate::GlobalNamespace::NodePoseSyncState,
        >,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::NodePoseSyncState> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::ByRefMut<
                    crate::GlobalNamespace::NodePoseSyncState,
                >),
                crate::GlobalNamespace::NodePoseSyncState,
                1usize,
            >("GetDelta")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "GetDelta", 1usize
                )
            });
        let __cordl_ret: crate::GlobalNamespace::NodePoseSyncState = unsafe {
            method.invoke_unchecked(self, (latest))
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetSize(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), i32, 0usize>("GetSize")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "GetSize", 0usize
                )
            });
        let __cordl_ret: i32 = unsafe { method.invoke_unchecked(self, ()) };
        Ok(__cordl_ret.into())
    }
    pub fn GetState(
        &mut self,
        nodePose: crate::GlobalNamespace::NodePoseSyncState_NodePose,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::PoseSerializable> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (crate::GlobalNamespace::NodePoseSyncState_NodePose),
                crate::GlobalNamespace::PoseSerializable,
                1usize,
            >("GetState")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "GetState", 1usize
                )
            });
        let __cordl_ret: crate::GlobalNamespace::PoseSerializable = unsafe {
            method.invoke_unchecked(self, (nodePose))
        };
        Ok(__cordl_ret.into())
    }
    pub fn IEquatableByReference_NodePoseSyncState__Equals(
        &mut self,
        other: quest_hook::libil2cpp::ByRefMut<crate::GlobalNamespace::NodePoseSyncState>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::ByRefMut<
                    crate::GlobalNamespace::NodePoseSyncState,
                >),
                bool,
                1usize,
            >("IEquatableByReference<NodePoseSyncState>.Equals")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "IEquatableByReference<NodePoseSyncState>.Equals", 1usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked(self, (other)) };
        Ok(__cordl_ret.into())
    }
    pub fn IStateTable_NodePoseSyncState_NodePoseSyncState_NodePose_PoseSerializable__ApplyDelta(
        &mut self,
        delta: quest_hook::libil2cpp::ByRefMut<crate::GlobalNamespace::NodePoseSyncState>,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::NodePoseSyncState> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::ByRefMut<
                    crate::GlobalNamespace::NodePoseSyncState,
                >),
                crate::GlobalNamespace::NodePoseSyncState,
                1usize,
            >(
                "IStateTable<NodePoseSyncState,NodePoseSyncState.NodePose,PoseSerializable>.ApplyDelta",
            )
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self,
                    "IStateTable<NodePoseSyncState,NodePoseSyncState.NodePose,PoseSerializable>.ApplyDelta",
                    1usize
                )
            });
        let __cordl_ret: crate::GlobalNamespace::NodePoseSyncState = unsafe {
            method.invoke_unchecked(self, (delta))
        };
        Ok(__cordl_ret.into())
    }
    pub fn IStateTable_NodePoseSyncState_NodePoseSyncState_NodePose_PoseSerializable__GetDelta(
        &mut self,
        stateTable: quest_hook::libil2cpp::ByRefMut<
            crate::GlobalNamespace::NodePoseSyncState,
        >,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::NodePoseSyncState> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::ByRefMut<
                    crate::GlobalNamespace::NodePoseSyncState,
                >),
                crate::GlobalNamespace::NodePoseSyncState,
                1usize,
            >(
                "IStateTable<NodePoseSyncState,NodePoseSyncState.NodePose,PoseSerializable>.GetDelta",
            )
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self,
                    "IStateTable<NodePoseSyncState,NodePoseSyncState.NodePose,PoseSerializable>.GetDelta",
                    1usize
                )
            });
        let __cordl_ret: crate::GlobalNamespace::NodePoseSyncState = unsafe {
            method.invoke_unchecked(self, (stateTable))
        };
        Ok(__cordl_ret.into())
    }
    pub fn Serialize(
        &mut self,
        writer: quest_hook::libil2cpp::Gc<crate::LiteNetLib::Utils::NetDataWriter>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<crate::LiteNetLib::Utils::NetDataWriter>),
                quest_hook::libil2cpp::Void,
                1usize,
            >("Serialize")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "Serialize", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (writer))
        };
        Ok(__cordl_ret.into())
    }
    pub fn SetState(
        &mut self,
        nodePose: crate::GlobalNamespace::NodePoseSyncState_NodePose,
        pose: crate::GlobalNamespace::PoseSerializable,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    crate::GlobalNamespace::NodePoseSyncState_NodePose,
                    crate::GlobalNamespace::PoseSerializable,
                ),
                quest_hook::libil2cpp::Void,
                2usize,
            >("SetState")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "SetState", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (nodePose, pose))
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "NodePoseSyncState")]
impl AsRef<
    crate::GlobalNamespace::IEquatableByReference_1<
        crate::GlobalNamespace::NodePoseSyncState,
    >,
> for crate::GlobalNamespace::NodePoseSyncState {
    fn as_ref(
        &self,
    ) -> &crate::GlobalNamespace::IEquatableByReference_1<
        crate::GlobalNamespace::NodePoseSyncState,
    > {
        todo!()
    }
}
#[cfg(feature = "NodePoseSyncState")]
impl AsMut<
    crate::GlobalNamespace::IEquatableByReference_1<
        crate::GlobalNamespace::NodePoseSyncState,
    >,
> for crate::GlobalNamespace::NodePoseSyncState {
    fn as_mut(
        &mut self,
    ) -> &mut crate::GlobalNamespace::IEquatableByReference_1<
        crate::GlobalNamespace::NodePoseSyncState,
    > {
        todo!()
    }
}
#[cfg(feature = "NodePoseSyncState")]
impl AsRef<
    crate::GlobalNamespace::IStateTable_3<
        crate::GlobalNamespace::NodePoseSyncState,
        crate::GlobalNamespace::NodePoseSyncState_NodePose,
        crate::GlobalNamespace::PoseSerializable,
    >,
> for crate::GlobalNamespace::NodePoseSyncState {
    fn as_ref(
        &self,
    ) -> &crate::GlobalNamespace::IStateTable_3<
        crate::GlobalNamespace::NodePoseSyncState,
        crate::GlobalNamespace::NodePoseSyncState_NodePose,
        crate::GlobalNamespace::PoseSerializable,
    > {
        todo!()
    }
}
#[cfg(feature = "NodePoseSyncState")]
impl AsMut<
    crate::GlobalNamespace::IStateTable_3<
        crate::GlobalNamespace::NodePoseSyncState,
        crate::GlobalNamespace::NodePoseSyncState_NodePose,
        crate::GlobalNamespace::PoseSerializable,
    >,
> for crate::GlobalNamespace::NodePoseSyncState {
    fn as_mut(
        &mut self,
    ) -> &mut crate::GlobalNamespace::IStateTable_3<
        crate::GlobalNamespace::NodePoseSyncState,
        crate::GlobalNamespace::NodePoseSyncState_NodePose,
        crate::GlobalNamespace::PoseSerializable,
    > {
        todo!()
    }
}
#[cfg(feature = "NodePoseSyncState")]
impl AsRef<crate::LiteNetLib::Utils::INetSerializable>
for crate::GlobalNamespace::NodePoseSyncState {
    fn as_ref(&self) -> &crate::LiteNetLib::Utils::INetSerializable {
        todo!()
    }
}
#[cfg(feature = "NodePoseSyncState")]
impl AsMut<crate::LiteNetLib::Utils::INetSerializable>
for crate::GlobalNamespace::NodePoseSyncState {
    fn as_mut(&mut self) -> &mut crate::LiteNetLib::Utils::INetSerializable {
        todo!()
    }
}
#[cfg(feature = "NodePoseSyncState+NodePose")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum NodePoseSyncState_NodePose {
    #[default]
    Count = 3i32,
    Head = 0i32,
    LeftController = 1i32,
    RightController = 2i32,
}
#[cfg(feature = "NodePoseSyncState+NodePose")]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::NodePoseSyncState_NodePose {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "NodePoseSyncState/NodePose";
    fn matches_value_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        !ty.is_ref()
            && ty
                .class()
                .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref()
            && ty
                .class()
                .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_value_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        !ty.is_ref()
            && <Self as quest_hook::libil2cpp::Type>::class()
                .is_assignable_from(ty.class())
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref()
            && <Self as quest_hook::libil2cpp::Type>::class()
                .is_assignable_from(ty.class())
    }
}
#[cfg(feature = "NodePoseSyncState+NodePose")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::GlobalNamespace::NodePoseSyncState_NodePose {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "NodePoseSyncState+NodePose")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::GlobalNamespace::NodePoseSyncState_NodePose {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_parameter(ty)
    }
    fn from_actual(actual: Self::Actual) -> Self {
        actual
    }
    fn into_actual(self) -> Self::Actual {
        self
    }
}
#[cfg(feature = "NodePoseSyncState+NodePose")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::GlobalNamespace::NodePoseSyncState_NodePose {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_returned(ty)
    }
    fn from_object(object: Option<&mut quest_hook::libil2cpp::Il2CppObject>) -> Self {
        unsafe {
            quest_hook::libil2cpp::raw::unbox(
                quest_hook::libil2cpp::WrapRaw::raw(object.unwrap()),
            )
        }
    }
}
#[cfg(feature = "NodePoseSyncState+NodePose")]
unsafe impl quest_hook::libil2cpp::Return
for crate::GlobalNamespace::NodePoseSyncState_NodePose {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_return(ty)
    }
    fn into_actual(self) -> Self::Actual {
        self
    }
    fn from_actual(actual: Self::Actual) -> Self {
        actual
    }
}
