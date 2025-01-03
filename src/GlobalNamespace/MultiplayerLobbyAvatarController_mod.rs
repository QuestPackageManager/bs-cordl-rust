#[cfg(feature = "MultiplayerLobbyAvatarController")]
#[repr(C)]
#[derive(Debug)]
pub struct MultiplayerLobbyAvatarController {
    __cordl_parent: crate::UnityEngine::MonoBehaviour,
    pub _spawnEffect: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::Playables::PlayableDirector,
    >,
    pub _despawnVFXController: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::VFXController,
    >,
    pub _spawnAvatarDelay: f32,
    pub _despawnAvatarDelay: f32,
    pub _destroyAvatarDelay: f32,
    pub _visualObjects: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<*mut crate::UnityEngine::GameObject>,
    >,
}
#[cfg(feature = "MultiplayerLobbyAvatarController")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::MultiplayerLobbyAvatarController => ""
    ."MultiplayerLobbyAvatarController"
);
#[cfg(feature = "MultiplayerLobbyAvatarController")]
impl std::ops::Deref for crate::GlobalNamespace::MultiplayerLobbyAvatarController {
    type Target = crate::UnityEngine::MonoBehaviour;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "MultiplayerLobbyAvatarController")]
impl std::ops::DerefMut for crate::GlobalNamespace::MultiplayerLobbyAvatarController {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "MultiplayerLobbyAvatarController")]
impl crate::GlobalNamespace::MultiplayerLobbyAvatarController {
    #[cfg(feature = "MultiplayerLobbyAvatarController+Factory")]
    pub type Factory = crate::GlobalNamespace::MultiplayerLobbyAvatarController_Factory;
    pub fn ActivateVisualObjects(
        &mut self,
        on: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ActivateVisualObjects", (on))?;
        Ok(__cordl_ret.into())
    }
    pub fn DespawnAnimationCoroutine(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Collections::IEnumerator>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::IEnumerator,
        > = __cordl_object.invoke("DespawnAnimationCoroutine", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn DestroySelf(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("DestroySelf", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn ShowDespawnAnimationAndDestroy(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Collections::IEnumerator>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::IEnumerator,
        > = __cordl_object.invoke("ShowDespawnAnimationAndDestroy", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn ShowSpawnAnimation(
        &mut self,
        position: crate::UnityEngine::Vector3,
        rotation: crate::UnityEngine::Quaternion,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ShowSpawnAnimation", (position, rotation))?;
        Ok(__cordl_ret.into())
    }
    pub fn SpawnAnimationCoroutine(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Collections::IEnumerator>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::IEnumerator,
        > = __cordl_object.invoke("SpawnAnimationCoroutine", ())?;
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
}
#[cfg(feature = "MultiplayerLobbyAvatarController")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::MultiplayerLobbyAvatarController {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "MultiplayerLobbyAvatarController+Factory")]
#[repr(C)]
#[derive(Debug)]
pub struct MultiplayerLobbyAvatarController_Factory {
    __cordl_parent: crate::Zenject::PlaceholderFactory_2<
        *mut crate::GlobalNamespace::IConnectedPlayer,
        *mut crate::GlobalNamespace::MultiplayerLobbyAvatarController,
    >,
}
#[cfg(feature = "MultiplayerLobbyAvatarController+Factory")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::MultiplayerLobbyAvatarController_Factory => ""
    ."MultiplayerLobbyAvatarController/Factory"
);
#[cfg(feature = "MultiplayerLobbyAvatarController+Factory")]
impl std::ops::Deref
for crate::GlobalNamespace::MultiplayerLobbyAvatarController_Factory {
    type Target = crate::Zenject::PlaceholderFactory_2<
        *mut crate::GlobalNamespace::IConnectedPlayer,
        *mut crate::GlobalNamespace::MultiplayerLobbyAvatarController,
    >;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "MultiplayerLobbyAvatarController+Factory")]
impl std::ops::DerefMut
for crate::GlobalNamespace::MultiplayerLobbyAvatarController_Factory {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "MultiplayerLobbyAvatarController+Factory")]
impl crate::GlobalNamespace::MultiplayerLobbyAvatarController_Factory {
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
}
#[cfg(feature = "MultiplayerLobbyAvatarController+Factory")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::MultiplayerLobbyAvatarController_Factory {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
