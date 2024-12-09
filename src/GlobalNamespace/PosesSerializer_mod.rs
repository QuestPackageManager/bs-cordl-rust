#[cfg(feature = "PosesSerializer")]
#[repr(C)]
#[derive(Debug)]
pub struct PosesSerializer {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _logger: *mut crate::GlobalNamespace::IBeatSaberLogger,
    pub _recordingConverter: *mut crate::GlobalNamespace::RecordingConverter,
}
#[cfg(feature = "PosesSerializer")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::PosesSerializer => ""
    ."PosesSerializer"
);
#[cfg(feature = "PosesSerializer")]
impl std::ops::Deref for crate::GlobalNamespace::PosesSerializer {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "PosesSerializer")]
impl std::ops::DerefMut for crate::GlobalNamespace::PosesSerializer {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "PosesSerializer")]
impl crate::GlobalNamespace::PosesSerializer {
    pub const kDataFileName: &'static str = "Data.rcd";
    pub const kInfoFileName: &'static str = "Info.json";
    pub fn LoadDataFile(
        &mut self,
        filePath: *mut quest_hook::libil2cpp::Il2CppString,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Collections::Generic::List_1<
            *mut crate::GlobalNamespace::PosesRecordingData_TransformsKeyframe,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Collections::Generic::List_1<
            *mut crate::GlobalNamespace::PosesRecordingData_TransformsKeyframe,
        > = __cordl_object.invoke("LoadDataFile", (filePath))?;
        Ok(__cordl_ret)
    }
    pub fn LoadInfoFile(
        &mut self,
        filePath: *mut quest_hook::libil2cpp::Il2CppString,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::GlobalNamespace::PosesRecordingInfoSaveData,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::GlobalNamespace::PosesRecordingInfoSaveData = __cordl_object
            .invoke("LoadInfoFile", (filePath))?;
        Ok(__cordl_ret)
    }
    pub fn LoadRecording(
        &mut self,
        path: *mut quest_hook::libil2cpp::Il2CppString,
    ) -> quest_hook::libil2cpp::Result<*mut crate::GlobalNamespace::PosesRecordingData> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::GlobalNamespace::PosesRecordingData = __cordl_object
            .invoke("LoadRecording", (path))?;
        Ok(__cordl_ret)
    }
    pub fn LoadRecordingFromDirectory(
        &mut self,
        path: *mut quest_hook::libil2cpp::Il2CppString,
    ) -> quest_hook::libil2cpp::Result<*mut crate::GlobalNamespace::PosesRecordingData> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::GlobalNamespace::PosesRecordingData = __cordl_object
            .invoke("LoadRecordingFromDirectory", (path))?;
        Ok(__cordl_ret)
    }
    pub fn New(
        logger: *mut crate::GlobalNamespace::IBeatSaberLogger,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (logger))?;
        Ok(__cordl_object)
    }
    pub fn RecordingCanBeCreated(
        &mut self,
        path: *mut quest_hook::libil2cpp::Il2CppString,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("RecordingCanBeCreated", (path))?;
        Ok(__cordl_ret)
    }
    pub fn RecordingExists(
        &mut self,
        path: *mut quest_hook::libil2cpp::Il2CppString,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("RecordingExists", (path))?;
        Ok(__cordl_ret)
    }
    pub fn SaveRecording(
        &mut self,
        path: *mut quest_hook::libil2cpp::Il2CppString,
        data: *mut crate::GlobalNamespace::PosesRecordingData,
        saveToOldFormat: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SaveRecording", (path, data, saveToOldFormat))?;
        Ok(__cordl_ret)
    }
    pub fn SaveRecordingIntoDirectory(
        &mut self,
        path: *mut quest_hook::libil2cpp::Il2CppString,
        data: *mut crate::GlobalNamespace::PosesRecordingData,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SaveRecordingIntoDirectory", (path, data))?;
        Ok(__cordl_ret)
    }
    pub fn SaveToOldFormat(
        &mut self,
        path: *mut quest_hook::libil2cpp::Il2CppString,
        data: *mut crate::GlobalNamespace::PosesRecordingData,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SaveToOldFormat", (path, data))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
        logger: *mut crate::GlobalNamespace::IBeatSaberLogger,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (logger))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "PosesSerializer")]
impl quest_hook::libil2cpp::ObjectType for crate::GlobalNamespace::PosesSerializer {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
