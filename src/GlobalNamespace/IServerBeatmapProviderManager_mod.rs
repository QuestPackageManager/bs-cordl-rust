#[cfg(feature = "IServerBeatmapProviderManager")]
#[repr(C)]
#[derive(Debug)]
pub struct IServerBeatmapProviderManager {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "IServerBeatmapProviderManager")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for IServerBeatmapProviderManager => ""
    ."IServerBeatmapProviderManager"
);
#[cfg(feature = "IServerBeatmapProviderManager")]
impl std::ops::Deref for IServerBeatmapProviderManager {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "IServerBeatmapProviderManager")]
impl std::ops::DerefMut for IServerBeatmapProviderManager {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "IServerBeatmapProviderManager")]
impl IServerBeatmapProviderManager {
    pub fn GetServerBeatmapProvider(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut IServerBeatmapProvider> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut IServerBeatmapProvider = __cordl_object
            .invoke("GetServerBeatmapProvider", ())?;
        Ok(__cordl_ret)
    }
    pub fn RefreshAsync(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Threading::Tasks::Task> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Threading::Tasks::Task = __cordl_object
            .invoke("RefreshAsync", ())?;
        Ok(__cordl_ret)
    }
    pub fn from_object_mut(
        object_param: *mut quest_hook::libil2cpp::Il2CppObject,
    ) -> *mut Self {
        unsafe { (object_param as *mut Self) }
    }
}
#[cfg(feature = "IServerBeatmapProviderManager")]
impl quest_hook::libil2cpp::ObjectType for IServerBeatmapProviderManager {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
