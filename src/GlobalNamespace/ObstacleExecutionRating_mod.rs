#[cfg(feature = "ObstacleExecutionRating")]
#[repr(C)]
#[derive(Debug)]
pub struct ObstacleExecutionRating {
    __cordl_parent: crate::GlobalNamespace::BeatmapObjectExecutionRating,
    pub _rating_k__BackingField: crate::GlobalNamespace::ObstacleExecutionRating_Rating,
}
#[cfg(feature = "ObstacleExecutionRating")]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::ObstacleExecutionRating {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "ObstacleExecutionRating";
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
#[cfg(feature = "ObstacleExecutionRating")]
impl std::ops::Deref for crate::GlobalNamespace::ObstacleExecutionRating {
    type Target = crate::GlobalNamespace::BeatmapObjectExecutionRating;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "ObstacleExecutionRating")]
impl std::ops::DerefMut for crate::GlobalNamespace::ObstacleExecutionRating {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "ObstacleExecutionRating")]
impl crate::GlobalNamespace::ObstacleExecutionRating {
    #[cfg(feature = "ObstacleExecutionRating+Rating")]
    pub type Rating = crate::GlobalNamespace::ObstacleExecutionRating_Rating;
    pub fn New(
        _cordl_time: f32,
        rating: crate::GlobalNamespace::ObstacleExecutionRating_Rating,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (_cordl_time, rating))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        _cordl_time: f32,
        rating: crate::GlobalNamespace::ObstacleExecutionRating_Rating,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (f32, crate::GlobalNamespace::ObstacleExecutionRating_Rating),
                quest_hook::libil2cpp::Void,
                2usize,
            >(".ctor")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, ".ctor", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (_cordl_time, rating))
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_rating(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::GlobalNamespace::ObstacleExecutionRating_Rating,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                crate::GlobalNamespace::ObstacleExecutionRating_Rating,
                0usize,
            >("get_rating")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "get_rating", 0usize
                )
            });
        let __cordl_ret: crate::GlobalNamespace::ObstacleExecutionRating_Rating = unsafe {
            method.invoke_unchecked(self, ())
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "ObstacleExecutionRating")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::ObstacleExecutionRating {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "ObstacleExecutionRating+Rating")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ObstacleExecutionRating_Rating {
    #[default]
    NotGood = 1i32,
    OK = 0i32,
}
#[cfg(feature = "ObstacleExecutionRating+Rating")]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::ObstacleExecutionRating_Rating {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "ObstacleExecutionRating/Rating";
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
#[cfg(feature = "ObstacleExecutionRating+Rating")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::GlobalNamespace::ObstacleExecutionRating_Rating {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "ObstacleExecutionRating+Rating")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::GlobalNamespace::ObstacleExecutionRating_Rating {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_parameter(ty)
    }
    fn from_actual(actual: Self::Actual) -> Self {
        actual
    }
    fn into_actual(self) -> Self::Actual {
        self
    }
}
#[cfg(feature = "ObstacleExecutionRating+Rating")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::GlobalNamespace::ObstacleExecutionRating_Rating {
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
#[cfg(feature = "ObstacleExecutionRating+Rating")]
unsafe impl quest_hook::libil2cpp::Return
for crate::GlobalNamespace::ObstacleExecutionRating_Rating {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_return(ty)
    }
    fn into_actual(self) -> Self::Actual {
        self
    }
    fn from_actual(actual: Self::Actual) -> Self {
        actual
    }
}
