#[cfg(feature = "BasicMockPlayerScoreCalculator")]
#[repr(C)]
#[derive(Debug)]
pub struct BasicMockPlayerScoreCalculator {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _hitFrequency: f32,
    pub _minScore: i32,
    pub _maxScore: i32,
    pub _random: quest_hook::libil2cpp::Gc<crate::System::Random>,
}
#[cfg(feature = "BasicMockPlayerScoreCalculator")]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::BasicMockPlayerScoreCalculator {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "BasicMockPlayerScoreCalculator";
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
#[cfg(feature = "BasicMockPlayerScoreCalculator")]
impl std::ops::Deref for crate::GlobalNamespace::BasicMockPlayerScoreCalculator {
    type Target = quest_hook::libil2cpp::Il2CppObject;
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
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<crate::GlobalNamespace::MockNoteData>),
                i32,
                1usize,
            >("GetScoreForNote")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "GetScoreForNote", 1usize
                )
            });
        let __cordl_ret: i32 = unsafe { method.invoke_unchecked(self, (noteData)) };
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
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<(f32, i32, i32), quest_hook::libil2cpp::Void, 3usize>(".ctor")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, ".ctor", 3usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (hitFrequency, minScore, maxScore))
        };
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
impl AsRef<crate::GlobalNamespace::IMockPlayerScoreCalculator>
for crate::GlobalNamespace::BasicMockPlayerScoreCalculator {
    fn as_ref(&self) -> &crate::GlobalNamespace::IMockPlayerScoreCalculator {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "BasicMockPlayerScoreCalculator")]
impl AsMut<crate::GlobalNamespace::IMockPlayerScoreCalculator>
for crate::GlobalNamespace::BasicMockPlayerScoreCalculator {
    fn as_mut(&mut self) -> &mut crate::GlobalNamespace::IMockPlayerScoreCalculator {
        unsafe { std::mem::transmute(self) }
    }
}
