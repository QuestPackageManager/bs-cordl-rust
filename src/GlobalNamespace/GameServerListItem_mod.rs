#[cfg(feature = "GameServerListItem")]
#[repr(C)]
#[derive(Debug)]
pub struct GameServerListItem {
    __cordl_parent: crate::System::Object,
    pub serverName: *mut crate::System::String,
    pub capacity: i32,
    pub occupied: i32,
    pub password: bool,
}
#[cfg(feature = "GameServerListItem")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::GameServerListItem => ""
    ."GameServerListItem"
);
#[cfg(feature = "GameServerListItem")]
impl std::ops::Deref for crate::GlobalNamespace::GameServerListItem {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "GameServerListItem")]
impl std::ops::DerefMut for crate::GlobalNamespace::GameServerListItem {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "GameServerListItem")]
impl crate::GlobalNamespace::GameServerListItem {
    pub fn New(
        serverName: *mut crate::System::String,
        capacity: i32,
        occupied: i32,
        password: bool,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (serverName, capacity, occupied, password))?;
        Ok(__cordl_object)
    }
    pub fn _ctor(
        &mut self,
        serverName: *mut crate::System::String,
        capacity: i32,
        occupied: i32,
        password: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (serverName, capacity, occupied, password))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "GameServerListItem")]
impl quest_hook::libil2cpp::ObjectType for crate::GlobalNamespace::GameServerListItem {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
