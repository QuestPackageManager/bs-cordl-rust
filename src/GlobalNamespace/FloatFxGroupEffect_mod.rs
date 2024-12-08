#[cfg(feature = "FloatFxGroupEffect")]
#[repr(C)]
#[derive(Debug)]
pub struct FloatFxGroupEffect {
    __cordl_parent: crate::System::Object,
    pub _tweeningManager: *mut crate::Tweening::SongTimeTweeningManager,
    pub _beatmapCallbacksController: *mut BeatmapCallbacksController,
    pub _floatTween: *mut crate::Tweening::FloatTween,
    pub _floatFxBeatmapEventCallbackWrapper: *mut BeatmapDataCallbackWrapper,
    pub _initData: *mut crate::GlobalNamespace::FloatFxGroupEffect_InitData,
}
#[cfg(feature = "FloatFxGroupEffect")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for FloatFxGroupEffect => ""."FloatFxGroupEffect"
);
#[cfg(feature = "FloatFxGroupEffect")]
impl std::ops::Deref for FloatFxGroupEffect {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "FloatFxGroupEffect")]
impl std::ops::DerefMut for FloatFxGroupEffect {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "FloatFxGroupEffect")]
impl FloatFxGroupEffect {
    #[cfg(feature = "FloatFxGroupEffect+InitData")]
    pub type InitData = crate::GlobalNamespace::FloatFxGroupEffect_InitData;
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
    pub fn SetValue(
        &mut self,
        value: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetValue", (value))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
        initData: *mut crate::GlobalNamespace::FloatFxGroupEffect_InitData,
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
    pub fn HandleFloatFxBeatmapEventCallback(
        &mut self,
        currentEvent: *mut FloatFxBeatmapEventData,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleFloatFxBeatmapEventCallback", (currentEvent))?;
        Ok(__cordl_ret)
    }
    pub fn New(
        initData: *mut crate::GlobalNamespace::FloatFxGroupEffect_InitData,
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
#[cfg(feature = "FloatFxGroupEffect")]
impl quest_hook::libil2cpp::ObjectType for FloatFxGroupEffect {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "FloatFxGroupEffect+InitData")]
#[repr(C)]
#[derive(Debug)]
pub struct FloatFxGroupEffect_InitData {
    __cordl_parent: crate::System::Object,
    pub groupId: i32,
    pub elementId: i32,
    pub target: *mut FloatFxGroupEffectTarget,
}
#[cfg(feature = "FloatFxGroupEffect+InitData")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::FloatFxGroupEffect_InitData =>
    ""."FloatFxGroupEffect/InitData"
);
#[cfg(feature = "FloatFxGroupEffect+InitData")]
impl std::ops::Deref for crate::GlobalNamespace::FloatFxGroupEffect_InitData {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "FloatFxGroupEffect+InitData")]
impl std::ops::DerefMut for crate::GlobalNamespace::FloatFxGroupEffect_InitData {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "FloatFxGroupEffect+InitData")]
impl crate::GlobalNamespace::FloatFxGroupEffect_InitData {
    pub fn _ctor(
        &mut self,
        groupId: i32,
        elementId: i32,
        target: *mut FloatFxGroupEffectTarget,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (groupId, elementId, target))?;
        Ok(__cordl_ret)
    }
    pub fn New(
        groupId: i32,
        elementId: i32,
        target: *mut FloatFxGroupEffectTarget,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (groupId, elementId, target))?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "FloatFxGroupEffect+InitData")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::FloatFxGroupEffect_InitData {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
