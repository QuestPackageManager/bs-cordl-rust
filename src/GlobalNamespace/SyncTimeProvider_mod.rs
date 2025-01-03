#[cfg(feature = "SyncTimeProvider")]
#[repr(C)]
#[derive(Debug)]
pub struct SyncTimeProvider {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _multiplayerSessionManager: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::IMultiplayerSessionManager,
    >,
}
#[cfg(feature = "SyncTimeProvider")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::SyncTimeProvider => ""
    ."SyncTimeProvider"
);
#[cfg(feature = "SyncTimeProvider")]
impl std::ops::Deref for crate::GlobalNamespace::SyncTimeProvider {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "SyncTimeProvider")]
impl std::ops::DerefMut for crate::GlobalNamespace::SyncTimeProvider {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "SyncTimeProvider")]
impl crate::GlobalNamespace::SyncTimeProvider {
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_time(&mut self) -> quest_hook::libil2cpp::Result<i64> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i64 = __cordl_object.invoke("get_time", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "SyncTimeProvider")]
impl quest_hook::libil2cpp::ObjectType for crate::GlobalNamespace::SyncTimeProvider {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "SyncTimeProvider")]
impl AsRef<crate::GlobalNamespace::ITimeProvider>
for crate::GlobalNamespace::SyncTimeProvider {
    fn as_ref(&self) -> &crate::GlobalNamespace::ITimeProvider {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "SyncTimeProvider")]
impl AsMut<crate::GlobalNamespace::ITimeProvider>
for crate::GlobalNamespace::SyncTimeProvider {
    fn as_mut(&mut self) -> &mut crate::GlobalNamespace::ITimeProvider {
        unsafe { std::mem::transmute(self) }
    }
}
