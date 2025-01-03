#[cfg(feature = "GameObjectIntSwitchEventEffect")]
#[repr(C)]
#[derive(Debug)]
pub struct GameObjectIntSwitchEventEffect {
    __cordl_parent: crate::UnityEngine::MonoBehaviour,
    pub _beatmapEventType: crate::GlobalNamespace::BasicBeatmapEventType,
    pub _defaultValue: i32,
    pub _gameObjectsValueLists: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<
            *mut crate::GlobalNamespace::GameObjectIntSwitchEventEffect_GameObjectValueList,
        >,
    >,
    pub _beatmapCallbacksController: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::BeatmapCallbacksController,
    >,
    pub _beatmapDataCallbackWrapper: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::BeatmapDataCallbackWrapper,
    >,
    pub _valueToListMap: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::Dictionary_2<
            i32,
            *mut crate::GlobalNamespace::GameObjectIntSwitchEventEffect_GameObjectValueList,
        >,
    >,
    pub _previousActiveIdx: i32,
}
#[cfg(feature = "GameObjectIntSwitchEventEffect")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::GameObjectIntSwitchEventEffect
    => ""."GameObjectIntSwitchEventEffect"
);
#[cfg(feature = "GameObjectIntSwitchEventEffect")]
impl std::ops::Deref for crate::GlobalNamespace::GameObjectIntSwitchEventEffect {
    type Target = crate::UnityEngine::MonoBehaviour;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "GameObjectIntSwitchEventEffect")]
impl std::ops::DerefMut for crate::GlobalNamespace::GameObjectIntSwitchEventEffect {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "GameObjectIntSwitchEventEffect")]
impl crate::GlobalNamespace::GameObjectIntSwitchEventEffect {
    #[cfg(feature = "GameObjectIntSwitchEventEffect+GameObjectValueList")]
    pub type GameObjectValueList = crate::GlobalNamespace::GameObjectIntSwitchEventEffect_GameObjectValueList;
    pub fn HandleBasicBeatmapEventData(
        &mut self,
        data: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::BasicBeatmapEventData>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleBasicBeatmapEventData", (data))?;
        Ok(__cordl_ret.into())
    }
    pub fn Initialize(
        &mut self,
        beatmapCallbacksController: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::BeatmapCallbacksController,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Initialize", (beatmapCallbacksController))?;
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn OnDestroy(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnDestroy", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "GameObjectIntSwitchEventEffect")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::GameObjectIntSwitchEventEffect {
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
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub value: i32,
    pub gameObjects: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<*mut crate::UnityEngine::GameObject>,
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
    type Target = quest_hook::libil2cpp::Il2CppObject;
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
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
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
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_isActive(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_isActive", ())?;
        Ok(__cordl_ret.into())
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
        Ok(__cordl_ret.into())
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
