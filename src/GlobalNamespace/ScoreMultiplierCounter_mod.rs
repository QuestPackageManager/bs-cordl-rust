#[cfg(feature = "ScoreMultiplierCounter")]
#[repr(C)]
#[derive(Debug)]
pub struct ScoreMultiplierCounter {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _multiplier: i32,
    pub _multiplierIncreaseProgress: i32,
    pub _multiplierIncreaseMaxProgress: i32,
}
#[cfg(feature = "ScoreMultiplierCounter")]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::ScoreMultiplierCounter {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "ScoreMultiplierCounter";
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
#[cfg(feature = "ScoreMultiplierCounter")]
impl std::ops::Deref for crate::GlobalNamespace::ScoreMultiplierCounter {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "ScoreMultiplierCounter")]
impl std::ops::DerefMut for crate::GlobalNamespace::ScoreMultiplierCounter {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "ScoreMultiplierCounter")]
impl crate::GlobalNamespace::ScoreMultiplierCounter {
    #[cfg(feature = "ScoreMultiplierCounter+MultiplierEventType")]
    pub type MultiplierEventType = crate::GlobalNamespace::ScoreMultiplierCounter_MultiplierEventType;
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn ProcessMultiplierEvent(
        &mut self,
        multiplierEventType: crate::GlobalNamespace::ScoreMultiplierCounter_MultiplierEventType,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::GlobalNamespace::ScoreMultiplierCounter as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (crate::GlobalNamespace::ScoreMultiplierCounter_MultiplierEventType),
                bool,
                1usize,
            >("ProcessMultiplierEvent")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::GlobalNamespace::ScoreMultiplierCounter as
                    quest_hook::libil2cpp::Type > ::class(), "ProcessMultiplierEvent",
                    1usize
                )
            });
        let __cordl_ret: bool = unsafe {
            method.invoke_unchecked(self, (multiplierEventType))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Reset(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::GlobalNamespace::ScoreMultiplierCounter as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), quest_hook::libil2cpp::Void, 0usize>("Reset")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::GlobalNamespace::ScoreMultiplierCounter as
                    quest_hook::libil2cpp::Type > ::class(), "Reset", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::GlobalNamespace::ScoreMultiplierCounter as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), quest_hook::libil2cpp::Void, 0usize>(".ctor")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::GlobalNamespace::ScoreMultiplierCounter as
                    quest_hook::libil2cpp::Type > ::class(), ".ctor", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_multiplier(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::GlobalNamespace::ScoreMultiplierCounter as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), i32, 0usize>("get_multiplier")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::GlobalNamespace::ScoreMultiplierCounter as
                    quest_hook::libil2cpp::Type > ::class(), "get_multiplier", 0usize
                )
            });
        let __cordl_ret: i32 = unsafe { method.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_normalizedProgress(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::GlobalNamespace::ScoreMultiplierCounter as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), f32, 0usize>("get_normalizedProgress")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::GlobalNamespace::ScoreMultiplierCounter as
                    quest_hook::libil2cpp::Type > ::class(), "get_normalizedProgress",
                    0usize
                )
            });
        let __cordl_ret: f32 = unsafe { method.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "ScoreMultiplierCounter")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::ScoreMultiplierCounter {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "ScoreMultiplierCounter+MultiplierEventType")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ScoreMultiplierCounter_MultiplierEventType {
    #[default]
    Negative = 2i32,
    Neutral = 1i32,
    Positive = 0i32,
}
#[cfg(feature = "ScoreMultiplierCounter+MultiplierEventType")]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::ScoreMultiplierCounter_MultiplierEventType {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "ScoreMultiplierCounter/MultiplierEventType";
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
#[cfg(feature = "ScoreMultiplierCounter+MultiplierEventType")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::GlobalNamespace::ScoreMultiplierCounter_MultiplierEventType {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "ScoreMultiplierCounter+MultiplierEventType")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::GlobalNamespace::ScoreMultiplierCounter_MultiplierEventType {
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
#[cfg(feature = "ScoreMultiplierCounter+MultiplierEventType")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::GlobalNamespace::ScoreMultiplierCounter_MultiplierEventType {
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
#[cfg(feature = "ScoreMultiplierCounter+MultiplierEventType")]
unsafe impl quest_hook::libil2cpp::Return
for crate::GlobalNamespace::ScoreMultiplierCounter_MultiplierEventType {
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
