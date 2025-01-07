#[cfg(feature = "GameObjectKeywordSwitchEffect")]
#[repr(C)]
#[derive(Debug)]
pub struct GameObjectKeywordSwitchEffect {
    __cordl_parent: crate::UnityEngine::MonoBehaviour,
    pub _defaultGameObject: quest_hook::libil2cpp::Gc<crate::UnityEngine::GameObject>,
    pub _gameObjectKeywordItems: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<
            quest_hook::libil2cpp::Gc<
                crate::GlobalNamespace::GameObjectKeywordSwitchEffect_GameObjectKeywordItem,
            >,
        >,
    >,
}
#[cfg(feature = "GameObjectKeywordSwitchEffect")]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::GameObjectKeywordSwitchEffect {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "GameObjectKeywordSwitchEffect";
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
#[cfg(feature = "GameObjectKeywordSwitchEffect")]
impl std::ops::Deref for crate::GlobalNamespace::GameObjectKeywordSwitchEffect {
    type Target = crate::UnityEngine::MonoBehaviour;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "GameObjectKeywordSwitchEffect")]
impl std::ops::DerefMut for crate::GlobalNamespace::GameObjectKeywordSwitchEffect {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "GameObjectKeywordSwitchEffect")]
impl crate::GlobalNamespace::GameObjectKeywordSwitchEffect {
    #[cfg(feature = "GameObjectKeywordSwitchEffect+GameObjectKeywordItem")]
    pub type GameObjectKeywordItem = crate::GlobalNamespace::GameObjectKeywordSwitchEffect_GameObjectKeywordItem;
    pub fn Initialize(
        &mut self,
        beatmapData: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::IReadonlyBeatmapData,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Initialize", (beatmapData))?;
        Ok(__cordl_ret.into())
    }
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
#[cfg(feature = "GameObjectKeywordSwitchEffect")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::GameObjectKeywordSwitchEffect {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "GameObjectKeywordSwitchEffect+GameObjectKeywordItem")]
#[repr(C)]
#[derive(Debug)]
pub struct GameObjectKeywordSwitchEffect_GameObjectKeywordItem {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub gameObject: quest_hook::libil2cpp::Gc<crate::UnityEngine::GameObject>,
    pub keyword: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
}
#[cfg(feature = "GameObjectKeywordSwitchEffect+GameObjectKeywordItem")]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::GameObjectKeywordSwitchEffect_GameObjectKeywordItem {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "GameObjectKeywordItem";
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
#[cfg(feature = "GameObjectKeywordSwitchEffect+GameObjectKeywordItem")]
impl std::ops::Deref
for crate::GlobalNamespace::GameObjectKeywordSwitchEffect_GameObjectKeywordItem {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "GameObjectKeywordSwitchEffect+GameObjectKeywordItem")]
impl std::ops::DerefMut
for crate::GlobalNamespace::GameObjectKeywordSwitchEffect_GameObjectKeywordItem {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "GameObjectKeywordSwitchEffect+GameObjectKeywordItem")]
impl crate::GlobalNamespace::GameObjectKeywordSwitchEffect_GameObjectKeywordItem {
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
#[cfg(feature = "GameObjectKeywordSwitchEffect+GameObjectKeywordItem")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::GameObjectKeywordSwitchEffect_GameObjectKeywordItem {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
