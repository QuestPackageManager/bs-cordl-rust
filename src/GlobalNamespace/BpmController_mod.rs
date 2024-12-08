#[cfg(feature = "BpmController")]
#[repr(C)]
#[derive(Debug)]
pub struct BpmController {
    __cordl_parent: crate::System::Object,
    pub _beatmapCallbacksController: *mut BeatmapCallbacksController,
    pub _beatmapDataCallback: *mut BeatmapDataCallbackWrapper,
    pub _currentBpm: f32,
}
#[cfg(feature = "BpmController")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for BpmController => ""."BpmController"
);
#[cfg(feature = "BpmController")]
impl std::ops::Deref for BpmController {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BpmController")]
impl std::ops::DerefMut for BpmController {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BpmController")]
impl BpmController {
    #[cfg(feature = "BpmController+InitData")]
    pub type InitData = crate::GlobalNamespace::BpmController_InitData;
    pub fn _ctor(
        &mut self,
        initData: *mut crate::GlobalNamespace::BpmController_InitData,
        beatmapCallbacksController: *mut BeatmapCallbacksController,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (initData, beatmapCallbacksController))?;
        Ok(__cordl_ret)
    }
    pub fn HandleBpmChangeBeatmapEvent(
        &mut self,
        bpmChangeBeatmapEventData: *mut BPMChangeBeatmapEventData,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleBpmChangeBeatmapEvent", (bpmChangeBeatmapEventData))?;
        Ok(__cordl_ret)
    }
    pub fn get_oneBeatDuration(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("get_oneBeatDuration", ())?;
        Ok(__cordl_ret)
    }
    pub fn Dispose(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Dispose", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_currentBpm(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("get_currentBpm", ())?;
        Ok(__cordl_ret)
    }
    pub fn New(
        initData: *mut crate::GlobalNamespace::BpmController_InitData,
        beatmapCallbacksController: *mut BeatmapCallbacksController,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (initData, beatmapCallbacksController))?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "BpmController")]
impl quest_hook::libil2cpp::ObjectType for BpmController {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "BpmController+InitData")]
#[repr(C)]
#[derive(Debug)]
pub struct BpmController_InitData {
    __cordl_parent: crate::System::Object,
    pub startBpm: f32,
}
#[cfg(feature = "BpmController+InitData")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::BpmController_InitData => ""
    ."BpmController/InitData"
);
#[cfg(feature = "BpmController+InitData")]
impl std::ops::Deref for crate::GlobalNamespace::BpmController_InitData {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BpmController+InitData")]
impl std::ops::DerefMut for crate::GlobalNamespace::BpmController_InitData {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BpmController+InitData")]
impl crate::GlobalNamespace::BpmController_InitData {
    pub fn _ctor(
        &mut self,
        startBpm: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (startBpm))?;
        Ok(__cordl_ret)
    }
    pub fn New(startBpm: f32) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (startBpm))?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "BpmController+InitData")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::BpmController_InitData {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
