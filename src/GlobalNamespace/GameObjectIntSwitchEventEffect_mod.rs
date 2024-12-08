#[cfg(feature = "GameObjectIntSwitchEventEffect")]
#[repr(C)]
#[derive(Debug)]
pub struct GameObjectIntSwitchEventEffect {
    __cordl_parent: crate::UnityEngine::MonoBehaviour,
    pub _beatmapEventType: BasicBeatmapEventType,
    pub _defaultValue: i32,
    pub _gameObjectsValueLists: *mut quest_hook::libil2cpp::Il2CppArray<
        *mut crate::GlobalNamespace::GameObjectIntSwitchEventEffect_GameObjectValueList,
    >,
    pub _beatmapCallbacksController: *mut BeatmapCallbacksController,
    pub _beatmapDataCallbackWrapper: *mut BeatmapDataCallbackWrapper,
    pub _valueToListMap: *mut crate::System::Collections::Generic::Dictionary_2<
        i32,
        *mut crate::GlobalNamespace::GameObjectIntSwitchEventEffect_GameObjectValueList,
    >,
    pub _previousActiveIdx: i32,
}
#[cfg(feature = "GameObjectIntSwitchEventEffect")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for GameObjectIntSwitchEventEffect => ""
    ."GameObjectIntSwitchEventEffect"
);
#[cfg(feature = "GameObjectIntSwitchEventEffect")]
impl std::ops::Deref for GameObjectIntSwitchEventEffect {
    type Target = crate::UnityEngine::MonoBehaviour;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "GameObjectIntSwitchEventEffect")]
impl std::ops::DerefMut for GameObjectIntSwitchEventEffect {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "GameObjectIntSwitchEventEffect")]
impl GameObjectIntSwitchEventEffect {
    #[cfg(feature = "GameObjectIntSwitchEventEffect+GameObjectValueList")]
    pub type GameObjectValueList = crate::GlobalNamespace::GameObjectIntSwitchEventEffect_GameObjectValueList;
    pub fn HandleBasicBeatmapEventData(
        &mut self,
        data: *mut BasicBeatmapEventData,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleBasicBeatmapEventData", (data))?;
        Ok(__cordl_ret)
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
    pub fn Initialize(
        &mut self,
        beatmapCallbacksController: *mut BeatmapCallbacksController,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Initialize", (beatmapCallbacksController))?;
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
    pub fn New() -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "GameObjectIntSwitchEventEffect")]
impl quest_hook::libil2cpp::ObjectType for GameObjectIntSwitchEventEffect {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "GameObjectIntSwitchEventEffect+GameObjectValueList")]
#[repr(C)]
#[derive(Debug)]
pub struct GameObjectIntSwitchEventEffect_GameObjectValueList {
    __cordl_parent: crate::System::Object,
    pub value: i32,
    pub gameObjects: *mut quest_hook::libil2cpp::Il2CppArray<
        *mut crate::UnityEngine::GameObject,
    >,
    pub _isActive_k__BackingField: bool,
}
#[cfg(feature = "GameObjectIntSwitchEventEffect+GameObjectValueList")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::GameObjectIntSwitchEventEffect_GameObjectValueList => ""
    ."GameObjectIntSwitchEventEffect/GameObjectValueList"
);
#[cfg(feature = "GameObjectIntSwitchEventEffect+GameObjectValueList")]
impl std::ops::Deref
for crate::GlobalNamespace::GameObjectIntSwitchEventEffect_GameObjectValueList {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "GameObjectIntSwitchEventEffect+GameObjectValueList")]
impl std::ops::DerefMut
for crate::GlobalNamespace::GameObjectIntSwitchEventEffect_GameObjectValueList {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "GameObjectIntSwitchEventEffect+GameObjectValueList")]
impl crate::GlobalNamespace::GameObjectIntSwitchEventEffect_GameObjectValueList {
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
    pub fn set_isActive(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_isActive", (value))?;
        Ok(__cordl_ret)
    }
    pub fn get_isActive(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_isActive", ())?;
        Ok(__cordl_ret)
    }
    pub fn SetActive(
        &mut self,
        active: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetActive", (active))?;
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
#[cfg(feature = "GameObjectIntSwitchEventEffect+GameObjectValueList")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::GameObjectIntSwitchEventEffect_GameObjectValueList {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
