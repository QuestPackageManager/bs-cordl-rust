#[cfg(feature = "System+Math")]
#[repr(C)]
#[derive(Debug)]
pub struct Math {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "System+Math")]
unsafe impl quest_hook::libil2cpp::Type for crate::System::Math {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "System";
    const CLASS_NAME: &'static str = "Math";
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
#[cfg(feature = "System+Math")]
impl std::ops::Deref for crate::System::Math {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Math")]
impl std::ops::DerefMut for crate::System::Math {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Math")]
impl crate::System::Math {
    pub fn Abs_Decimal2(
        value: crate::System::Decimal,
    ) -> quest_hook::libil2cpp::Result<crate::System::Decimal> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_static_method::<
                        (crate::System::Decimal),
                        crate::System::Decimal,
                        1usize,
                    >("Abs")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "Abs", 1usize
                        )
                    })
            });
        let __cordl_ret: crate::System::Decimal = unsafe {
            method.invoke_unchecked((), (value))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Abs_f32_4(value: f32) -> quest_hook::libil2cpp::Result<f32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_static_method::<(f32), f32, 1usize>("Abs")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "Abs", 1usize
                        )
                    })
            });
        let __cordl_ret: f32 = unsafe { method.invoke_unchecked((), (value))? };
        Ok(__cordl_ret.into())
    }
    pub fn Abs_f64_3(value: f64) -> quest_hook::libil2cpp::Result<f64> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_static_method::<(f64), f64, 1usize>("Abs")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "Abs", 1usize
                        )
                    })
            });
        let __cordl_ret: f64 = unsafe { method.invoke_unchecked((), (value))? };
        Ok(__cordl_ret.into())
    }
    pub fn Abs_i32_0(value: i32) -> quest_hook::libil2cpp::Result<i32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_static_method::<(i32), i32, 1usize>("Abs")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "Abs", 1usize
                        )
                    })
            });
        let __cordl_ret: i32 = unsafe { method.invoke_unchecked((), (value))? };
        Ok(__cordl_ret.into())
    }
    pub fn Abs_i64_1(value: i64) -> quest_hook::libil2cpp::Result<i64> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_static_method::<(i64), i64, 1usize>("Abs")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "Abs", 1usize
                        )
                    })
            });
        let __cordl_ret: i64 = unsafe { method.invoke_unchecked((), (value))? };
        Ok(__cordl_ret.into())
    }
    pub fn Acos(d: f64) -> quest_hook::libil2cpp::Result<f64> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_static_method::<(f64), f64, 1usize>("Acos")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "Acos", 1usize
                        )
                    })
            });
        let __cordl_ret: f64 = unsafe { method.invoke_unchecked((), (d))? };
        Ok(__cordl_ret.into())
    }
    pub fn Asin(d: f64) -> quest_hook::libil2cpp::Result<f64> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_static_method::<(f64), f64, 1usize>("Asin")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "Asin", 1usize
                        )
                    })
            });
        let __cordl_ret: f64 = unsafe { method.invoke_unchecked((), (d))? };
        Ok(__cordl_ret.into())
    }
    pub fn Atan(d: f64) -> quest_hook::libil2cpp::Result<f64> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_static_method::<(f64), f64, 1usize>("Atan")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "Atan", 1usize
                        )
                    })
            });
        let __cordl_ret: f64 = unsafe { method.invoke_unchecked((), (d))? };
        Ok(__cordl_ret.into())
    }
    pub fn Atan2(y: f64, x: f64) -> quest_hook::libil2cpp::Result<f64> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_static_method::<(f64, f64), f64, 2usize>("Atan2")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "Atan2", 2usize
                        )
                    })
            });
        let __cordl_ret: f64 = unsafe { method.invoke_unchecked((), (y, x))? };
        Ok(__cordl_ret.into())
    }
    pub fn Ceiling(a: f64) -> quest_hook::libil2cpp::Result<f64> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_static_method::<(f64), f64, 1usize>("Ceiling")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "Ceiling", 1usize
                        )
                    })
            });
        let __cordl_ret: f64 = unsafe { method.invoke_unchecked((), (a))? };
        Ok(__cordl_ret.into())
    }
    pub fn Clamp(value: i32, min: i32, max: i32) -> quest_hook::libil2cpp::Result<i32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_static_method::<(i32, i32, i32), i32, 3usize>("Clamp")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "Clamp", 3usize
                        )
                    })
            });
        let __cordl_ret: i32 = unsafe {
            method.invoke_unchecked((), (value, min, max))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Cos(d: f64) -> quest_hook::libil2cpp::Result<f64> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_static_method::<(f64), f64, 1usize>("Cos")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "Cos", 1usize
                        )
                    })
            });
        let __cordl_ret: f64 = unsafe { method.invoke_unchecked((), (d))? };
        Ok(__cordl_ret.into())
    }
    pub fn Cosh(value: f64) -> quest_hook::libil2cpp::Result<f64> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_static_method::<(f64), f64, 1usize>("Cosh")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "Cosh", 1usize
                        )
                    })
            });
        let __cordl_ret: f64 = unsafe { method.invoke_unchecked((), (value))? };
        Ok(__cordl_ret.into())
    }
    pub fn DivRem(
        a: i32,
        b: i32,
        result: quest_hook::libil2cpp::ByRefMut<i32>,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_static_method::<
                        (i32, i32, quest_hook::libil2cpp::ByRefMut<i32>),
                        i32,
                        3usize,
                    >("DivRem")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "DivRem", 3usize
                        )
                    })
            });
        let __cordl_ret: i32 = unsafe { method.invoke_unchecked((), (a, b, result))? };
        Ok(__cordl_ret.into())
    }
    pub fn Exp(d: f64) -> quest_hook::libil2cpp::Result<f64> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_static_method::<(f64), f64, 1usize>("Exp")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "Exp", 1usize
                        )
                    })
            });
        let __cordl_ret: f64 = unsafe { method.invoke_unchecked((), (d))? };
        Ok(__cordl_ret.into())
    }
    pub fn Floor(d: f64) -> quest_hook::libil2cpp::Result<f64> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_static_method::<(f64), f64, 1usize>("Floor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "Floor", 1usize
                        )
                    })
            });
        let __cordl_ret: f64 = unsafe { method.invoke_unchecked((), (d))? };
        Ok(__cordl_ret.into())
    }
    pub fn Log10(d: f64) -> quest_hook::libil2cpp::Result<f64> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_static_method::<(f64), f64, 1usize>("Log10")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "Log10", 1usize
                        )
                    })
            });
        let __cordl_ret: f64 = unsafe { method.invoke_unchecked((), (d))? };
        Ok(__cordl_ret.into())
    }
    pub fn Log_f64_0(a: f64, newBase: f64) -> quest_hook::libil2cpp::Result<f64> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_static_method::<(f64, f64), f64, 2usize>("Log")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "Log", 2usize
                        )
                    })
            });
        let __cordl_ret: f64 = unsafe { method.invoke_unchecked((), (a, newBase))? };
        Ok(__cordl_ret.into())
    }
    pub fn Log_f64_1(d: f64) -> quest_hook::libil2cpp::Result<f64> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_static_method::<(f64), f64, 1usize>("Log")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "Log", 1usize
                        )
                    })
            });
        let __cordl_ret: f64 = unsafe { method.invoke_unchecked((), (d))? };
        Ok(__cordl_ret.into())
    }
    pub fn Max_Decimal_Decimal1(
        val1: crate::System::Decimal,
        val2: crate::System::Decimal,
    ) -> quest_hook::libil2cpp::Result<crate::System::Decimal> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_static_method::<
                        (crate::System::Decimal, crate::System::Decimal),
                        crate::System::Decimal,
                        2usize,
                    >("Max")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "Max", 2usize
                        )
                    })
            });
        let __cordl_ret: crate::System::Decimal = unsafe {
            method.invoke_unchecked((), (val1, val2))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Max_f32_f32_7(val1: f32, val2: f32) -> quest_hook::libil2cpp::Result<f32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_static_method::<(f32, f32), f32, 2usize>("Max")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "Max", 2usize
                        )
                    })
            });
        let __cordl_ret: f32 = unsafe { method.invoke_unchecked((), (val1, val2))? };
        Ok(__cordl_ret.into())
    }
    pub fn Max_f64_f64_2(val1: f64, val2: f64) -> quest_hook::libil2cpp::Result<f64> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_static_method::<(f64, f64), f64, 2usize>("Max")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "Max", 2usize
                        )
                    })
            });
        let __cordl_ret: f64 = unsafe { method.invoke_unchecked((), (val1, val2))? };
        Ok(__cordl_ret.into())
    }
    pub fn Max_i16_i16_3(val1: i16, val2: i16) -> quest_hook::libil2cpp::Result<i16> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_static_method::<(i16, i16), i16, 2usize>("Max")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "Max", 2usize
                        )
                    })
            });
        let __cordl_ret: i16 = unsafe { method.invoke_unchecked((), (val1, val2))? };
        Ok(__cordl_ret.into())
    }
    pub fn Max_i32_i32_4(val1: i32, val2: i32) -> quest_hook::libil2cpp::Result<i32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_static_method::<(i32, i32), i32, 2usize>("Max")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "Max", 2usize
                        )
                    })
            });
        let __cordl_ret: i32 = unsafe { method.invoke_unchecked((), (val1, val2))? };
        Ok(__cordl_ret.into())
    }
    pub fn Max_i64_i64_5(val1: i64, val2: i64) -> quest_hook::libil2cpp::Result<i64> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_static_method::<(i64, i64), i64, 2usize>("Max")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "Max", 2usize
                        )
                    })
            });
        let __cordl_ret: i64 = unsafe { method.invoke_unchecked((), (val1, val2))? };
        Ok(__cordl_ret.into())
    }
    pub fn Max_i8_i8_6(val1: i8, val2: i8) -> quest_hook::libil2cpp::Result<i8> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_static_method::<(i8, i8), i8, 2usize>("Max")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "Max", 2usize
                        )
                    })
            });
        let __cordl_ret: i8 = unsafe { method.invoke_unchecked((), (val1, val2))? };
        Ok(__cordl_ret.into())
    }
    pub fn Max_u16_u16_8(val1: u16, val2: u16) -> quest_hook::libil2cpp::Result<u16> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_static_method::<(u16, u16), u16, 2usize>("Max")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "Max", 2usize
                        )
                    })
            });
        let __cordl_ret: u16 = unsafe { method.invoke_unchecked((), (val1, val2))? };
        Ok(__cordl_ret.into())
    }
    pub fn Max_u32_u32_9(val1: u32, val2: u32) -> quest_hook::libil2cpp::Result<u32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_static_method::<(u32, u32), u32, 2usize>("Max")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "Max", 2usize
                        )
                    })
            });
        let __cordl_ret: u32 = unsafe { method.invoke_unchecked((), (val1, val2))? };
        Ok(__cordl_ret.into())
    }
    pub fn Max_u64_u64_10(val1: u64, val2: u64) -> quest_hook::libil2cpp::Result<u64> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_static_method::<(u64, u64), u64, 2usize>("Max")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "Max", 2usize
                        )
                    })
            });
        let __cordl_ret: u64 = unsafe { method.invoke_unchecked((), (val1, val2))? };
        Ok(__cordl_ret.into())
    }
    pub fn Max_u8_u8_0(val1: u8, val2: u8) -> quest_hook::libil2cpp::Result<u8> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_static_method::<(u8, u8), u8, 2usize>("Max")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "Max", 2usize
                        )
                    })
            });
        let __cordl_ret: u8 = unsafe { method.invoke_unchecked((), (val1, val2))? };
        Ok(__cordl_ret.into())
    }
    pub fn Min_Decimal_Decimal1(
        val1: crate::System::Decimal,
        val2: crate::System::Decimal,
    ) -> quest_hook::libil2cpp::Result<crate::System::Decimal> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_static_method::<
                        (crate::System::Decimal, crate::System::Decimal),
                        crate::System::Decimal,
                        2usize,
                    >("Min")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "Min", 2usize
                        )
                    })
            });
        let __cordl_ret: crate::System::Decimal = unsafe {
            method.invoke_unchecked((), (val1, val2))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Min_f32_f32_7(val1: f32, val2: f32) -> quest_hook::libil2cpp::Result<f32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_static_method::<(f32, f32), f32, 2usize>("Min")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "Min", 2usize
                        )
                    })
            });
        let __cordl_ret: f32 = unsafe { method.invoke_unchecked((), (val1, val2))? };
        Ok(__cordl_ret.into())
    }
    pub fn Min_f64_f64_2(val1: f64, val2: f64) -> quest_hook::libil2cpp::Result<f64> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_static_method::<(f64, f64), f64, 2usize>("Min")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "Min", 2usize
                        )
                    })
            });
        let __cordl_ret: f64 = unsafe { method.invoke_unchecked((), (val1, val2))? };
        Ok(__cordl_ret.into())
    }
    pub fn Min_i16_i16_3(val1: i16, val2: i16) -> quest_hook::libil2cpp::Result<i16> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_static_method::<(i16, i16), i16, 2usize>("Min")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "Min", 2usize
                        )
                    })
            });
        let __cordl_ret: i16 = unsafe { method.invoke_unchecked((), (val1, val2))? };
        Ok(__cordl_ret.into())
    }
    pub fn Min_i32_i32_4(val1: i32, val2: i32) -> quest_hook::libil2cpp::Result<i32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_static_method::<(i32, i32), i32, 2usize>("Min")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "Min", 2usize
                        )
                    })
            });
        let __cordl_ret: i32 = unsafe { method.invoke_unchecked((), (val1, val2))? };
        Ok(__cordl_ret.into())
    }
    pub fn Min_i64_i64_5(val1: i64, val2: i64) -> quest_hook::libil2cpp::Result<i64> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_static_method::<(i64, i64), i64, 2usize>("Min")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "Min", 2usize
                        )
                    })
            });
        let __cordl_ret: i64 = unsafe { method.invoke_unchecked((), (val1, val2))? };
        Ok(__cordl_ret.into())
    }
    pub fn Min_i8_i8_6(val1: i8, val2: i8) -> quest_hook::libil2cpp::Result<i8> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_static_method::<(i8, i8), i8, 2usize>("Min")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "Min", 2usize
                        )
                    })
            });
        let __cordl_ret: i8 = unsafe { method.invoke_unchecked((), (val1, val2))? };
        Ok(__cordl_ret.into())
    }
    pub fn Min_u16_u16_8(val1: u16, val2: u16) -> quest_hook::libil2cpp::Result<u16> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_static_method::<(u16, u16), u16, 2usize>("Min")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "Min", 2usize
                        )
                    })
            });
        let __cordl_ret: u16 = unsafe { method.invoke_unchecked((), (val1, val2))? };
        Ok(__cordl_ret.into())
    }
    pub fn Min_u32_u32_9(val1: u32, val2: u32) -> quest_hook::libil2cpp::Result<u32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_static_method::<(u32, u32), u32, 2usize>("Min")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "Min", 2usize
                        )
                    })
            });
        let __cordl_ret: u32 = unsafe { method.invoke_unchecked((), (val1, val2))? };
        Ok(__cordl_ret.into())
    }
    pub fn Min_u64_u64_10(val1: u64, val2: u64) -> quest_hook::libil2cpp::Result<u64> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_static_method::<(u64, u64), u64, 2usize>("Min")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "Min", 2usize
                        )
                    })
            });
        let __cordl_ret: u64 = unsafe { method.invoke_unchecked((), (val1, val2))? };
        Ok(__cordl_ret.into())
    }
    pub fn Min_u8_u8_0(val1: u8, val2: u8) -> quest_hook::libil2cpp::Result<u8> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_static_method::<(u8, u8), u8, 2usize>("Min")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "Min", 2usize
                        )
                    })
            });
        let __cordl_ret: u8 = unsafe { method.invoke_unchecked((), (val1, val2))? };
        Ok(__cordl_ret.into())
    }
    pub fn ModF(
        x: f64,
        intptr: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<f64> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_static_method::<
                        (
                            f64,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppObject,
                            >,
                        ),
                        f64,
                        2usize,
                    >("ModF")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "ModF", 2usize
                        )
                    })
            });
        let __cordl_ret: f64 = unsafe { method.invoke_unchecked((), (x, intptr))? };
        Ok(__cordl_ret.into())
    }
    pub fn Pow(x: f64, y: f64) -> quest_hook::libil2cpp::Result<f64> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_static_method::<(f64, f64), f64, 2usize>("Pow")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "Pow", 2usize
                        )
                    })
            });
        let __cordl_ret: f64 = unsafe { method.invoke_unchecked((), (x, y))? };
        Ok(__cordl_ret.into())
    }
    pub fn Round_Decimal0(
        d: crate::System::Decimal,
    ) -> quest_hook::libil2cpp::Result<crate::System::Decimal> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_static_method::<
                        (crate::System::Decimal),
                        crate::System::Decimal,
                        1usize,
                    >("Round")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "Round", 1usize
                        )
                    })
            });
        let __cordl_ret: crate::System::Decimal = unsafe {
            method.invoke_unchecked((), (d))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Round_f64_1(a: f64) -> quest_hook::libil2cpp::Result<f64> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_static_method::<(f64), f64, 1usize>("Round")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "Round", 1usize
                        )
                    })
            });
        let __cordl_ret: f64 = unsafe { method.invoke_unchecked((), (a))? };
        Ok(__cordl_ret.into())
    }
    pub fn Round_f64_MidpointRounding3(
        value: f64,
        mode: crate::System::MidpointRounding,
    ) -> quest_hook::libil2cpp::Result<f64> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_static_method::<
                        (f64, crate::System::MidpointRounding),
                        f64,
                        2usize,
                    >("Round")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "Round", 2usize
                        )
                    })
            });
        let __cordl_ret: f64 = unsafe { method.invoke_unchecked((), (value, mode))? };
        Ok(__cordl_ret.into())
    }
    pub fn Round_f64_i32_2(
        value: f64,
        digits: i32,
    ) -> quest_hook::libil2cpp::Result<f64> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_static_method::<(f64, i32), f64, 2usize>("Round")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "Round", 2usize
                        )
                    })
            });
        let __cordl_ret: f64 = unsafe { method.invoke_unchecked((), (value, digits))? };
        Ok(__cordl_ret.into())
    }
    pub fn Round_f64_i32_MidpointRounding4(
        value: f64,
        digits: i32,
        mode: crate::System::MidpointRounding,
    ) -> quest_hook::libil2cpp::Result<f64> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_static_method::<
                        (f64, i32, crate::System::MidpointRounding),
                        f64,
                        3usize,
                    >("Round")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "Round", 3usize
                        )
                    })
            });
        let __cordl_ret: f64 = unsafe {
            method.invoke_unchecked((), (value, digits, mode))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Sign_f64_0(value: f64) -> quest_hook::libil2cpp::Result<i32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_static_method::<(f64), i32, 1usize>("Sign")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "Sign", 1usize
                        )
                    })
            });
        let __cordl_ret: i32 = unsafe { method.invoke_unchecked((), (value))? };
        Ok(__cordl_ret.into())
    }
    pub fn Sign_i64_1(value: i64) -> quest_hook::libil2cpp::Result<i32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_static_method::<(i64), i32, 1usize>("Sign")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "Sign", 1usize
                        )
                    })
            });
        let __cordl_ret: i32 = unsafe { method.invoke_unchecked((), (value))? };
        Ok(__cordl_ret.into())
    }
    pub fn Sin(a: f64) -> quest_hook::libil2cpp::Result<f64> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_static_method::<(f64), f64, 1usize>("Sin")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "Sin", 1usize
                        )
                    })
            });
        let __cordl_ret: f64 = unsafe { method.invoke_unchecked((), (a))? };
        Ok(__cordl_ret.into())
    }
    pub fn Sinh(value: f64) -> quest_hook::libil2cpp::Result<f64> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_static_method::<(f64), f64, 1usize>("Sinh")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "Sinh", 1usize
                        )
                    })
            });
        let __cordl_ret: f64 = unsafe { method.invoke_unchecked((), (value))? };
        Ok(__cordl_ret.into())
    }
    pub fn Sqrt(d: f64) -> quest_hook::libil2cpp::Result<f64> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_static_method::<(f64), f64, 1usize>("Sqrt")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "Sqrt", 1usize
                        )
                    })
            });
        let __cordl_ret: f64 = unsafe { method.invoke_unchecked((), (d))? };
        Ok(__cordl_ret.into())
    }
    pub fn Tan(a: f64) -> quest_hook::libil2cpp::Result<f64> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_static_method::<(f64), f64, 1usize>("Tan")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "Tan", 1usize
                        )
                    })
            });
        let __cordl_ret: f64 = unsafe { method.invoke_unchecked((), (a))? };
        Ok(__cordl_ret.into())
    }
    pub fn Tanh(value: f64) -> quest_hook::libil2cpp::Result<f64> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_static_method::<(f64), f64, 1usize>("Tanh")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "Tanh", 1usize
                        )
                    })
            });
        let __cordl_ret: f64 = unsafe { method.invoke_unchecked((), (value))? };
        Ok(__cordl_ret.into())
    }
    pub fn ThrowAbsOverflow() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Void,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_static_method::<
                        (),
                        quest_hook::libil2cpp::Void,
                        0usize,
                    >("ThrowAbsOverflow")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "ThrowAbsOverflow", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ThrowMinMaxException<T>(
        min: T,
        max: T,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_static_method::<
                        (T, T),
                        quest_hook::libil2cpp::Void,
                        2usize,
                    >("ThrowMinMaxException")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "ThrowMinMaxException", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (min, max))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Truncate_Decimal0(
        d: crate::System::Decimal,
    ) -> quest_hook::libil2cpp::Result<crate::System::Decimal> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_static_method::<
                        (crate::System::Decimal),
                        crate::System::Decimal,
                        1usize,
                    >("Truncate")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "Truncate", 1usize
                        )
                    })
            });
        let __cordl_ret: crate::System::Decimal = unsafe {
            method.invoke_unchecked((), (d))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Truncate_f64_1(d: f64) -> quest_hook::libil2cpp::Result<f64> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_static_method::<(f64), f64, 1usize>("Truncate")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "Truncate", 1usize
                        )
                    })
            });
        let __cordl_ret: f64 = unsafe { method.invoke_unchecked((), (d))? };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Math")]
impl quest_hook::libil2cpp::ObjectType for crate::System::Math {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
