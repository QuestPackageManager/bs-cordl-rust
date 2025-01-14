#[cfg(feature = "IMockPlayerScoreCalculator")]
#[repr(C)]
#[derive(Debug)]
pub struct IMockPlayerScoreCalculator {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "IMockPlayerScoreCalculator")]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::IMockPlayerScoreCalculator {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "IMockPlayerScoreCalculator";
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
#[cfg(feature = "IMockPlayerScoreCalculator")]
impl std::ops::Deref for crate::GlobalNamespace::IMockPlayerScoreCalculator {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "IMockPlayerScoreCalculator")]
impl std::ops::DerefMut for crate::GlobalNamespace::IMockPlayerScoreCalculator {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "IMockPlayerScoreCalculator")]
impl crate::GlobalNamespace::IMockPlayerScoreCalculator {
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
    pub fn from_object_mut(
        object_param: *mut quest_hook::libil2cpp::Il2CppObject,
    ) -> *mut Self {
        unsafe { (object_param as *mut Self) }
    }
}
#[cfg(feature = "IMockPlayerScoreCalculator")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::IMockPlayerScoreCalculator {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
