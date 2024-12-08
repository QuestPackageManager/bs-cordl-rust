#[cfg(feature = "IPosesSerializer")]
#[repr(C)]
#[derive(Debug)]
pub struct IPosesSerializer {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "IPosesSerializer")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for IPosesSerializer => ""."IPosesSerializer"
);
#[cfg(feature = "IPosesSerializer")]
impl std::ops::Deref for IPosesSerializer {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "IPosesSerializer")]
impl std::ops::DerefMut for IPosesSerializer {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "IPosesSerializer")]
impl IPosesSerializer {
    pub fn SaveToOldFormat(
        &mut self,
        path: *mut crate::System::String,
        data: *mut PosesRecordingData,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SaveToOldFormat", (path, data))?;
        Ok(__cordl_ret)
    }
    pub fn RecordingCanBeCreated(
        &mut self,
        path: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("RecordingCanBeCreated", (path))?;
        Ok(__cordl_ret)
    }
    pub fn RecordingExists(
        &mut self,
        path: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("RecordingExists", (path))?;
        Ok(__cordl_ret)
    }
    pub fn LoadRecording(
        &mut self,
        path: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<*mut PosesRecordingData> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut PosesRecordingData = __cordl_object
            .invoke("LoadRecording", (path))?;
        Ok(__cordl_ret)
    }
    pub fn SaveRecording(
        &mut self,
        path: *mut crate::System::String,
        data: *mut PosesRecordingData,
        saveToOldFormat: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SaveRecording", (path, data, saveToOldFormat))?;
        Ok(__cordl_ret)
    }
    pub fn from_object_mut(
        object_param: *mut quest_hook::libil2cpp::Il2CppObject,
    ) -> *mut Self {
        unsafe { (object_param as *mut Self) }
    }
}
#[cfg(feature = "IPosesSerializer")]
impl quest_hook::libil2cpp::ObjectType for IPosesSerializer {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
