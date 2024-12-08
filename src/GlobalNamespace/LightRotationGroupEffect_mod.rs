#[cfg(feature = "LightRotationGroupEffect+InitData")]
#[repr(C)]
#[derive(Debug)]
pub struct LightRotationGroupEffect_InitData {
    __cordl_parent: crate::System::Object,
    pub groupId: i32,
    pub elementId: i32,
    pub axis: LightAxis,
    pub mirrored: bool,
    pub transform: *mut crate::UnityEngine::Transform,
}
#[cfg(feature = "LightRotationGroupEffect+InitData")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::LightRotationGroupEffect_InitData => ""
    ."LightRotationGroupEffect/InitData"
);
#[cfg(feature = "LightRotationGroupEffect+InitData")]
impl std::ops::Deref for crate::GlobalNamespace::LightRotationGroupEffect_InitData {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "LightRotationGroupEffect+InitData")]
impl std::ops::DerefMut for crate::GlobalNamespace::LightRotationGroupEffect_InitData {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "LightRotationGroupEffect+InitData")]
impl crate::GlobalNamespace::LightRotationGroupEffect_InitData {
    pub fn _ctor(
        &mut self,
        groupId: i32,
        elementId: i32,
        axis: LightAxis,
        mirrored: bool,
        transform: *mut crate::UnityEngine::Transform,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (groupId, elementId, axis, mirrored, transform))?;
        Ok(__cordl_ret)
    }
    pub fn New(
        groupId: i32,
        elementId: i32,
        axis: LightAxis,
        mirrored: bool,
        transform: *mut crate::UnityEngine::Transform,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (groupId, elementId, axis, mirrored, transform))?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "LightRotationGroupEffect+InitData")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::LightRotationGroupEffect_InitData {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "LightRotationGroupEffect")]
#[repr(C)]
#[derive(Debug)]
pub struct LightRotationGroupEffect {
    __cordl_parent: crate::System::Object,
    pub _transform: *mut crate::UnityEngine::Transform,
    pub _axis: LightAxis,
    pub _mirrored: bool,
    pub _tweeningManager: *mut crate::Tweening::SongTimeTweeningManager,
    pub _beatmapCallbacksController: *mut BeatmapCallbacksController,
    pub _rotationTween: *mut crate::Tweening::FloatTween,
    pub _lightRotationBeatmapEventCallbackWrapper: *mut BeatmapDataCallbackWrapper,
}
#[cfg(feature = "LightRotationGroupEffect")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for LightRotationGroupEffect => ""
    ."LightRotationGroupEffect"
);
#[cfg(feature = "LightRotationGroupEffect")]
impl std::ops::Deref for LightRotationGroupEffect {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "LightRotationGroupEffect")]
impl std::ops::DerefMut for LightRotationGroupEffect {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "LightRotationGroupEffect")]
impl LightRotationGroupEffect {
    #[cfg(feature = "LightRotationGroupEffect+InitData")]
    pub type InitData = crate::GlobalNamespace::LightRotationGroupEffect_InitData;
    pub fn Cleanup(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Cleanup", ())?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
        initData: *mut crate::GlobalNamespace::LightRotationGroupEffect_InitData,
        tweeningManager: *mut crate::Tweening::SongTimeTweeningManager,
        beatmapCallbacksController: *mut BeatmapCallbacksController,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (initData, tweeningManager, beatmapCallbacksController))?;
        Ok(__cordl_ret)
    }
    pub fn HandleRotationChangeBeatmapEvent(
        &mut self,
        currentEventData: *mut LightRotationBeatmapEventData,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleRotationChangeBeatmapEvent", (currentEventData))?;
        Ok(__cordl_ret)
    }
    pub fn SetRotation(
        &mut self,
        rotation: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetRotation", (rotation))?;
        Ok(__cordl_ret)
    }
    pub fn New(
        initData: *mut crate::GlobalNamespace::LightRotationGroupEffect_InitData,
        tweeningManager: *mut crate::Tweening::SongTimeTweeningManager,
        beatmapCallbacksController: *mut BeatmapCallbacksController,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(
                ".ctor",
                (initData, tweeningManager, beatmapCallbacksController),
            )?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "LightRotationGroupEffect")]
impl quest_hook::libil2cpp::ObjectType for LightRotationGroupEffect {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
