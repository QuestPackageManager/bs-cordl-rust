#[cfg(feature = "MultiplayerScoreRingItem")]
#[repr(C)]
#[derive(Debug)]
pub struct MultiplayerScoreRingItem {
    __cordl_parent: crate::GlobalNamespace::MultiplayerScoreItem,
}
#[cfg(feature = "MultiplayerScoreRingItem")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::MultiplayerScoreRingItem => ""
    ."MultiplayerScoreRingItem"
);
#[cfg(feature = "MultiplayerScoreRingItem")]
impl std::ops::Deref for crate::GlobalNamespace::MultiplayerScoreRingItem {
    type Target = crate::GlobalNamespace::MultiplayerScoreItem;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "MultiplayerScoreRingItem")]
impl std::ops::DerefMut for crate::GlobalNamespace::MultiplayerScoreRingItem {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "MultiplayerScoreRingItem")]
impl crate::GlobalNamespace::MultiplayerScoreRingItem {
    #[cfg(feature = "MultiplayerScoreRingItem+Pool")]
    pub type Pool = crate::GlobalNamespace::MultiplayerScoreRingItem_Pool;
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
}
#[cfg(feature = "MultiplayerScoreRingItem")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::MultiplayerScoreRingItem {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "MultiplayerScoreRingItem+Pool")]
#[repr(C)]
#[derive(Debug)]
pub struct MultiplayerScoreRingItem_Pool {
    __cordl_parent: crate::Zenject::MonoMemoryPool_1<
        *mut crate::GlobalNamespace::MultiplayerScoreRingItem,
    >,
}
#[cfg(feature = "MultiplayerScoreRingItem+Pool")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::MultiplayerScoreRingItem_Pool
    => ""."MultiplayerScoreRingItem/Pool"
);
#[cfg(feature = "MultiplayerScoreRingItem+Pool")]
impl std::ops::Deref for crate::GlobalNamespace::MultiplayerScoreRingItem_Pool {
    type Target = crate::Zenject::MonoMemoryPool_1<
        *mut crate::GlobalNamespace::MultiplayerScoreRingItem,
    >;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "MultiplayerScoreRingItem+Pool")]
impl std::ops::DerefMut for crate::GlobalNamespace::MultiplayerScoreRingItem_Pool {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "MultiplayerScoreRingItem+Pool")]
impl crate::GlobalNamespace::MultiplayerScoreRingItem_Pool {
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
}
#[cfg(feature = "MultiplayerScoreRingItem+Pool")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::MultiplayerScoreRingItem_Pool {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
