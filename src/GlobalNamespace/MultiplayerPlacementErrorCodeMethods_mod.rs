#[cfg(feature = "MultiplayerPlacementErrorCodeMethods")]
#[repr(C)]
#[derive(Debug)]
pub struct MultiplayerPlacementErrorCodeMethods {
    __cordl_parent: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
}
#[cfg(feature = "MultiplayerPlacementErrorCodeMethods")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::MultiplayerPlacementErrorCodeMethods => ""
    ."MultiplayerPlacementErrorCodeMethods"
);
#[cfg(feature = "MultiplayerPlacementErrorCodeMethods")]
impl std::ops::Deref for crate::GlobalNamespace::MultiplayerPlacementErrorCodeMethods {
    type Target = quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "MultiplayerPlacementErrorCodeMethods")]
impl std::ops::DerefMut
for crate::GlobalNamespace::MultiplayerPlacementErrorCodeMethods {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "MultiplayerPlacementErrorCodeMethods")]
impl crate::GlobalNamespace::MultiplayerPlacementErrorCodeMethods {
    pub fn ToConnectionFailedReason(
        errorCode: crate::GlobalNamespace::MultiplayerPlacementErrorCode,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::ConnectionFailedReason> {
        let __cordl_ret: crate::GlobalNamespace::ConnectionFailedReason = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ToConnectionFailedReason", (errorCode))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "MultiplayerPlacementErrorCodeMethods")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::MultiplayerPlacementErrorCodeMethods {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
