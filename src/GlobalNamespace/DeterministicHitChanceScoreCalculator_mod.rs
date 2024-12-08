#[cfg(feature = "DeterministicHitChanceScoreCalculator")]
#[repr(C)]
#[derive(Debug)]
pub struct DeterministicHitChanceScoreCalculator {
    __cordl_parent: crate::System::Object,
    pub _hitChance: f32,
    pub _chanceAggregated: f32,
}
#[cfg(feature = "DeterministicHitChanceScoreCalculator")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for DeterministicHitChanceScoreCalculator => ""
    ."DeterministicHitChanceScoreCalculator"
);
#[cfg(feature = "DeterministicHitChanceScoreCalculator")]
impl std::ops::Deref for DeterministicHitChanceScoreCalculator {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "DeterministicHitChanceScoreCalculator")]
impl std::ops::DerefMut for DeterministicHitChanceScoreCalculator {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "DeterministicHitChanceScoreCalculator")]
impl DeterministicHitChanceScoreCalculator {
    pub const kScorePerHit: i32 = 105i32;
    pub fn GetScoreForNote(
        &mut self,
        noteData: *mut MockNoteData,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("GetScoreForNote", (noteData))?;
        Ok(__cordl_ret)
    }
    pub fn New(hitChance: f32) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (hitChance))?;
        Ok(__cordl_object)
    }
    pub fn _ctor(
        &mut self,
        hitChance: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (hitChance))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "DeterministicHitChanceScoreCalculator")]
impl quest_hook::libil2cpp::ObjectType for DeterministicHitChanceScoreCalculator {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
