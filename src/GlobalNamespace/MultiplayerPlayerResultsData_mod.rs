#[cfg(feature = "MultiplayerPlayerResultsData")]
#[repr(C)]
#[derive(Debug)]
pub struct MultiplayerPlayerResultsData {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub connectedPlayer: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::IConnectedPlayer,
    >,
    pub multiplayerLevelCompletionResults: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::MultiplayerLevelCompletionResults,
    >,
    pub badge: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::MultiplayerBadgeAwardData,
    >,
}
#[cfg(feature = "MultiplayerPlayerResultsData")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::MultiplayerPlayerResultsData =>
    ""."MultiplayerPlayerResultsData"
);
#[cfg(feature = "MultiplayerPlayerResultsData")]
impl std::ops::Deref for crate::GlobalNamespace::MultiplayerPlayerResultsData {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "MultiplayerPlayerResultsData")]
impl std::ops::DerefMut for crate::GlobalNamespace::MultiplayerPlayerResultsData {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "MultiplayerPlayerResultsData")]
impl crate::GlobalNamespace::MultiplayerPlayerResultsData {
    pub fn CompareTo(
        &mut self,
        obj: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("CompareTo", (obj))?;
        Ok(__cordl_ret.into())
    }
    pub fn New(
        connectedPlayer: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::IConnectedPlayer,
        >,
        multiplayerLevelCompletionResults: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::MultiplayerLevelCompletionResults,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (connectedPlayer, multiplayerLevelCompletionResults))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        connectedPlayer: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::IConnectedPlayer,
        >,
        multiplayerLevelCompletionResults: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::MultiplayerLevelCompletionResults,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (connectedPlayer, multiplayerLevelCompletionResults))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "MultiplayerPlayerResultsData")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::MultiplayerPlayerResultsData {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "MultiplayerPlayerResultsData")]
impl AsRef<crate::System::IComparable>
for crate::GlobalNamespace::MultiplayerPlayerResultsData {
    fn as_ref(&self) -> &crate::System::IComparable {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "MultiplayerPlayerResultsData")]
impl AsMut<crate::System::IComparable>
for crate::GlobalNamespace::MultiplayerPlayerResultsData {
    fn as_mut(&mut self) -> &mut crate::System::IComparable {
        unsafe { std::mem::transmute(self) }
    }
}
