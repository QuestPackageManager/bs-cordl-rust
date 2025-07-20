#[cfg(feature = "MockNodePoseSyncStateSender")]
#[repr(C)]
#[derive(Debug)]
pub struct MockNodePoseSyncStateSender {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _multiplayerSessionManager: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::IMultiplayerSessionManager,
    >,
}
#[cfg(feature = "MockNodePoseSyncStateSender")]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::MockNodePoseSyncStateSender {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "MockNodePoseSyncStateSender";
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
#[cfg(feature = "MockNodePoseSyncStateSender")]
impl std::ops::Deref for crate::GlobalNamespace::MockNodePoseSyncStateSender {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "MockNodePoseSyncStateSender")]
impl std::ops::DerefMut for crate::GlobalNamespace::MockNodePoseSyncStateSender {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "MockNodePoseSyncStateSender")]
impl crate::GlobalNamespace::MockNodePoseSyncStateSender {
    pub fn Dispose(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::GlobalNamespace::MockNodePoseSyncStateSender as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), quest_hook::libil2cpp::Void, 0usize>("Dispose")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::GlobalNamespace::MockNodePoseSyncStateSender as
                    quest_hook::libil2cpp::Type > ::class(), "Dispose", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn HandleNodePoseSyncStateUpdate(
        &mut self,
        nodePose: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::NodePoseSyncStateNetSerializable,
        >,
        connectedPlayer: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::IConnectedPlayer,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::GlobalNamespace::MockNodePoseSyncStateSender as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<
                        crate::GlobalNamespace::NodePoseSyncStateNetSerializable,
                    >,
                    quest_hook::libil2cpp::Gc<crate::GlobalNamespace::IConnectedPlayer>,
                ),
                quest_hook::libil2cpp::Void,
                2usize,
            >("HandleNodePoseSyncStateUpdate")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::GlobalNamespace::MockNodePoseSyncStateSender as
                    quest_hook::libil2cpp::Type > ::class(),
                    "HandleNodePoseSyncStateUpdate", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (nodePose, connectedPlayer))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn New(
        msm: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::IMultiplayerSessionManager,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (msm))?;
        Ok(__cordl_object.into())
    }
    pub fn SendPose(
        &mut self,
        headPose: crate::GlobalNamespace::PoseSerializable,
        leftHandPose: crate::GlobalNamespace::PoseSerializable,
        rightHandPose: crate::GlobalNamespace::PoseSerializable,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::GlobalNamespace::MockNodePoseSyncStateSender as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    crate::GlobalNamespace::PoseSerializable,
                    crate::GlobalNamespace::PoseSerializable,
                    crate::GlobalNamespace::PoseSerializable,
                ),
                quest_hook::libil2cpp::Void,
                3usize,
            >("SendPose")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::GlobalNamespace::MockNodePoseSyncStateSender as
                    quest_hook::libil2cpp::Type > ::class(), "SendPose", 3usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (headPose, leftHandPose, rightHandPose))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        msm: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::IMultiplayerSessionManager,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::GlobalNamespace::MockNodePoseSyncStateSender as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<
                    crate::GlobalNamespace::IMultiplayerSessionManager,
                >),
                quest_hook::libil2cpp::Void,
                1usize,
            >(".ctor")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::GlobalNamespace::MockNodePoseSyncStateSender as
                    quest_hook::libil2cpp::Type > ::class(), ".ctor", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (msm))?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "MockNodePoseSyncStateSender")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::MockNodePoseSyncStateSender {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "MockNodePoseSyncStateSender")]
impl AsRef<crate::System::IDisposable>
for crate::GlobalNamespace::MockNodePoseSyncStateSender {
    fn as_ref(&self) -> &crate::System::IDisposable {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "MockNodePoseSyncStateSender")]
impl AsMut<crate::System::IDisposable>
for crate::GlobalNamespace::MockNodePoseSyncStateSender {
    fn as_mut(&mut self) -> &mut crate::System::IDisposable {
        unsafe { std::mem::transmute(self) }
    }
}
