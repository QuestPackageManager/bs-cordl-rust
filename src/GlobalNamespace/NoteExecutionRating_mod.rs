#[cfg(feature = "NoteExecutionRating")]
#[repr(C)]
#[derive(Debug)]
pub struct NoteExecutionRating {
    __cordl_parent: crate::GlobalNamespace::BeatmapObjectExecutionRating,
    pub rating: crate::GlobalNamespace::NoteExecutionRating_Rating,
    pub cutScore: i32,
    pub beforeCutScore: i32,
    pub centerDistanceCutScore: i32,
    pub afterCutScore: i32,
    pub scoringType: crate::GlobalNamespace::NoteData_ScoringType,
}
#[cfg(feature = "NoteExecutionRating")]
unsafe impl quest_hook::libil2cpp::Type for crate::GlobalNamespace::NoteExecutionRating {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "NoteExecutionRating";
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
#[cfg(feature = "NoteExecutionRating")]
impl std::ops::Deref for crate::GlobalNamespace::NoteExecutionRating {
    type Target = crate::GlobalNamespace::BeatmapObjectExecutionRating;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "NoteExecutionRating")]
impl std::ops::DerefMut for crate::GlobalNamespace::NoteExecutionRating {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "NoteExecutionRating")]
impl crate::GlobalNamespace::NoteExecutionRating {
    #[cfg(feature = "NoteExecutionRating+Rating")]
    pub type Rating = crate::GlobalNamespace::NoteExecutionRating_Rating;
    pub fn New(
        _cordl_time: f32,
        scoringType: crate::GlobalNamespace::NoteData_ScoringType,
        rating: crate::GlobalNamespace::NoteExecutionRating_Rating,
        cutScore: i32,
        beforeCutScore: i32,
        centerDistanceCutScore: i32,
        afterCutScore: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(
                ".ctor",
                (
                    _cordl_time,
                    scoringType,
                    rating,
                    cutScore,
                    beforeCutScore,
                    centerDistanceCutScore,
                    afterCutScore,
                ),
            )?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        _cordl_time: f32,
        scoringType: crate::GlobalNamespace::NoteData_ScoringType,
        rating: crate::GlobalNamespace::NoteExecutionRating_Rating,
        cutScore: i32,
        beforeCutScore: i32,
        centerDistanceCutScore: i32,
        afterCutScore: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            f32,
                            crate::GlobalNamespace::NoteData_ScoringType,
                            crate::GlobalNamespace::NoteExecutionRating_Rating,
                            i32,
                            i32,
                            i32,
                            i32,
                        ),
                        quest_hook::libil2cpp::Void,
                        7usize,
                    >(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), ".ctor",
                            7usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info
                .invoke_unchecked(
                    self,
                    (
                        _cordl_time,
                        scoringType,
                        rating,
                        cutScore,
                        beforeCutScore,
                        centerDistanceCutScore,
                        afterCutScore,
                    ),
                )?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "NoteExecutionRating")]
impl quest_hook::libil2cpp::ObjectType for crate::GlobalNamespace::NoteExecutionRating {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "NoteExecutionRating+Rating")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum NoteExecutionRating_Rating {
    #[default]
    BadCut = 2i32,
    GoodCut = 0i32,
    Miss = 1i32,
}
#[cfg(feature = "NoteExecutionRating+Rating")]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::NoteExecutionRating_Rating {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "NoteExecutionRating/Rating";
    fn matches_value_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        !ty.is_ref()
            && ty
                .class()
                .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref()
            && ty
                .class()
                .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_value_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        !ty.is_ref()
            && <Self as quest_hook::libil2cpp::Type>::class()
                .is_assignable_from(ty.class())
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref()
            && <Self as quest_hook::libil2cpp::Type>::class()
                .is_assignable_from(ty.class())
    }
}
#[cfg(feature = "NoteExecutionRating+Rating")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::GlobalNamespace::NoteExecutionRating_Rating {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "NoteExecutionRating+Rating")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::GlobalNamespace::NoteExecutionRating_Rating {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_parameter(ty)
    }
    fn from_actual(actual: <Self as quest_hook::libil2cpp::Parameter>::Actual) -> Self {
        actual
    }
    fn into_actual(self) -> <Self as quest_hook::libil2cpp::Parameter>::Actual {
        self
    }
}
#[cfg(feature = "NoteExecutionRating+Rating")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::GlobalNamespace::NoteExecutionRating_Rating {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_returned(ty)
    }
    fn from_object(object: Option<&mut quest_hook::libil2cpp::Il2CppObject>) -> Self {
        unsafe {
            quest_hook::libil2cpp::raw::unbox(
                quest_hook::libil2cpp::WrapRaw::raw(object.unwrap()),
            )
        }
    }
}
#[cfg(feature = "NoteExecutionRating+Rating")]
unsafe impl quest_hook::libil2cpp::Return
for crate::GlobalNamespace::NoteExecutionRating_Rating {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_return(ty)
    }
    fn into_actual(self) -> <Self as quest_hook::libil2cpp::Return>::Actual {
        self
    }
    fn from_actual(actual: <Self as quest_hook::libil2cpp::Return>::Actual) -> Self {
        actual
    }
}
