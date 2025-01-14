#[cfg(feature = "SelectLevelDestination")]
#[repr(C)]
#[derive(Debug)]
pub struct SelectLevelDestination {
    __cordl_parent: crate::GlobalNamespace::MenuDestination,
    pub beatmapLevelPack: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::BeatmapLevelPack,
    >,
    pub beatmapLevel: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::BeatmapLevel>,
    pub beatmapDifficulty: crate::GlobalNamespace::BeatmapDifficulty,
    pub beatmapCharacteristic: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::BeatmapCharacteristicSO,
    >,
}
#[cfg(feature = "SelectLevelDestination")]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::SelectLevelDestination {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "SelectLevelDestination";
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
#[cfg(feature = "SelectLevelDestination")]
impl std::ops::Deref for crate::GlobalNamespace::SelectLevelDestination {
    type Target = crate::GlobalNamespace::MenuDestination;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "SelectLevelDestination")]
impl std::ops::DerefMut for crate::GlobalNamespace::SelectLevelDestination {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "SelectLevelDestination")]
impl crate::GlobalNamespace::SelectLevelDestination {
    pub fn New(
        beatmapLevelPack: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::BeatmapLevelPack,
        >,
        beatmapLevel: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::BeatmapLevel>,
        beatmapDifficulty: crate::GlobalNamespace::BeatmapDifficulty,
        beatmapCharacteristic: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::BeatmapCharacteristicSO,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(
                ".ctor",
                (
                    beatmapLevelPack,
                    beatmapLevel,
                    beatmapDifficulty,
                    beatmapCharacteristic,
                ),
            )?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        beatmapLevelPack: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::BeatmapLevelPack,
        >,
        beatmapLevel: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::BeatmapLevel>,
        beatmapDifficulty: crate::GlobalNamespace::BeatmapDifficulty,
        beatmapCharacteristic: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::BeatmapCharacteristicSO,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<crate::GlobalNamespace::BeatmapLevelPack>,
                    quest_hook::libil2cpp::Gc<crate::GlobalNamespace::BeatmapLevel>,
                    crate::GlobalNamespace::BeatmapDifficulty,
                    quest_hook::libil2cpp::Gc<
                        crate::GlobalNamespace::BeatmapCharacteristicSO,
                    >,
                ),
                quest_hook::libil2cpp::Void,
                4usize,
            >(".ctor")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, ".ctor", 4usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method
                .invoke_unchecked(
                    self,
                    (
                        beatmapLevelPack,
                        beatmapLevel,
                        beatmapDifficulty,
                        beatmapCharacteristic,
                    ),
                )
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "SelectLevelDestination")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::SelectLevelDestination {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
