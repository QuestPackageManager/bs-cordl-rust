#[cfg(feature = "MissionLevelReturnToMenuController")]
#[repr(C)]
#[derive(Debug)]
pub struct MissionLevelReturnToMenuController {
    __cordl_parent: crate::UnityEngine::MonoBehaviour,
    pub _missionLevelSceneSetupData: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::MissionLevelScenesTransitionSetupDataSO,
    >,
    pub _prepareLevelCompletionResults: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::PrepareLevelCompletionResults,
    >,
    pub _missionObjectiveCheckersManager: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::MissionObjectiveCheckersManager,
    >,
}
#[cfg(feature = "MissionLevelReturnToMenuController")]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::MissionLevelReturnToMenuController {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "MissionLevelReturnToMenuController";
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
#[cfg(feature = "MissionLevelReturnToMenuController")]
impl std::ops::Deref for crate::GlobalNamespace::MissionLevelReturnToMenuController {
    type Target = crate::UnityEngine::MonoBehaviour;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "MissionLevelReturnToMenuController")]
impl std::ops::DerefMut for crate::GlobalNamespace::MissionLevelReturnToMenuController {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "MissionLevelReturnToMenuController")]
impl crate::GlobalNamespace::MissionLevelReturnToMenuController {
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn ReturnToMenu(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), quest_hook::libil2cpp::Void, 0usize>("ReturnToMenu")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ReturnToMenu", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())
        };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), quest_hook::libil2cpp::Void, 0usize>(".ctor")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, ".ctor", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "MissionLevelReturnToMenuController")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::MissionLevelReturnToMenuController {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "MissionLevelReturnToMenuController")]
impl AsRef<crate::GlobalNamespace::IReturnToMenuController>
for crate::GlobalNamespace::MissionLevelReturnToMenuController {
    fn as_ref(&self) -> &crate::GlobalNamespace::IReturnToMenuController {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "MissionLevelReturnToMenuController")]
impl AsMut<crate::GlobalNamespace::IReturnToMenuController>
for crate::GlobalNamespace::MissionLevelReturnToMenuController {
    fn as_mut(&mut self) -> &mut crate::GlobalNamespace::IReturnToMenuController {
        unsafe { std::mem::transmute(self) }
    }
}
