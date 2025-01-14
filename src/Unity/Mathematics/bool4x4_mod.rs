#[cfg(feature = "Unity+Mathematics+bool4x4")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct bool4x4 {
    pub c0: crate::Unity::Mathematics::bool4,
    pub c1: crate::Unity::Mathematics::bool4,
    pub c2: crate::Unity::Mathematics::bool4,
    pub c3: crate::Unity::Mathematics::bool4,
}
#[cfg(feature = "Unity+Mathematics+bool4x4")]
unsafe impl quest_hook::libil2cpp::Type for crate::Unity::Mathematics::bool4x4 {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "Unity.Mathematics";
    const CLASS_NAME: &'static str = "bool4x4";
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
#[cfg(feature = "Unity+Mathematics+bool4x4")]
unsafe impl quest_hook::libil2cpp::Argument for crate::Unity::Mathematics::bool4x4 {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "Unity+Mathematics+bool4x4")]
unsafe impl quest_hook::libil2cpp::Parameter for crate::Unity::Mathematics::bool4x4 {
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
#[cfg(feature = "Unity+Mathematics+bool4x4")]
unsafe impl quest_hook::libil2cpp::Returned for crate::Unity::Mathematics::bool4x4 {
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
#[cfg(feature = "Unity+Mathematics+bool4x4")]
unsafe impl quest_hook::libil2cpp::Return for crate::Unity::Mathematics::bool4x4 {
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
#[cfg(feature = "Unity+Mathematics+bool4x4")]
unsafe impl quest_hook::libil2cpp::ThisArgument for crate::Unity::Mathematics::bool4x4 {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "Unity+Mathematics+bool4x4")]
impl crate::Unity::Mathematics::bool4x4 {
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
    pub fn Equals_bool4x4_0(
        &mut self,
        rhs: crate::Unity::Mathematics::bool4x4,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<(crate::Unity::Mathematics::bool4x4), bool, 1usize>("Equals")
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
    pub fn ToString(
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
    pub fn _ctor__cordl_bool2(
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
    pub fn _ctor__cordl_bool__cordl_bool__cordl_bool__cordl_bool__cordl_bool__cordl_bool__cordl_bool__cordl_bool__cordl_bool__cordl_bool__cordl_bool__cordl_bool__cordl_bool__cordl_bool__cordl_bool__cordl_bool1(
        &mut self,
        m00: bool,
        m01: bool,
        m02: bool,
        m03: bool,
        m10: bool,
        m11: bool,
        m12: bool,
        m13: bool,
        m20: bool,
        m21: bool,
        m22: bool,
        m23: bool,
        m30: bool,
        m31: bool,
        m32: bool,
        m33: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    bool,
                    bool,
                    bool,
                    bool,
                    bool,
                    bool,
                    bool,
                    bool,
                    bool,
                    bool,
                    bool,
                    bool,
                    bool,
                    bool,
                    bool,
                    bool,
                ),
                quest_hook::libil2cpp::Void,
                16usize,
            >(".ctor")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, ".ctor", 16usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method
                .invoke_unchecked(
                    self,
                    (
                        m00,
                        m01,
                        m02,
                        m03,
                        m10,
                        m11,
                        m12,
                        m13,
                        m20,
                        m21,
                        m22,
                        m23,
                        m30,
                        m31,
                        m32,
                        m33,
                    ),
                )
        };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_bool4_bool4_bool4_bool4_0(
        &mut self,
        c0: crate::Unity::Mathematics::bool4,
        c1: crate::Unity::Mathematics::bool4,
        c2: crate::Unity::Mathematics::bool4,
        c3: crate::Unity::Mathematics::bool4,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    crate::Unity::Mathematics::bool4,
                    crate::Unity::Mathematics::bool4,
                    crate::Unity::Mathematics::bool4,
                    crate::Unity::Mathematics::bool4,
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
    pub fn get_Item(
        &mut self,
        index: i32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::ByRefMut<crate::Unity::Mathematics::bool4>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (i32),
                quest_hook::libil2cpp::ByRefMut<crate::Unity::Mathematics::bool4>,
                1usize,
            >("get_Item")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "get_Item", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::ByRefMut<
            crate::Unity::Mathematics::bool4,
        > = unsafe { method.invoke_unchecked(self, (index)) };
        Ok(__cordl_ret.into())
    }
    pub fn op_BitwiseAnd__cordl_bool_bool4x4_2(
        lhs: bool,
        rhs: crate::Unity::Mathematics::bool4x4,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::bool4x4> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (bool, crate::Unity::Mathematics::bool4x4),
                crate::Unity::Mathematics::bool4x4,
                2usize,
            >("op_BitwiseAnd")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "op_BitwiseAnd", 2usize
                )
            });
        let __cordl_ret: crate::Unity::Mathematics::bool4x4 = unsafe {
            method.invoke_unchecked((), (lhs, rhs))
        };
        Ok(__cordl_ret.into())
    }
    pub fn op_BitwiseAnd_bool4x4__cordl_bool1(
        lhs: crate::Unity::Mathematics::bool4x4,
        rhs: bool,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::bool4x4> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::Unity::Mathematics::bool4x4, bool),
                crate::Unity::Mathematics::bool4x4,
                2usize,
            >("op_BitwiseAnd")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "op_BitwiseAnd", 2usize
                )
            });
        let __cordl_ret: crate::Unity::Mathematics::bool4x4 = unsafe {
            method.invoke_unchecked((), (lhs, rhs))
        };
        Ok(__cordl_ret.into())
    }
    pub fn op_BitwiseAnd_bool4x4_bool4x4_0(
        lhs: crate::Unity::Mathematics::bool4x4,
        rhs: crate::Unity::Mathematics::bool4x4,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::bool4x4> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::Unity::Mathematics::bool4x4, crate::Unity::Mathematics::bool4x4),
                crate::Unity::Mathematics::bool4x4,
                2usize,
            >("op_BitwiseAnd")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "op_BitwiseAnd", 2usize
                )
            });
        let __cordl_ret: crate::Unity::Mathematics::bool4x4 = unsafe {
            method.invoke_unchecked((), (lhs, rhs))
        };
        Ok(__cordl_ret.into())
    }
    pub fn op_BitwiseOr__cordl_bool_bool4x4_2(
        lhs: bool,
        rhs: crate::Unity::Mathematics::bool4x4,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::bool4x4> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (bool, crate::Unity::Mathematics::bool4x4),
                crate::Unity::Mathematics::bool4x4,
                2usize,
            >("op_BitwiseOr")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "op_BitwiseOr", 2usize
                )
            });
        let __cordl_ret: crate::Unity::Mathematics::bool4x4 = unsafe {
            method.invoke_unchecked((), (lhs, rhs))
        };
        Ok(__cordl_ret.into())
    }
    pub fn op_BitwiseOr_bool4x4__cordl_bool1(
        lhs: crate::Unity::Mathematics::bool4x4,
        rhs: bool,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::bool4x4> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::Unity::Mathematics::bool4x4, bool),
                crate::Unity::Mathematics::bool4x4,
                2usize,
            >("op_BitwiseOr")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "op_BitwiseOr", 2usize
                )
            });
        let __cordl_ret: crate::Unity::Mathematics::bool4x4 = unsafe {
            method.invoke_unchecked((), (lhs, rhs))
        };
        Ok(__cordl_ret.into())
    }
    pub fn op_BitwiseOr_bool4x4_bool4x4_0(
        lhs: crate::Unity::Mathematics::bool4x4,
        rhs: crate::Unity::Mathematics::bool4x4,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::bool4x4> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::Unity::Mathematics::bool4x4, crate::Unity::Mathematics::bool4x4),
                crate::Unity::Mathematics::bool4x4,
                2usize,
            >("op_BitwiseOr")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "op_BitwiseOr", 2usize
                )
            });
        let __cordl_ret: crate::Unity::Mathematics::bool4x4 = unsafe {
            method.invoke_unchecked((), (lhs, rhs))
        };
        Ok(__cordl_ret.into())
    }
    pub fn op_Equality__cordl_bool_bool4x4_2(
        lhs: bool,
        rhs: crate::Unity::Mathematics::bool4x4,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::bool4x4> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (bool, crate::Unity::Mathematics::bool4x4),
                crate::Unity::Mathematics::bool4x4,
                2usize,
            >("op_Equality")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "op_Equality", 2usize
                )
            });
        let __cordl_ret: crate::Unity::Mathematics::bool4x4 = unsafe {
            method.invoke_unchecked((), (lhs, rhs))
        };
        Ok(__cordl_ret.into())
    }
    pub fn op_Equality_bool4x4__cordl_bool1(
        lhs: crate::Unity::Mathematics::bool4x4,
        rhs: bool,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::bool4x4> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::Unity::Mathematics::bool4x4, bool),
                crate::Unity::Mathematics::bool4x4,
                2usize,
            >("op_Equality")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "op_Equality", 2usize
                )
            });
        let __cordl_ret: crate::Unity::Mathematics::bool4x4 = unsafe {
            method.invoke_unchecked((), (lhs, rhs))
        };
        Ok(__cordl_ret.into())
    }
    pub fn op_Equality_bool4x4_bool4x4_0(
        lhs: crate::Unity::Mathematics::bool4x4,
        rhs: crate::Unity::Mathematics::bool4x4,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::bool4x4> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::Unity::Mathematics::bool4x4, crate::Unity::Mathematics::bool4x4),
                crate::Unity::Mathematics::bool4x4,
                2usize,
            >("op_Equality")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "op_Equality", 2usize
                )
            });
        let __cordl_ret: crate::Unity::Mathematics::bool4x4 = unsafe {
            method.invoke_unchecked((), (lhs, rhs))
        };
        Ok(__cordl_ret.into())
    }
    pub fn op_ExclusiveOr__cordl_bool_bool4x4_2(
        lhs: bool,
        rhs: crate::Unity::Mathematics::bool4x4,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::bool4x4> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (bool, crate::Unity::Mathematics::bool4x4),
                crate::Unity::Mathematics::bool4x4,
                2usize,
            >("op_ExclusiveOr")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "op_ExclusiveOr", 2usize
                )
            });
        let __cordl_ret: crate::Unity::Mathematics::bool4x4 = unsafe {
            method.invoke_unchecked((), (lhs, rhs))
        };
        Ok(__cordl_ret.into())
    }
    pub fn op_ExclusiveOr_bool4x4__cordl_bool1(
        lhs: crate::Unity::Mathematics::bool4x4,
        rhs: bool,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::bool4x4> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::Unity::Mathematics::bool4x4, bool),
                crate::Unity::Mathematics::bool4x4,
                2usize,
            >("op_ExclusiveOr")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "op_ExclusiveOr", 2usize
                )
            });
        let __cordl_ret: crate::Unity::Mathematics::bool4x4 = unsafe {
            method.invoke_unchecked((), (lhs, rhs))
        };
        Ok(__cordl_ret.into())
    }
    pub fn op_ExclusiveOr_bool4x4_bool4x4_0(
        lhs: crate::Unity::Mathematics::bool4x4,
        rhs: crate::Unity::Mathematics::bool4x4,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::bool4x4> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::Unity::Mathematics::bool4x4, crate::Unity::Mathematics::bool4x4),
                crate::Unity::Mathematics::bool4x4,
                2usize,
            >("op_ExclusiveOr")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "op_ExclusiveOr", 2usize
                )
            });
        let __cordl_ret: crate::Unity::Mathematics::bool4x4 = unsafe {
            method.invoke_unchecked((), (lhs, rhs))
        };
        Ok(__cordl_ret.into())
    }
    pub fn op_Implicit(
        v: bool,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::bool4x4> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (bool),
                crate::Unity::Mathematics::bool4x4,
                1usize,
            >("op_Implicit")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "op_Implicit", 1usize
                )
            });
        let __cordl_ret: crate::Unity::Mathematics::bool4x4 = unsafe {
            method.invoke_unchecked((), (v))
        };
        Ok(__cordl_ret.into())
    }
    pub fn op_Inequality__cordl_bool_bool4x4_2(
        lhs: bool,
        rhs: crate::Unity::Mathematics::bool4x4,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::bool4x4> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (bool, crate::Unity::Mathematics::bool4x4),
                crate::Unity::Mathematics::bool4x4,
                2usize,
            >("op_Inequality")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "op_Inequality", 2usize
                )
            });
        let __cordl_ret: crate::Unity::Mathematics::bool4x4 = unsafe {
            method.invoke_unchecked((), (lhs, rhs))
        };
        Ok(__cordl_ret.into())
    }
    pub fn op_Inequality_bool4x4__cordl_bool1(
        lhs: crate::Unity::Mathematics::bool4x4,
        rhs: bool,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::bool4x4> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::Unity::Mathematics::bool4x4, bool),
                crate::Unity::Mathematics::bool4x4,
                2usize,
            >("op_Inequality")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "op_Inequality", 2usize
                )
            });
        let __cordl_ret: crate::Unity::Mathematics::bool4x4 = unsafe {
            method.invoke_unchecked((), (lhs, rhs))
        };
        Ok(__cordl_ret.into())
    }
    pub fn op_Inequality_bool4x4_bool4x4_0(
        lhs: crate::Unity::Mathematics::bool4x4,
        rhs: crate::Unity::Mathematics::bool4x4,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::bool4x4> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::Unity::Mathematics::bool4x4, crate::Unity::Mathematics::bool4x4),
                crate::Unity::Mathematics::bool4x4,
                2usize,
            >("op_Inequality")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "op_Inequality", 2usize
                )
            });
        let __cordl_ret: crate::Unity::Mathematics::bool4x4 = unsafe {
            method.invoke_unchecked((), (lhs, rhs))
        };
        Ok(__cordl_ret.into())
    }
    pub fn op_LogicalNot(
        val: crate::Unity::Mathematics::bool4x4,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::bool4x4> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::Unity::Mathematics::bool4x4),
                crate::Unity::Mathematics::bool4x4,
                1usize,
            >("op_LogicalNot")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "op_LogicalNot", 1usize
                )
            });
        let __cordl_ret: crate::Unity::Mathematics::bool4x4 = unsafe {
            method.invoke_unchecked((), (val))
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Unity+Mathematics+bool4x4")]
impl AsRef<crate::System::IEquatable_1<crate::Unity::Mathematics::bool4x4>>
for crate::Unity::Mathematics::bool4x4 {
    fn as_ref(
        &self,
    ) -> &crate::System::IEquatable_1<crate::Unity::Mathematics::bool4x4> {
        todo!()
    }
}
#[cfg(feature = "Unity+Mathematics+bool4x4")]
impl AsMut<crate::System::IEquatable_1<crate::Unity::Mathematics::bool4x4>>
for crate::Unity::Mathematics::bool4x4 {
    fn as_mut(
        &mut self,
    ) -> &mut crate::System::IEquatable_1<crate::Unity::Mathematics::bool4x4> {
        todo!()
    }
}
