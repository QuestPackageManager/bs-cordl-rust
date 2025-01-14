#[cfg(feature = "MissionObjectiveGameUIView")]
#[repr(C)]
#[derive(Debug)]
pub struct MissionObjectiveGameUIView {
    __cordl_parent: crate::UnityEngine::MonoBehaviour,
    pub _notFailedIcon: quest_hook::libil2cpp::Gc<crate::UnityEngine::Sprite>,
    pub _failedIcon: quest_hook::libil2cpp::Gc<crate::UnityEngine::Sprite>,
    pub _notClearedIcon: quest_hook::libil2cpp::Gc<crate::UnityEngine::Sprite>,
    pub _clearedIcon: quest_hook::libil2cpp::Gc<crate::UnityEngine::Sprite>,
    pub _resultIcon: quest_hook::libil2cpp::Gc<crate::UnityEngine::UI::Image>,
    pub _finalClearIconColor: crate::UnityEngine::Color,
    pub _finalFailIconColor: crate::UnityEngine::Color,
    pub _nonFinalIconColor: crate::UnityEngine::Color,
    pub _clearedPS: quest_hook::libil2cpp::Gc<crate::UnityEngine::ParticleSystem>,
    pub _numberOfParticles: i32,
    pub _nameText: quest_hook::libil2cpp::Gc<crate::TMPro::TextMeshProUGUI>,
    pub _valueText: quest_hook::libil2cpp::Gc<crate::TMPro::TextMeshProUGUI>,
    pub _conditionText: quest_hook::libil2cpp::Gc<crate::TMPro::TextMeshProUGUI>,
    pub _missionObjectiveChecker: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::MissionObjectiveChecker,
    >,
}
#[cfg(feature = "MissionObjectiveGameUIView")]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::MissionObjectiveGameUIView {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "MissionObjectiveGameUIView";
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
#[cfg(feature = "MissionObjectiveGameUIView")]
impl std::ops::Deref for crate::GlobalNamespace::MissionObjectiveGameUIView {
    type Target = crate::UnityEngine::MonoBehaviour;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "MissionObjectiveGameUIView")]
impl std::ops::DerefMut for crate::GlobalNamespace::MissionObjectiveGameUIView {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "MissionObjectiveGameUIView")]
impl crate::GlobalNamespace::MissionObjectiveGameUIView {
    pub fn HandleMissionObjectiveCheckedValueDidChange(
        &mut self,
        missionObjectiveChecker: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::MissionObjectiveChecker,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<
                    crate::GlobalNamespace::MissionObjectiveChecker,
                >),
                quest_hook::libil2cpp::Void,
                1usize,
            >("HandleMissionObjectiveCheckedValueDidChange")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "HandleMissionObjectiveCheckedValueDidChange", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (missionObjectiveChecker))
        };
        Ok(__cordl_ret.into())
    }
    pub fn HandleMissionObjectiveStatusDidChange(
        &mut self,
        missionObjectiveChecker: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::MissionObjectiveChecker,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<
                    crate::GlobalNamespace::MissionObjectiveChecker,
                >),
                quest_hook::libil2cpp::Void,
                1usize,
            >("HandleMissionObjectiveStatusDidChange")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "HandleMissionObjectiveStatusDidChange", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (missionObjectiveChecker))
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
    pub fn RefreshIcon(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), quest_hook::libil2cpp::Void, 0usize>("RefreshIcon")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "RefreshIcon", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())
        };
        Ok(__cordl_ret.into())
    }
    pub fn RefreshValue(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), quest_hook::libil2cpp::Void, 0usize>("RefreshValue")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "RefreshValue", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())
        };
        Ok(__cordl_ret.into())
    }
    pub fn SetMissionObjectiveChecker(
        &mut self,
        missionObjectiveChecker: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::MissionObjectiveChecker,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<
                    crate::GlobalNamespace::MissionObjectiveChecker,
                >),
                quest_hook::libil2cpp::Void,
                1usize,
            >("SetMissionObjectiveChecker")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "SetMissionObjectiveChecker", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (missionObjectiveChecker))
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
#[cfg(feature = "MissionObjectiveGameUIView")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::MissionObjectiveGameUIView {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
