#[cfg(feature = "Unity+Mathematics+Random")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct Random {
    pub state: u32,
}
#[cfg(feature = "Unity+Mathematics+Random")]
unsafe impl quest_hook::libil2cpp::Type for crate::Unity::Mathematics::Random {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "Unity.Mathematics";
    const CLASS_NAME: &'static str = "Random";
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
#[cfg(feature = "Unity+Mathematics+Random")]
unsafe impl quest_hook::libil2cpp::Argument for crate::Unity::Mathematics::Random {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "Unity+Mathematics+Random")]
unsafe impl quest_hook::libil2cpp::Parameter for crate::Unity::Mathematics::Random {
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
#[cfg(feature = "Unity+Mathematics+Random")]
unsafe impl quest_hook::libil2cpp::Returned for crate::Unity::Mathematics::Random {
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
#[cfg(feature = "Unity+Mathematics+Random")]
unsafe impl quest_hook::libil2cpp::Return for crate::Unity::Mathematics::Random {
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
#[cfg(feature = "Unity+Mathematics+Random")]
unsafe impl quest_hook::libil2cpp::ThisArgument for crate::Unity::Mathematics::Random {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "Unity+Mathematics+Random")]
impl crate::Unity::Mathematics::Random {
    pub fn CheckIndexForHash(
        index: u32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (u32),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("CheckIndexForHash")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "CheckIndexForHash", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked((), (index))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn CheckInitState(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        quest_hook::libil2cpp::Void,
                        0usize,
                    >("CheckInitState")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "CheckInitState", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn CheckNextIntMax(
        &mut self,
        max: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (i32),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("CheckNextIntMax")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "CheckNextIntMax", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (max))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn CheckNextIntMinMax(
        &mut self,
        min: i32,
        max: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (i32, i32),
                        quest_hook::libil2cpp::Void,
                        2usize,
                    >("CheckNextIntMinMax")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "CheckNextIntMinMax", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (min, max))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn CheckNextUIntMinMax(
        &mut self,
        min: u32,
        max: u32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (u32, u32),
                        quest_hook::libil2cpp::Void,
                        2usize,
                    >("CheckNextUIntMinMax")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "CheckNextUIntMinMax", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (min, max))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn CheckState(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), quest_hook::libil2cpp::Void, 0usize>("CheckState")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "CheckState", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn CreateFromIndex(
        index: u32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::Random> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (u32),
                        crate::Unity::Mathematics::Random,
                        1usize,
                    >("CreateFromIndex")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "CreateFromIndex", 1usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Mathematics::Random = unsafe {
            cordl_method_info.invoke_unchecked((), (index))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn InitState(
        &mut self,
        seed: u32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (u32),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("InitState")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "InitState", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (seed))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn NextBool(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), bool, 0usize>("NextBool")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "NextBool", 0usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn NextBool2(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::bool2> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        crate::Unity::Mathematics::bool2,
                        0usize,
                    >("NextBool2")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "NextBool2", 0usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Mathematics::bool2 = unsafe {
            cordl_method_info.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn NextBool3(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::bool3> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        crate::Unity::Mathematics::bool3,
                        0usize,
                    >("NextBool3")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "NextBool3", 0usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Mathematics::bool3 = unsafe {
            cordl_method_info.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn NextBool4(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::bool4> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        crate::Unity::Mathematics::bool4,
                        0usize,
                    >("NextBool4")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "NextBool4", 0usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Mathematics::bool4 = unsafe {
            cordl_method_info.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn NextDouble2Direction(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double2> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        crate::Unity::Mathematics::double2,
                        0usize,
                    >("NextDouble2Direction")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "NextDouble2Direction", 0usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Mathematics::double2 = unsafe {
            cordl_method_info.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn NextDouble2_0(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double2> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        crate::Unity::Mathematics::double2,
                        0usize,
                    >("NextDouble2")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "NextDouble2", 0usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Mathematics::double2 = unsafe {
            cordl_method_info.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn NextDouble2_double2_1(
        &mut self,
        max: crate::Unity::Mathematics::double2,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double2> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (crate::Unity::Mathematics::double2),
                        crate::Unity::Mathematics::double2,
                        1usize,
                    >("NextDouble2")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "NextDouble2", 1usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Mathematics::double2 = unsafe {
            cordl_method_info.invoke_unchecked(self, (max))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn NextDouble2_double2_double2_2(
        &mut self,
        min: crate::Unity::Mathematics::double2,
        max: crate::Unity::Mathematics::double2,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double2> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            crate::Unity::Mathematics::double2,
                            crate::Unity::Mathematics::double2,
                        ),
                        crate::Unity::Mathematics::double2,
                        2usize,
                    >("NextDouble2")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "NextDouble2", 2usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Mathematics::double2 = unsafe {
            cordl_method_info.invoke_unchecked(self, (min, max))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn NextDouble3Direction(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double3> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        crate::Unity::Mathematics::double3,
                        0usize,
                    >("NextDouble3Direction")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "NextDouble3Direction", 0usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Mathematics::double3 = unsafe {
            cordl_method_info.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn NextDouble3_0(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double3> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        crate::Unity::Mathematics::double3,
                        0usize,
                    >("NextDouble3")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "NextDouble3", 0usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Mathematics::double3 = unsafe {
            cordl_method_info.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn NextDouble3_double3_1(
        &mut self,
        max: crate::Unity::Mathematics::double3,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double3> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (crate::Unity::Mathematics::double3),
                        crate::Unity::Mathematics::double3,
                        1usize,
                    >("NextDouble3")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "NextDouble3", 1usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Mathematics::double3 = unsafe {
            cordl_method_info.invoke_unchecked(self, (max))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn NextDouble3_double3_double3_2(
        &mut self,
        min: crate::Unity::Mathematics::double3,
        max: crate::Unity::Mathematics::double3,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double3> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            crate::Unity::Mathematics::double3,
                            crate::Unity::Mathematics::double3,
                        ),
                        crate::Unity::Mathematics::double3,
                        2usize,
                    >("NextDouble3")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "NextDouble3", 2usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Mathematics::double3 = unsafe {
            cordl_method_info.invoke_unchecked(self, (min, max))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn NextDouble4_0(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double4> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        crate::Unity::Mathematics::double4,
                        0usize,
                    >("NextDouble4")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "NextDouble4", 0usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Mathematics::double4 = unsafe {
            cordl_method_info.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn NextDouble4_double4_1(
        &mut self,
        max: crate::Unity::Mathematics::double4,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double4> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (crate::Unity::Mathematics::double4),
                        crate::Unity::Mathematics::double4,
                        1usize,
                    >("NextDouble4")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "NextDouble4", 1usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Mathematics::double4 = unsafe {
            cordl_method_info.invoke_unchecked(self, (max))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn NextDouble4_double4_double4_2(
        &mut self,
        min: crate::Unity::Mathematics::double4,
        max: crate::Unity::Mathematics::double4,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double4> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            crate::Unity::Mathematics::double4,
                            crate::Unity::Mathematics::double4,
                        ),
                        crate::Unity::Mathematics::double4,
                        2usize,
                    >("NextDouble4")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "NextDouble4", 2usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Mathematics::double4 = unsafe {
            cordl_method_info.invoke_unchecked(self, (min, max))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn NextDouble_0(&mut self) -> quest_hook::libil2cpp::Result<f64> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), f64, 0usize>("NextDouble")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "NextDouble", 0usize
                        )
                    })
            });
        let __cordl_ret: f64 = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn NextDouble_f64_1(&mut self, max: f64) -> quest_hook::libil2cpp::Result<f64> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(f64), f64, 1usize>("NextDouble")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "NextDouble", 1usize
                        )
                    })
            });
        let __cordl_ret: f64 = unsafe {
            cordl_method_info.invoke_unchecked(self, (max))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn NextDouble_f64_f64_2(
        &mut self,
        min: f64,
        max: f64,
    ) -> quest_hook::libil2cpp::Result<f64> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(f64, f64), f64, 2usize>("NextDouble")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "NextDouble", 2usize
                        )
                    })
            });
        let __cordl_ret: f64 = unsafe {
            cordl_method_info.invoke_unchecked(self, (min, max))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn NextFloat2Direction(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float2> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        crate::Unity::Mathematics::float2,
                        0usize,
                    >("NextFloat2Direction")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "NextFloat2Direction", 0usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Mathematics::float2 = unsafe {
            cordl_method_info.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn NextFloat2_0(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float2> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        crate::Unity::Mathematics::float2,
                        0usize,
                    >("NextFloat2")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "NextFloat2", 0usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Mathematics::float2 = unsafe {
            cordl_method_info.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn NextFloat2_float2_1(
        &mut self,
        max: crate::Unity::Mathematics::float2,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float2> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (crate::Unity::Mathematics::float2),
                        crate::Unity::Mathematics::float2,
                        1usize,
                    >("NextFloat2")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "NextFloat2", 1usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Mathematics::float2 = unsafe {
            cordl_method_info.invoke_unchecked(self, (max))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn NextFloat2_float2_float2_2(
        &mut self,
        min: crate::Unity::Mathematics::float2,
        max: crate::Unity::Mathematics::float2,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float2> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            crate::Unity::Mathematics::float2,
                            crate::Unity::Mathematics::float2,
                        ),
                        crate::Unity::Mathematics::float2,
                        2usize,
                    >("NextFloat2")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "NextFloat2", 2usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Mathematics::float2 = unsafe {
            cordl_method_info.invoke_unchecked(self, (min, max))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn NextFloat3Direction(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float3> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        crate::Unity::Mathematics::float3,
                        0usize,
                    >("NextFloat3Direction")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "NextFloat3Direction", 0usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Mathematics::float3 = unsafe {
            cordl_method_info.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn NextFloat3_0(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float3> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        crate::Unity::Mathematics::float3,
                        0usize,
                    >("NextFloat3")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "NextFloat3", 0usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Mathematics::float3 = unsafe {
            cordl_method_info.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn NextFloat3_float3_1(
        &mut self,
        max: crate::Unity::Mathematics::float3,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float3> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (crate::Unity::Mathematics::float3),
                        crate::Unity::Mathematics::float3,
                        1usize,
                    >("NextFloat3")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "NextFloat3", 1usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Mathematics::float3 = unsafe {
            cordl_method_info.invoke_unchecked(self, (max))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn NextFloat3_float3_float3_2(
        &mut self,
        min: crate::Unity::Mathematics::float3,
        max: crate::Unity::Mathematics::float3,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float3> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            crate::Unity::Mathematics::float3,
                            crate::Unity::Mathematics::float3,
                        ),
                        crate::Unity::Mathematics::float3,
                        2usize,
                    >("NextFloat3")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "NextFloat3", 2usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Mathematics::float3 = unsafe {
            cordl_method_info.invoke_unchecked(self, (min, max))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn NextFloat4_0(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float4> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        crate::Unity::Mathematics::float4,
                        0usize,
                    >("NextFloat4")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "NextFloat4", 0usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Mathematics::float4 = unsafe {
            cordl_method_info.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn NextFloat4_float4_1(
        &mut self,
        max: crate::Unity::Mathematics::float4,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float4> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (crate::Unity::Mathematics::float4),
                        crate::Unity::Mathematics::float4,
                        1usize,
                    >("NextFloat4")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "NextFloat4", 1usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Mathematics::float4 = unsafe {
            cordl_method_info.invoke_unchecked(self, (max))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn NextFloat4_float4_float4_2(
        &mut self,
        min: crate::Unity::Mathematics::float4,
        max: crate::Unity::Mathematics::float4,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float4> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            crate::Unity::Mathematics::float4,
                            crate::Unity::Mathematics::float4,
                        ),
                        crate::Unity::Mathematics::float4,
                        2usize,
                    >("NextFloat4")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "NextFloat4", 2usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Mathematics::float4 = unsafe {
            cordl_method_info.invoke_unchecked(self, (min, max))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn NextFloat_0(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), f32, 0usize>("NextFloat")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "NextFloat", 0usize
                        )
                    })
            });
        let __cordl_ret: f32 = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn NextFloat_f32_1(&mut self, max: f32) -> quest_hook::libil2cpp::Result<f32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(f32), f32, 1usize>("NextFloat")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "NextFloat", 1usize
                        )
                    })
            });
        let __cordl_ret: f32 = unsafe {
            cordl_method_info.invoke_unchecked(self, (max))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn NextFloat_f32_f32_2(
        &mut self,
        min: f32,
        max: f32,
    ) -> quest_hook::libil2cpp::Result<f32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(f32, f32), f32, 2usize>("NextFloat")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "NextFloat", 2usize
                        )
                    })
            });
        let __cordl_ret: f32 = unsafe {
            cordl_method_info.invoke_unchecked(self, (min, max))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn NextInt2_0(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::int2> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        crate::Unity::Mathematics::int2,
                        0usize,
                    >("NextInt2")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "NextInt2", 0usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Mathematics::int2 = unsafe {
            cordl_method_info.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn NextInt2_int2_1(
        &mut self,
        max: crate::Unity::Mathematics::int2,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::int2> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (crate::Unity::Mathematics::int2),
                        crate::Unity::Mathematics::int2,
                        1usize,
                    >("NextInt2")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "NextInt2", 1usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Mathematics::int2 = unsafe {
            cordl_method_info.invoke_unchecked(self, (max))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn NextInt2_int2_int2_2(
        &mut self,
        min: crate::Unity::Mathematics::int2,
        max: crate::Unity::Mathematics::int2,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::int2> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            crate::Unity::Mathematics::int2,
                            crate::Unity::Mathematics::int2,
                        ),
                        crate::Unity::Mathematics::int2,
                        2usize,
                    >("NextInt2")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "NextInt2", 2usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Mathematics::int2 = unsafe {
            cordl_method_info.invoke_unchecked(self, (min, max))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn NextInt3_0(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::int3> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        crate::Unity::Mathematics::int3,
                        0usize,
                    >("NextInt3")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "NextInt3", 0usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Mathematics::int3 = unsafe {
            cordl_method_info.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn NextInt3_int3_1(
        &mut self,
        max: crate::Unity::Mathematics::int3,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::int3> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (crate::Unity::Mathematics::int3),
                        crate::Unity::Mathematics::int3,
                        1usize,
                    >("NextInt3")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "NextInt3", 1usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Mathematics::int3 = unsafe {
            cordl_method_info.invoke_unchecked(self, (max))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn NextInt3_int3_int3_2(
        &mut self,
        min: crate::Unity::Mathematics::int3,
        max: crate::Unity::Mathematics::int3,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::int3> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            crate::Unity::Mathematics::int3,
                            crate::Unity::Mathematics::int3,
                        ),
                        crate::Unity::Mathematics::int3,
                        2usize,
                    >("NextInt3")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "NextInt3", 2usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Mathematics::int3 = unsafe {
            cordl_method_info.invoke_unchecked(self, (min, max))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn NextInt4_0(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::int4> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        crate::Unity::Mathematics::int4,
                        0usize,
                    >("NextInt4")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "NextInt4", 0usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Mathematics::int4 = unsafe {
            cordl_method_info.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn NextInt4_int4_1(
        &mut self,
        max: crate::Unity::Mathematics::int4,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::int4> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (crate::Unity::Mathematics::int4),
                        crate::Unity::Mathematics::int4,
                        1usize,
                    >("NextInt4")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "NextInt4", 1usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Mathematics::int4 = unsafe {
            cordl_method_info.invoke_unchecked(self, (max))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn NextInt4_int4_int4_2(
        &mut self,
        min: crate::Unity::Mathematics::int4,
        max: crate::Unity::Mathematics::int4,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::int4> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            crate::Unity::Mathematics::int4,
                            crate::Unity::Mathematics::int4,
                        ),
                        crate::Unity::Mathematics::int4,
                        2usize,
                    >("NextInt4")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "NextInt4", 2usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Mathematics::int4 = unsafe {
            cordl_method_info.invoke_unchecked(self, (min, max))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn NextInt_0(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), i32, 0usize>("NextInt")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "NextInt",
                            0usize
                        )
                    })
            });
        let __cordl_ret: i32 = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn NextInt_i32_1(&mut self, max: i32) -> quest_hook::libil2cpp::Result<i32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(i32), i32, 1usize>("NextInt")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "NextInt",
                            1usize
                        )
                    })
            });
        let __cordl_ret: i32 = unsafe {
            cordl_method_info.invoke_unchecked(self, (max))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn NextInt_i32_i32_2(
        &mut self,
        min: i32,
        max: i32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(i32, i32), i32, 2usize>("NextInt")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "NextInt",
                            2usize
                        )
                    })
            });
        let __cordl_ret: i32 = unsafe {
            cordl_method_info.invoke_unchecked(self, (min, max))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn NextQuaternionRotation(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::quaternion> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        crate::Unity::Mathematics::quaternion,
                        0usize,
                    >("NextQuaternionRotation")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "NextQuaternionRotation", 0usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Mathematics::quaternion = unsafe {
            cordl_method_info.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn NextState(&mut self) -> quest_hook::libil2cpp::Result<u32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), u32, 0usize>("NextState")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "NextState", 0usize
                        )
                    })
            });
        let __cordl_ret: u32 = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn NextUInt2_0(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::uint2> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        crate::Unity::Mathematics::uint2,
                        0usize,
                    >("NextUInt2")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "NextUInt2", 0usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Mathematics::uint2 = unsafe {
            cordl_method_info.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn NextUInt2_uint2_1(
        &mut self,
        max: crate::Unity::Mathematics::uint2,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::uint2> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (crate::Unity::Mathematics::uint2),
                        crate::Unity::Mathematics::uint2,
                        1usize,
                    >("NextUInt2")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "NextUInt2", 1usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Mathematics::uint2 = unsafe {
            cordl_method_info.invoke_unchecked(self, (max))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn NextUInt2_uint2_uint2_2(
        &mut self,
        min: crate::Unity::Mathematics::uint2,
        max: crate::Unity::Mathematics::uint2,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::uint2> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            crate::Unity::Mathematics::uint2,
                            crate::Unity::Mathematics::uint2,
                        ),
                        crate::Unity::Mathematics::uint2,
                        2usize,
                    >("NextUInt2")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "NextUInt2", 2usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Mathematics::uint2 = unsafe {
            cordl_method_info.invoke_unchecked(self, (min, max))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn NextUInt3_0(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::uint3> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        crate::Unity::Mathematics::uint3,
                        0usize,
                    >("NextUInt3")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "NextUInt3", 0usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Mathematics::uint3 = unsafe {
            cordl_method_info.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn NextUInt3_uint3_1(
        &mut self,
        max: crate::Unity::Mathematics::uint3,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::uint3> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (crate::Unity::Mathematics::uint3),
                        crate::Unity::Mathematics::uint3,
                        1usize,
                    >("NextUInt3")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "NextUInt3", 1usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Mathematics::uint3 = unsafe {
            cordl_method_info.invoke_unchecked(self, (max))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn NextUInt3_uint3_uint3_2(
        &mut self,
        min: crate::Unity::Mathematics::uint3,
        max: crate::Unity::Mathematics::uint3,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::uint3> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            crate::Unity::Mathematics::uint3,
                            crate::Unity::Mathematics::uint3,
                        ),
                        crate::Unity::Mathematics::uint3,
                        2usize,
                    >("NextUInt3")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "NextUInt3", 2usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Mathematics::uint3 = unsafe {
            cordl_method_info.invoke_unchecked(self, (min, max))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn NextUInt4_0(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::uint4> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        crate::Unity::Mathematics::uint4,
                        0usize,
                    >("NextUInt4")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "NextUInt4", 0usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Mathematics::uint4 = unsafe {
            cordl_method_info.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn NextUInt4_uint4_1(
        &mut self,
        max: crate::Unity::Mathematics::uint4,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::uint4> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (crate::Unity::Mathematics::uint4),
                        crate::Unity::Mathematics::uint4,
                        1usize,
                    >("NextUInt4")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "NextUInt4", 1usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Mathematics::uint4 = unsafe {
            cordl_method_info.invoke_unchecked(self, (max))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn NextUInt4_uint4_uint4_2(
        &mut self,
        min: crate::Unity::Mathematics::uint4,
        max: crate::Unity::Mathematics::uint4,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::uint4> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            crate::Unity::Mathematics::uint4,
                            crate::Unity::Mathematics::uint4,
                        ),
                        crate::Unity::Mathematics::uint4,
                        2usize,
                    >("NextUInt4")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "NextUInt4", 2usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Mathematics::uint4 = unsafe {
            cordl_method_info.invoke_unchecked(self, (min, max))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn NextUInt_0(&mut self) -> quest_hook::libil2cpp::Result<u32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), u32, 0usize>("NextUInt")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "NextUInt", 0usize
                        )
                    })
            });
        let __cordl_ret: u32 = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn NextUInt_u32_1(&mut self, max: u32) -> quest_hook::libil2cpp::Result<u32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(u32), u32, 1usize>("NextUInt")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "NextUInt", 1usize
                        )
                    })
            });
        let __cordl_ret: u32 = unsafe {
            cordl_method_info.invoke_unchecked(self, (max))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn NextUInt_u32_u32_2(
        &mut self,
        min: u32,
        max: u32,
    ) -> quest_hook::libil2cpp::Result<u32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(u32, u32), u32, 2usize>("NextUInt")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "NextUInt", 2usize
                        )
                    })
            });
        let __cordl_ret: u32 = unsafe {
            cordl_method_info.invoke_unchecked(self, (min, max))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn WangHash(n: u32) -> quest_hook::libil2cpp::Result<u32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(u32), u32, 1usize>("WangHash")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "WangHash", 1usize
                        )
                    })
            });
        let __cordl_ret: u32 = unsafe { cordl_method_info.invoke_unchecked((), (n))? };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        seed: u32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(u32), quest_hook::libil2cpp::Void, 1usize>(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), ".ctor",
                            1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (seed))?
        };
        Ok(__cordl_ret.into())
    }
}
