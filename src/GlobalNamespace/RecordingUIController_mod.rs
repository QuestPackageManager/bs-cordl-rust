#[cfg(feature = "RecordingUIController+InitData")]
#[repr(C)]
#[derive(Debug)]
pub struct RecordingUIController_InitData {
    __cordl_parent: crate::System::Object,
    pub recordingEnabled: bool,
}
#[cfg(feature = "RecordingUIController+InitData")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::RecordingUIController_InitData
    => ""."RecordingUIController/InitData"
);
#[cfg(feature = "RecordingUIController+InitData")]
impl std::ops::Deref for crate::GlobalNamespace::RecordingUIController_InitData {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "RecordingUIController+InitData")]
impl std::ops::DerefMut for crate::GlobalNamespace::RecordingUIController_InitData {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "RecordingUIController+InitData")]
impl crate::GlobalNamespace::RecordingUIController_InitData {
    pub fn New(recordingEnabled: bool) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (recordingEnabled))?;
        Ok(__cordl_object)
    }
    pub fn _ctor(
        &mut self,
        recordingEnabled: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (recordingEnabled))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "RecordingUIController+InitData")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::RecordingUIController_InitData {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "RecordingUIController")]
#[repr(C)]
#[derive(Debug)]
pub struct RecordingUIController {
    __cordl_parent: crate::UnityEngine::MonoBehaviour,
    pub _circle: *mut crate::UnityEngine::GameObject,
    pub _updateTimeSpan: f32,
    pub _initData: *mut crate::GlobalNamespace::RecordingUIController_InitData,
    pub _lastUpdateTime: f32,
}
#[cfg(feature = "RecordingUIController")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::RecordingUIController => ""
    ."RecordingUIController"
);
#[cfg(feature = "RecordingUIController")]
impl std::ops::Deref for crate::GlobalNamespace::RecordingUIController {
    type Target = crate::UnityEngine::MonoBehaviour;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "RecordingUIController")]
impl std::ops::DerefMut for crate::GlobalNamespace::RecordingUIController {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "RecordingUIController")]
impl crate::GlobalNamespace::RecordingUIController {
    #[cfg(feature = "RecordingUIController+InitData")]
    pub type InitData = crate::GlobalNamespace::RecordingUIController_InitData;
    pub fn Init(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Init", ())?;
        Ok(__cordl_ret)
    }
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
    pub fn Update(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Update", ())?;
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
#[cfg(feature = "RecordingUIController")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::RecordingUIController {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
