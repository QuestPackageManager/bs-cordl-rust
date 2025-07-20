#[cfg(feature = "Ice+NoteCutFloorLightStreakTileEffectSpawnerLightWithId")]
#[repr(C)]
#[derive(Debug)]
pub struct NoteCutFloorLightStreakTileEffectSpawnerLightWithId {
    __cordl_parent: crate::GlobalNamespace::LightWithIdMonoBehaviour,
    pub _noteCutFloorLightStreakTileEffectSpawner: quest_hook::libil2cpp::Gc<
        crate::Ice::NoteCutFloorLightStreakTileEffectSpawner,
    >,
}
#[cfg(feature = "Ice+NoteCutFloorLightStreakTileEffectSpawnerLightWithId")]
unsafe impl quest_hook::libil2cpp::Type
for crate::Ice::NoteCutFloorLightStreakTileEffectSpawnerLightWithId {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "Ice";
    const CLASS_NAME: &'static str = "NoteCutFloorLightStreakTileEffectSpawnerLightWithId";
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
#[cfg(feature = "Ice+NoteCutFloorLightStreakTileEffectSpawnerLightWithId")]
impl std::ops::Deref
for crate::Ice::NoteCutFloorLightStreakTileEffectSpawnerLightWithId {
    type Target = crate::GlobalNamespace::LightWithIdMonoBehaviour;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Ice+NoteCutFloorLightStreakTileEffectSpawnerLightWithId")]
impl std::ops::DerefMut
for crate::Ice::NoteCutFloorLightStreakTileEffectSpawnerLightWithId {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Ice+NoteCutFloorLightStreakTileEffectSpawnerLightWithId")]
impl crate::Ice::NoteCutFloorLightStreakTileEffectSpawnerLightWithId {
    pub fn ColorWasSet(
        &mut self,
        color: crate::UnityEngine::Color,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::Ice::NoteCutFloorLightStreakTileEffectSpawnerLightWithId as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (crate::UnityEngine::Color),
                quest_hook::libil2cpp::Void,
                1usize,
            >("ColorWasSet")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::Ice::NoteCutFloorLightStreakTileEffectSpawnerLightWithId as
                    quest_hook::libil2cpp::Type > ::class(), "ColorWasSet", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (color))?
        };
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
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::Ice::NoteCutFloorLightStreakTileEffectSpawnerLightWithId as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), quest_hook::libil2cpp::Void, 0usize>(".ctor")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::Ice::NoteCutFloorLightStreakTileEffectSpawnerLightWithId as
                    quest_hook::libil2cpp::Type > ::class(), ".ctor", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Ice+NoteCutFloorLightStreakTileEffectSpawnerLightWithId")]
impl quest_hook::libil2cpp::ObjectType
for crate::Ice::NoteCutFloorLightStreakTileEffectSpawnerLightWithId {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
