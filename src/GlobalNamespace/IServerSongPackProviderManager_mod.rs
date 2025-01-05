#[cfg(feature = "IServerSongPackProviderManager")]
#[repr(C)]
#[derive(Debug)]
pub struct IServerSongPackProviderManager {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "IServerSongPackProviderManager")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::IServerSongPackProviderManager
    => ""."IServerSongPackProviderManager"
);
#[cfg(feature = "IServerSongPackProviderManager")]
impl std::ops::Deref for crate::GlobalNamespace::IServerSongPackProviderManager {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "IServerSongPackProviderManager")]
impl std::ops::DerefMut for crate::GlobalNamespace::IServerSongPackProviderManager {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "IServerSongPackProviderManager")]
impl crate::GlobalNamespace::IServerSongPackProviderManager {
    pub fn GetServerSongPackProvider(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::IServerSongPackProvider>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::IServerSongPackProvider,
        > = __cordl_object.invoke("GetServerSongPackProvider", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn RefreshAsync(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Threading::Tasks::Task>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task,
        > = __cordl_object.invoke("RefreshAsync", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn from_object_mut(
        object_param: *mut quest_hook::libil2cpp::Il2CppObject,
    ) -> *mut Self {
        unsafe { (object_param as *mut Self) }
    }
}
#[cfg(feature = "IServerSongPackProviderManager")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::IServerSongPackProviderManager {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "IServerSongPackProviderManager")]
impl AsRef<quest_hook::libil2cpp::Gc<crate::System::IDisposable>>
for crate::GlobalNamespace::IServerSongPackProviderManager {
    fn as_ref(&self) -> &quest_hook::libil2cpp::Gc<crate::System::IDisposable> {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "IServerSongPackProviderManager")]
impl AsMut<quest_hook::libil2cpp::Gc<crate::System::IDisposable>>
for crate::GlobalNamespace::IServerSongPackProviderManager {
    fn as_mut(&mut self) -> &mut quest_hook::libil2cpp::Gc<crate::System::IDisposable> {
        unsafe { std::mem::transmute(self) }
    }
}
