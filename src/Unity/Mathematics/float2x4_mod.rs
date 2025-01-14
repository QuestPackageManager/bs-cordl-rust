#[cfg(feature = "Unity+Mathematics+float2x4")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct float2x4 {
    pub c0: crate::Unity::Mathematics::float2,
    pub c1: crate::Unity::Mathematics::float2,
    pub c2: crate::Unity::Mathematics::float2,
    pub c3: crate::Unity::Mathematics::float2,
}
#[cfg(feature = "Unity+Mathematics+float2x4")]
unsafe impl quest_hook::libil2cpp::Type for crate::Unity::Mathematics::float2x4 {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "Unity.Mathematics";
    const CLASS_NAME: &'static str = "float2x4";
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
#[cfg(feature = "Unity+Mathematics+float2x4")]
unsafe impl quest_hook::libil2cpp::Argument for crate::Unity::Mathematics::float2x4 {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "Unity+Mathematics+float2x4")]
unsafe impl quest_hook::libil2cpp::Parameter for crate::Unity::Mathematics::float2x4 {
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
#[cfg(feature = "Unity+Mathematics+float2x4")]
unsafe impl quest_hook::libil2cpp::Returned for crate::Unity::Mathematics::float2x4 {
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
#[cfg(feature = "Unity+Mathematics+float2x4")]
unsafe impl quest_hook::libil2cpp::Return for crate::Unity::Mathematics::float2x4 {
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
#[cfg(feature = "Unity+Mathematics+float2x4")]
unsafe impl quest_hook::libil2cpp::ThisArgument for crate::Unity::Mathematics::float2x4 {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "Unity+Mathematics+float2x4")]
impl crate::Unity::Mathematics::float2x4 {
    pub fn Equals_Il2CppObject1(
        &mut self,
        o: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>),
                bool,
                1usize,
            >("Equals")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "Equals", 1usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked(self, (o)) };
        Ok(__cordl_ret.into())
    }
    pub fn Equals_float2x4_0(
        &mut self,
        rhs: crate::Unity::Mathematics::float2x4,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<(crate::Unity::Mathematics::float2x4), bool, 1usize>("Equals")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "Equals", 1usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked(self, (rhs)) };
        Ok(__cordl_ret.into())
    }
    pub fn GetHashCode(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), i32, 0usize>("GetHashCode")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "GetHashCode", 0usize
                )
            });
        let __cordl_ret: i32 = unsafe { method.invoke_unchecked(self, ()) };
        Ok(__cordl_ret.into())
    }
    pub fn ToString_0(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                0usize,
            >("ToString")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ToString", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = unsafe { method.invoke_unchecked(self, ()) };
        Ok(__cordl_ret.into())
    }
    pub fn ToString_Il2CppString_IFormatProvider1(
        &mut self,
        format: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        formatProvider: quest_hook::libil2cpp::Gc<crate::System::IFormatProvider>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                    quest_hook::libil2cpp::Gc<crate::System::IFormatProvider>,
                ),
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                2usize,
            >("ToString")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ToString", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = unsafe { method.invoke_unchecked(self, (format, formatProvider)) };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor__cordl_bool3(
        &mut self,
        v: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<(bool), quest_hook::libil2cpp::Void, 1usize>(".ctor")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, ".ctor", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (v))
        };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_bool2x4_4(
        &mut self,
        v: crate::Unity::Mathematics::bool2x4,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (crate::Unity::Mathematics::bool2x4),
                quest_hook::libil2cpp::Void,
                1usize,
            >(".ctor")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, ".ctor", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (v))
        };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_double2x4_10(
        &mut self,
        v: crate::Unity::Mathematics::double2x4,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (crate::Unity::Mathematics::double2x4),
                quest_hook::libil2cpp::Void,
                1usize,
            >(".ctor")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, ".ctor", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (v))
        };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_f32_2(
        &mut self,
        v: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<(f32), quest_hook::libil2cpp::Void, 1usize>(".ctor")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, ".ctor", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (v))
        };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_f32_f32_f32_f32_f32_f32_f32_f32_1(
        &mut self,
        m00: f32,
        m01: f32,
        m02: f32,
        m03: f32,
        m10: f32,
        m11: f32,
        m12: f32,
        m13: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (f32, f32, f32, f32, f32, f32, f32, f32),
                quest_hook::libil2cpp::Void,
                8usize,
            >(".ctor")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, ".ctor", 8usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (m00, m01, m02, m03, m10, m11, m12, m13))
        };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_f64_9(
        &mut self,
        v: f64,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<(f64), quest_hook::libil2cpp::Void, 1usize>(".ctor")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, ".ctor", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (v))
        };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_float2_float2_float2_float2_0(
        &mut self,
        c0: crate::Unity::Mathematics::float2,
        c1: crate::Unity::Mathematics::float2,
        c2: crate::Unity::Mathematics::float2,
        c3: crate::Unity::Mathematics::float2,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    crate::Unity::Mathematics::float2,
                    crate::Unity::Mathematics::float2,
                    crate::Unity::Mathematics::float2,
                    crate::Unity::Mathematics::float2,
                ),
                quest_hook::libil2cpp::Void,
                4usize,
            >(".ctor")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, ".ctor", 4usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (c0, c1, c2, c3))
        };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_i32_5(
        &mut self,
        v: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<(i32), quest_hook::libil2cpp::Void, 1usize>(".ctor")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, ".ctor", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (v))
        };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_int2x4_6(
        &mut self,
        v: crate::Unity::Mathematics::int2x4,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (crate::Unity::Mathematics::int2x4),
                quest_hook::libil2cpp::Void,
                1usize,
            >(".ctor")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, ".ctor", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (v))
        };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_u32_7(
        &mut self,
        v: u32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<(u32), quest_hook::libil2cpp::Void, 1usize>(".ctor")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, ".ctor", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (v))
        };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_uint2x4_8(
        &mut self,
        v: crate::Unity::Mathematics::uint2x4,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (crate::Unity::Mathematics::uint2x4),
                quest_hook::libil2cpp::Void,
                1usize,
            >(".ctor")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, ".ctor", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (v))
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_Item(
        &mut self,
        index: i32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::ByRefMut<crate::Unity::Mathematics::float2>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (i32),
                quest_hook::libil2cpp::ByRefMut<crate::Unity::Mathematics::float2>,
                1usize,
            >("get_Item")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "get_Item", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::ByRefMut<
            crate::Unity::Mathematics::float2,
        > = unsafe { method.invoke_unchecked(self, (index)) };
        Ok(__cordl_ret.into())
    }
    pub fn op_Addition_f32_float2x4_2(
        lhs: f32,
        rhs: crate::Unity::Mathematics::float2x4,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float2x4> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (f32, crate::Unity::Mathematics::float2x4),
                crate::Unity::Mathematics::float2x4,
                2usize,
            >("op_Addition")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "op_Addition", 2usize
                )
            });
        let __cordl_ret: crate::Unity::Mathematics::float2x4 = unsafe {
            method.invoke_unchecked((), (lhs, rhs))
        };
        Ok(__cordl_ret.into())
    }
    pub fn op_Addition_float2x4_f32_1(
        lhs: crate::Unity::Mathematics::float2x4,
        rhs: f32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float2x4> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::Unity::Mathematics::float2x4, f32),
                crate::Unity::Mathematics::float2x4,
                2usize,
            >("op_Addition")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "op_Addition", 2usize
                )
            });
        let __cordl_ret: crate::Unity::Mathematics::float2x4 = unsafe {
            method.invoke_unchecked((), (lhs, rhs))
        };
        Ok(__cordl_ret.into())
    }
    pub fn op_Addition_float2x4_float2x4_0(
        lhs: crate::Unity::Mathematics::float2x4,
        rhs: crate::Unity::Mathematics::float2x4,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float2x4> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::Unity::Mathematics::float2x4,
                    crate::Unity::Mathematics::float2x4,
                ),
                crate::Unity::Mathematics::float2x4,
                2usize,
            >("op_Addition")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "op_Addition", 2usize
                )
            });
        let __cordl_ret: crate::Unity::Mathematics::float2x4 = unsafe {
            method.invoke_unchecked((), (lhs, rhs))
        };
        Ok(__cordl_ret.into())
    }
    pub fn op_Decrement(
        val: crate::Unity::Mathematics::float2x4,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float2x4> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::Unity::Mathematics::float2x4),
                crate::Unity::Mathematics::float2x4,
                1usize,
            >("op_Decrement")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "op_Decrement", 1usize
                )
            });
        let __cordl_ret: crate::Unity::Mathematics::float2x4 = unsafe {
            method.invoke_unchecked((), (val))
        };
        Ok(__cordl_ret.into())
    }
    pub fn op_Division_f32_float2x4_2(
        lhs: f32,
        rhs: crate::Unity::Mathematics::float2x4,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float2x4> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (f32, crate::Unity::Mathematics::float2x4),
                crate::Unity::Mathematics::float2x4,
                2usize,
            >("op_Division")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "op_Division", 2usize
                )
            });
        let __cordl_ret: crate::Unity::Mathematics::float2x4 = unsafe {
            method.invoke_unchecked((), (lhs, rhs))
        };
        Ok(__cordl_ret.into())
    }
    pub fn op_Division_float2x4_f32_1(
        lhs: crate::Unity::Mathematics::float2x4,
        rhs: f32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float2x4> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::Unity::Mathematics::float2x4, f32),
                crate::Unity::Mathematics::float2x4,
                2usize,
            >("op_Division")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "op_Division", 2usize
                )
            });
        let __cordl_ret: crate::Unity::Mathematics::float2x4 = unsafe {
            method.invoke_unchecked((), (lhs, rhs))
        };
        Ok(__cordl_ret.into())
    }
    pub fn op_Division_float2x4_float2x4_0(
        lhs: crate::Unity::Mathematics::float2x4,
        rhs: crate::Unity::Mathematics::float2x4,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float2x4> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::Unity::Mathematics::float2x4,
                    crate::Unity::Mathematics::float2x4,
                ),
                crate::Unity::Mathematics::float2x4,
                2usize,
            >("op_Division")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "op_Division", 2usize
                )
            });
        let __cordl_ret: crate::Unity::Mathematics::float2x4 = unsafe {
            method.invoke_unchecked((), (lhs, rhs))
        };
        Ok(__cordl_ret.into())
    }
    pub fn op_Equality_f32_float2x4_2(
        lhs: f32,
        rhs: crate::Unity::Mathematics::float2x4,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::bool2x4> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (f32, crate::Unity::Mathematics::float2x4),
                crate::Unity::Mathematics::bool2x4,
                2usize,
            >("op_Equality")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "op_Equality", 2usize
                )
            });
        let __cordl_ret: crate::Unity::Mathematics::bool2x4 = unsafe {
            method.invoke_unchecked((), (lhs, rhs))
        };
        Ok(__cordl_ret.into())
    }
    pub fn op_Equality_float2x4_f32_1(
        lhs: crate::Unity::Mathematics::float2x4,
        rhs: f32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::bool2x4> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::Unity::Mathematics::float2x4, f32),
                crate::Unity::Mathematics::bool2x4,
                2usize,
            >("op_Equality")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "op_Equality", 2usize
                )
            });
        let __cordl_ret: crate::Unity::Mathematics::bool2x4 = unsafe {
            method.invoke_unchecked((), (lhs, rhs))
        };
        Ok(__cordl_ret.into())
    }
    pub fn op_Equality_float2x4_float2x4_0(
        lhs: crate::Unity::Mathematics::float2x4,
        rhs: crate::Unity::Mathematics::float2x4,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::bool2x4> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::Unity::Mathematics::float2x4,
                    crate::Unity::Mathematics::float2x4,
                ),
                crate::Unity::Mathematics::bool2x4,
                2usize,
            >("op_Equality")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "op_Equality", 2usize
                )
            });
        let __cordl_ret: crate::Unity::Mathematics::bool2x4 = unsafe {
            method.invoke_unchecked((), (lhs, rhs))
        };
        Ok(__cordl_ret.into())
    }
    pub fn op_Explicit__cordl_bool0(
        v: bool,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float2x4> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (bool),
                crate::Unity::Mathematics::float2x4,
                1usize,
            >("op_Explicit")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "op_Explicit", 1usize
                )
            });
        let __cordl_ret: crate::Unity::Mathematics::float2x4 = unsafe {
            method.invoke_unchecked((), (v))
        };
        Ok(__cordl_ret.into())
    }
    pub fn op_Explicit_bool2x4_1(
        v: crate::Unity::Mathematics::bool2x4,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float2x4> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::Unity::Mathematics::bool2x4),
                crate::Unity::Mathematics::float2x4,
                1usize,
            >("op_Explicit")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "op_Explicit", 1usize
                )
            });
        let __cordl_ret: crate::Unity::Mathematics::float2x4 = unsafe {
            method.invoke_unchecked((), (v))
        };
        Ok(__cordl_ret.into())
    }
    pub fn op_Explicit_double2x4_3(
        v: crate::Unity::Mathematics::double2x4,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float2x4> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::Unity::Mathematics::double2x4),
                crate::Unity::Mathematics::float2x4,
                1usize,
            >("op_Explicit")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "op_Explicit", 1usize
                )
            });
        let __cordl_ret: crate::Unity::Mathematics::float2x4 = unsafe {
            method.invoke_unchecked((), (v))
        };
        Ok(__cordl_ret.into())
    }
    pub fn op_Explicit_f64_2(
        v: f64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float2x4> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (f64),
                crate::Unity::Mathematics::float2x4,
                1usize,
            >("op_Explicit")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "op_Explicit", 1usize
                )
            });
        let __cordl_ret: crate::Unity::Mathematics::float2x4 = unsafe {
            method.invoke_unchecked((), (v))
        };
        Ok(__cordl_ret.into())
    }
    pub fn op_GreaterThanOrEqual_f32_float2x4_2(
        lhs: f32,
        rhs: crate::Unity::Mathematics::float2x4,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::bool2x4> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (f32, crate::Unity::Mathematics::float2x4),
                crate::Unity::Mathematics::bool2x4,
                2usize,
            >("op_GreaterThanOrEqual")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "op_GreaterThanOrEqual", 2usize
                )
            });
        let __cordl_ret: crate::Unity::Mathematics::bool2x4 = unsafe {
            method.invoke_unchecked((), (lhs, rhs))
        };
        Ok(__cordl_ret.into())
    }
    pub fn op_GreaterThanOrEqual_float2x4_f32_1(
        lhs: crate::Unity::Mathematics::float2x4,
        rhs: f32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::bool2x4> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::Unity::Mathematics::float2x4, f32),
                crate::Unity::Mathematics::bool2x4,
                2usize,
            >("op_GreaterThanOrEqual")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "op_GreaterThanOrEqual", 2usize
                )
            });
        let __cordl_ret: crate::Unity::Mathematics::bool2x4 = unsafe {
            method.invoke_unchecked((), (lhs, rhs))
        };
        Ok(__cordl_ret.into())
    }
    pub fn op_GreaterThanOrEqual_float2x4_float2x4_0(
        lhs: crate::Unity::Mathematics::float2x4,
        rhs: crate::Unity::Mathematics::float2x4,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::bool2x4> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::Unity::Mathematics::float2x4,
                    crate::Unity::Mathematics::float2x4,
                ),
                crate::Unity::Mathematics::bool2x4,
                2usize,
            >("op_GreaterThanOrEqual")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "op_GreaterThanOrEqual", 2usize
                )
            });
        let __cordl_ret: crate::Unity::Mathematics::bool2x4 = unsafe {
            method.invoke_unchecked((), (lhs, rhs))
        };
        Ok(__cordl_ret.into())
    }
    pub fn op_GreaterThan_f32_float2x4_2(
        lhs: f32,
        rhs: crate::Unity::Mathematics::float2x4,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::bool2x4> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (f32, crate::Unity::Mathematics::float2x4),
                crate::Unity::Mathematics::bool2x4,
                2usize,
            >("op_GreaterThan")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "op_GreaterThan", 2usize
                )
            });
        let __cordl_ret: crate::Unity::Mathematics::bool2x4 = unsafe {
            method.invoke_unchecked((), (lhs, rhs))
        };
        Ok(__cordl_ret.into())
    }
    pub fn op_GreaterThan_float2x4_f32_1(
        lhs: crate::Unity::Mathematics::float2x4,
        rhs: f32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::bool2x4> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::Unity::Mathematics::float2x4, f32),
                crate::Unity::Mathematics::bool2x4,
                2usize,
            >("op_GreaterThan")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "op_GreaterThan", 2usize
                )
            });
        let __cordl_ret: crate::Unity::Mathematics::bool2x4 = unsafe {
            method.invoke_unchecked((), (lhs, rhs))
        };
        Ok(__cordl_ret.into())
    }
    pub fn op_GreaterThan_float2x4_float2x4_0(
        lhs: crate::Unity::Mathematics::float2x4,
        rhs: crate::Unity::Mathematics::float2x4,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::bool2x4> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::Unity::Mathematics::float2x4,
                    crate::Unity::Mathematics::float2x4,
                ),
                crate::Unity::Mathematics::bool2x4,
                2usize,
            >("op_GreaterThan")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "op_GreaterThan", 2usize
                )
            });
        let __cordl_ret: crate::Unity::Mathematics::bool2x4 = unsafe {
            method.invoke_unchecked((), (lhs, rhs))
        };
        Ok(__cordl_ret.into())
    }
    pub fn op_Implicit_f32_0(
        v: f32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float2x4> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (f32),
                crate::Unity::Mathematics::float2x4,
                1usize,
            >("op_Implicit")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "op_Implicit", 1usize
                )
            });
        let __cordl_ret: crate::Unity::Mathematics::float2x4 = unsafe {
            method.invoke_unchecked((), (v))
        };
        Ok(__cordl_ret.into())
    }
    pub fn op_Implicit_i32_1(
        v: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float2x4> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (i32),
                crate::Unity::Mathematics::float2x4,
                1usize,
            >("op_Implicit")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "op_Implicit", 1usize
                )
            });
        let __cordl_ret: crate::Unity::Mathematics::float2x4 = unsafe {
            method.invoke_unchecked((), (v))
        };
        Ok(__cordl_ret.into())
    }
    pub fn op_Implicit_int2x4_2(
        v: crate::Unity::Mathematics::int2x4,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float2x4> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::Unity::Mathematics::int2x4),
                crate::Unity::Mathematics::float2x4,
                1usize,
            >("op_Implicit")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "op_Implicit", 1usize
                )
            });
        let __cordl_ret: crate::Unity::Mathematics::float2x4 = unsafe {
            method.invoke_unchecked((), (v))
        };
        Ok(__cordl_ret.into())
    }
    pub fn op_Implicit_u32_3(
        v: u32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float2x4> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (u32),
                crate::Unity::Mathematics::float2x4,
                1usize,
            >("op_Implicit")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "op_Implicit", 1usize
                )
            });
        let __cordl_ret: crate::Unity::Mathematics::float2x4 = unsafe {
            method.invoke_unchecked((), (v))
        };
        Ok(__cordl_ret.into())
    }
    pub fn op_Implicit_uint2x4_4(
        v: crate::Unity::Mathematics::uint2x4,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float2x4> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::Unity::Mathematics::uint2x4),
                crate::Unity::Mathematics::float2x4,
                1usize,
            >("op_Implicit")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "op_Implicit", 1usize
                )
            });
        let __cordl_ret: crate::Unity::Mathematics::float2x4 = unsafe {
            method.invoke_unchecked((), (v))
        };
        Ok(__cordl_ret.into())
    }
    pub fn op_Increment(
        val: crate::Unity::Mathematics::float2x4,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float2x4> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::Unity::Mathematics::float2x4),
                crate::Unity::Mathematics::float2x4,
                1usize,
            >("op_Increment")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "op_Increment", 1usize
                )
            });
        let __cordl_ret: crate::Unity::Mathematics::float2x4 = unsafe {
            method.invoke_unchecked((), (val))
        };
        Ok(__cordl_ret.into())
    }
    pub fn op_Inequality_f32_float2x4_2(
        lhs: f32,
        rhs: crate::Unity::Mathematics::float2x4,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::bool2x4> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (f32, crate::Unity::Mathematics::float2x4),
                crate::Unity::Mathematics::bool2x4,
                2usize,
            >("op_Inequality")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "op_Inequality", 2usize
                )
            });
        let __cordl_ret: crate::Unity::Mathematics::bool2x4 = unsafe {
            method.invoke_unchecked((), (lhs, rhs))
        };
        Ok(__cordl_ret.into())
    }
    pub fn op_Inequality_float2x4_f32_1(
        lhs: crate::Unity::Mathematics::float2x4,
        rhs: f32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::bool2x4> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::Unity::Mathematics::float2x4, f32),
                crate::Unity::Mathematics::bool2x4,
                2usize,
            >("op_Inequality")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "op_Inequality", 2usize
                )
            });
        let __cordl_ret: crate::Unity::Mathematics::bool2x4 = unsafe {
            method.invoke_unchecked((), (lhs, rhs))
        };
        Ok(__cordl_ret.into())
    }
    pub fn op_Inequality_float2x4_float2x4_0(
        lhs: crate::Unity::Mathematics::float2x4,
        rhs: crate::Unity::Mathematics::float2x4,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::bool2x4> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::Unity::Mathematics::float2x4,
                    crate::Unity::Mathematics::float2x4,
                ),
                crate::Unity::Mathematics::bool2x4,
                2usize,
            >("op_Inequality")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "op_Inequality", 2usize
                )
            });
        let __cordl_ret: crate::Unity::Mathematics::bool2x4 = unsafe {
            method.invoke_unchecked((), (lhs, rhs))
        };
        Ok(__cordl_ret.into())
    }
    pub fn op_LessThanOrEqual_f32_float2x4_2(
        lhs: f32,
        rhs: crate::Unity::Mathematics::float2x4,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::bool2x4> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (f32, crate::Unity::Mathematics::float2x4),
                crate::Unity::Mathematics::bool2x4,
                2usize,
            >("op_LessThanOrEqual")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "op_LessThanOrEqual", 2usize
                )
            });
        let __cordl_ret: crate::Unity::Mathematics::bool2x4 = unsafe {
            method.invoke_unchecked((), (lhs, rhs))
        };
        Ok(__cordl_ret.into())
    }
    pub fn op_LessThanOrEqual_float2x4_f32_1(
        lhs: crate::Unity::Mathematics::float2x4,
        rhs: f32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::bool2x4> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::Unity::Mathematics::float2x4, f32),
                crate::Unity::Mathematics::bool2x4,
                2usize,
            >("op_LessThanOrEqual")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "op_LessThanOrEqual", 2usize
                )
            });
        let __cordl_ret: crate::Unity::Mathematics::bool2x4 = unsafe {
            method.invoke_unchecked((), (lhs, rhs))
        };
        Ok(__cordl_ret.into())
    }
    pub fn op_LessThanOrEqual_float2x4_float2x4_0(
        lhs: crate::Unity::Mathematics::float2x4,
        rhs: crate::Unity::Mathematics::float2x4,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::bool2x4> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::Unity::Mathematics::float2x4,
                    crate::Unity::Mathematics::float2x4,
                ),
                crate::Unity::Mathematics::bool2x4,
                2usize,
            >("op_LessThanOrEqual")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "op_LessThanOrEqual", 2usize
                )
            });
        let __cordl_ret: crate::Unity::Mathematics::bool2x4 = unsafe {
            method.invoke_unchecked((), (lhs, rhs))
        };
        Ok(__cordl_ret.into())
    }
    pub fn op_LessThan_f32_float2x4_2(
        lhs: f32,
        rhs: crate::Unity::Mathematics::float2x4,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::bool2x4> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (f32, crate::Unity::Mathematics::float2x4),
                crate::Unity::Mathematics::bool2x4,
                2usize,
            >("op_LessThan")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "op_LessThan", 2usize
                )
            });
        let __cordl_ret: crate::Unity::Mathematics::bool2x4 = unsafe {
            method.invoke_unchecked((), (lhs, rhs))
        };
        Ok(__cordl_ret.into())
    }
    pub fn op_LessThan_float2x4_f32_1(
        lhs: crate::Unity::Mathematics::float2x4,
        rhs: f32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::bool2x4> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::Unity::Mathematics::float2x4, f32),
                crate::Unity::Mathematics::bool2x4,
                2usize,
            >("op_LessThan")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "op_LessThan", 2usize
                )
            });
        let __cordl_ret: crate::Unity::Mathematics::bool2x4 = unsafe {
            method.invoke_unchecked((), (lhs, rhs))
        };
        Ok(__cordl_ret.into())
    }
    pub fn op_LessThan_float2x4_float2x4_0(
        lhs: crate::Unity::Mathematics::float2x4,
        rhs: crate::Unity::Mathematics::float2x4,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::bool2x4> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::Unity::Mathematics::float2x4,
                    crate::Unity::Mathematics::float2x4,
                ),
                crate::Unity::Mathematics::bool2x4,
                2usize,
            >("op_LessThan")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "op_LessThan", 2usize
                )
            });
        let __cordl_ret: crate::Unity::Mathematics::bool2x4 = unsafe {
            method.invoke_unchecked((), (lhs, rhs))
        };
        Ok(__cordl_ret.into())
    }
    pub fn op_Modulus_f32_float2x4_2(
        lhs: f32,
        rhs: crate::Unity::Mathematics::float2x4,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float2x4> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (f32, crate::Unity::Mathematics::float2x4),
                crate::Unity::Mathematics::float2x4,
                2usize,
            >("op_Modulus")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "op_Modulus", 2usize
                )
            });
        let __cordl_ret: crate::Unity::Mathematics::float2x4 = unsafe {
            method.invoke_unchecked((), (lhs, rhs))
        };
        Ok(__cordl_ret.into())
    }
    pub fn op_Modulus_float2x4_f32_1(
        lhs: crate::Unity::Mathematics::float2x4,
        rhs: f32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float2x4> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::Unity::Mathematics::float2x4, f32),
                crate::Unity::Mathematics::float2x4,
                2usize,
            >("op_Modulus")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "op_Modulus", 2usize
                )
            });
        let __cordl_ret: crate::Unity::Mathematics::float2x4 = unsafe {
            method.invoke_unchecked((), (lhs, rhs))
        };
        Ok(__cordl_ret.into())
    }
    pub fn op_Modulus_float2x4_float2x4_0(
        lhs: crate::Unity::Mathematics::float2x4,
        rhs: crate::Unity::Mathematics::float2x4,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float2x4> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::Unity::Mathematics::float2x4,
                    crate::Unity::Mathematics::float2x4,
                ),
                crate::Unity::Mathematics::float2x4,
                2usize,
            >("op_Modulus")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "op_Modulus", 2usize
                )
            });
        let __cordl_ret: crate::Unity::Mathematics::float2x4 = unsafe {
            method.invoke_unchecked((), (lhs, rhs))
        };
        Ok(__cordl_ret.into())
    }
    pub fn op_Multiply_f32_float2x4_2(
        lhs: f32,
        rhs: crate::Unity::Mathematics::float2x4,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float2x4> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (f32, crate::Unity::Mathematics::float2x4),
                crate::Unity::Mathematics::float2x4,
                2usize,
            >("op_Multiply")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "op_Multiply", 2usize
                )
            });
        let __cordl_ret: crate::Unity::Mathematics::float2x4 = unsafe {
            method.invoke_unchecked((), (lhs, rhs))
        };
        Ok(__cordl_ret.into())
    }
    pub fn op_Multiply_float2x4_f32_1(
        lhs: crate::Unity::Mathematics::float2x4,
        rhs: f32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float2x4> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::Unity::Mathematics::float2x4, f32),
                crate::Unity::Mathematics::float2x4,
                2usize,
            >("op_Multiply")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "op_Multiply", 2usize
                )
            });
        let __cordl_ret: crate::Unity::Mathematics::float2x4 = unsafe {
            method.invoke_unchecked((), (lhs, rhs))
        };
        Ok(__cordl_ret.into())
    }
    pub fn op_Multiply_float2x4_float2x4_0(
        lhs: crate::Unity::Mathematics::float2x4,
        rhs: crate::Unity::Mathematics::float2x4,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float2x4> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::Unity::Mathematics::float2x4,
                    crate::Unity::Mathematics::float2x4,
                ),
                crate::Unity::Mathematics::float2x4,
                2usize,
            >("op_Multiply")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "op_Multiply", 2usize
                )
            });
        let __cordl_ret: crate::Unity::Mathematics::float2x4 = unsafe {
            method.invoke_unchecked((), (lhs, rhs))
        };
        Ok(__cordl_ret.into())
    }
    pub fn op_Subtraction_f32_float2x4_2(
        lhs: f32,
        rhs: crate::Unity::Mathematics::float2x4,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float2x4> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (f32, crate::Unity::Mathematics::float2x4),
                crate::Unity::Mathematics::float2x4,
                2usize,
            >("op_Subtraction")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "op_Subtraction", 2usize
                )
            });
        let __cordl_ret: crate::Unity::Mathematics::float2x4 = unsafe {
            method.invoke_unchecked((), (lhs, rhs))
        };
        Ok(__cordl_ret.into())
    }
    pub fn op_Subtraction_float2x4_f32_1(
        lhs: crate::Unity::Mathematics::float2x4,
        rhs: f32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float2x4> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::Unity::Mathematics::float2x4, f32),
                crate::Unity::Mathematics::float2x4,
                2usize,
            >("op_Subtraction")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "op_Subtraction", 2usize
                )
            });
        let __cordl_ret: crate::Unity::Mathematics::float2x4 = unsafe {
            method.invoke_unchecked((), (lhs, rhs))
        };
        Ok(__cordl_ret.into())
    }
    pub fn op_Subtraction_float2x4_float2x4_0(
        lhs: crate::Unity::Mathematics::float2x4,
        rhs: crate::Unity::Mathematics::float2x4,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float2x4> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::Unity::Mathematics::float2x4,
                    crate::Unity::Mathematics::float2x4,
                ),
                crate::Unity::Mathematics::float2x4,
                2usize,
            >("op_Subtraction")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "op_Subtraction", 2usize
                )
            });
        let __cordl_ret: crate::Unity::Mathematics::float2x4 = unsafe {
            method.invoke_unchecked((), (lhs, rhs))
        };
        Ok(__cordl_ret.into())
    }
    pub fn op_UnaryNegation(
        val: crate::Unity::Mathematics::float2x4,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float2x4> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::Unity::Mathematics::float2x4),
                crate::Unity::Mathematics::float2x4,
                1usize,
            >("op_UnaryNegation")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "op_UnaryNegation", 1usize
                )
            });
        let __cordl_ret: crate::Unity::Mathematics::float2x4 = unsafe {
            method.invoke_unchecked((), (val))
        };
        Ok(__cordl_ret.into())
    }
    pub fn op_UnaryPlus(
        val: crate::Unity::Mathematics::float2x4,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float2x4> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::Unity::Mathematics::float2x4),
                crate::Unity::Mathematics::float2x4,
                1usize,
            >("op_UnaryPlus")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "op_UnaryPlus", 1usize
                )
            });
        let __cordl_ret: crate::Unity::Mathematics::float2x4 = unsafe {
            method.invoke_unchecked((), (val))
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Unity+Mathematics+float2x4")]
impl AsRef<crate::System::IEquatable_1<crate::Unity::Mathematics::float2x4>>
for crate::Unity::Mathematics::float2x4 {
    fn as_ref(
        &self,
    ) -> &crate::System::IEquatable_1<crate::Unity::Mathematics::float2x4> {
        todo!()
    }
}
#[cfg(feature = "Unity+Mathematics+float2x4")]
impl AsMut<crate::System::IEquatable_1<crate::Unity::Mathematics::float2x4>>
for crate::Unity::Mathematics::float2x4 {
    fn as_mut(
        &mut self,
    ) -> &mut crate::System::IEquatable_1<crate::Unity::Mathematics::float2x4> {
        todo!()
    }
}
#[cfg(feature = "Unity+Mathematics+float2x4")]
impl AsRef<crate::System::IFormattable> for crate::Unity::Mathematics::float2x4 {
    fn as_ref(&self) -> &crate::System::IFormattable {
        todo!()
    }
}
#[cfg(feature = "Unity+Mathematics+float2x4")]
impl AsMut<crate::System::IFormattable> for crate::Unity::Mathematics::float2x4 {
    fn as_mut(&mut self) -> &mut crate::System::IFormattable {
        todo!()
    }
}
