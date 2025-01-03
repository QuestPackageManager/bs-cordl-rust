#[cfg(feature = "RecordingConverter")]
#[repr(C)]
#[derive(Debug)]
pub struct RecordingConverter {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _logger: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::IBeatSaberLogger>,
}
#[cfg(feature = "RecordingConverter")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::RecordingConverter => ""
    ."RecordingConverter"
);
#[cfg(feature = "RecordingConverter")]
impl std::ops::Deref for crate::GlobalNamespace::RecordingConverter {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "RecordingConverter")]
impl std::ops::DerefMut for crate::GlobalNamespace::RecordingConverter {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "RecordingConverter")]
impl crate::GlobalNamespace::RecordingConverter {
    pub fn New(
        logger: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::IBeatSaberLogger>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (logger))?;
        Ok(__cordl_object.into())
    }
    pub fn SaveToOldFormat(
        &mut self,
        path: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        data: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::PosesRecordingData>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SaveToOldFormat", (path, data))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        logger: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::IBeatSaberLogger>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (logger))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "RecordingConverter")]
impl quest_hook::libil2cpp::ObjectType for crate::GlobalNamespace::RecordingConverter {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
