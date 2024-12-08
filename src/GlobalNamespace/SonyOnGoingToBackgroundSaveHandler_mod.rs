#[cfg(feature = "SonyOnGoingToBackgroundSaveHandler")]
#[repr(C)]
#[derive(Debug)]
pub struct SonyOnGoingToBackgroundSaveHandler {
    __cordl_parent: crate::UnityEngine::MonoBehaviour,
    pub _playerDataModel: *mut crate::GlobalNamespace::PlayerDataModel,
    pub _flushingService: *mut crate::BGLib::SaveDataCore::SaveDataFlushingService,
    pub _sonyBackgroundExecutionHelper: *mut crate::GlobalNamespace::SonyBackgroundExecutionHelper,
    pub _localLeaderboardModel: *mut crate::GlobalNamespace::LocalLeaderboardsModel,
}
#[cfg(feature = "SonyOnGoingToBackgroundSaveHandler")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::SonyOnGoingToBackgroundSaveHandler => ""
    ."SonyOnGoingToBackgroundSaveHandler"
);
#[cfg(feature = "SonyOnGoingToBackgroundSaveHandler")]
impl std::ops::Deref for crate::GlobalNamespace::SonyOnGoingToBackgroundSaveHandler {
    type Target = crate::UnityEngine::MonoBehaviour;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "SonyOnGoingToBackgroundSaveHandler")]
impl std::ops::DerefMut for crate::GlobalNamespace::SonyOnGoingToBackgroundSaveHandler {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "SonyOnGoingToBackgroundSaveHandler")]
impl crate::GlobalNamespace::SonyOnGoingToBackgroundSaveHandler {
    pub fn HandlePlatformBackgroundExecutionHelperDidGoToBackgroundExecution(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "HandlePlatformBackgroundExecutionHelperDidGoToBackgroundExecution",
                (),
            )?;
        Ok(__cordl_ret)
    }
    pub fn Initialize(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Initialize", ())?;
        Ok(__cordl_ret)
    }
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
    pub fn OnDestroy(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnDestroy", ())?;
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
}
#[cfg(feature = "SonyOnGoingToBackgroundSaveHandler")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::SonyOnGoingToBackgroundSaveHandler {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
