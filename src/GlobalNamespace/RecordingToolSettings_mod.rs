#[cfg(feature = "RecordingToolSettings")]
#[repr(C)]
#[derive(Debug)]
pub struct RecordingToolSettings {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub recordingSettings: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<
            *mut crate::GlobalNamespace::RecordingSettings,
        >,
    >,
}
#[cfg(feature = "RecordingToolSettings")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::RecordingToolSettings => ""
    ."RecordingToolSettings"
);
#[cfg(feature = "RecordingToolSettings")]
impl std::ops::Deref for crate::GlobalNamespace::RecordingToolSettings {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "RecordingToolSettings")]
impl std::ops::DerefMut for crate::GlobalNamespace::RecordingToolSettings {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "RecordingToolSettings")]
impl crate::GlobalNamespace::RecordingToolSettings {
    pub fn New(
        recordingSettings: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                *mut crate::GlobalNamespace::RecordingSettings,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (recordingSettings))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        recordingSettings: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                *mut crate::GlobalNamespace::RecordingSettings,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (recordingSettings))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "RecordingToolSettings")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::RecordingToolSettings {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
