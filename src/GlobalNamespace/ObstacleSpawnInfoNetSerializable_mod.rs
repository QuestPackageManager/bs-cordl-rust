#[cfg(feature = "ObstacleSpawnInfoNetSerializable")]
#[repr(C)]
#[derive(Debug)]
pub struct ObstacleSpawnInfoNetSerializable {
    __cordl_parent: PoolableSerializable,
    pub _cordl_time: f32,
    pub lineIndex: i32,
    pub lineLayer: NoteLineLayer,
    pub duration: f32,
    pub width: i32,
    pub height: i32,
    pub moveStartPos: Vector3Serializable,
    pub moveEndPos: Vector3Serializable,
    pub jumpEndPos: Vector3Serializable,
    pub obstacleHeight: f32,
    pub moveDuration: f32,
    pub jumpDuration: f32,
    pub noteLinesDistance: f32,
    pub rotation: f32,
}
#[cfg(feature = "ObstacleSpawnInfoNetSerializable")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for ObstacleSpawnInfoNetSerializable => ""
    ."ObstacleSpawnInfoNetSerializable"
);
#[cfg(feature = "ObstacleSpawnInfoNetSerializable")]
impl std::ops::Deref for ObstacleSpawnInfoNetSerializable {
    type Target = PoolableSerializable;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "ObstacleSpawnInfoNetSerializable")]
impl std::ops::DerefMut for ObstacleSpawnInfoNetSerializable {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "ObstacleSpawnInfoNetSerializable")]
impl ObstacleSpawnInfoNetSerializable {
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
    pub fn Deserialize(
        &mut self,
        reader: *mut crate::LiteNetLib::Utils::NetDataReader,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Deserialize", (reader))?;
        Ok(__cordl_ret)
    }
    pub fn Serialize(
        &mut self,
        writer: *mut crate::LiteNetLib::Utils::NetDataWriter,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Serialize", (writer))?;
        Ok(__cordl_ret)
    }
    pub fn Init(
        &mut self,
        _cordl_time: f32,
        lineIndex: i32,
        lineLayer: NoteLineLayer,
        duration: f32,
        width: i32,
        height: i32,
        moveStartPos: crate::UnityEngine::Vector3,
        moveEndPos: crate::UnityEngine::Vector3,
        jumpEndPos: crate::UnityEngine::Vector3,
        obstacleHeight: f32,
        moveDuration: f32,
        jumpDuration: f32,
        noteLinesDistance: f32,
        rotation: f32,
    ) -> quest_hook::libil2cpp::Result<*mut ObstacleSpawnInfoNetSerializable> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut ObstacleSpawnInfoNetSerializable = __cordl_object
            .invoke(
                "Init",
                (
                    _cordl_time,
                    lineIndex,
                    lineLayer,
                    duration,
                    width,
                    height,
                    moveStartPos,
                    moveEndPos,
                    jumpEndPos,
                    obstacleHeight,
                    moveDuration,
                    jumpDuration,
                    noteLinesDistance,
                    rotation,
                ),
            )?;
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
#[cfg(feature = "ObstacleSpawnInfoNetSerializable")]
impl quest_hook::libil2cpp::ObjectType for ObstacleSpawnInfoNetSerializable {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
