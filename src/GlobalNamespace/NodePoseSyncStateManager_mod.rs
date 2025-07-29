#[cfg(feature = "cordl_class_NodePoseSyncStateManager")]
#[repr(C)]
#[derive(Debug)]
pub struct NodePoseSyncStateManager {
    __cordl_parent: crate::GlobalNamespace::MultiplayerSyncStateManager_5<
        crate::GlobalNamespace::NodePoseSyncState,
        crate::GlobalNamespace::NodePoseSyncState_NodePose,
        crate::GlobalNamespace::PoseSerializable,
        quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::NodePoseSyncStateNetSerializable,
        >,
        quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::NodePoseSyncStateDeltaNetSerializable,
        >,
    >,
}
#[cfg(feature = "cordl_class_NodePoseSyncStateManager")]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::NodePoseSyncStateManager {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "NodePoseSyncStateManager";
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
#[cfg(feature = "cordl_class_NodePoseSyncStateManager")]
impl std::ops::Deref for crate::GlobalNamespace::NodePoseSyncStateManager {
    type Target = crate::GlobalNamespace::MultiplayerSyncStateManager_5<
        crate::GlobalNamespace::NodePoseSyncState,
        crate::GlobalNamespace::NodePoseSyncState_NodePose,
        crate::GlobalNamespace::PoseSerializable,
        quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::NodePoseSyncStateNetSerializable,
        >,
        quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::NodePoseSyncStateDeltaNetSerializable,
        >,
    >;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "cordl_class_NodePoseSyncStateManager")]
impl std::ops::DerefMut for crate::GlobalNamespace::NodePoseSyncStateManager {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "NodePoseSyncStateManager")]
impl crate::GlobalNamespace::NodePoseSyncStateManager {
    pub fn Interpolate(
        &mut self,
        prev: crate::GlobalNamespace::PoseSerializable,
        prevTime: i64,
        curr: crate::GlobalNamespace::PoseSerializable,
        currTime: i64,
        _cordl_time: i64,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::PoseSerializable> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            crate::GlobalNamespace::PoseSerializable,
                            i64,
                            crate::GlobalNamespace::PoseSerializable,
                            i64,
                            i64,
                        ),
                        crate::GlobalNamespace::PoseSerializable,
                        5usize,
                    >("Interpolate")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "Interpolate", 5usize
                        )
                    })
            });
        let __cordl_ret: crate::GlobalNamespace::PoseSerializable = unsafe {
            cordl_method_info
                .invoke_unchecked(self, (prev, prevTime, curr, currTime, _cordl_time))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn Smooth(
        &mut self,
        a: crate::GlobalNamespace::PoseSerializable,
        b: crate::GlobalNamespace::PoseSerializable,
        smooth: f32,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::PoseSerializable> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            crate::GlobalNamespace::PoseSerializable,
                            crate::GlobalNamespace::PoseSerializable,
                            f32,
                        ),
                        crate::GlobalNamespace::PoseSerializable,
                        3usize,
                    >("Smooth")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "Smooth",
                            3usize
                        )
                    })
            });
        let __cordl_ret: crate::GlobalNamespace::PoseSerializable = unsafe {
            cordl_method_info.invoke_unchecked(self, (a, b, smooth))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), quest_hook::libil2cpp::Void, 0usize>(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), ".ctor",
                            0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_deltaMessageType(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::GlobalNamespace::MultiplayerSessionManager_MessageType,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        crate::GlobalNamespace::MultiplayerSessionManager_MessageType,
                        0usize,
                    >("get_deltaMessageType")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "get_deltaMessageType", 0usize
                        )
                    })
            });
        let __cordl_ret: crate::GlobalNamespace::MultiplayerSessionManager_MessageType = unsafe {
            cordl_method_info.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_deltaSerializablePool(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::IPacketPool_1<
                quest_hook::libil2cpp::Gc<
                    crate::GlobalNamespace::NodePoseSyncStateDeltaNetSerializable,
                >,
            >,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        quest_hook::libil2cpp::Gc<
                            crate::GlobalNamespace::IPacketPool_1<
                                quest_hook::libil2cpp::Gc<
                                    crate::GlobalNamespace::NodePoseSyncStateDeltaNetSerializable,
                                >,
                            >,
                        >,
                        0usize,
                    >("get_deltaSerializablePool")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "get_deltaSerializablePool", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::IPacketPool_1<
                quest_hook::libil2cpp::Gc<
                    crate::GlobalNamespace::NodePoseSyncStateDeltaNetSerializable,
                >,
            >,
        > = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_deltaUpdateFrequencyMs(&mut self) -> quest_hook::libil2cpp::Result<i64> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), i64, 0usize>("get_deltaUpdateFrequencyMs")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "get_deltaUpdateFrequencyMs", 0usize
                        )
                    })
            });
        let __cordl_ret: i64 = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_fullStateUpdateFrequencyMs(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<i64> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), i64, 0usize>("get_fullStateUpdateFrequencyMs")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "get_fullStateUpdateFrequencyMs", 0usize
                        )
                    })
            });
        let __cordl_ret: i64 = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_localBufferSize(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), i32, 0usize>("get_localBufferSize")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "get_localBufferSize", 0usize
                        )
                    })
            });
        let __cordl_ret: i32 = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_messageType(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::GlobalNamespace::MultiplayerSessionManager_MessageType,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        crate::GlobalNamespace::MultiplayerSessionManager_MessageType,
                        0usize,
                    >("get_messageType")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "get_messageType", 0usize
                        )
                    })
            });
        let __cordl_ret: crate::GlobalNamespace::MultiplayerSessionManager_MessageType = unsafe {
            cordl_method_info.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_remoteBufferSize(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), i32, 0usize>("get_remoteBufferSize")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "get_remoteBufferSize", 0usize
                        )
                    })
            });
        let __cordl_ret: i32 = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_serializablePool(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::IPacketPool_1<
                quest_hook::libil2cpp::Gc<
                    crate::GlobalNamespace::NodePoseSyncStateNetSerializable,
                >,
            >,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        quest_hook::libil2cpp::Gc<
                            crate::GlobalNamespace::IPacketPool_1<
                                quest_hook::libil2cpp::Gc<
                                    crate::GlobalNamespace::NodePoseSyncStateNetSerializable,
                                >,
                            >,
                        >,
                        0usize,
                    >("get_serializablePool")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "get_serializablePool", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::IPacketPool_1<
                quest_hook::libil2cpp::Gc<
                    crate::GlobalNamespace::NodePoseSyncStateNetSerializable,
                >,
            >,
        > = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "cordl_class_NodePoseSyncStateManager")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::NodePoseSyncStateManager {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "NodePoseSyncStateManager")]
impl AsRef<crate::GlobalNamespace::INodePoseSyncStateManager>
for crate::GlobalNamespace::NodePoseSyncStateManager {
    fn as_ref(&self) -> &crate::GlobalNamespace::INodePoseSyncStateManager {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "NodePoseSyncStateManager")]
impl AsMut<crate::GlobalNamespace::INodePoseSyncStateManager>
for crate::GlobalNamespace::NodePoseSyncStateManager {
    fn as_mut(&mut self) -> &mut crate::GlobalNamespace::INodePoseSyncStateManager {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "NodePoseSyncStateManager")]
impl AsRef<
    crate::GlobalNamespace::INodePoseSyncStateManager_5<
        crate::GlobalNamespace::NodePoseSyncState,
        crate::GlobalNamespace::NodePoseSyncState_NodePose,
        crate::GlobalNamespace::PoseSerializable,
        quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::NodePoseSyncStateNetSerializable,
        >,
        quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::NodePoseSyncStateDeltaNetSerializable,
        >,
    >,
> for crate::GlobalNamespace::NodePoseSyncStateManager {
    fn as_ref(
        &self,
    ) -> &crate::GlobalNamespace::INodePoseSyncStateManager_5<
        crate::GlobalNamespace::NodePoseSyncState,
        crate::GlobalNamespace::NodePoseSyncState_NodePose,
        crate::GlobalNamespace::PoseSerializable,
        quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::NodePoseSyncStateNetSerializable,
        >,
        quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::NodePoseSyncStateDeltaNetSerializable,
        >,
    > {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "NodePoseSyncStateManager")]
impl AsMut<
    crate::GlobalNamespace::INodePoseSyncStateManager_5<
        crate::GlobalNamespace::NodePoseSyncState,
        crate::GlobalNamespace::NodePoseSyncState_NodePose,
        crate::GlobalNamespace::PoseSerializable,
        quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::NodePoseSyncStateNetSerializable,
        >,
        quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::NodePoseSyncStateDeltaNetSerializable,
        >,
    >,
> for crate::GlobalNamespace::NodePoseSyncStateManager {
    fn as_mut(
        &mut self,
    ) -> &mut crate::GlobalNamespace::INodePoseSyncStateManager_5<
        crate::GlobalNamespace::NodePoseSyncState,
        crate::GlobalNamespace::NodePoseSyncState_NodePose,
        crate::GlobalNamespace::PoseSerializable,
        quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::NodePoseSyncStateNetSerializable,
        >,
        quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::NodePoseSyncStateDeltaNetSerializable,
        >,
    > {
        unsafe { std::mem::transmute(self) }
    }
}
