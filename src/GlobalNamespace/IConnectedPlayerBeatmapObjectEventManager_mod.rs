#[cfg(feature = "IConnectedPlayerBeatmapObjectEventManager")]
#[repr(C)]
#[derive(Debug)]
pub struct IConnectedPlayerBeatmapObjectEventManager {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "IConnectedPlayerBeatmapObjectEventManager")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for IConnectedPlayerBeatmapObjectEventManager => ""
    ."IConnectedPlayerBeatmapObjectEventManager"
);
#[cfg(feature = "IConnectedPlayerBeatmapObjectEventManager")]
impl std::ops::Deref for IConnectedPlayerBeatmapObjectEventManager {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "IConnectedPlayerBeatmapObjectEventManager")]
impl std::ops::DerefMut for IConnectedPlayerBeatmapObjectEventManager {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "IConnectedPlayerBeatmapObjectEventManager")]
impl IConnectedPlayerBeatmapObjectEventManager {
    pub fn Pause(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Pause", ())?;
        Ok(__cordl_ret)
    }
    pub fn Resume(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Resume", ())?;
        Ok(__cordl_ret)
    }
    pub fn add_connectedPlayerNoteWasCutEvent(
        &mut self,
        value: *mut crate::System::Action_1<*mut NoteCutInfoNetSerializable>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_connectedPlayerNoteWasCutEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn add_connectedPlayerNoteWasMissedEvent(
        &mut self,
        value: *mut crate::System::Action_1<*mut NoteMissInfoNetSerializable>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_connectedPlayerNoteWasMissedEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn add_connectedPlayerNoteWasSpawnedEvent(
        &mut self,
        value: *mut crate::System::Action_1<*mut NoteSpawnInfoNetSerializable>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_connectedPlayerNoteWasSpawnedEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn add_connectedPlayerObstacleWasSpawnedEvent(
        &mut self,
        value: *mut crate::System::Action_1<*mut ObstacleSpawnInfoNetSerializable>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_connectedPlayerObstacleWasSpawnedEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn add_connectedPlayerSliderWasSpawnedEvent(
        &mut self,
        value: *mut crate::System::Action_1<*mut SliderSpawnInfoNetSerializable>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_connectedPlayerSliderWasSpawnedEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn from_object_mut(
        object_param: *mut quest_hook::libil2cpp::Il2CppObject,
    ) -> *mut Self {
        unsafe { (object_param as *mut Self) }
    }
    pub fn remove_connectedPlayerNoteWasCutEvent(
        &mut self,
        value: *mut crate::System::Action_1<*mut NoteCutInfoNetSerializable>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_connectedPlayerNoteWasCutEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn remove_connectedPlayerNoteWasMissedEvent(
        &mut self,
        value: *mut crate::System::Action_1<*mut NoteMissInfoNetSerializable>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_connectedPlayerNoteWasMissedEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn remove_connectedPlayerNoteWasSpawnedEvent(
        &mut self,
        value: *mut crate::System::Action_1<*mut NoteSpawnInfoNetSerializable>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_connectedPlayerNoteWasSpawnedEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn remove_connectedPlayerObstacleWasSpawnedEvent(
        &mut self,
        value: *mut crate::System::Action_1<*mut ObstacleSpawnInfoNetSerializable>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_connectedPlayerObstacleWasSpawnedEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn remove_connectedPlayerSliderWasSpawnedEvent(
        &mut self,
        value: *mut crate::System::Action_1<*mut SliderSpawnInfoNetSerializable>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_connectedPlayerSliderWasSpawnedEvent", (value))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "IConnectedPlayerBeatmapObjectEventManager")]
impl quest_hook::libil2cpp::ObjectType for IConnectedPlayerBeatmapObjectEventManager {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}