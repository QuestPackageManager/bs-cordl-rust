#[cfg(feature = "IScoreSyncStateManager")]
#[repr(C)]
#[derive(Debug)]
pub struct IScoreSyncStateManager {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "IScoreSyncStateManager")]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::IScoreSyncStateManager {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "IScoreSyncStateManager";
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
#[cfg(feature = "IScoreSyncStateManager")]
impl std::ops::Deref for crate::GlobalNamespace::IScoreSyncStateManager {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "IScoreSyncStateManager")]
impl std::ops::DerefMut for crate::GlobalNamespace::IScoreSyncStateManager {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "IScoreSyncStateManager")]
impl crate::GlobalNamespace::IScoreSyncStateManager {
    pub fn from_object_mut(
        object_param: *mut quest_hook::libil2cpp::Il2CppObject,
    ) -> *mut Self {
        unsafe { (object_param as *mut Self) }
    }
}
#[cfg(feature = "IScoreSyncStateManager")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::IScoreSyncStateManager {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "IScoreSyncStateManager")]
impl AsRef<
    crate::GlobalNamespace::IScoreSyncStateManager_5<
        crate::GlobalNamespace::StandardScoreSyncState,
        crate::GlobalNamespace::StandardScoreSyncState_Score,
        i32,
        quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::StandardScoreSyncStateNetSerializable,
        >,
        quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::StandardScoreSyncStateDeltaNetSerializable,
        >,
    >,
> for crate::GlobalNamespace::IScoreSyncStateManager {
    fn as_ref(
        &self,
    ) -> &crate::GlobalNamespace::IScoreSyncStateManager_5<
        crate::GlobalNamespace::StandardScoreSyncState,
        crate::GlobalNamespace::StandardScoreSyncState_Score,
        i32,
        quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::StandardScoreSyncStateNetSerializable,
        >,
        quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::StandardScoreSyncStateDeltaNetSerializable,
        >,
    > {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "IScoreSyncStateManager")]
impl AsMut<
    crate::GlobalNamespace::IScoreSyncStateManager_5<
        crate::GlobalNamespace::StandardScoreSyncState,
        crate::GlobalNamespace::StandardScoreSyncState_Score,
        i32,
        quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::StandardScoreSyncStateNetSerializable,
        >,
        quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::StandardScoreSyncStateDeltaNetSerializable,
        >,
    >,
> for crate::GlobalNamespace::IScoreSyncStateManager {
    fn as_mut(
        &mut self,
    ) -> &mut crate::GlobalNamespace::IScoreSyncStateManager_5<
        crate::GlobalNamespace::StandardScoreSyncState,
        crate::GlobalNamespace::StandardScoreSyncState_Score,
        i32,
        quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::StandardScoreSyncStateNetSerializable,
        >,
        quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::StandardScoreSyncStateDeltaNetSerializable,
        >,
    > {
        unsafe { std::mem::transmute(self) }
    }
}
