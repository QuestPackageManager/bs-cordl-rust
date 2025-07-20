#[cfg(feature = "DeterministicHitChanceScoreCalculator")]
#[repr(C)]
#[derive(Debug)]
pub struct DeterministicHitChanceScoreCalculator {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _hitChance: f32,
    pub _chanceAggregated: f32,
}
#[cfg(feature = "DeterministicHitChanceScoreCalculator")]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::DeterministicHitChanceScoreCalculator {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "DeterministicHitChanceScoreCalculator";
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
#[cfg(feature = "DeterministicHitChanceScoreCalculator")]
impl std::ops::Deref for crate::GlobalNamespace::DeterministicHitChanceScoreCalculator {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "DeterministicHitChanceScoreCalculator")]
impl std::ops::DerefMut
for crate::GlobalNamespace::DeterministicHitChanceScoreCalculator {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "DeterministicHitChanceScoreCalculator")]
impl crate::GlobalNamespace::DeterministicHitChanceScoreCalculator {
    pub const kScorePerHit: i32 = 105i32;
    pub fn GetScoreForNote(
        &mut self,
        noteData: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::MockNoteData>,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<
                            crate::GlobalNamespace::MockNoteData,
                        >),
                        i32,
                        1usize,
                    >("GetScoreForNote")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "GetScoreForNote", 1usize
                        )
                    })
            });
        let __cordl_ret: i32 = unsafe { method.invoke_unchecked(self, (noteData))? };
        Ok(__cordl_ret.into())
    }
    pub fn New(
        hitChance: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (hitChance))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        hitChance: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<(f32), quest_hook::libil2cpp::Void, 1usize>(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), ".ctor", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (hitChance))?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "DeterministicHitChanceScoreCalculator")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::DeterministicHitChanceScoreCalculator {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "DeterministicHitChanceScoreCalculator")]
impl AsRef<crate::GlobalNamespace::IMockPlayerScoreCalculator>
for crate::GlobalNamespace::DeterministicHitChanceScoreCalculator {
    fn as_ref(&self) -> &crate::GlobalNamespace::IMockPlayerScoreCalculator {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "DeterministicHitChanceScoreCalculator")]
impl AsMut<crate::GlobalNamespace::IMockPlayerScoreCalculator>
for crate::GlobalNamespace::DeterministicHitChanceScoreCalculator {
    fn as_mut(&mut self) -> &mut crate::GlobalNamespace::IMockPlayerScoreCalculator {
        unsafe { std::mem::transmute(self) }
    }
}
