#[cfg(feature = "BasicMockPlayerScoreCalculator")]
#[repr(C)]
#[derive(Debug)]
pub struct BasicMockPlayerScoreCalculator {
    __cordl_parent: crate::System::Object,
    pub _hitFrequency: f32,
    pub _minScore: i32,
    pub _maxScore: i32,
    pub _random: *mut crate::System::Random,
}
#[cfg(feature = "BasicMockPlayerScoreCalculator")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for BasicMockPlayerScoreCalculator => ""
    ."BasicMockPlayerScoreCalculator"
);
#[cfg(feature = "BasicMockPlayerScoreCalculator")]
impl std::ops::Deref for BasicMockPlayerScoreCalculator {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BasicMockPlayerScoreCalculator")]
impl std::ops::DerefMut for BasicMockPlayerScoreCalculator {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BasicMockPlayerScoreCalculator")]
impl BasicMockPlayerScoreCalculator {
    pub fn _ctor(
        &mut self,
        hitFrequency: f32,
        minScore: i32,
        maxScore: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (hitFrequency, minScore, maxScore))?;
        Ok(__cordl_ret)
    }
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
    pub fn New(
        hitFrequency: f32,
        minScore: i32,
        maxScore: i32,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (hitFrequency, minScore, maxScore))?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "BasicMockPlayerScoreCalculator")]
impl quest_hook::libil2cpp::ObjectType for BasicMockPlayerScoreCalculator {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
