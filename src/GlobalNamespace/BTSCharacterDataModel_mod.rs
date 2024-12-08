#[cfg(feature = "BTSCharacterDataModel+AnimationClipWithId")]
#[repr(C)]
#[derive(Debug)]
pub struct BTSCharacterDataModel_AnimationClipWithId {
    __cordl_parent: crate::System::Object,
    pub _id: i32,
    pub _animationClipAssetReference: *mut crate::UnityEngine::AddressableAssets::AssetReference,
}
#[cfg(feature = "BTSCharacterDataModel+AnimationClipWithId")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::BTSCharacterDataModel_AnimationClipWithId => ""
    ."BTSCharacterDataModel/AnimationClipWithId"
);
#[cfg(feature = "BTSCharacterDataModel+AnimationClipWithId")]
impl std::ops::Deref
for crate::GlobalNamespace::BTSCharacterDataModel_AnimationClipWithId {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BTSCharacterDataModel+AnimationClipWithId")]
impl std::ops::DerefMut
for crate::GlobalNamespace::BTSCharacterDataModel_AnimationClipWithId {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BTSCharacterDataModel+AnimationClipWithId")]
impl crate::GlobalNamespace::BTSCharacterDataModel_AnimationClipWithId {
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_animationClipAssetReference(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::UnityEngine::AddressableAssets::AssetReference,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::AddressableAssets::AssetReference = __cordl_object
            .invoke("get_animationClipAssetReference", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_id(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_id", ())?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "BTSCharacterDataModel+AnimationClipWithId")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::BTSCharacterDataModel_AnimationClipWithId {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "BTSCharacterDataModel")]
#[repr(C)]
#[derive(Debug)]
pub struct BTSCharacterDataModel {
    __cordl_parent: crate::UnityEngine::MonoBehaviour,
    pub _prefabsWithIds: *mut quest_hook::libil2cpp::Il2CppArray<
        *mut crate::GlobalNamespace::BTSCharacterDataModel_PrefabWithId,
    >,
    pub _animationClipsWithIds: *mut quest_hook::libil2cpp::Il2CppArray<
        *mut crate::GlobalNamespace::BTSCharacterDataModel_AnimationClipWithId,
    >,
}
#[cfg(feature = "BTSCharacterDataModel")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for BTSCharacterDataModel => ""."BTSCharacterDataModel"
);
#[cfg(feature = "BTSCharacterDataModel")]
impl std::ops::Deref for BTSCharacterDataModel {
    type Target = crate::UnityEngine::MonoBehaviour;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BTSCharacterDataModel")]
impl std::ops::DerefMut for BTSCharacterDataModel {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BTSCharacterDataModel")]
impl BTSCharacterDataModel {
    #[cfg(feature = "BTSCharacterDataModel+AnimationClipWithId")]
    pub type AnimationClipWithId = crate::GlobalNamespace::BTSCharacterDataModel_AnimationClipWithId;
    #[cfg(feature = "BTSCharacterDataModel+PrefabWithId")]
    pub type PrefabWithId = crate::GlobalNamespace::BTSCharacterDataModel_PrefabWithId;
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_animationClipsWithIds(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut quest_hook::libil2cpp::Il2CppArray<
            *mut crate::GlobalNamespace::BTSCharacterDataModel_AnimationClipWithId,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppArray<
            *mut crate::GlobalNamespace::BTSCharacterDataModel_AnimationClipWithId,
        > = __cordl_object.invoke("get_animationClipsWithIds", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_prefabsWithIds(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut quest_hook::libil2cpp::Il2CppArray<
            *mut crate::GlobalNamespace::BTSCharacterDataModel_PrefabWithId,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppArray<
            *mut crate::GlobalNamespace::BTSCharacterDataModel_PrefabWithId,
        > = __cordl_object.invoke("get_prefabsWithIds", ())?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "BTSCharacterDataModel")]
impl quest_hook::libil2cpp::ObjectType for BTSCharacterDataModel {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "BTSCharacterDataModel+PrefabWithId")]
#[repr(C)]
#[derive(Debug)]
pub struct BTSCharacterDataModel_PrefabWithId {
    __cordl_parent: crate::System::Object,
    pub _id: i32,
    pub _prefabAssetReference: *mut crate::UnityEngine::AddressableAssets::AssetReference,
}
#[cfg(feature = "BTSCharacterDataModel+PrefabWithId")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::BTSCharacterDataModel_PrefabWithId => ""
    ."BTSCharacterDataModel/PrefabWithId"
);
#[cfg(feature = "BTSCharacterDataModel+PrefabWithId")]
impl std::ops::Deref for crate::GlobalNamespace::BTSCharacterDataModel_PrefabWithId {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BTSCharacterDataModel+PrefabWithId")]
impl std::ops::DerefMut for crate::GlobalNamespace::BTSCharacterDataModel_PrefabWithId {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BTSCharacterDataModel+PrefabWithId")]
impl crate::GlobalNamespace::BTSCharacterDataModel_PrefabWithId {
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_id(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_id", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_prefabAssetReference(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::UnityEngine::AddressableAssets::AssetReference,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::AddressableAssets::AssetReference = __cordl_object
            .invoke("get_prefabAssetReference", ())?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "BTSCharacterDataModel+PrefabWithId")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::BTSCharacterDataModel_PrefabWithId {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
