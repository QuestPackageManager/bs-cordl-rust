#[cfg(feature = "BTSCharacterDataModel")]
#[repr(C)]
#[derive(Debug)]
pub struct BTSCharacterDataModel {
    __cordl_parent: crate::UnityEngine::MonoBehaviour,
    pub _prefabsWithIds: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<
            quest_hook::libil2cpp::Gc<
                crate::GlobalNamespace::BTSCharacterDataModel_PrefabWithId,
            >,
        >,
    >,
    pub _animationClipsWithIds: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<
            quest_hook::libil2cpp::Gc<
                crate::GlobalNamespace::BTSCharacterDataModel_AnimationClipWithId,
            >,
        >,
    >,
}
#[cfg(feature = "BTSCharacterDataModel")]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::BTSCharacterDataModel {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "BTSCharacterDataModel";
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.class().is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_value_argument(_: &quest_hook::libil2cpp::Il2CppType) -> bool {
        false
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
    fn matches_value_parameter(_: &quest_hook::libil2cpp::Il2CppType) -> bool {
        false
    }
}
#[cfg(feature = "BTSCharacterDataModel")]
impl std::ops::Deref for crate::GlobalNamespace::BTSCharacterDataModel {
    type Target = crate::UnityEngine::MonoBehaviour;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BTSCharacterDataModel")]
impl std::ops::DerefMut for crate::GlobalNamespace::BTSCharacterDataModel {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BTSCharacterDataModel")]
impl crate::GlobalNamespace::BTSCharacterDataModel {
    #[cfg(feature = "BTSCharacterDataModel+AnimationClipWithId")]
    pub type AnimationClipWithId = crate::GlobalNamespace::BTSCharacterDataModel_AnimationClipWithId;
    #[cfg(feature = "BTSCharacterDataModel+PrefabWithId")]
    pub type PrefabWithId = crate::GlobalNamespace::BTSCharacterDataModel_PrefabWithId;
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
    pub fn get_animationClipsWithIds(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<
                    crate::GlobalNamespace::BTSCharacterDataModel_AnimationClipWithId,
                >,
            >,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<
                    crate::GlobalNamespace::BTSCharacterDataModel_AnimationClipWithId,
                >,
            >,
        > = __cordl_object.invoke("get_animationClipsWithIds", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_prefabsWithIds(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<
                    crate::GlobalNamespace::BTSCharacterDataModel_PrefabWithId,
                >,
            >,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<
                    crate::GlobalNamespace::BTSCharacterDataModel_PrefabWithId,
                >,
            >,
        > = __cordl_object.invoke("get_prefabsWithIds", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "BTSCharacterDataModel")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::BTSCharacterDataModel {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "BTSCharacterDataModel+AnimationClipWithId")]
#[repr(C)]
#[derive(Debug)]
pub struct BTSCharacterDataModel_AnimationClipWithId {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _id: i32,
    pub _animationClipAssetReference: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::AddressableAssets::AssetReference,
    >,
}
#[cfg(feature = "BTSCharacterDataModel+AnimationClipWithId")]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::BTSCharacterDataModel_AnimationClipWithId {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "BTSCharacterDataModel/AnimationClipWithId";
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.class().is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_value_argument(_: &quest_hook::libil2cpp::Il2CppType) -> bool {
        false
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
    fn matches_value_parameter(_: &quest_hook::libil2cpp::Il2CppType) -> bool {
        false
    }
}
#[cfg(feature = "BTSCharacterDataModel+AnimationClipWithId")]
impl std::ops::Deref
for crate::GlobalNamespace::BTSCharacterDataModel_AnimationClipWithId {
    type Target = quest_hook::libil2cpp::Il2CppObject;
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
    pub fn get_animationClipAssetReference(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::AddressableAssets::AssetReference>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::AddressableAssets::AssetReference,
        > = __cordl_object.invoke("get_animationClipAssetReference", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_id(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_id", ())?;
        Ok(__cordl_ret.into())
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
#[cfg(feature = "BTSCharacterDataModel+PrefabWithId")]
#[repr(C)]
#[derive(Debug)]
pub struct BTSCharacterDataModel_PrefabWithId {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _id: i32,
    pub _prefabAssetReference: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::AddressableAssets::AssetReference,
    >,
}
#[cfg(feature = "BTSCharacterDataModel+PrefabWithId")]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::BTSCharacterDataModel_PrefabWithId {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "BTSCharacterDataModel/PrefabWithId";
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.class().is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_value_argument(_: &quest_hook::libil2cpp::Il2CppType) -> bool {
        false
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
    fn matches_value_parameter(_: &quest_hook::libil2cpp::Il2CppType) -> bool {
        false
    }
}
#[cfg(feature = "BTSCharacterDataModel+PrefabWithId")]
impl std::ops::Deref for crate::GlobalNamespace::BTSCharacterDataModel_PrefabWithId {
    type Target = quest_hook::libil2cpp::Il2CppObject;
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
    pub fn get_id(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_id", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_prefabAssetReference(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::AddressableAssets::AssetReference>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::AddressableAssets::AssetReference,
        > = __cordl_object.invoke("get_prefabAssetReference", ())?;
        Ok(__cordl_ret.into())
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
