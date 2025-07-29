#[cfg(feature = "cordl_class_INodePoseSyncStateManager")]
#[repr(C)]
#[derive(Debug)]
pub struct INodePoseSyncStateManager {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "cordl_class_INodePoseSyncStateManager")]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::INodePoseSyncStateManager {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "INodePoseSyncStateManager";
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
#[cfg(feature = "INodePoseSyncStateManager")]
impl std::ops::Deref for crate::GlobalNamespace::INodePoseSyncStateManager {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "INodePoseSyncStateManager")]
impl std::ops::DerefMut for crate::GlobalNamespace::INodePoseSyncStateManager {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "INodePoseSyncStateManager")]
impl crate::GlobalNamespace::INodePoseSyncStateManager {
    pub fn from_object_mut(
        object_param: *mut quest_hook::libil2cpp::Il2CppObject,
    ) -> *mut Self {
        unsafe { (object_param as *mut Self) }
    }
}
#[cfg(feature = "cordl_class_INodePoseSyncStateManager")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::INodePoseSyncStateManager {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "INodePoseSyncStateManager")]
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
> for crate::GlobalNamespace::INodePoseSyncStateManager {
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
#[cfg(feature = "INodePoseSyncStateManager")]
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
> for crate::GlobalNamespace::INodePoseSyncStateManager {
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
