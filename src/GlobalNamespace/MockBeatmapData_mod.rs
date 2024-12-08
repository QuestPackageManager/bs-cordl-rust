#[cfg(feature = "MockBeatmapData")]
#[repr(C)]
#[derive(Debug)]
pub struct MockBeatmapData {
    __cordl_parent: crate::System::Object,
    pub _numberOfLines_k__BackingField: i32,
    pub _songEndTime_k__BackingField: f32,
    pub _leftNotes_k__BackingField: *mut quest_hook::libil2cpp::Il2CppArray<
        *mut MockNoteData,
    >,
    pub _rightNotes_k__BackingField: *mut quest_hook::libil2cpp::Il2CppArray<
        *mut MockNoteData,
    >,
    pub _bombNotes_k__BackingField: *mut quest_hook::libil2cpp::Il2CppArray<
        *mut MockNoteData,
    >,
    pub _obstacles_k__BackingField: *mut quest_hook::libil2cpp::Il2CppArray<
        *mut MockObstacleData,
    >,
}
#[cfg(feature = "MockBeatmapData")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for MockBeatmapData => ""."MockBeatmapData"
);
#[cfg(feature = "MockBeatmapData")]
impl std::ops::Deref for MockBeatmapData {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "MockBeatmapData")]
impl std::ops::DerefMut for MockBeatmapData {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "MockBeatmapData")]
impl MockBeatmapData {
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
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
    pub fn get_bombNotes(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut quest_hook::libil2cpp::Il2CppArray<*mut MockNoteData>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppArray<*mut MockNoteData> = __cordl_object
            .invoke("get_bombNotes", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_leftNotes(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut quest_hook::libil2cpp::Il2CppArray<*mut MockNoteData>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppArray<*mut MockNoteData> = __cordl_object
            .invoke("get_leftNotes", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_numberOfLines(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_numberOfLines", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_obstacles(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut quest_hook::libil2cpp::Il2CppArray<*mut MockObstacleData>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppArray<
            *mut MockObstacleData,
        > = __cordl_object.invoke("get_obstacles", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_rightNotes(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut quest_hook::libil2cpp::Il2CppArray<*mut MockNoteData>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppArray<*mut MockNoteData> = __cordl_object
            .invoke("get_rightNotes", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_songEndTime(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("get_songEndTime", ())?;
        Ok(__cordl_ret)
    }
    pub fn set_bombNotes(
        &mut self,
        value: *mut quest_hook::libil2cpp::Il2CppArray<*mut MockNoteData>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_bombNotes", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_leftNotes(
        &mut self,
        value: *mut quest_hook::libil2cpp::Il2CppArray<*mut MockNoteData>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_leftNotes", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_numberOfLines(
        &mut self,
        value: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_numberOfLines", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_obstacles(
        &mut self,
        value: *mut quest_hook::libil2cpp::Il2CppArray<*mut MockObstacleData>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_obstacles", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_rightNotes(
        &mut self,
        value: *mut quest_hook::libil2cpp::Il2CppArray<*mut MockNoteData>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_rightNotes", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_songEndTime(
        &mut self,
        value: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_songEndTime", (value))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "MockBeatmapData")]
impl quest_hook::libil2cpp::ObjectType for MockBeatmapData {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}