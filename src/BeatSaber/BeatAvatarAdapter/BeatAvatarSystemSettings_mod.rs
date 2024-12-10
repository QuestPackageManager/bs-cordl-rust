#[cfg(feature = "BeatSaber+BeatAvatarAdapter+BeatAvatarSystemSettings")]
#[repr(C)]
#[derive(Debug)]
pub struct BeatAvatarSystemSettings {
    __cordl_parent: crate::GlobalNamespace::PersistentScriptableObject,
    pub _avatarGameplayPrefab: *mut crate::UnityEngine::AddressableAssets::AssetReferenceGameObject,
    pub _avatarResultsPrefab: *mut crate::UnityEngine::AddressableAssets::AssetReferenceGameObject,
    pub _avatarHologramPrefab: *mut crate::UnityEngine::AddressableAssets::AssetReferenceGameObject,
    pub _avatarEditorPrefab: *mut crate::UnityEngine::AddressableAssets::AssetReferenceGameObject,
    pub _avatarSelectionViewPrefab: *mut crate::UnityEngine::AddressableAssets::AssetReferenceGameObject,
    pub _avatarForUnityEditorPrefab: *mut crate::UnityEngine::AddressableAssets::AssetReferenceGameObject,
}
#[cfg(feature = "BeatSaber+BeatAvatarAdapter+BeatAvatarSystemSettings")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::BeatSaber::BeatAvatarAdapter::BeatAvatarSystemSettings =>
    "BeatSaber.BeatAvatarAdapter"."BeatAvatarSystemSettings"
);
#[cfg(feature = "BeatSaber+BeatAvatarAdapter+BeatAvatarSystemSettings")]
impl std::ops::Deref for crate::BeatSaber::BeatAvatarAdapter::BeatAvatarSystemSettings {
    type Target = crate::GlobalNamespace::PersistentScriptableObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BeatSaber+BeatAvatarAdapter+BeatAvatarSystemSettings")]
impl std::ops::DerefMut
for crate::BeatSaber::BeatAvatarAdapter::BeatAvatarSystemSettings {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BeatSaber+BeatAvatarAdapter+BeatAvatarSystemSettings")]
impl crate::BeatSaber::BeatAvatarAdapter::BeatAvatarSystemSettings {
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
    pub fn get_avatarEditorPrefab(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::AddressableAssets::AssetReferenceGameObject,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::AddressableAssets::AssetReferenceGameObject,
        > = __cordl_object.invoke("get_avatarEditorPrefab", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_avatarForUnityEditorPrefab(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::AddressableAssets::AssetReferenceGameObject,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::AddressableAssets::AssetReferenceGameObject,
        > = __cordl_object.invoke("get_avatarForUnityEditorPrefab", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_avatarGameplayPrefab(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::AddressableAssets::AssetReferenceGameObject,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::AddressableAssets::AssetReferenceGameObject,
        > = __cordl_object.invoke("get_avatarGameplayPrefab", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_avatarHologramPrefab(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::AddressableAssets::AssetReferenceGameObject,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::AddressableAssets::AssetReferenceGameObject,
        > = __cordl_object.invoke("get_avatarHologramPrefab", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_avatarResultsPrefab(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::AddressableAssets::AssetReferenceGameObject,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::AddressableAssets::AssetReferenceGameObject,
        > = __cordl_object.invoke("get_avatarResultsPrefab", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_avatarSelectionViewPrefab(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::AddressableAssets::AssetReferenceGameObject,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::AddressableAssets::AssetReferenceGameObject,
        > = __cordl_object.invoke("get_avatarSelectionViewPrefab", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "BeatSaber+BeatAvatarAdapter+BeatAvatarSystemSettings")]
impl quest_hook::libil2cpp::ObjectType
for crate::BeatSaber::BeatAvatarAdapter::BeatAvatarSystemSettings {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
