#[cfg(feature = "GameplayModifierMaskExtensions")]
#[repr(C)]
#[derive(Debug)]
pub struct GameplayModifierMaskExtensions {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "GameplayModifierMaskExtensions")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::GameplayModifierMaskExtensions
    => ""."GameplayModifierMaskExtensions"
);
#[cfg(feature = "GameplayModifierMaskExtensions")]
impl std::ops::Deref for crate::GlobalNamespace::GameplayModifierMaskExtensions {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "GameplayModifierMaskExtensions")]
impl std::ops::DerefMut for crate::GlobalNamespace::GameplayModifierMaskExtensions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "GameplayModifierMaskExtensions")]
impl crate::GlobalNamespace::GameplayModifierMaskExtensions {
    pub fn Contains(
        mask: crate::GlobalNamespace::GameplayModifierMask,
        other: crate::GlobalNamespace::GameplayModifierMask,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Contains", (mask, other))?;
        Ok(__cordl_ret.into())
    }
    pub fn DifferenceFrom(
        mask: crate::GlobalNamespace::GameplayModifierMask,
        other: crate::GlobalNamespace::GameplayModifierMask,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("DifferenceFrom", (mask, other))?;
        Ok(__cordl_ret.into())
    }
    pub fn ToGameplayModifierMask(
        gameplayModifiers: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::GameplayModifiers,
        >,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::GameplayModifierMask> {
        let __cordl_ret: crate::GlobalNamespace::GameplayModifierMask = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ToGameplayModifierMask", (gameplayModifiers))?;
        Ok(__cordl_ret.into())
    }
    pub fn ToHexString(
        mask: crate::GlobalNamespace::GameplayModifierMask,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ToHexString", (mask))?;
        Ok(__cordl_ret.into())
    }
    pub fn ToMask(
        gameplayModifiers: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::GameplayModifiers,
        >,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::GameplayModifierMask> {
        let __cordl_ret: crate::GlobalNamespace::GameplayModifierMask = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ToMask", (gameplayModifiers))?;
        Ok(__cordl_ret.into())
    }
    pub fn ToModifiers(
        gameplayModifierMask: crate::GlobalNamespace::GameplayModifierMask,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::GameplayModifiers>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::GameplayModifiers,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ToModifiers", (gameplayModifierMask))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "GameplayModifierMaskExtensions")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::GameplayModifierMaskExtensions {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
