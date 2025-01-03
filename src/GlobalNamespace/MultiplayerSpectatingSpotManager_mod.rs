#[cfg(feature = "MultiplayerSpectatingSpotManager")]
#[repr(C)]
#[derive(Debug)]
pub struct MultiplayerSpectatingSpotManager {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _spectatingSpots: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::List_1<
            *mut crate::GlobalNamespace::IMultiplayerSpectatingSpot,
        >,
    >,
    pub _spotIndexBySpot: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::Dictionary_2<
            *mut crate::GlobalNamespace::IMultiplayerSpectatingSpot,
            i32,
        >,
    >,
}
#[cfg(feature = "MultiplayerSpectatingSpotManager")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::MultiplayerSpectatingSpotManager => ""
    ."MultiplayerSpectatingSpotManager"
);
#[cfg(feature = "MultiplayerSpectatingSpotManager")]
impl std::ops::Deref for crate::GlobalNamespace::MultiplayerSpectatingSpotManager {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "MultiplayerSpectatingSpotManager")]
impl std::ops::DerefMut for crate::GlobalNamespace::MultiplayerSpectatingSpotManager {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "MultiplayerSpectatingSpotManager")]
impl crate::GlobalNamespace::MultiplayerSpectatingSpotManager {
    pub fn GetAdjacentSpot(
        &mut self,
        spectatingSpot: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::IMultiplayerSpectatingSpot,
        >,
        offset: i32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::IMultiplayerSpectatingSpot>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::IMultiplayerSpectatingSpot,
        > = __cordl_object.invoke("GetAdjacentSpot", (spectatingSpot, offset))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetIndexBySpot(
        &mut self,
        spectatingSpot: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::IMultiplayerSpectatingSpot,
        >,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object
            .invoke("GetIndexBySpot", (spectatingSpot))?;
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn RegisterSpectatingSpot(
        &mut self,
        spectatingSpot: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::IMultiplayerSpectatingSpot,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("RegisterSpectatingSpot", (spectatingSpot))?;
        Ok(__cordl_ret.into())
    }
    pub fn SpotOnHasBeenRemoved(
        &mut self,
        spectatingSpot: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::IMultiplayerSpectatingSpot,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SpotOnHasBeenRemoved", (spectatingSpot))?;
        Ok(__cordl_ret.into())
    }
    pub fn UpdateIndexBySpotDictionary(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UpdateIndexBySpotDictionary", ())?;
        Ok(__cordl_ret.into())
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
    pub fn get_defaultSpot(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::IMultiplayerSpectatingSpot>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::IMultiplayerSpectatingSpot,
        > = __cordl_object.invoke("get_defaultSpot", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_spectatingSpots(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IReadOnlyList_1<
                *mut crate::GlobalNamespace::IMultiplayerSpectatingSpot,
            >,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IReadOnlyList_1<
                *mut crate::GlobalNamespace::IMultiplayerSpectatingSpot,
            >,
        > = __cordl_object.invoke("get_spectatingSpots", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "MultiplayerSpectatingSpotManager")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::MultiplayerSpectatingSpotManager {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
