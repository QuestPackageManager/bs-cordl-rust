#[cfg(feature = "RankModel")]
#[repr(C)]
#[derive(Debug)]
pub struct RankModel {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "RankModel")]
unsafe impl quest_hook::libil2cpp::Type for crate::GlobalNamespace::RankModel {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "RankModel";
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
#[cfg(feature = "RankModel")]
impl std::ops::Deref for crate::GlobalNamespace::RankModel {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "RankModel")]
impl std::ops::DerefMut for crate::GlobalNamespace::RankModel {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "RankModel")]
impl crate::GlobalNamespace::RankModel {
    #[cfg(feature = "RankModel+Rank")]
    pub type Rank = crate::GlobalNamespace::RankModel_Rank;
    pub fn GetRankForScore(
        multipliedScore: i32,
        modifiedScore: i32,
        maxMultipliedScore: i32,
        maxModifiedScore: i32,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::RankModel_Rank> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (i32, i32, i32, i32),
                crate::GlobalNamespace::RankModel_Rank,
                4usize,
            >("GetRankForScore")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "GetRankForScore", 4usize
                )
            });
        let __cordl_ret: crate::GlobalNamespace::RankModel_Rank = unsafe {
            method
                .invoke_unchecked(
                    (),
                    (
                        multipliedScore,
                        modifiedScore,
                        maxMultipliedScore,
                        maxModifiedScore,
                    ),
                )
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetRankName(
        rank: crate::GlobalNamespace::RankModel_Rank,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::GlobalNamespace::RankModel_Rank),
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                1usize,
            >("GetRankName")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "GetRankName", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = unsafe { method.invoke_unchecked((), (rank)) };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "RankModel")]
impl quest_hook::libil2cpp::ObjectType for crate::GlobalNamespace::RankModel {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "RankModel+Rank")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum RankModel_Rank {
    #[default]
    A = 4i32,
    B = 3i32,
    C = 2i32,
    D = 1i32,
    E = 0i32,
    S = 5i32,
    SS = 6i32,
    SSS = 7i32,
}
#[cfg(feature = "RankModel+Rank")]
unsafe impl quest_hook::libil2cpp::Type for crate::GlobalNamespace::RankModel_Rank {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "RankModel/Rank";
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
#[cfg(feature = "RankModel+Rank")]
unsafe impl quest_hook::libil2cpp::Argument for crate::GlobalNamespace::RankModel_Rank {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "RankModel+Rank")]
unsafe impl quest_hook::libil2cpp::Parameter for crate::GlobalNamespace::RankModel_Rank {
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
#[cfg(feature = "RankModel+Rank")]
unsafe impl quest_hook::libil2cpp::Returned for crate::GlobalNamespace::RankModel_Rank {
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
#[cfg(feature = "RankModel+Rank")]
unsafe impl quest_hook::libil2cpp::Return for crate::GlobalNamespace::RankModel_Rank {
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
