#[cfg(feature = "cordl_class_Unity+Mathematics+bool2x3")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct bool2x3 {
    pub c0: crate::Unity::Mathematics::bool2,
    pub c1: crate::Unity::Mathematics::bool2,
    pub c2: crate::Unity::Mathematics::bool2,
}
#[cfg(feature = "cordl_class_Unity+Mathematics+bool2x3")]
unsafe impl quest_hook::libil2cpp::Type for crate::Unity::Mathematics::bool2x3 {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "Unity.Mathematics";
    const CLASS_NAME: &'static str = "bool2x3";
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
#[cfg(feature = "cordl_class_Unity+Mathematics+bool2x3")]
unsafe impl quest_hook::libil2cpp::Argument for crate::Unity::Mathematics::bool2x3 {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "cordl_class_Unity+Mathematics+bool2x3")]
unsafe impl quest_hook::libil2cpp::Parameter for crate::Unity::Mathematics::bool2x3 {
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
#[cfg(feature = "cordl_class_Unity+Mathematics+bool2x3")]
unsafe impl quest_hook::libil2cpp::Returned for crate::Unity::Mathematics::bool2x3 {
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
#[cfg(feature = "cordl_class_Unity+Mathematics+bool2x3")]
unsafe impl quest_hook::libil2cpp::Return for crate::Unity::Mathematics::bool2x3 {
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
#[cfg(feature = "cordl_class_Unity+Mathematics+bool2x3")]
unsafe impl quest_hook::libil2cpp::ThisArgument for crate::Unity::Mathematics::bool2x3 {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "Unity+Mathematics+bool2x3")]
impl crate::Unity::Mathematics::bool2x3 {
    pub fn Equals_Il2CppObject1(
        &mut self,
        o: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>),
                        bool,
                        1usize,
                    >("Equals")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "Equals",
                            1usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            cordl_method_info.invoke_unchecked(self, (o))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Equals_bool2x3_0(
        &mut self,
        rhs: crate::Unity::Mathematics::bool2x3,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (crate::Unity::Mathematics::bool2x3),
                        bool,
                        1usize,
                    >("Equals")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "Equals",
                            1usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            cordl_method_info.invoke_unchecked(self, (rhs))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetHashCode(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), i32, 0usize>("GetHashCode")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "GetHashCode", 0usize
                        )
                    })
            });
        let __cordl_ret: i32 = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn ToString(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        0usize,
                    >("ToString")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "ToString", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor__cordl_bool2(
        &mut self,
        v: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(bool), quest_hook::libil2cpp::Void, 1usize>(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), ".ctor",
                            1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (v))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor__cordl_bool__cordl_bool__cordl_bool__cordl_bool__cordl_bool__cordl_bool1(
        &mut self,
        m00: bool,
        m01: bool,
        m02: bool,
        m10: bool,
        m11: bool,
        m12: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (bool, bool, bool, bool, bool, bool),
                        quest_hook::libil2cpp::Void,
                        6usize,
                    >(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), ".ctor",
                            6usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (m00, m01, m02, m10, m11, m12))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_bool2_bool2_bool2_0(
        &mut self,
        c0: crate::Unity::Mathematics::bool2,
        c1: crate::Unity::Mathematics::bool2,
        c2: crate::Unity::Mathematics::bool2,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            crate::Unity::Mathematics::bool2,
                            crate::Unity::Mathematics::bool2,
                            crate::Unity::Mathematics::bool2,
                        ),
                        quest_hook::libil2cpp::Void,
                        3usize,
                    >(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), ".ctor",
                            3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (c0, c1, c2))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_Item(
        &mut self,
        index: i32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::ByRefMut<crate::Unity::Mathematics::bool2>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (i32),
                        quest_hook::libil2cpp::ByRefMut<
                            crate::Unity::Mathematics::bool2,
                        >,
                        1usize,
                    >("get_Item")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "get_Item", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::ByRefMut<
            crate::Unity::Mathematics::bool2,
        > = unsafe { cordl_method_info.invoke_unchecked(self, (index))? };
        Ok(__cordl_ret.into())
    }
    pub fn op_BitwiseAnd__cordl_bool_bool2x3_2(
        lhs: bool,
        rhs: crate::Unity::Mathematics::bool2x3,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::bool2x3> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (bool, crate::Unity::Mathematics::bool2x3),
                        crate::Unity::Mathematics::bool2x3,
                        2usize,
                    >("op_BitwiseAnd")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "op_BitwiseAnd", 2usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Mathematics::bool2x3 = unsafe {
            cordl_method_info.invoke_unchecked((), (lhs, rhs))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn op_BitwiseAnd_bool2x3__cordl_bool1(
        lhs: crate::Unity::Mathematics::bool2x3,
        rhs: bool,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::bool2x3> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (crate::Unity::Mathematics::bool2x3, bool),
                        crate::Unity::Mathematics::bool2x3,
                        2usize,
                    >("op_BitwiseAnd")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "op_BitwiseAnd", 2usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Mathematics::bool2x3 = unsafe {
            cordl_method_info.invoke_unchecked((), (lhs, rhs))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn op_BitwiseAnd_bool2x3_bool2x3_0(
        lhs: crate::Unity::Mathematics::bool2x3,
        rhs: crate::Unity::Mathematics::bool2x3,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::bool2x3> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            crate::Unity::Mathematics::bool2x3,
                            crate::Unity::Mathematics::bool2x3,
                        ),
                        crate::Unity::Mathematics::bool2x3,
                        2usize,
                    >("op_BitwiseAnd")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "op_BitwiseAnd", 2usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Mathematics::bool2x3 = unsafe {
            cordl_method_info.invoke_unchecked((), (lhs, rhs))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn op_BitwiseOr__cordl_bool_bool2x3_2(
        lhs: bool,
        rhs: crate::Unity::Mathematics::bool2x3,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::bool2x3> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (bool, crate::Unity::Mathematics::bool2x3),
                        crate::Unity::Mathematics::bool2x3,
                        2usize,
                    >("op_BitwiseOr")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "op_BitwiseOr", 2usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Mathematics::bool2x3 = unsafe {
            cordl_method_info.invoke_unchecked((), (lhs, rhs))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn op_BitwiseOr_bool2x3__cordl_bool1(
        lhs: crate::Unity::Mathematics::bool2x3,
        rhs: bool,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::bool2x3> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (crate::Unity::Mathematics::bool2x3, bool),
                        crate::Unity::Mathematics::bool2x3,
                        2usize,
                    >("op_BitwiseOr")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "op_BitwiseOr", 2usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Mathematics::bool2x3 = unsafe {
            cordl_method_info.invoke_unchecked((), (lhs, rhs))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn op_BitwiseOr_bool2x3_bool2x3_0(
        lhs: crate::Unity::Mathematics::bool2x3,
        rhs: crate::Unity::Mathematics::bool2x3,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::bool2x3> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            crate::Unity::Mathematics::bool2x3,
                            crate::Unity::Mathematics::bool2x3,
                        ),
                        crate::Unity::Mathematics::bool2x3,
                        2usize,
                    >("op_BitwiseOr")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "op_BitwiseOr", 2usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Mathematics::bool2x3 = unsafe {
            cordl_method_info.invoke_unchecked((), (lhs, rhs))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn op_Equality__cordl_bool_bool2x3_2(
        lhs: bool,
        rhs: crate::Unity::Mathematics::bool2x3,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::bool2x3> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (bool, crate::Unity::Mathematics::bool2x3),
                        crate::Unity::Mathematics::bool2x3,
                        2usize,
                    >("op_Equality")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "op_Equality", 2usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Mathematics::bool2x3 = unsafe {
            cordl_method_info.invoke_unchecked((), (lhs, rhs))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn op_Equality_bool2x3__cordl_bool1(
        lhs: crate::Unity::Mathematics::bool2x3,
        rhs: bool,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::bool2x3> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (crate::Unity::Mathematics::bool2x3, bool),
                        crate::Unity::Mathematics::bool2x3,
                        2usize,
                    >("op_Equality")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "op_Equality", 2usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Mathematics::bool2x3 = unsafe {
            cordl_method_info.invoke_unchecked((), (lhs, rhs))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn op_Equality_bool2x3_bool2x3_0(
        lhs: crate::Unity::Mathematics::bool2x3,
        rhs: crate::Unity::Mathematics::bool2x3,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::bool2x3> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            crate::Unity::Mathematics::bool2x3,
                            crate::Unity::Mathematics::bool2x3,
                        ),
                        crate::Unity::Mathematics::bool2x3,
                        2usize,
                    >("op_Equality")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "op_Equality", 2usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Mathematics::bool2x3 = unsafe {
            cordl_method_info.invoke_unchecked((), (lhs, rhs))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn op_ExclusiveOr__cordl_bool_bool2x3_2(
        lhs: bool,
        rhs: crate::Unity::Mathematics::bool2x3,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::bool2x3> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (bool, crate::Unity::Mathematics::bool2x3),
                        crate::Unity::Mathematics::bool2x3,
                        2usize,
                    >("op_ExclusiveOr")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "op_ExclusiveOr", 2usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Mathematics::bool2x3 = unsafe {
            cordl_method_info.invoke_unchecked((), (lhs, rhs))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn op_ExclusiveOr_bool2x3__cordl_bool1(
        lhs: crate::Unity::Mathematics::bool2x3,
        rhs: bool,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::bool2x3> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (crate::Unity::Mathematics::bool2x3, bool),
                        crate::Unity::Mathematics::bool2x3,
                        2usize,
                    >("op_ExclusiveOr")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "op_ExclusiveOr", 2usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Mathematics::bool2x3 = unsafe {
            cordl_method_info.invoke_unchecked((), (lhs, rhs))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn op_ExclusiveOr_bool2x3_bool2x3_0(
        lhs: crate::Unity::Mathematics::bool2x3,
        rhs: crate::Unity::Mathematics::bool2x3,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::bool2x3> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            crate::Unity::Mathematics::bool2x3,
                            crate::Unity::Mathematics::bool2x3,
                        ),
                        crate::Unity::Mathematics::bool2x3,
                        2usize,
                    >("op_ExclusiveOr")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "op_ExclusiveOr", 2usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Mathematics::bool2x3 = unsafe {
            cordl_method_info.invoke_unchecked((), (lhs, rhs))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn op_Implicit(
        v: bool,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::bool2x3> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (bool),
                        crate::Unity::Mathematics::bool2x3,
                        1usize,
                    >("op_Implicit")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "op_Implicit", 1usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Mathematics::bool2x3 = unsafe {
            cordl_method_info.invoke_unchecked((), (v))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn op_Inequality__cordl_bool_bool2x3_2(
        lhs: bool,
        rhs: crate::Unity::Mathematics::bool2x3,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::bool2x3> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (bool, crate::Unity::Mathematics::bool2x3),
                        crate::Unity::Mathematics::bool2x3,
                        2usize,
                    >("op_Inequality")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "op_Inequality", 2usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Mathematics::bool2x3 = unsafe {
            cordl_method_info.invoke_unchecked((), (lhs, rhs))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn op_Inequality_bool2x3__cordl_bool1(
        lhs: crate::Unity::Mathematics::bool2x3,
        rhs: bool,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::bool2x3> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (crate::Unity::Mathematics::bool2x3, bool),
                        crate::Unity::Mathematics::bool2x3,
                        2usize,
                    >("op_Inequality")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "op_Inequality", 2usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Mathematics::bool2x3 = unsafe {
            cordl_method_info.invoke_unchecked((), (lhs, rhs))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn op_Inequality_bool2x3_bool2x3_0(
        lhs: crate::Unity::Mathematics::bool2x3,
        rhs: crate::Unity::Mathematics::bool2x3,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::bool2x3> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            crate::Unity::Mathematics::bool2x3,
                            crate::Unity::Mathematics::bool2x3,
                        ),
                        crate::Unity::Mathematics::bool2x3,
                        2usize,
                    >("op_Inequality")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "op_Inequality", 2usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Mathematics::bool2x3 = unsafe {
            cordl_method_info.invoke_unchecked((), (lhs, rhs))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn op_LogicalNot(
        val: crate::Unity::Mathematics::bool2x3,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::bool2x3> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (crate::Unity::Mathematics::bool2x3),
                        crate::Unity::Mathematics::bool2x3,
                        1usize,
                    >("op_LogicalNot")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "op_LogicalNot", 1usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Mathematics::bool2x3 = unsafe {
            cordl_method_info.invoke_unchecked((), (val))?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Unity+Mathematics+bool2x3")]
impl AsRef<crate::System::IEquatable_1<crate::Unity::Mathematics::bool2x3>>
for crate::Unity::Mathematics::bool2x3 {
    fn as_ref(
        &self,
    ) -> &crate::System::IEquatable_1<crate::Unity::Mathematics::bool2x3> {
        todo!()
    }
}
#[cfg(feature = "Unity+Mathematics+bool2x3")]
impl AsMut<crate::System::IEquatable_1<crate::Unity::Mathematics::bool2x3>>
for crate::Unity::Mathematics::bool2x3 {
    fn as_mut(
        &mut self,
    ) -> &mut crate::System::IEquatable_1<crate::Unity::Mathematics::bool2x3> {
        todo!()
    }
}
