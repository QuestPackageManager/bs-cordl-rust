#[cfg(feature = "FloatFxGroupEffect")]
#[repr(C)]
#[derive(Debug)]
pub struct FloatFxGroupEffect {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _tweeningManager: quest_hook::libil2cpp::Gc<
        crate::Tweening::SongTimeTweeningManager,
    >,
    pub _beatmapCallbacksController: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::BeatmapCallbacksController,
    >,
    pub _floatTween: quest_hook::libil2cpp::Gc<crate::Tweening::FloatTween>,
    pub _floatFxBeatmapEventCallbackWrapper: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::BeatmapDataCallbackWrapper,
    >,
    pub _initData: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::FloatFxGroupEffect_InitData,
    >,
}
#[cfg(feature = "FloatFxGroupEffect")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::FloatFxGroupEffect => ""
    ."FloatFxGroupEffect"
);
#[cfg(feature = "FloatFxGroupEffect")]
impl std::ops::Deref for crate::GlobalNamespace::FloatFxGroupEffect {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "FloatFxGroupEffect")]
impl std::ops::DerefMut for crate::GlobalNamespace::FloatFxGroupEffect {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "FloatFxGroupEffect")]
impl crate::GlobalNamespace::FloatFxGroupEffect {
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
        Ok(__cordl_ret.into())
    }
    pub fn HandleFloatFxBeatmapEventCallback(
        &mut self,
        currentEvent: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::FloatFxBeatmapEventData,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleFloatFxBeatmapEventCallback", (currentEvent))?;
        Ok(__cordl_ret.into())
    }
    pub fn New(
        initData: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::FloatFxGroupEffect_InitData,
        >,
        tweeningManager: quest_hook::libil2cpp::Gc<
            crate::Tweening::SongTimeTweeningManager,
        >,
        beatmapCallbacksController: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::BeatmapCallbacksController,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(
                ".ctor",
                (initData, tweeningManager, beatmapCallbacksController),
            )?;
        Ok(__cordl_object.into())
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
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        initData: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::FloatFxGroupEffect_InitData,
        >,
        tweeningManager: quest_hook::libil2cpp::Gc<
            crate::Tweening::SongTimeTweeningManager,
        >,
        beatmapCallbacksController: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::BeatmapCallbacksController,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (initData, tweeningManager, beatmapCallbacksController))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "FloatFxGroupEffect")]
impl quest_hook::libil2cpp::ObjectType for crate::GlobalNamespace::FloatFxGroupEffect {
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
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub groupId: i32,
    pub elementId: i32,
    pub target: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::FloatFxGroupEffectTarget,
    >,
}
#[cfg(feature = "FloatFxGroupEffect+InitData")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::FloatFxGroupEffect_InitData =>
    ""."FloatFxGroupEffect/InitData"
);
#[cfg(feature = "FloatFxGroupEffect+InitData")]
impl std::ops::Deref for crate::GlobalNamespace::FloatFxGroupEffect_InitData {
    type Target = quest_hook::libil2cpp::Il2CppObject;
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
    pub fn New(
        groupId: i32,
        elementId: i32,
        target: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::FloatFxGroupEffectTarget,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (groupId, elementId, target))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        groupId: i32,
        elementId: i32,
        target: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::FloatFxGroupEffectTarget,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (groupId, elementId, target))?;
        Ok(__cordl_ret.into())
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
