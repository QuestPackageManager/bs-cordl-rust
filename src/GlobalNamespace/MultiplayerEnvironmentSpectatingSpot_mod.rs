#[cfg(feature = "MultiplayerEnvironmentSpectatingSpot")]
#[repr(C)]
#[derive(Debug)]
pub struct MultiplayerEnvironmentSpectatingSpot {
    __cordl_parent: crate::UnityEngine::MonoBehaviour,
    pub _preferredSpectatingSpot: bool,
    pub _displaySpotNumber: bool,
    pub _spotNumber: i32,
    pub _spectatingSpotManager: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::MultiplayerSpectatingSpotManager,
    >,
    pub _activePlayersTimeOffsetAverage: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::MultiplayerActivePlayersTimeOffsetAverage,
    >,
    pub hasBeenRemovedEvent: quest_hook::libil2cpp::Gc<
        crate::System::Action_1<
            quest_hook::libil2cpp::Gc<crate::GlobalNamespace::IMultiplayerSpectatingSpot>,
        >,
    >,
}
#[cfg(feature = "MultiplayerEnvironmentSpectatingSpot")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::MultiplayerEnvironmentSpectatingSpot => ""
    ."MultiplayerEnvironmentSpectatingSpot"
);
#[cfg(feature = "MultiplayerEnvironmentSpectatingSpot")]
impl std::ops::Deref for crate::GlobalNamespace::MultiplayerEnvironmentSpectatingSpot {
    type Target = crate::UnityEngine::MonoBehaviour;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "MultiplayerEnvironmentSpectatingSpot")]
impl std::ops::DerefMut
for crate::GlobalNamespace::MultiplayerEnvironmentSpectatingSpot {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "MultiplayerEnvironmentSpectatingSpot")]
impl crate::GlobalNamespace::MultiplayerEnvironmentSpectatingSpot {
    pub fn IMultiplayerSpectatingSpot_get_transform(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::Transform>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::Transform> = __cordl_object
            .invoke("IMultiplayerSpectatingSpot.get_transform", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn OnDisable(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnDisable", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn SetIsObserved(
        &mut self,
        isObserved: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetIsObserved", (isObserved))?;
        Ok(__cordl_ret.into())
    }
    pub fn Start(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Start", ())?;
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
    pub fn add_hasBeenRemovedEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::System::Action_1<
                quest_hook::libil2cpp::Gc<
                    crate::GlobalNamespace::IMultiplayerSpectatingSpot,
                >,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_hasBeenRemovedEvent", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_isMain(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_isMain", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_observable(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::IMultiplayerObservable>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::IMultiplayerObservable,
        > = __cordl_object.invoke("get_observable", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_spotName(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("get_spotName", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn remove_hasBeenRemovedEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::System::Action_1<
                quest_hook::libil2cpp::Gc<
                    crate::GlobalNamespace::IMultiplayerSpectatingSpot,
                >,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_hasBeenRemovedEvent", (value))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "MultiplayerEnvironmentSpectatingSpot")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::MultiplayerEnvironmentSpectatingSpot {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "MultiplayerEnvironmentSpectatingSpot")]
impl AsRef<crate::GlobalNamespace::IMultiplayerSpectatingSpot>
for crate::GlobalNamespace::MultiplayerEnvironmentSpectatingSpot {
    fn as_ref(&self) -> &crate::GlobalNamespace::IMultiplayerSpectatingSpot {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "MultiplayerEnvironmentSpectatingSpot")]
impl AsMut<crate::GlobalNamespace::IMultiplayerSpectatingSpot>
for crate::GlobalNamespace::MultiplayerEnvironmentSpectatingSpot {
    fn as_mut(&mut self) -> &mut crate::GlobalNamespace::IMultiplayerSpectatingSpot {
        unsafe { std::mem::transmute(self) }
    }
}
