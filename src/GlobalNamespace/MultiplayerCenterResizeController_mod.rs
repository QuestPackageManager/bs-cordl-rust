#[cfg(feature = "MultiplayerCenterResizeController")]
#[repr(C)]
#[derive(Debug)]
pub struct MultiplayerCenterResizeController {
    __cordl_parent: crate::UnityEngine::MonoBehaviour,
    pub _platformWidth: f32,
    pub _layoutProvider: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::MultiplayerLayoutProvider,
    >,
    pub edgeDistanceFromCenterWasCalculatedEvent: quest_hook::libil2cpp::Gc<
        crate::System::Action_1<f32>,
    >,
    pub _isEdgeDistanceFromCenterCalculated_k__BackingField: bool,
    pub _edgeDistanceFromCenter_k__BackingField: f32,
}
#[cfg(feature = "MultiplayerCenterResizeController")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::MultiplayerCenterResizeController => ""
    ."MultiplayerCenterResizeController"
);
#[cfg(feature = "MultiplayerCenterResizeController")]
impl std::ops::Deref for crate::GlobalNamespace::MultiplayerCenterResizeController {
    type Target = crate::UnityEngine::MonoBehaviour;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "MultiplayerCenterResizeController")]
impl std::ops::DerefMut for crate::GlobalNamespace::MultiplayerCenterResizeController {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "MultiplayerCenterResizeController")]
impl crate::GlobalNamespace::MultiplayerCenterResizeController {
    pub fn HandlePlayersLayoutWasCalculated(
        &mut self,
        layout: crate::GlobalNamespace::MultiplayerPlayerLayout,
        numberOfPlayers: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandlePlayersLayoutWasCalculated", (layout, numberOfPlayers))?;
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
    pub fn Start(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Start", ())?;
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
    pub fn add_edgeDistanceFromCenterWasCalculatedEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<crate::System::Action_1<f32>>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_edgeDistanceFromCenterWasCalculatedEvent", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_edgeDistanceFromCenter(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("get_edgeDistanceFromCenter", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_isEdgeDistanceFromCenterCalculated(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("get_isEdgeDistanceFromCenterCalculated", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn remove_edgeDistanceFromCenterWasCalculatedEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<crate::System::Action_1<f32>>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_edgeDistanceFromCenterWasCalculatedEvent", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_edgeDistanceFromCenter(
        &mut self,
        value: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_edgeDistanceFromCenter", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_isEdgeDistanceFromCenterCalculated(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_isEdgeDistanceFromCenterCalculated", (value))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "MultiplayerCenterResizeController")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::MultiplayerCenterResizeController {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
