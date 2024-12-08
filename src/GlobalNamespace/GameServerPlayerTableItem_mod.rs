#[cfg(feature = "GameServerPlayerTableItem")]
#[repr(C)]
#[derive(Debug)]
pub struct GameServerPlayerTableItem {
    __cordl_parent: crate::System::Object,
    pub playerName: *mut crate::System::String,
    pub suggestedLevel: *mut crate::System::String,
    pub suggestedModifiers: *mut crate::System::String,
    pub isReady: bool,
}
#[cfg(feature = "GameServerPlayerTableItem")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::GameServerPlayerTableItem => ""
    ."GameServerPlayerTableItem"
);
#[cfg(feature = "GameServerPlayerTableItem")]
impl std::ops::Deref for crate::GlobalNamespace::GameServerPlayerTableItem {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "GameServerPlayerTableItem")]
impl std::ops::DerefMut for crate::GlobalNamespace::GameServerPlayerTableItem {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "GameServerPlayerTableItem")]
impl crate::GlobalNamespace::GameServerPlayerTableItem {
    pub fn New(
        playerName: *mut crate::System::String,
        suggestedLevel: *mut crate::System::String,
        suggestedModifiers: *mut crate::System::String,
        isReady: bool,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(
                ".ctor",
                (playerName, suggestedLevel, suggestedModifiers, isReady),
            )?;
        Ok(__cordl_object)
    }
    pub fn _ctor(
        &mut self,
        playerName: *mut crate::System::String,
        suggestedLevel: *mut crate::System::String,
        suggestedModifiers: *mut crate::System::String,
        isReady: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (playerName, suggestedLevel, suggestedModifiers, isReady))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "GameServerPlayerTableItem")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::GameServerPlayerTableItem {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
