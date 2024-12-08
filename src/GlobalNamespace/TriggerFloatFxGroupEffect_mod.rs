#[cfg(feature = "TriggerFloatFxGroupEffect+InitData")]
#[repr(C)]
#[derive(Debug)]
pub struct TriggerFloatFxGroupEffect_InitData {
    __cordl_parent: crate::System::Object,
    pub groupId: i32,
    pub elementId: i32,
    pub target: *mut crate::GlobalNamespace::FloatFxGroupEffectTarget,
}
#[cfg(feature = "TriggerFloatFxGroupEffect+InitData")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::TriggerFloatFxGroupEffect_InitData => ""
    ."TriggerFloatFxGroupEffect/InitData"
);
#[cfg(feature = "TriggerFloatFxGroupEffect+InitData")]
impl std::ops::Deref for crate::GlobalNamespace::TriggerFloatFxGroupEffect_InitData {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "TriggerFloatFxGroupEffect+InitData")]
impl std::ops::DerefMut for crate::GlobalNamespace::TriggerFloatFxGroupEffect_InitData {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "TriggerFloatFxGroupEffect+InitData")]
impl crate::GlobalNamespace::TriggerFloatFxGroupEffect_InitData {
    pub fn New(
        groupId: i32,
        elementId: i32,
        target: *mut crate::GlobalNamespace::FloatFxGroupEffectTarget,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (groupId, elementId, target))?;
        Ok(__cordl_object)
    }
    pub fn _ctor(
        &mut self,
        groupId: i32,
        elementId: i32,
        target: *mut crate::GlobalNamespace::FloatFxGroupEffectTarget,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (groupId, elementId, target))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "TriggerFloatFxGroupEffect+InitData")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::TriggerFloatFxGroupEffect_InitData {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "TriggerFloatFxGroupEffect")]
#[repr(C)]
#[derive(Debug)]
pub struct TriggerFloatFxGroupEffect {
    __cordl_parent: crate::System::Object,
    pub _beatmapCallbacksController: *mut crate::GlobalNamespace::BeatmapCallbacksController,
    pub _floatFxBeatmapEventCallbackWrapper: *mut crate::GlobalNamespace::BeatmapDataCallbackWrapper,
    pub _initData: *mut crate::GlobalNamespace::TriggerFloatFxGroupEffect_InitData,
}
#[cfg(feature = "TriggerFloatFxGroupEffect")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::TriggerFloatFxGroupEffect => ""
    ."TriggerFloatFxGroupEffect"
);
#[cfg(feature = "TriggerFloatFxGroupEffect")]
impl std::ops::Deref for crate::GlobalNamespace::TriggerFloatFxGroupEffect {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "TriggerFloatFxGroupEffect")]
impl std::ops::DerefMut for crate::GlobalNamespace::TriggerFloatFxGroupEffect {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "TriggerFloatFxGroupEffect")]
impl crate::GlobalNamespace::TriggerFloatFxGroupEffect {
    #[cfg(feature = "TriggerFloatFxGroupEffect+InitData")]
    pub type InitData = crate::GlobalNamespace::TriggerFloatFxGroupEffect_InitData;
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
    pub fn HandleFloatFxBeatmapEventCallback(
        &mut self,
        currentEvent: *mut crate::GlobalNamespace::FloatFxBeatmapEventData,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleFloatFxBeatmapEventCallback", (currentEvent))?;
        Ok(__cordl_ret)
    }
    pub fn New(
        initData: *mut crate::GlobalNamespace::TriggerFloatFxGroupEffect_InitData,
        beatmapCallbacksController: *mut crate::GlobalNamespace::BeatmapCallbacksController,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (initData, beatmapCallbacksController))?;
        Ok(__cordl_object)
    }
    pub fn _ctor(
        &mut self,
        initData: *mut crate::GlobalNamespace::TriggerFloatFxGroupEffect_InitData,
        beatmapCallbacksController: *mut crate::GlobalNamespace::BeatmapCallbacksController,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (initData, beatmapCallbacksController))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "TriggerFloatFxGroupEffect")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::TriggerFloatFxGroupEffect {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
