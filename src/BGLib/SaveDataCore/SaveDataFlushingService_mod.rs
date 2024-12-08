#[cfg(feature = "BGLib+SaveDataCore+SaveDataFlushingService")]
#[repr(C)]
#[derive(Debug)]
pub struct SaveDataFlushingService {
    __cordl_parent: crate::System::Object,
    pub blockingObjects: *mut crate::System::Collections::Generic::HashSet_1<
        *mut crate::UnityEngine::Object,
    >,
    pub _handlers: *mut crate::System::Collections::Generic::List_1<
        *mut crate::BGLib::SaveDataCore::ISaveDataHandler,
    >,
}
#[cfg(feature = "BGLib+SaveDataCore+SaveDataFlushingService")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::BGLib::SaveDataCore::SaveDataFlushingService =>
    "BGLib.SaveDataCore"."SaveDataFlushingService"
);
#[cfg(feature = "BGLib+SaveDataCore+SaveDataFlushingService")]
impl std::ops::Deref for crate::BGLib::SaveDataCore::SaveDataFlushingService {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BGLib+SaveDataCore+SaveDataFlushingService")]
impl std::ops::DerefMut for crate::BGLib::SaveDataCore::SaveDataFlushingService {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BGLib+SaveDataCore+SaveDataFlushingService")]
impl crate::BGLib::SaveDataCore::SaveDataFlushingService {
    #[cfg(
        feature = "BGLib+SaveDataCore+SaveDataFlushingService+_ResetChangesAsync_d__4"
    )]
    pub type _ResetChangesAsync_d__4 = crate::BGLib::SaveDataCore::SaveDataFlushingService__ResetChangesAsync_d__4;
    #[cfg(
        feature = "BGLib+SaveDataCore+SaveDataFlushingService+_FlushSaveFilesAsync_d__3"
    )]
    pub type _FlushSaveFilesAsync_d__3 = crate::BGLib::SaveDataCore::SaveDataFlushingService__FlushSaveFilesAsync_d__3;
    pub fn FlushSaveFilesAsync(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Threading::Tasks::Task_1<bool>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Threading::Tasks::Task_1<bool> = __cordl_object
            .invoke("FlushSaveFilesAsync", ())?;
        Ok(__cordl_ret)
    }
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
    pub fn Register(
        &mut self,
        handler: *mut crate::BGLib::SaveDataCore::ISaveDataHandler,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Register", (handler))?;
        Ok(__cordl_ret)
    }
    pub fn ReleaseSaveBlocker(
        &mut self,
        o: *mut crate::UnityEngine::Object,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("ReleaseSaveBlocker", (o))?;
        Ok(__cordl_ret)
    }
    pub fn ResetChangesAsync(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Threading::Tasks::Task_1<bool>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Threading::Tasks::Task_1<bool> = __cordl_object
            .invoke("ResetChangesAsync", ())?;
        Ok(__cordl_ret)
    }
    pub fn TrackSaveBlocker(
        &mut self,
        o: *mut crate::UnityEngine::Object,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("TrackSaveBlocker", (o))?;
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
}
#[cfg(feature = "BGLib+SaveDataCore+SaveDataFlushingService")]
impl quest_hook::libil2cpp::ObjectType
for crate::BGLib::SaveDataCore::SaveDataFlushingService {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
