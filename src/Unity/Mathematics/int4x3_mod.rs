#[cfg(feature = "Unity+Mathematics+int4x3")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct int4x3 {
    pub c0: crate::Unity::Mathematics::int4,
    pub c1: crate::Unity::Mathematics::int4,
    pub c2: crate::Unity::Mathematics::int4,
}
#[cfg(feature = "Unity+Mathematics+int4x3")]
unsafe impl quest_hook::libil2cpp::Type for crate::Unity::Mathematics::int4x3 {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "Unity.Mathematics";
    const CLASS_NAME: &'static str = "int4x3";
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
#[cfg(feature = "Unity+Mathematics+int4x3")]
unsafe impl quest_hook::libil2cpp::Argument for crate::Unity::Mathematics::int4x3 {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "Unity+Mathematics+int4x3")]
unsafe impl quest_hook::libil2cpp::Parameter for crate::Unity::Mathematics::int4x3 {
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
#[cfg(feature = "Unity+Mathematics+int4x3")]
unsafe impl quest_hook::libil2cpp::Returned for crate::Unity::Mathematics::int4x3 {
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
#[cfg(feature = "Unity+Mathematics+int4x3")]
unsafe impl quest_hook::libil2cpp::Return for crate::Unity::Mathematics::int4x3 {
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
#[cfg(feature = "Unity+Mathematics+int4x3")]
unsafe impl quest_hook::libil2cpp::ThisArgument for crate::Unity::Mathematics::int4x3 {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "Unity+Mathematics+int4x3")]
impl crate::Unity::Mathematics::int4x3 {
    pub fn Equals_Il2CppObject1(
        &mut self,
        o: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>),
                        bool,
                        1usize,
                    >("Equals")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "Equals", 1usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked(self, (o))? };
        Ok(__cordl_ret.into())
    }
    pub fn Equals_int4x3_0(
        &mut self,
        rhs: crate::Unity::Mathematics::int4x3,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (crate::Unity::Mathematics::int4x3),
                        bool,
                        1usize,
                    >("Equals")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "Equals", 1usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked(self, (rhs))? };
        Ok(__cordl_ret.into())
    }
    pub fn GetHashCode(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<(), i32, 0usize>("GetHashCode")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "GetHashCode", 0usize
                        )
                    })
            });
        let __cordl_ret: i32 = unsafe { method.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn ToString_0(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (),
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        0usize,
                    >("ToString")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "ToString", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = unsafe { method.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn ToString_Il2CppString_IFormatProvider1(
        &mut self,
        format: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        formatProvider: quest_hook::libil2cpp::Gc<crate::System::IFormatProvider>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                            quest_hook::libil2cpp::Gc<crate::System::IFormatProvider>,
                        ),
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        2usize,
                    >("ToString")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "ToString", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = unsafe { method.invoke_unchecked(self, (format, formatProvider))? };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor__cordl_bool3(
        &mut self,
        v: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<(bool), quest_hook::libil2cpp::Void, 1usize>(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), ".ctor", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (v))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_bool4x3_4(
        &mut self,
        v: crate::Unity::Mathematics::bool4x3,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (crate::Unity::Mathematics::bool4x3),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), ".ctor", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (v))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_double4x3_10(
        &mut self,
        v: crate::Unity::Mathematics::double4x3,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (crate::Unity::Mathematics::double4x3),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), ".ctor", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (v))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_f32_7(
        &mut self,
        v: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<(f32), quest_hook::libil2cpp::Void, 1usize>(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), ".ctor", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (v))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_f64_9(
        &mut self,
        v: f64,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<(f64), quest_hook::libil2cpp::Void, 1usize>(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), ".ctor", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (v))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_float4x3_8(
        &mut self,
        v: crate::Unity::Mathematics::float4x3,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (crate::Unity::Mathematics::float4x3),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), ".ctor", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (v))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_i32_2(
        &mut self,
        v: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<(i32), quest_hook::libil2cpp::Void, 1usize>(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), ".ctor", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (v))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_i32_i32_i32_i32_i32_i32_i32_i32_i32_i32_i32_i32_1(
        &mut self,
        m00: i32,
        m01: i32,
        m02: i32,
        m10: i32,
        m11: i32,
        m12: i32,
        m20: i32,
        m21: i32,
        m22: i32,
        m30: i32,
        m31: i32,
        m32: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32),
                        quest_hook::libil2cpp::Void,
                        12usize,
                    >(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), ".ctor", 12usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method
                .invoke_unchecked(
                    self,
                    (m00, m01, m02, m10, m11, m12, m20, m21, m22, m30, m31, m32),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_int4_int4_int4_0(
        &mut self,
        c0: crate::Unity::Mathematics::int4,
        c1: crate::Unity::Mathematics::int4,
        c2: crate::Unity::Mathematics::int4,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (
                            crate::Unity::Mathematics::int4,
                            crate::Unity::Mathematics::int4,
                            crate::Unity::Mathematics::int4,
                        ),
                        quest_hook::libil2cpp::Void,
                        3usize,
                    >(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), ".ctor", 3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (c0, c1, c2))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_u32_5(
        &mut self,
        v: u32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<(u32), quest_hook::libil2cpp::Void, 1usize>(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), ".ctor", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (v))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_uint4x3_6(
        &mut self,
        v: crate::Unity::Mathematics::uint4x3,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (crate::Unity::Mathematics::uint4x3),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), ".ctor", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (v))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_Item(
        &mut self,
        index: i32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::ByRefMut<crate::Unity::Mathematics::int4>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (i32),
                        quest_hook::libil2cpp::ByRefMut<crate::Unity::Mathematics::int4>,
                        1usize,
                    >("get_Item")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "get_Item", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::ByRefMut<
            crate::Unity::Mathematics::int4,
        > = unsafe { method.invoke_unchecked(self, (index))? };
        Ok(__cordl_ret.into())
    }
    pub fn op_Addition_i32_int4x3_2(
        lhs: i32,
        rhs: crate::Unity::Mathematics::int4x3,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::int4x3> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_static_method::<
                        (i32, crate::Unity::Mathematics::int4x3),
                        crate::Unity::Mathematics::int4x3,
                        2usize,
                    >("op_Addition")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "op_Addition", 2usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Mathematics::int4x3 = unsafe {
            method.invoke_unchecked((), (lhs, rhs))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn op_Addition_int4x3_i32_1(
        lhs: crate::Unity::Mathematics::int4x3,
        rhs: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::int4x3> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_static_method::<
                        (crate::Unity::Mathematics::int4x3, i32),
                        crate::Unity::Mathematics::int4x3,
                        2usize,
                    >("op_Addition")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "op_Addition", 2usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Mathematics::int4x3 = unsafe {
            method.invoke_unchecked((), (lhs, rhs))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn op_Addition_int4x3_int4x3_0(
        lhs: crate::Unity::Mathematics::int4x3,
        rhs: crate::Unity::Mathematics::int4x3,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::int4x3> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_static_method::<
                        (
                            crate::Unity::Mathematics::int4x3,
                            crate::Unity::Mathematics::int4x3,
                        ),
                        crate::Unity::Mathematics::int4x3,
                        2usize,
                    >("op_Addition")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "op_Addition", 2usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Mathematics::int4x3 = unsafe {
            method.invoke_unchecked((), (lhs, rhs))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn op_BitwiseAnd_i32_int4x3_2(
        lhs: i32,
        rhs: crate::Unity::Mathematics::int4x3,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::int4x3> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_static_method::<
                        (i32, crate::Unity::Mathematics::int4x3),
                        crate::Unity::Mathematics::int4x3,
                        2usize,
                    >("op_BitwiseAnd")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "op_BitwiseAnd", 2usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Mathematics::int4x3 = unsafe {
            method.invoke_unchecked((), (lhs, rhs))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn op_BitwiseAnd_int4x3_i32_1(
        lhs: crate::Unity::Mathematics::int4x3,
        rhs: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::int4x3> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_static_method::<
                        (crate::Unity::Mathematics::int4x3, i32),
                        crate::Unity::Mathematics::int4x3,
                        2usize,
                    >("op_BitwiseAnd")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "op_BitwiseAnd", 2usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Mathematics::int4x3 = unsafe {
            method.invoke_unchecked((), (lhs, rhs))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn op_BitwiseAnd_int4x3_int4x3_0(
        lhs: crate::Unity::Mathematics::int4x3,
        rhs: crate::Unity::Mathematics::int4x3,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::int4x3> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_static_method::<
                        (
                            crate::Unity::Mathematics::int4x3,
                            crate::Unity::Mathematics::int4x3,
                        ),
                        crate::Unity::Mathematics::int4x3,
                        2usize,
                    >("op_BitwiseAnd")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "op_BitwiseAnd", 2usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Mathematics::int4x3 = unsafe {
            method.invoke_unchecked((), (lhs, rhs))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn op_BitwiseOr_i32_int4x3_2(
        lhs: i32,
        rhs: crate::Unity::Mathematics::int4x3,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::int4x3> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_static_method::<
                        (i32, crate::Unity::Mathematics::int4x3),
                        crate::Unity::Mathematics::int4x3,
                        2usize,
                    >("op_BitwiseOr")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "op_BitwiseOr", 2usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Mathematics::int4x3 = unsafe {
            method.invoke_unchecked((), (lhs, rhs))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn op_BitwiseOr_int4x3_i32_1(
        lhs: crate::Unity::Mathematics::int4x3,
        rhs: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::int4x3> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_static_method::<
                        (crate::Unity::Mathematics::int4x3, i32),
                        crate::Unity::Mathematics::int4x3,
                        2usize,
                    >("op_BitwiseOr")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "op_BitwiseOr", 2usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Mathematics::int4x3 = unsafe {
            method.invoke_unchecked((), (lhs, rhs))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn op_BitwiseOr_int4x3_int4x3_0(
        lhs: crate::Unity::Mathematics::int4x3,
        rhs: crate::Unity::Mathematics::int4x3,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::int4x3> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_static_method::<
                        (
                            crate::Unity::Mathematics::int4x3,
                            crate::Unity::Mathematics::int4x3,
                        ),
                        crate::Unity::Mathematics::int4x3,
                        2usize,
                    >("op_BitwiseOr")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "op_BitwiseOr", 2usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Mathematics::int4x3 = unsafe {
            method.invoke_unchecked((), (lhs, rhs))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn op_Decrement(
        val: crate::Unity::Mathematics::int4x3,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::int4x3> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_static_method::<
                        (crate::Unity::Mathematics::int4x3),
                        crate::Unity::Mathematics::int4x3,
                        1usize,
                    >("op_Decrement")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "op_Decrement", 1usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Mathematics::int4x3 = unsafe {
            method.invoke_unchecked((), (val))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn op_Division_i32_int4x3_2(
        lhs: i32,
        rhs: crate::Unity::Mathematics::int4x3,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::int4x3> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_static_method::<
                        (i32, crate::Unity::Mathematics::int4x3),
                        crate::Unity::Mathematics::int4x3,
                        2usize,
                    >("op_Division")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "op_Division", 2usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Mathematics::int4x3 = unsafe {
            method.invoke_unchecked((), (lhs, rhs))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn op_Division_int4x3_i32_1(
        lhs: crate::Unity::Mathematics::int4x3,
        rhs: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::int4x3> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_static_method::<
                        (crate::Unity::Mathematics::int4x3, i32),
                        crate::Unity::Mathematics::int4x3,
                        2usize,
                    >("op_Division")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "op_Division", 2usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Mathematics::int4x3 = unsafe {
            method.invoke_unchecked((), (lhs, rhs))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn op_Division_int4x3_int4x3_0(
        lhs: crate::Unity::Mathematics::int4x3,
        rhs: crate::Unity::Mathematics::int4x3,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::int4x3> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_static_method::<
                        (
                            crate::Unity::Mathematics::int4x3,
                            crate::Unity::Mathematics::int4x3,
                        ),
                        crate::Unity::Mathematics::int4x3,
                        2usize,
                    >("op_Division")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "op_Division", 2usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Mathematics::int4x3 = unsafe {
            method.invoke_unchecked((), (lhs, rhs))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn op_Equality_i32_int4x3_2(
        lhs: i32,
        rhs: crate::Unity::Mathematics::int4x3,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::bool4x3> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_static_method::<
                        (i32, crate::Unity::Mathematics::int4x3),
                        crate::Unity::Mathematics::bool4x3,
                        2usize,
                    >("op_Equality")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "op_Equality", 2usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Mathematics::bool4x3 = unsafe {
            method.invoke_unchecked((), (lhs, rhs))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn op_Equality_int4x3_i32_1(
        lhs: crate::Unity::Mathematics::int4x3,
        rhs: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::bool4x3> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_static_method::<
                        (crate::Unity::Mathematics::int4x3, i32),
                        crate::Unity::Mathematics::bool4x3,
                        2usize,
                    >("op_Equality")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "op_Equality", 2usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Mathematics::bool4x3 = unsafe {
            method.invoke_unchecked((), (lhs, rhs))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn op_Equality_int4x3_int4x3_0(
        lhs: crate::Unity::Mathematics::int4x3,
        rhs: crate::Unity::Mathematics::int4x3,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::bool4x3> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_static_method::<
                        (
                            crate::Unity::Mathematics::int4x3,
                            crate::Unity::Mathematics::int4x3,
                        ),
                        crate::Unity::Mathematics::bool4x3,
                        2usize,
                    >("op_Equality")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "op_Equality", 2usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Mathematics::bool4x3 = unsafe {
            method.invoke_unchecked((), (lhs, rhs))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn op_ExclusiveOr_i32_int4x3_2(
        lhs: i32,
        rhs: crate::Unity::Mathematics::int4x3,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::int4x3> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_static_method::<
                        (i32, crate::Unity::Mathematics::int4x3),
                        crate::Unity::Mathematics::int4x3,
                        2usize,
                    >("op_ExclusiveOr")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "op_ExclusiveOr", 2usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Mathematics::int4x3 = unsafe {
            method.invoke_unchecked((), (lhs, rhs))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn op_ExclusiveOr_int4x3_i32_1(
        lhs: crate::Unity::Mathematics::int4x3,
        rhs: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::int4x3> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_static_method::<
                        (crate::Unity::Mathematics::int4x3, i32),
                        crate::Unity::Mathematics::int4x3,
                        2usize,
                    >("op_ExclusiveOr")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "op_ExclusiveOr", 2usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Mathematics::int4x3 = unsafe {
            method.invoke_unchecked((), (lhs, rhs))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn op_ExclusiveOr_int4x3_int4x3_0(
        lhs: crate::Unity::Mathematics::int4x3,
        rhs: crate::Unity::Mathematics::int4x3,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::int4x3> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_static_method::<
                        (
                            crate::Unity::Mathematics::int4x3,
                            crate::Unity::Mathematics::int4x3,
                        ),
                        crate::Unity::Mathematics::int4x3,
                        2usize,
                    >("op_ExclusiveOr")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "op_ExclusiveOr", 2usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Mathematics::int4x3 = unsafe {
            method.invoke_unchecked((), (lhs, rhs))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn op_Explicit__cordl_bool0(
        v: bool,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::int4x3> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_static_method::<
                        (bool),
                        crate::Unity::Mathematics::int4x3,
                        1usize,
                    >("op_Explicit")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "op_Explicit", 1usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Mathematics::int4x3 = unsafe {
            method.invoke_unchecked((), (v))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn op_Explicit_bool4x3_1(
        v: crate::Unity::Mathematics::bool4x3,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::int4x3> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_static_method::<
                        (crate::Unity::Mathematics::bool4x3),
                        crate::Unity::Mathematics::int4x3,
                        1usize,
                    >("op_Explicit")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "op_Explicit", 1usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Mathematics::int4x3 = unsafe {
            method.invoke_unchecked((), (v))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn op_Explicit_double4x3_7(
        v: crate::Unity::Mathematics::double4x3,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::int4x3> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_static_method::<
                        (crate::Unity::Mathematics::double4x3),
                        crate::Unity::Mathematics::int4x3,
                        1usize,
                    >("op_Explicit")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "op_Explicit", 1usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Mathematics::int4x3 = unsafe {
            method.invoke_unchecked((), (v))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn op_Explicit_f32_4(
        v: f32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::int4x3> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_static_method::<
                        (f32),
                        crate::Unity::Mathematics::int4x3,
                        1usize,
                    >("op_Explicit")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "op_Explicit", 1usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Mathematics::int4x3 = unsafe {
            method.invoke_unchecked((), (v))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn op_Explicit_f64_6(
        v: f64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::int4x3> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_static_method::<
                        (f64),
                        crate::Unity::Mathematics::int4x3,
                        1usize,
                    >("op_Explicit")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "op_Explicit", 1usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Mathematics::int4x3 = unsafe {
            method.invoke_unchecked((), (v))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn op_Explicit_float4x3_5(
        v: crate::Unity::Mathematics::float4x3,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::int4x3> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_static_method::<
                        (crate::Unity::Mathematics::float4x3),
                        crate::Unity::Mathematics::int4x3,
                        1usize,
                    >("op_Explicit")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "op_Explicit", 1usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Mathematics::int4x3 = unsafe {
            method.invoke_unchecked((), (v))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn op_Explicit_u32_2(
        v: u32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::int4x3> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_static_method::<
                        (u32),
                        crate::Unity::Mathematics::int4x3,
                        1usize,
                    >("op_Explicit")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "op_Explicit", 1usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Mathematics::int4x3 = unsafe {
            method.invoke_unchecked((), (v))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn op_Explicit_uint4x3_3(
        v: crate::Unity::Mathematics::uint4x3,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::int4x3> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_static_method::<
                        (crate::Unity::Mathematics::uint4x3),
                        crate::Unity::Mathematics::int4x3,
                        1usize,
                    >("op_Explicit")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "op_Explicit", 1usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Mathematics::int4x3 = unsafe {
            method.invoke_unchecked((), (v))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn op_GreaterThanOrEqual_i32_int4x3_2(
        lhs: i32,
        rhs: crate::Unity::Mathematics::int4x3,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::bool4x3> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_static_method::<
                        (i32, crate::Unity::Mathematics::int4x3),
                        crate::Unity::Mathematics::bool4x3,
                        2usize,
                    >("op_GreaterThanOrEqual")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "op_GreaterThanOrEqual", 2usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Mathematics::bool4x3 = unsafe {
            method.invoke_unchecked((), (lhs, rhs))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn op_GreaterThanOrEqual_int4x3_i32_1(
        lhs: crate::Unity::Mathematics::int4x3,
        rhs: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::bool4x3> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_static_method::<
                        (crate::Unity::Mathematics::int4x3, i32),
                        crate::Unity::Mathematics::bool4x3,
                        2usize,
                    >("op_GreaterThanOrEqual")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "op_GreaterThanOrEqual", 2usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Mathematics::bool4x3 = unsafe {
            method.invoke_unchecked((), (lhs, rhs))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn op_GreaterThanOrEqual_int4x3_int4x3_0(
        lhs: crate::Unity::Mathematics::int4x3,
        rhs: crate::Unity::Mathematics::int4x3,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::bool4x3> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_static_method::<
                        (
                            crate::Unity::Mathematics::int4x3,
                            crate::Unity::Mathematics::int4x3,
                        ),
                        crate::Unity::Mathematics::bool4x3,
                        2usize,
                    >("op_GreaterThanOrEqual")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "op_GreaterThanOrEqual", 2usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Mathematics::bool4x3 = unsafe {
            method.invoke_unchecked((), (lhs, rhs))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn op_GreaterThan_i32_int4x3_2(
        lhs: i32,
        rhs: crate::Unity::Mathematics::int4x3,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::bool4x3> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_static_method::<
                        (i32, crate::Unity::Mathematics::int4x3),
                        crate::Unity::Mathematics::bool4x3,
                        2usize,
                    >("op_GreaterThan")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "op_GreaterThan", 2usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Mathematics::bool4x3 = unsafe {
            method.invoke_unchecked((), (lhs, rhs))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn op_GreaterThan_int4x3_i32_1(
        lhs: crate::Unity::Mathematics::int4x3,
        rhs: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::bool4x3> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_static_method::<
                        (crate::Unity::Mathematics::int4x3, i32),
                        crate::Unity::Mathematics::bool4x3,
                        2usize,
                    >("op_GreaterThan")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "op_GreaterThan", 2usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Mathematics::bool4x3 = unsafe {
            method.invoke_unchecked((), (lhs, rhs))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn op_GreaterThan_int4x3_int4x3_0(
        lhs: crate::Unity::Mathematics::int4x3,
        rhs: crate::Unity::Mathematics::int4x3,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::bool4x3> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_static_method::<
                        (
                            crate::Unity::Mathematics::int4x3,
                            crate::Unity::Mathematics::int4x3,
                        ),
                        crate::Unity::Mathematics::bool4x3,
                        2usize,
                    >("op_GreaterThan")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "op_GreaterThan", 2usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Mathematics::bool4x3 = unsafe {
            method.invoke_unchecked((), (lhs, rhs))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn op_Implicit(
        v: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::int4x3> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_static_method::<
                        (i32),
                        crate::Unity::Mathematics::int4x3,
                        1usize,
                    >("op_Implicit")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "op_Implicit", 1usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Mathematics::int4x3 = unsafe {
            method.invoke_unchecked((), (v))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn op_Increment(
        val: crate::Unity::Mathematics::int4x3,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::int4x3> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_static_method::<
                        (crate::Unity::Mathematics::int4x3),
                        crate::Unity::Mathematics::int4x3,
                        1usize,
                    >("op_Increment")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "op_Increment", 1usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Mathematics::int4x3 = unsafe {
            method.invoke_unchecked((), (val))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn op_Inequality_i32_int4x3_2(
        lhs: i32,
        rhs: crate::Unity::Mathematics::int4x3,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::bool4x3> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_static_method::<
                        (i32, crate::Unity::Mathematics::int4x3),
                        crate::Unity::Mathematics::bool4x3,
                        2usize,
                    >("op_Inequality")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "op_Inequality", 2usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Mathematics::bool4x3 = unsafe {
            method.invoke_unchecked((), (lhs, rhs))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn op_Inequality_int4x3_i32_1(
        lhs: crate::Unity::Mathematics::int4x3,
        rhs: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::bool4x3> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_static_method::<
                        (crate::Unity::Mathematics::int4x3, i32),
                        crate::Unity::Mathematics::bool4x3,
                        2usize,
                    >("op_Inequality")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "op_Inequality", 2usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Mathematics::bool4x3 = unsafe {
            method.invoke_unchecked((), (lhs, rhs))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn op_Inequality_int4x3_int4x3_0(
        lhs: crate::Unity::Mathematics::int4x3,
        rhs: crate::Unity::Mathematics::int4x3,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::bool4x3> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_static_method::<
                        (
                            crate::Unity::Mathematics::int4x3,
                            crate::Unity::Mathematics::int4x3,
                        ),
                        crate::Unity::Mathematics::bool4x3,
                        2usize,
                    >("op_Inequality")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "op_Inequality", 2usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Mathematics::bool4x3 = unsafe {
            method.invoke_unchecked((), (lhs, rhs))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn op_LeftShift(
        x: crate::Unity::Mathematics::int4x3,
        n: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::int4x3> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_static_method::<
                        (crate::Unity::Mathematics::int4x3, i32),
                        crate::Unity::Mathematics::int4x3,
                        2usize,
                    >("op_LeftShift")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "op_LeftShift", 2usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Mathematics::int4x3 = unsafe {
            method.invoke_unchecked((), (x, n))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn op_LessThanOrEqual_i32_int4x3_2(
        lhs: i32,
        rhs: crate::Unity::Mathematics::int4x3,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::bool4x3> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_static_method::<
                        (i32, crate::Unity::Mathematics::int4x3),
                        crate::Unity::Mathematics::bool4x3,
                        2usize,
                    >("op_LessThanOrEqual")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "op_LessThanOrEqual", 2usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Mathematics::bool4x3 = unsafe {
            method.invoke_unchecked((), (lhs, rhs))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn op_LessThanOrEqual_int4x3_i32_1(
        lhs: crate::Unity::Mathematics::int4x3,
        rhs: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::bool4x3> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_static_method::<
                        (crate::Unity::Mathematics::int4x3, i32),
                        crate::Unity::Mathematics::bool4x3,
                        2usize,
                    >("op_LessThanOrEqual")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "op_LessThanOrEqual", 2usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Mathematics::bool4x3 = unsafe {
            method.invoke_unchecked((), (lhs, rhs))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn op_LessThanOrEqual_int4x3_int4x3_0(
        lhs: crate::Unity::Mathematics::int4x3,
        rhs: crate::Unity::Mathematics::int4x3,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::bool4x3> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_static_method::<
                        (
                            crate::Unity::Mathematics::int4x3,
                            crate::Unity::Mathematics::int4x3,
                        ),
                        crate::Unity::Mathematics::bool4x3,
                        2usize,
                    >("op_LessThanOrEqual")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "op_LessThanOrEqual", 2usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Mathematics::bool4x3 = unsafe {
            method.invoke_unchecked((), (lhs, rhs))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn op_LessThan_i32_int4x3_2(
        lhs: i32,
        rhs: crate::Unity::Mathematics::int4x3,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::bool4x3> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_static_method::<
                        (i32, crate::Unity::Mathematics::int4x3),
                        crate::Unity::Mathematics::bool4x3,
                        2usize,
                    >("op_LessThan")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "op_LessThan", 2usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Mathematics::bool4x3 = unsafe {
            method.invoke_unchecked((), (lhs, rhs))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn op_LessThan_int4x3_i32_1(
        lhs: crate::Unity::Mathematics::int4x3,
        rhs: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::bool4x3> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_static_method::<
                        (crate::Unity::Mathematics::int4x3, i32),
                        crate::Unity::Mathematics::bool4x3,
                        2usize,
                    >("op_LessThan")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "op_LessThan", 2usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Mathematics::bool4x3 = unsafe {
            method.invoke_unchecked((), (lhs, rhs))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn op_LessThan_int4x3_int4x3_0(
        lhs: crate::Unity::Mathematics::int4x3,
        rhs: crate::Unity::Mathematics::int4x3,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::bool4x3> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_static_method::<
                        (
                            crate::Unity::Mathematics::int4x3,
                            crate::Unity::Mathematics::int4x3,
                        ),
                        crate::Unity::Mathematics::bool4x3,
                        2usize,
                    >("op_LessThan")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "op_LessThan", 2usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Mathematics::bool4x3 = unsafe {
            method.invoke_unchecked((), (lhs, rhs))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn op_Modulus_i32_int4x3_2(
        lhs: i32,
        rhs: crate::Unity::Mathematics::int4x3,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::int4x3> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_static_method::<
                        (i32, crate::Unity::Mathematics::int4x3),
                        crate::Unity::Mathematics::int4x3,
                        2usize,
                    >("op_Modulus")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "op_Modulus", 2usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Mathematics::int4x3 = unsafe {
            method.invoke_unchecked((), (lhs, rhs))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn op_Modulus_int4x3_i32_1(
        lhs: crate::Unity::Mathematics::int4x3,
        rhs: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::int4x3> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_static_method::<
                        (crate::Unity::Mathematics::int4x3, i32),
                        crate::Unity::Mathematics::int4x3,
                        2usize,
                    >("op_Modulus")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "op_Modulus", 2usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Mathematics::int4x3 = unsafe {
            method.invoke_unchecked((), (lhs, rhs))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn op_Modulus_int4x3_int4x3_0(
        lhs: crate::Unity::Mathematics::int4x3,
        rhs: crate::Unity::Mathematics::int4x3,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::int4x3> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_static_method::<
                        (
                            crate::Unity::Mathematics::int4x3,
                            crate::Unity::Mathematics::int4x3,
                        ),
                        crate::Unity::Mathematics::int4x3,
                        2usize,
                    >("op_Modulus")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "op_Modulus", 2usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Mathematics::int4x3 = unsafe {
            method.invoke_unchecked((), (lhs, rhs))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn op_Multiply_i32_int4x3_2(
        lhs: i32,
        rhs: crate::Unity::Mathematics::int4x3,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::int4x3> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_static_method::<
                        (i32, crate::Unity::Mathematics::int4x3),
                        crate::Unity::Mathematics::int4x3,
                        2usize,
                    >("op_Multiply")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "op_Multiply", 2usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Mathematics::int4x3 = unsafe {
            method.invoke_unchecked((), (lhs, rhs))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn op_Multiply_int4x3_i32_1(
        lhs: crate::Unity::Mathematics::int4x3,
        rhs: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::int4x3> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_static_method::<
                        (crate::Unity::Mathematics::int4x3, i32),
                        crate::Unity::Mathematics::int4x3,
                        2usize,
                    >("op_Multiply")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "op_Multiply", 2usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Mathematics::int4x3 = unsafe {
            method.invoke_unchecked((), (lhs, rhs))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn op_Multiply_int4x3_int4x3_0(
        lhs: crate::Unity::Mathematics::int4x3,
        rhs: crate::Unity::Mathematics::int4x3,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::int4x3> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_static_method::<
                        (
                            crate::Unity::Mathematics::int4x3,
                            crate::Unity::Mathematics::int4x3,
                        ),
                        crate::Unity::Mathematics::int4x3,
                        2usize,
                    >("op_Multiply")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "op_Multiply", 2usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Mathematics::int4x3 = unsafe {
            method.invoke_unchecked((), (lhs, rhs))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn op_OnesComplement(
        val: crate::Unity::Mathematics::int4x3,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::int4x3> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_static_method::<
                        (crate::Unity::Mathematics::int4x3),
                        crate::Unity::Mathematics::int4x3,
                        1usize,
                    >("op_OnesComplement")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "op_OnesComplement", 1usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Mathematics::int4x3 = unsafe {
            method.invoke_unchecked((), (val))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn op_RightShift(
        x: crate::Unity::Mathematics::int4x3,
        n: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::int4x3> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_static_method::<
                        (crate::Unity::Mathematics::int4x3, i32),
                        crate::Unity::Mathematics::int4x3,
                        2usize,
                    >("op_RightShift")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "op_RightShift", 2usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Mathematics::int4x3 = unsafe {
            method.invoke_unchecked((), (x, n))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn op_Subtraction_i32_int4x3_2(
        lhs: i32,
        rhs: crate::Unity::Mathematics::int4x3,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::int4x3> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_static_method::<
                        (i32, crate::Unity::Mathematics::int4x3),
                        crate::Unity::Mathematics::int4x3,
                        2usize,
                    >("op_Subtraction")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "op_Subtraction", 2usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Mathematics::int4x3 = unsafe {
            method.invoke_unchecked((), (lhs, rhs))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn op_Subtraction_int4x3_i32_1(
        lhs: crate::Unity::Mathematics::int4x3,
        rhs: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::int4x3> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_static_method::<
                        (crate::Unity::Mathematics::int4x3, i32),
                        crate::Unity::Mathematics::int4x3,
                        2usize,
                    >("op_Subtraction")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "op_Subtraction", 2usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Mathematics::int4x3 = unsafe {
            method.invoke_unchecked((), (lhs, rhs))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn op_Subtraction_int4x3_int4x3_0(
        lhs: crate::Unity::Mathematics::int4x3,
        rhs: crate::Unity::Mathematics::int4x3,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::int4x3> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_static_method::<
                        (
                            crate::Unity::Mathematics::int4x3,
                            crate::Unity::Mathematics::int4x3,
                        ),
                        crate::Unity::Mathematics::int4x3,
                        2usize,
                    >("op_Subtraction")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "op_Subtraction", 2usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Mathematics::int4x3 = unsafe {
            method.invoke_unchecked((), (lhs, rhs))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn op_UnaryNegation(
        val: crate::Unity::Mathematics::int4x3,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::int4x3> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_static_method::<
                        (crate::Unity::Mathematics::int4x3),
                        crate::Unity::Mathematics::int4x3,
                        1usize,
                    >("op_UnaryNegation")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "op_UnaryNegation", 1usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Mathematics::int4x3 = unsafe {
            method.invoke_unchecked((), (val))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn op_UnaryPlus(
        val: crate::Unity::Mathematics::int4x3,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::int4x3> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_static_method::<
                        (crate::Unity::Mathematics::int4x3),
                        crate::Unity::Mathematics::int4x3,
                        1usize,
                    >("op_UnaryPlus")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "op_UnaryPlus", 1usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Mathematics::int4x3 = unsafe {
            method.invoke_unchecked((), (val))?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Unity+Mathematics+int4x3")]
impl AsRef<crate::System::IEquatable_1<crate::Unity::Mathematics::int4x3>>
for crate::Unity::Mathematics::int4x3 {
    fn as_ref(&self) -> &crate::System::IEquatable_1<crate::Unity::Mathematics::int4x3> {
        todo!()
    }
}
#[cfg(feature = "Unity+Mathematics+int4x3")]
impl AsMut<crate::System::IEquatable_1<crate::Unity::Mathematics::int4x3>>
for crate::Unity::Mathematics::int4x3 {
    fn as_mut(
        &mut self,
    ) -> &mut crate::System::IEquatable_1<crate::Unity::Mathematics::int4x3> {
        todo!()
    }
}
#[cfg(feature = "Unity+Mathematics+int4x3")]
impl AsRef<crate::System::IFormattable> for crate::Unity::Mathematics::int4x3 {
    fn as_ref(&self) -> &crate::System::IFormattable {
        todo!()
    }
}
#[cfg(feature = "Unity+Mathematics+int4x3")]
impl AsMut<crate::System::IFormattable> for crate::Unity::Mathematics::int4x3 {
    fn as_mut(&mut self) -> &mut crate::System::IFormattable {
        todo!()
    }
}
