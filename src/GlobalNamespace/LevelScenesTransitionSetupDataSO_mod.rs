#[cfg(feature = "LevelScenesTransitionSetupDataSO")]
#[repr(C)]
#[derive(Debug)]
pub struct LevelScenesTransitionSetupDataSO {
    __cordl_parent: crate::GlobalNamespace::ScenesTransitionSetupDataSO,
    pub _gameplayCoreSceneSetupData_k__BackingField: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::GameplayCoreSceneSetupData,
    >,
}
#[cfg(feature = "LevelScenesTransitionSetupDataSO")]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::LevelScenesTransitionSetupDataSO {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "LevelScenesTransitionSetupDataSO";
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
#[cfg(feature = "LevelScenesTransitionSetupDataSO")]
impl std::ops::Deref for crate::GlobalNamespace::LevelScenesTransitionSetupDataSO {
    type Target = crate::GlobalNamespace::ScenesTransitionSetupDataSO;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "LevelScenesTransitionSetupDataSO")]
impl std::ops::DerefMut for crate::GlobalNamespace::LevelScenesTransitionSetupDataSO {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "LevelScenesTransitionSetupDataSO")]
impl crate::GlobalNamespace::LevelScenesTransitionSetupDataSO {
    pub fn BeforeScenesWillBeActivated(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::GlobalNamespace::LevelScenesTransitionSetupDataSO as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                quest_hook::libil2cpp::Void,
                0usize,
            >("BeforeScenesWillBeActivated")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::GlobalNamespace::LevelScenesTransitionSetupDataSO as
                    quest_hook::libil2cpp::Type > ::class(),
                    "BeforeScenesWillBeActivated", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn BeforeScenesWillBeActivatedAsync(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Threading::Tasks::Task>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::GlobalNamespace::LevelScenesTransitionSetupDataSO as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                quest_hook::libil2cpp::Gc<crate::System::Threading::Tasks::Task>,
                0usize,
            >("BeforeScenesWillBeActivatedAsync")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::GlobalNamespace::LevelScenesTransitionSetupDataSO as
                    quest_hook::libil2cpp::Type > ::class(),
                    "BeforeScenesWillBeActivatedAsync", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task,
        > = unsafe { method.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::GlobalNamespace::LevelScenesTransitionSetupDataSO as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), quest_hook::libil2cpp::Void, 0usize>(".ctor")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::GlobalNamespace::LevelScenesTransitionSetupDataSO as
                    quest_hook::libil2cpp::Type > ::class(), ".ctor", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_gameplayCoreSceneSetupData(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::GameplayCoreSceneSetupData>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::GlobalNamespace::LevelScenesTransitionSetupDataSO as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                quest_hook::libil2cpp::Gc<
                    crate::GlobalNamespace::GameplayCoreSceneSetupData,
                >,
                0usize,
            >("get_gameplayCoreSceneSetupData")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::GlobalNamespace::LevelScenesTransitionSetupDataSO as
                    quest_hook::libil2cpp::Type > ::class(),
                    "get_gameplayCoreSceneSetupData", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::GameplayCoreSceneSetupData,
        > = unsafe { method.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_transformedBeatmapData(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::IReadonlyBeatmapData>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::GlobalNamespace::LevelScenesTransitionSetupDataSO as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                quest_hook::libil2cpp::Gc<crate::GlobalNamespace::IReadonlyBeatmapData>,
                0usize,
            >("get_transformedBeatmapData")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::GlobalNamespace::LevelScenesTransitionSetupDataSO as
                    quest_hook::libil2cpp::Type > ::class(),
                    "get_transformedBeatmapData", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::IReadonlyBeatmapData,
        > = unsafe { method.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn set_gameplayCoreSceneSetupData(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::GameplayCoreSceneSetupData,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::GlobalNamespace::LevelScenesTransitionSetupDataSO as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<
                    crate::GlobalNamespace::GameplayCoreSceneSetupData,
                >),
                quest_hook::libil2cpp::Void,
                1usize,
            >("set_gameplayCoreSceneSetupData")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::GlobalNamespace::LevelScenesTransitionSetupDataSO as
                    quest_hook::libil2cpp::Type > ::class(),
                    "set_gameplayCoreSceneSetupData", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (value))?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "LevelScenesTransitionSetupDataSO")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::LevelScenesTransitionSetupDataSO {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
