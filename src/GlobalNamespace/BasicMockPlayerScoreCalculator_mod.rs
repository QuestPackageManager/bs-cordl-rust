#[cfg(feature = "BasicMockPlayerScoreCalculator")]
#[repr(C)]
#[derive(Debug)]
pub struct BasicMockPlayerScoreCalculator {
    __cordl_parent: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    pub _hitFrequency: f32,
    pub _minScore: i32,
    pub _maxScore: i32,
    pub _random: quest_hook::libil2cpp::Gc<crate::System::Random>,
}
#[cfg(feature = "BasicMockPlayerScoreCalculator")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::BasicMockPlayerScoreCalculator
    => ""."BasicMockPlayerScoreCalculator"
);
#[cfg(feature = "BasicMockPlayerScoreCalculator")]
impl std::ops::Deref for crate::GlobalNamespace::BasicMockPlayerScoreCalculator {
    type Target = quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BasicMockPlayerScoreCalculator")]
impl std::ops::DerefMut for crate::GlobalNamespace::BasicMockPlayerScoreCalculator {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BasicMockPlayerScoreCalculator")]
impl crate::GlobalNamespace::BasicMockPlayerScoreCalculator {
    pub fn GetScoreForNote(
        &mut self,
        noteData: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::MockNoteData>,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("GetScoreForNote", (noteData))?;
        Ok(__cordl_ret.into())
    }
    pub fn New(
        hitFrequency: f32,
        minScore: i32,
        maxScore: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (hitFrequency, minScore, maxScore))?;
        Ok(__cordl_object.into())
    }
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
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "BasicMockPlayerScoreCalculator")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::BasicMockPlayerScoreCalculator {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "BasicMockPlayerScoreCalculator")]
impl AsRef<quest_hook::libil2cpp::Gc<crate::GlobalNamespace::IMockPlayerScoreCalculator>>
for crate::GlobalNamespace::BasicMockPlayerScoreCalculator {
    fn as_ref(
        &self,
    ) -> &quest_hook::libil2cpp::Gc<crate::GlobalNamespace::IMockPlayerScoreCalculator> {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "BasicMockPlayerScoreCalculator")]
impl AsMut<quest_hook::libil2cpp::Gc<crate::GlobalNamespace::IMockPlayerScoreCalculator>>
for crate::GlobalNamespace::BasicMockPlayerScoreCalculator {
    fn as_mut(
        &mut self,
    ) -> &mut quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::IMockPlayerScoreCalculator,
    > {
        unsafe { std::mem::transmute(self) }
    }
}
