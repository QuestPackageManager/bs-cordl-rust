#[cfg(feature = "ResetPitchOnGameplayFinished")]
#[repr(C)]
#[derive(Debug)]
pub struct ResetPitchOnGameplayFinished {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _gameplayLevelSceneTransitionEvents: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::GameplayLevelSceneTransitionEvents,
    >,
    pub _audioManager: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::AudioManagerSO>,
}
#[cfg(feature = "ResetPitchOnGameplayFinished")]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::ResetPitchOnGameplayFinished {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "ResetPitchOnGameplayFinished";
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
#[cfg(feature = "ResetPitchOnGameplayFinished")]
impl std::ops::Deref for crate::GlobalNamespace::ResetPitchOnGameplayFinished {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "ResetPitchOnGameplayFinished")]
impl std::ops::DerefMut for crate::GlobalNamespace::ResetPitchOnGameplayFinished {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "ResetPitchOnGameplayFinished")]
impl crate::GlobalNamespace::ResetPitchOnGameplayFinished {
    pub fn Finalize(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<(), quest_hook::libil2cpp::Void, 0usize>("Finalize")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "Finalize", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn HandleAnyGameplayLevelDidFinish(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (),
                        quest_hook::libil2cpp::Void,
                        0usize,
                    >("HandleAnyGameplayLevelDidFinish")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "HandleAnyGameplayLevelDidFinish", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn New(
        gameplayLevelSceneTransitionEvents: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::GameplayLevelSceneTransitionEvents,
        >,
        audioManager: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::AudioManagerSO>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (gameplayLevelSceneTransitionEvents, audioManager))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        gameplayLevelSceneTransitionEvents: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::GameplayLevelSceneTransitionEvents,
        >,
        audioManager: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::AudioManagerSO>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::GlobalNamespace::GameplayLevelSceneTransitionEvents,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::GlobalNamespace::AudioManagerSO,
                            >,
                        ),
                        quest_hook::libil2cpp::Void,
                        2usize,
                    >(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), ".ctor", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method
                .invoke_unchecked(
                    self,
                    (gameplayLevelSceneTransitionEvents, audioManager),
                )?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "ResetPitchOnGameplayFinished")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::ResetPitchOnGameplayFinished {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
