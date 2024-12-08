#[cfg(feature = "MissionObjectiveGameUIView")]
#[repr(C)]
#[derive(Debug)]
pub struct MissionObjectiveGameUIView {
    __cordl_parent: crate::UnityEngine::MonoBehaviour,
    pub _notFailedIcon: *mut crate::UnityEngine::Sprite,
    pub _failedIcon: *mut crate::UnityEngine::Sprite,
    pub _notClearedIcon: *mut crate::UnityEngine::Sprite,
    pub _clearedIcon: *mut crate::UnityEngine::Sprite,
    pub _resultIcon: *mut crate::UnityEngine::UI::Image,
    pub _finalClearIconColor: crate::UnityEngine::Color,
    pub _finalFailIconColor: crate::UnityEngine::Color,
    pub _nonFinalIconColor: crate::UnityEngine::Color,
    pub _clearedPS: *mut crate::UnityEngine::ParticleSystem,
    pub _numberOfParticles: i32,
    pub _nameText: *mut crate::TMPro::TextMeshProUGUI,
    pub _valueText: *mut crate::TMPro::TextMeshProUGUI,
    pub _conditionText: *mut crate::TMPro::TextMeshProUGUI,
    pub _missionObjectiveChecker: *mut MissionObjectiveChecker,
}
#[cfg(feature = "MissionObjectiveGameUIView")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for MissionObjectiveGameUIView => ""
    ."MissionObjectiveGameUIView"
);
#[cfg(feature = "MissionObjectiveGameUIView")]
impl std::ops::Deref for MissionObjectiveGameUIView {
    type Target = crate::UnityEngine::MonoBehaviour;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "MissionObjectiveGameUIView")]
impl std::ops::DerefMut for MissionObjectiveGameUIView {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "MissionObjectiveGameUIView")]
impl MissionObjectiveGameUIView {
    pub fn HandleMissionObjectiveStatusDidChange(
        &mut self,
        missionObjectiveChecker: *mut MissionObjectiveChecker,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleMissionObjectiveStatusDidChange", (missionObjectiveChecker))?;
        Ok(__cordl_ret)
    }
    pub fn RefreshValue(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("RefreshValue", ())?;
        Ok(__cordl_ret)
    }
    pub fn HandleMissionObjectiveCheckedValueDidChange(
        &mut self,
        missionObjectiveChecker: *mut MissionObjectiveChecker,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "HandleMissionObjectiveCheckedValueDidChange",
                (missionObjectiveChecker),
            )?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret)
    }
    pub fn RefreshIcon(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("RefreshIcon", ())?;
        Ok(__cordl_ret)
    }
    pub fn SetMissionObjectiveChecker(
        &mut self,
        missionObjectiveChecker: *mut MissionObjectiveChecker,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetMissionObjectiveChecker", (missionObjectiveChecker))?;
        Ok(__cordl_ret)
    }
    pub fn New() -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "MissionObjectiveGameUIView")]
impl quest_hook::libil2cpp::ObjectType for MissionObjectiveGameUIView {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
