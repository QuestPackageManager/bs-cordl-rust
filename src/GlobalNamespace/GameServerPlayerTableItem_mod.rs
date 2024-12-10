#[cfg(feature = "GameServerPlayerTableItem")]
#[repr(C)]
#[derive(Debug)]
pub struct GameServerPlayerTableItem {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub playerName: *mut quest_hook::libil2cpp::Il2CppString,
    pub suggestedLevel: *mut quest_hook::libil2cpp::Il2CppString,
    pub suggestedModifiers: *mut quest_hook::libil2cpp::Il2CppString,
    pub isReady: bool,
}
#[cfg(feature = "GameServerPlayerTableItem")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::GameServerPlayerTableItem => ""
    ."GameServerPlayerTableItem"
);
#[cfg(feature = "GameServerPlayerTableItem")]
impl std::ops::Deref for crate::GlobalNamespace::GameServerPlayerTableItem {
    type Target = quest_hook::libil2cpp::Il2CppObject;
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
        playerName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        suggestedLevel: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        suggestedModifiers: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        >,
        isReady: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(
                ".ctor",
                (playerName, suggestedLevel, suggestedModifiers, isReady),
            )?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        playerName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        suggestedLevel: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        suggestedModifiers: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        >,
        isReady: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (playerName, suggestedLevel, suggestedModifiers, isReady))?;
        Ok(__cordl_ret.into())
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
