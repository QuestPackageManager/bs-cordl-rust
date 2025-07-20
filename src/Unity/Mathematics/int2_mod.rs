#[cfg(feature = "Unity+Mathematics+int2")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct int2 {
    pub x: i32,
    pub y: i32,
}
#[cfg(feature = "Unity+Mathematics+int2")]
unsafe impl quest_hook::libil2cpp::Type for crate::Unity::Mathematics::int2 {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "Unity.Mathematics";
    const CLASS_NAME: &'static str = "int2";
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
#[cfg(feature = "Unity+Mathematics+int2")]
unsafe impl quest_hook::libil2cpp::Argument for crate::Unity::Mathematics::int2 {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "Unity+Mathematics+int2")]
unsafe impl quest_hook::libil2cpp::Parameter for crate::Unity::Mathematics::int2 {
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
#[cfg(feature = "Unity+Mathematics+int2")]
unsafe impl quest_hook::libil2cpp::Returned for crate::Unity::Mathematics::int2 {
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
#[cfg(feature = "Unity+Mathematics+int2")]
unsafe impl quest_hook::libil2cpp::Return for crate::Unity::Mathematics::int2 {
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
#[cfg(feature = "Unity+Mathematics+int2")]
unsafe impl quest_hook::libil2cpp::ThisArgument for crate::Unity::Mathematics::int2 {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "Unity+Mathematics+int2")]
impl crate::Unity::Mathematics::int2 {
    #[cfg(feature = "Unity+Mathematics+int2+DebuggerProxy")]
    pub type DebuggerProxy = crate::Unity::Mathematics::int2_DebuggerProxy;
    pub fn Equals_Il2CppObject1(
        &mut self,
        o: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::Unity::Mathematics::int2 as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>),
                bool,
                1usize,
            >("Equals")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::Unity::Mathematics::int2 as quest_hook::libil2cpp::Type >
                    ::class(), "Equals", 1usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked(self, (o))? };
        Ok(__cordl_ret.into())
    }
    pub fn Equals_int2_0(
        &mut self,
        rhs: crate::Unity::Mathematics::int2,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::Unity::Mathematics::int2 as quest_hook::libil2cpp::Type>::class()
            .find_method::<(crate::Unity::Mathematics::int2), bool, 1usize>("Equals")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::Unity::Mathematics::int2 as quest_hook::libil2cpp::Type >
                    ::class(), "Equals", 1usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked(self, (rhs))? };
        Ok(__cordl_ret.into())
    }
    pub fn GetHashCode(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::Unity::Mathematics::int2 as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), i32, 0usize>("GetHashCode")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::Unity::Mathematics::int2 as quest_hook::libil2cpp::Type >
                    ::class(), "GetHashCode", 0usize
                )
            });
        let __cordl_ret: i32 = unsafe { method.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn ToString_0(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::Unity::Mathematics::int2 as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                0usize,
            >("ToString")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::Unity::Mathematics::int2 as quest_hook::libil2cpp::Type >
                    ::class(), "ToString", 0usize
                )
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
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::Unity::Mathematics::int2 as quest_hook::libil2cpp::Type>::class()
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
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::Unity::Mathematics::int2 as quest_hook::libil2cpp::Type >
                    ::class(), "ToString", 2usize
                )
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
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::Unity::Mathematics::int2 as quest_hook::libil2cpp::Type>::class()
            .find_method::<(bool), quest_hook::libil2cpp::Void, 1usize>(".ctor")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::Unity::Mathematics::int2 as quest_hook::libil2cpp::Type >
                    ::class(), ".ctor", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (v))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_bool2_4(
        &mut self,
        v: crate::Unity::Mathematics::bool2,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::Unity::Mathematics::int2 as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (crate::Unity::Mathematics::bool2),
                quest_hook::libil2cpp::Void,
                1usize,
            >(".ctor")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::Unity::Mathematics::int2 as quest_hook::libil2cpp::Type >
                    ::class(), ".ctor", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (v))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_double2_10(
        &mut self,
        v: crate::Unity::Mathematics::double2,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::Unity::Mathematics::int2 as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (crate::Unity::Mathematics::double2),
                quest_hook::libil2cpp::Void,
                1usize,
            >(".ctor")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::Unity::Mathematics::int2 as quest_hook::libil2cpp::Type >
                    ::class(), ".ctor", 1usize
                )
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
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::Unity::Mathematics::int2 as quest_hook::libil2cpp::Type>::class()
            .find_method::<(f32), quest_hook::libil2cpp::Void, 1usize>(".ctor")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::Unity::Mathematics::int2 as quest_hook::libil2cpp::Type >
                    ::class(), ".ctor", 1usize
                )
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
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::Unity::Mathematics::int2 as quest_hook::libil2cpp::Type>::class()
            .find_method::<(f64), quest_hook::libil2cpp::Void, 1usize>(".ctor")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::Unity::Mathematics::int2 as quest_hook::libil2cpp::Type >
                    ::class(), ".ctor", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (v))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_float2_8(
        &mut self,
        v: crate::Unity::Mathematics::float2,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::Unity::Mathematics::int2 as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (crate::Unity::Mathematics::float2),
                quest_hook::libil2cpp::Void,
                1usize,
            >(".ctor")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::Unity::Mathematics::int2 as quest_hook::libil2cpp::Type >
                    ::class(), ".ctor", 1usize
                )
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
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::Unity::Mathematics::int2 as quest_hook::libil2cpp::Type>::class()
            .find_method::<(i32), quest_hook::libil2cpp::Void, 1usize>(".ctor")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::Unity::Mathematics::int2 as quest_hook::libil2cpp::Type >
                    ::class(), ".ctor", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (v))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_i32_i32_0(
        &mut self,
        x: i32,
        y: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::Unity::Mathematics::int2 as quest_hook::libil2cpp::Type>::class()
            .find_method::<(i32, i32), quest_hook::libil2cpp::Void, 2usize>(".ctor")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::Unity::Mathematics::int2 as quest_hook::libil2cpp::Type >
                    ::class(), ".ctor", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (x, y))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_int2_1(
        &mut self,
        xy: crate::Unity::Mathematics::int2,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::Unity::Mathematics::int2 as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (crate::Unity::Mathematics::int2),
                quest_hook::libil2cpp::Void,
                1usize,
            >(".ctor")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::Unity::Mathematics::int2 as quest_hook::libil2cpp::Type >
                    ::class(), ".ctor", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (xy))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_u32_5(
        &mut self,
        v: u32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::Unity::Mathematics::int2 as quest_hook::libil2cpp::Type>::class()
            .find_method::<(u32), quest_hook::libil2cpp::Void, 1usize>(".ctor")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::Unity::Mathematics::int2 as quest_hook::libil2cpp::Type >
                    ::class(), ".ctor", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (v))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_uint2_6(
        &mut self,
        v: crate::Unity::Mathematics::uint2,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::Unity::Mathematics::int2 as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (crate::Unity::Mathematics::uint2),
                quest_hook::libil2cpp::Void,
                1usize,
            >(".ctor")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::Unity::Mathematics::int2 as quest_hook::libil2cpp::Type >
                    ::class(), ".ctor", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (v))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_Item(&mut self, index: i32) -> quest_hook::libil2cpp::Result<i32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::Unity::Mathematics::int2 as quest_hook::libil2cpp::Type>::class()
            .find_method::<(i32), i32, 1usize>("get_Item")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::Unity::Mathematics::int2 as quest_hook::libil2cpp::Type >
                    ::class(), "get_Item", 1usize
                )
            });
        let __cordl_ret: i32 = unsafe { method.invoke_unchecked(self, (index))? };
        Ok(__cordl_ret.into())
    }
    pub fn get_xx(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::int2> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::Unity::Mathematics::int2 as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), crate::Unity::Mathematics::int2, 0usize>("get_xx")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::Unity::Mathematics::int2 as quest_hook::libil2cpp::Type >
                    ::class(), "get_xx", 0usize
                )
            });
        let __cordl_ret: crate::Unity::Mathematics::int2 = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_xxx(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::int3> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::Unity::Mathematics::int2 as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), crate::Unity::Mathematics::int3, 0usize>("get_xxx")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::Unity::Mathematics::int2 as quest_hook::libil2cpp::Type >
                    ::class(), "get_xxx", 0usize
                )
            });
        let __cordl_ret: crate::Unity::Mathematics::int3 = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_xxxx(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::int4> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::Unity::Mathematics::int2 as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), crate::Unity::Mathematics::int4, 0usize>("get_xxxx")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::Unity::Mathematics::int2 as quest_hook::libil2cpp::Type >
                    ::class(), "get_xxxx", 0usize
                )
            });
        let __cordl_ret: crate::Unity::Mathematics::int4 = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_xxxy(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::int4> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::Unity::Mathematics::int2 as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), crate::Unity::Mathematics::int4, 0usize>("get_xxxy")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::Unity::Mathematics::int2 as quest_hook::libil2cpp::Type >
                    ::class(), "get_xxxy", 0usize
                )
            });
        let __cordl_ret: crate::Unity::Mathematics::int4 = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_xxy(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::int3> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::Unity::Mathematics::int2 as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), crate::Unity::Mathematics::int3, 0usize>("get_xxy")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::Unity::Mathematics::int2 as quest_hook::libil2cpp::Type >
                    ::class(), "get_xxy", 0usize
                )
            });
        let __cordl_ret: crate::Unity::Mathematics::int3 = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_xxyx(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::int4> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::Unity::Mathematics::int2 as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), crate::Unity::Mathematics::int4, 0usize>("get_xxyx")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::Unity::Mathematics::int2 as quest_hook::libil2cpp::Type >
                    ::class(), "get_xxyx", 0usize
                )
            });
        let __cordl_ret: crate::Unity::Mathematics::int4 = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_xxyy(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::int4> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::Unity::Mathematics::int2 as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), crate::Unity::Mathematics::int4, 0usize>("get_xxyy")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::Unity::Mathematics::int2 as quest_hook::libil2cpp::Type >
                    ::class(), "get_xxyy", 0usize
                )
            });
        let __cordl_ret: crate::Unity::Mathematics::int4 = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_xy(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::int2> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::Unity::Mathematics::int2 as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), crate::Unity::Mathematics::int2, 0usize>("get_xy")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::Unity::Mathematics::int2 as quest_hook::libil2cpp::Type >
                    ::class(), "get_xy", 0usize
                )
            });
        let __cordl_ret: crate::Unity::Mathematics::int2 = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_xyx(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::int3> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::Unity::Mathematics::int2 as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), crate::Unity::Mathematics::int3, 0usize>("get_xyx")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::Unity::Mathematics::int2 as quest_hook::libil2cpp::Type >
                    ::class(), "get_xyx", 0usize
                )
            });
        let __cordl_ret: crate::Unity::Mathematics::int3 = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_xyxx(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::int4> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::Unity::Mathematics::int2 as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), crate::Unity::Mathematics::int4, 0usize>("get_xyxx")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::Unity::Mathematics::int2 as quest_hook::libil2cpp::Type >
                    ::class(), "get_xyxx", 0usize
                )
            });
        let __cordl_ret: crate::Unity::Mathematics::int4 = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_xyxy(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::int4> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::Unity::Mathematics::int2 as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), crate::Unity::Mathematics::int4, 0usize>("get_xyxy")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::Unity::Mathematics::int2 as quest_hook::libil2cpp::Type >
                    ::class(), "get_xyxy", 0usize
                )
            });
        let __cordl_ret: crate::Unity::Mathematics::int4 = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_xyy(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::int3> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::Unity::Mathematics::int2 as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), crate::Unity::Mathematics::int3, 0usize>("get_xyy")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::Unity::Mathematics::int2 as quest_hook::libil2cpp::Type >
                    ::class(), "get_xyy", 0usize
                )
            });
        let __cordl_ret: crate::Unity::Mathematics::int3 = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_xyyx(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::int4> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::Unity::Mathematics::int2 as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), crate::Unity::Mathematics::int4, 0usize>("get_xyyx")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::Unity::Mathematics::int2 as quest_hook::libil2cpp::Type >
                    ::class(), "get_xyyx", 0usize
                )
            });
        let __cordl_ret: crate::Unity::Mathematics::int4 = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_xyyy(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::int4> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::Unity::Mathematics::int2 as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), crate::Unity::Mathematics::int4, 0usize>("get_xyyy")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::Unity::Mathematics::int2 as quest_hook::libil2cpp::Type >
                    ::class(), "get_xyyy", 0usize
                )
            });
        let __cordl_ret: crate::Unity::Mathematics::int4 = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_yx(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::int2> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::Unity::Mathematics::int2 as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), crate::Unity::Mathematics::int2, 0usize>("get_yx")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::Unity::Mathematics::int2 as quest_hook::libil2cpp::Type >
                    ::class(), "get_yx", 0usize
                )
            });
        let __cordl_ret: crate::Unity::Mathematics::int2 = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_yxx(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::int3> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::Unity::Mathematics::int2 as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), crate::Unity::Mathematics::int3, 0usize>("get_yxx")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::Unity::Mathematics::int2 as quest_hook::libil2cpp::Type >
                    ::class(), "get_yxx", 0usize
                )
            });
        let __cordl_ret: crate::Unity::Mathematics::int3 = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_yxxx(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::int4> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::Unity::Mathematics::int2 as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), crate::Unity::Mathematics::int4, 0usize>("get_yxxx")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::Unity::Mathematics::int2 as quest_hook::libil2cpp::Type >
                    ::class(), "get_yxxx", 0usize
                )
            });
        let __cordl_ret: crate::Unity::Mathematics::int4 = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_yxxy(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::int4> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::Unity::Mathematics::int2 as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), crate::Unity::Mathematics::int4, 0usize>("get_yxxy")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::Unity::Mathematics::int2 as quest_hook::libil2cpp::Type >
                    ::class(), "get_yxxy", 0usize
                )
            });
        let __cordl_ret: crate::Unity::Mathematics::int4 = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_yxy(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::int3> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::Unity::Mathematics::int2 as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), crate::Unity::Mathematics::int3, 0usize>("get_yxy")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::Unity::Mathematics::int2 as quest_hook::libil2cpp::Type >
                    ::class(), "get_yxy", 0usize
                )
            });
        let __cordl_ret: crate::Unity::Mathematics::int3 = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_yxyx(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::int4> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::Unity::Mathematics::int2 as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), crate::Unity::Mathematics::int4, 0usize>("get_yxyx")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::Unity::Mathematics::int2 as quest_hook::libil2cpp::Type >
                    ::class(), "get_yxyx", 0usize
                )
            });
        let __cordl_ret: crate::Unity::Mathematics::int4 = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_yxyy(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::int4> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::Unity::Mathematics::int2 as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), crate::Unity::Mathematics::int4, 0usize>("get_yxyy")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::Unity::Mathematics::int2 as quest_hook::libil2cpp::Type >
                    ::class(), "get_yxyy", 0usize
                )
            });
        let __cordl_ret: crate::Unity::Mathematics::int4 = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_yy(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::int2> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::Unity::Mathematics::int2 as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), crate::Unity::Mathematics::int2, 0usize>("get_yy")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::Unity::Mathematics::int2 as quest_hook::libil2cpp::Type >
                    ::class(), "get_yy", 0usize
                )
            });
        let __cordl_ret: crate::Unity::Mathematics::int2 = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_yyx(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::int3> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::Unity::Mathematics::int2 as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), crate::Unity::Mathematics::int3, 0usize>("get_yyx")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::Unity::Mathematics::int2 as quest_hook::libil2cpp::Type >
                    ::class(), "get_yyx", 0usize
                )
            });
        let __cordl_ret: crate::Unity::Mathematics::int3 = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_yyxx(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::int4> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::Unity::Mathematics::int2 as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), crate::Unity::Mathematics::int4, 0usize>("get_yyxx")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::Unity::Mathematics::int2 as quest_hook::libil2cpp::Type >
                    ::class(), "get_yyxx", 0usize
                )
            });
        let __cordl_ret: crate::Unity::Mathematics::int4 = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_yyxy(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::int4> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::Unity::Mathematics::int2 as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), crate::Unity::Mathematics::int4, 0usize>("get_yyxy")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::Unity::Mathematics::int2 as quest_hook::libil2cpp::Type >
                    ::class(), "get_yyxy", 0usize
                )
            });
        let __cordl_ret: crate::Unity::Mathematics::int4 = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_yyy(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::int3> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::Unity::Mathematics::int2 as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), crate::Unity::Mathematics::int3, 0usize>("get_yyy")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::Unity::Mathematics::int2 as quest_hook::libil2cpp::Type >
                    ::class(), "get_yyy", 0usize
                )
            });
        let __cordl_ret: crate::Unity::Mathematics::int3 = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_yyyx(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::int4> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::Unity::Mathematics::int2 as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), crate::Unity::Mathematics::int4, 0usize>("get_yyyx")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::Unity::Mathematics::int2 as quest_hook::libil2cpp::Type >
                    ::class(), "get_yyyx", 0usize
                )
            });
        let __cordl_ret: crate::Unity::Mathematics::int4 = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_yyyy(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::int4> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::Unity::Mathematics::int2 as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), crate::Unity::Mathematics::int4, 0usize>("get_yyyy")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::Unity::Mathematics::int2 as quest_hook::libil2cpp::Type >
                    ::class(), "get_yyyy", 0usize
                )
            });
        let __cordl_ret: crate::Unity::Mathematics::int4 = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn op_Addition_i32_int2_2(
        lhs: i32,
        rhs: crate::Unity::Mathematics::int2,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::int2> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::Unity::Mathematics::int2 as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (i32, crate::Unity::Mathematics::int2),
                crate::Unity::Mathematics::int2,
                2usize,
            >("op_Addition")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::Unity::Mathematics::int2 as quest_hook::libil2cpp::Type >
                    ::class(), "op_Addition", 2usize
                )
            });
        let __cordl_ret: crate::Unity::Mathematics::int2 = unsafe {
            method.invoke_unchecked((), (lhs, rhs))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn op_Addition_int2_i32_1(
        lhs: crate::Unity::Mathematics::int2,
        rhs: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::int2> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::Unity::Mathematics::int2 as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::Unity::Mathematics::int2, i32),
                crate::Unity::Mathematics::int2,
                2usize,
            >("op_Addition")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::Unity::Mathematics::int2 as quest_hook::libil2cpp::Type >
                    ::class(), "op_Addition", 2usize
                )
            });
        let __cordl_ret: crate::Unity::Mathematics::int2 = unsafe {
            method.invoke_unchecked((), (lhs, rhs))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn op_Addition_int2_int2_0(
        lhs: crate::Unity::Mathematics::int2,
        rhs: crate::Unity::Mathematics::int2,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::int2> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::Unity::Mathematics::int2 as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::Unity::Mathematics::int2, crate::Unity::Mathematics::int2),
                crate::Unity::Mathematics::int2,
                2usize,
            >("op_Addition")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::Unity::Mathematics::int2 as quest_hook::libil2cpp::Type >
                    ::class(), "op_Addition", 2usize
                )
            });
        let __cordl_ret: crate::Unity::Mathematics::int2 = unsafe {
            method.invoke_unchecked((), (lhs, rhs))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn op_BitwiseAnd_i32_int2_2(
        lhs: i32,
        rhs: crate::Unity::Mathematics::int2,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::int2> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::Unity::Mathematics::int2 as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (i32, crate::Unity::Mathematics::int2),
                crate::Unity::Mathematics::int2,
                2usize,
            >("op_BitwiseAnd")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::Unity::Mathematics::int2 as quest_hook::libil2cpp::Type >
                    ::class(), "op_BitwiseAnd", 2usize
                )
            });
        let __cordl_ret: crate::Unity::Mathematics::int2 = unsafe {
            method.invoke_unchecked((), (lhs, rhs))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn op_BitwiseAnd_int2_i32_1(
        lhs: crate::Unity::Mathematics::int2,
        rhs: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::int2> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::Unity::Mathematics::int2 as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::Unity::Mathematics::int2, i32),
                crate::Unity::Mathematics::int2,
                2usize,
            >("op_BitwiseAnd")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::Unity::Mathematics::int2 as quest_hook::libil2cpp::Type >
                    ::class(), "op_BitwiseAnd", 2usize
                )
            });
        let __cordl_ret: crate::Unity::Mathematics::int2 = unsafe {
            method.invoke_unchecked((), (lhs, rhs))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn op_BitwiseAnd_int2_int2_0(
        lhs: crate::Unity::Mathematics::int2,
        rhs: crate::Unity::Mathematics::int2,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::int2> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::Unity::Mathematics::int2 as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::Unity::Mathematics::int2, crate::Unity::Mathematics::int2),
                crate::Unity::Mathematics::int2,
                2usize,
            >("op_BitwiseAnd")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::Unity::Mathematics::int2 as quest_hook::libil2cpp::Type >
                    ::class(), "op_BitwiseAnd", 2usize
                )
            });
        let __cordl_ret: crate::Unity::Mathematics::int2 = unsafe {
            method.invoke_unchecked((), (lhs, rhs))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn op_BitwiseOr_i32_int2_2(
        lhs: i32,
        rhs: crate::Unity::Mathematics::int2,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::int2> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::Unity::Mathematics::int2 as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (i32, crate::Unity::Mathematics::int2),
                crate::Unity::Mathematics::int2,
                2usize,
            >("op_BitwiseOr")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::Unity::Mathematics::int2 as quest_hook::libil2cpp::Type >
                    ::class(), "op_BitwiseOr", 2usize
                )
            });
        let __cordl_ret: crate::Unity::Mathematics::int2 = unsafe {
            method.invoke_unchecked((), (lhs, rhs))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn op_BitwiseOr_int2_i32_1(
        lhs: crate::Unity::Mathematics::int2,
        rhs: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::int2> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::Unity::Mathematics::int2 as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::Unity::Mathematics::int2, i32),
                crate::Unity::Mathematics::int2,
                2usize,
            >("op_BitwiseOr")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::Unity::Mathematics::int2 as quest_hook::libil2cpp::Type >
                    ::class(), "op_BitwiseOr", 2usize
                )
            });
        let __cordl_ret: crate::Unity::Mathematics::int2 = unsafe {
            method.invoke_unchecked((), (lhs, rhs))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn op_BitwiseOr_int2_int2_0(
        lhs: crate::Unity::Mathematics::int2,
        rhs: crate::Unity::Mathematics::int2,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::int2> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::Unity::Mathematics::int2 as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::Unity::Mathematics::int2, crate::Unity::Mathematics::int2),
                crate::Unity::Mathematics::int2,
                2usize,
            >("op_BitwiseOr")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::Unity::Mathematics::int2 as quest_hook::libil2cpp::Type >
                    ::class(), "op_BitwiseOr", 2usize
                )
            });
        let __cordl_ret: crate::Unity::Mathematics::int2 = unsafe {
            method.invoke_unchecked((), (lhs, rhs))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn op_Decrement(
        val: crate::Unity::Mathematics::int2,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::int2> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::Unity::Mathematics::int2 as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::Unity::Mathematics::int2),
                crate::Unity::Mathematics::int2,
                1usize,
            >("op_Decrement")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::Unity::Mathematics::int2 as quest_hook::libil2cpp::Type >
                    ::class(), "op_Decrement", 1usize
                )
            });
        let __cordl_ret: crate::Unity::Mathematics::int2 = unsafe {
            method.invoke_unchecked((), (val))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn op_Division_i32_int2_2(
        lhs: i32,
        rhs: crate::Unity::Mathematics::int2,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::int2> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::Unity::Mathematics::int2 as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (i32, crate::Unity::Mathematics::int2),
                crate::Unity::Mathematics::int2,
                2usize,
            >("op_Division")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::Unity::Mathematics::int2 as quest_hook::libil2cpp::Type >
                    ::class(), "op_Division", 2usize
                )
            });
        let __cordl_ret: crate::Unity::Mathematics::int2 = unsafe {
            method.invoke_unchecked((), (lhs, rhs))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn op_Division_int2_i32_1(
        lhs: crate::Unity::Mathematics::int2,
        rhs: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::int2> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::Unity::Mathematics::int2 as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::Unity::Mathematics::int2, i32),
                crate::Unity::Mathematics::int2,
                2usize,
            >("op_Division")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::Unity::Mathematics::int2 as quest_hook::libil2cpp::Type >
                    ::class(), "op_Division", 2usize
                )
            });
        let __cordl_ret: crate::Unity::Mathematics::int2 = unsafe {
            method.invoke_unchecked((), (lhs, rhs))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn op_Division_int2_int2_0(
        lhs: crate::Unity::Mathematics::int2,
        rhs: crate::Unity::Mathematics::int2,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::int2> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::Unity::Mathematics::int2 as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::Unity::Mathematics::int2, crate::Unity::Mathematics::int2),
                crate::Unity::Mathematics::int2,
                2usize,
            >("op_Division")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::Unity::Mathematics::int2 as quest_hook::libil2cpp::Type >
                    ::class(), "op_Division", 2usize
                )
            });
        let __cordl_ret: crate::Unity::Mathematics::int2 = unsafe {
            method.invoke_unchecked((), (lhs, rhs))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn op_Equality_i32_int2_2(
        lhs: i32,
        rhs: crate::Unity::Mathematics::int2,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::bool2> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::Unity::Mathematics::int2 as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (i32, crate::Unity::Mathematics::int2),
                crate::Unity::Mathematics::bool2,
                2usize,
            >("op_Equality")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::Unity::Mathematics::int2 as quest_hook::libil2cpp::Type >
                    ::class(), "op_Equality", 2usize
                )
            });
        let __cordl_ret: crate::Unity::Mathematics::bool2 = unsafe {
            method.invoke_unchecked((), (lhs, rhs))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn op_Equality_int2_i32_1(
        lhs: crate::Unity::Mathematics::int2,
        rhs: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::bool2> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::Unity::Mathematics::int2 as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::Unity::Mathematics::int2, i32),
                crate::Unity::Mathematics::bool2,
                2usize,
            >("op_Equality")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::Unity::Mathematics::int2 as quest_hook::libil2cpp::Type >
                    ::class(), "op_Equality", 2usize
                )
            });
        let __cordl_ret: crate::Unity::Mathematics::bool2 = unsafe {
            method.invoke_unchecked((), (lhs, rhs))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn op_Equality_int2_int2_0(
        lhs: crate::Unity::Mathematics::int2,
        rhs: crate::Unity::Mathematics::int2,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::bool2> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::Unity::Mathematics::int2 as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::Unity::Mathematics::int2, crate::Unity::Mathematics::int2),
                crate::Unity::Mathematics::bool2,
                2usize,
            >("op_Equality")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::Unity::Mathematics::int2 as quest_hook::libil2cpp::Type >
                    ::class(), "op_Equality", 2usize
                )
            });
        let __cordl_ret: crate::Unity::Mathematics::bool2 = unsafe {
            method.invoke_unchecked((), (lhs, rhs))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn op_ExclusiveOr_i32_int2_2(
        lhs: i32,
        rhs: crate::Unity::Mathematics::int2,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::int2> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::Unity::Mathematics::int2 as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (i32, crate::Unity::Mathematics::int2),
                crate::Unity::Mathematics::int2,
                2usize,
            >("op_ExclusiveOr")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::Unity::Mathematics::int2 as quest_hook::libil2cpp::Type >
                    ::class(), "op_ExclusiveOr", 2usize
                )
            });
        let __cordl_ret: crate::Unity::Mathematics::int2 = unsafe {
            method.invoke_unchecked((), (lhs, rhs))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn op_ExclusiveOr_int2_i32_1(
        lhs: crate::Unity::Mathematics::int2,
        rhs: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::int2> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::Unity::Mathematics::int2 as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::Unity::Mathematics::int2, i32),
                crate::Unity::Mathematics::int2,
                2usize,
            >("op_ExclusiveOr")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::Unity::Mathematics::int2 as quest_hook::libil2cpp::Type >
                    ::class(), "op_ExclusiveOr", 2usize
                )
            });
        let __cordl_ret: crate::Unity::Mathematics::int2 = unsafe {
            method.invoke_unchecked((), (lhs, rhs))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn op_ExclusiveOr_int2_int2_0(
        lhs: crate::Unity::Mathematics::int2,
        rhs: crate::Unity::Mathematics::int2,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::int2> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::Unity::Mathematics::int2 as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::Unity::Mathematics::int2, crate::Unity::Mathematics::int2),
                crate::Unity::Mathematics::int2,
                2usize,
            >("op_ExclusiveOr")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::Unity::Mathematics::int2 as quest_hook::libil2cpp::Type >
                    ::class(), "op_ExclusiveOr", 2usize
                )
            });
        let __cordl_ret: crate::Unity::Mathematics::int2 = unsafe {
            method.invoke_unchecked((), (lhs, rhs))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn op_Explicit__cordl_bool0(
        v: bool,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::int2> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::Unity::Mathematics::int2 as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (bool),
                crate::Unity::Mathematics::int2,
                1usize,
            >("op_Explicit")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::Unity::Mathematics::int2 as quest_hook::libil2cpp::Type >
                    ::class(), "op_Explicit", 1usize
                )
            });
        let __cordl_ret: crate::Unity::Mathematics::int2 = unsafe {
            method.invoke_unchecked((), (v))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn op_Explicit_bool2_1(
        v: crate::Unity::Mathematics::bool2,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::int2> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::Unity::Mathematics::int2 as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::Unity::Mathematics::bool2),
                crate::Unity::Mathematics::int2,
                1usize,
            >("op_Explicit")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::Unity::Mathematics::int2 as quest_hook::libil2cpp::Type >
                    ::class(), "op_Explicit", 1usize
                )
            });
        let __cordl_ret: crate::Unity::Mathematics::int2 = unsafe {
            method.invoke_unchecked((), (v))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn op_Explicit_double2_7(
        v: crate::Unity::Mathematics::double2,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::int2> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::Unity::Mathematics::int2 as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::Unity::Mathematics::double2),
                crate::Unity::Mathematics::int2,
                1usize,
            >("op_Explicit")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::Unity::Mathematics::int2 as quest_hook::libil2cpp::Type >
                    ::class(), "op_Explicit", 1usize
                )
            });
        let __cordl_ret: crate::Unity::Mathematics::int2 = unsafe {
            method.invoke_unchecked((), (v))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn op_Explicit_f32_4(
        v: f32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::int2> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::Unity::Mathematics::int2 as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (f32),
                crate::Unity::Mathematics::int2,
                1usize,
            >("op_Explicit")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::Unity::Mathematics::int2 as quest_hook::libil2cpp::Type >
                    ::class(), "op_Explicit", 1usize
                )
            });
        let __cordl_ret: crate::Unity::Mathematics::int2 = unsafe {
            method.invoke_unchecked((), (v))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn op_Explicit_f64_6(
        v: f64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::int2> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::Unity::Mathematics::int2 as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (f64),
                crate::Unity::Mathematics::int2,
                1usize,
            >("op_Explicit")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::Unity::Mathematics::int2 as quest_hook::libil2cpp::Type >
                    ::class(), "op_Explicit", 1usize
                )
            });
        let __cordl_ret: crate::Unity::Mathematics::int2 = unsafe {
            method.invoke_unchecked((), (v))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn op_Explicit_float2_5(
        v: crate::Unity::Mathematics::float2,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::int2> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::Unity::Mathematics::int2 as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::Unity::Mathematics::float2),
                crate::Unity::Mathematics::int2,
                1usize,
            >("op_Explicit")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::Unity::Mathematics::int2 as quest_hook::libil2cpp::Type >
                    ::class(), "op_Explicit", 1usize
                )
            });
        let __cordl_ret: crate::Unity::Mathematics::int2 = unsafe {
            method.invoke_unchecked((), (v))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn op_Explicit_u32_2(
        v: u32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::int2> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::Unity::Mathematics::int2 as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (u32),
                crate::Unity::Mathematics::int2,
                1usize,
            >("op_Explicit")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::Unity::Mathematics::int2 as quest_hook::libil2cpp::Type >
                    ::class(), "op_Explicit", 1usize
                )
            });
        let __cordl_ret: crate::Unity::Mathematics::int2 = unsafe {
            method.invoke_unchecked((), (v))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn op_Explicit_uint2_3(
        v: crate::Unity::Mathematics::uint2,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::int2> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::Unity::Mathematics::int2 as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::Unity::Mathematics::uint2),
                crate::Unity::Mathematics::int2,
                1usize,
            >("op_Explicit")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::Unity::Mathematics::int2 as quest_hook::libil2cpp::Type >
                    ::class(), "op_Explicit", 1usize
                )
            });
        let __cordl_ret: crate::Unity::Mathematics::int2 = unsafe {
            method.invoke_unchecked((), (v))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn op_GreaterThanOrEqual_i32_int2_2(
        lhs: i32,
        rhs: crate::Unity::Mathematics::int2,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::bool2> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::Unity::Mathematics::int2 as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (i32, crate::Unity::Mathematics::int2),
                crate::Unity::Mathematics::bool2,
                2usize,
            >("op_GreaterThanOrEqual")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::Unity::Mathematics::int2 as quest_hook::libil2cpp::Type >
                    ::class(), "op_GreaterThanOrEqual", 2usize
                )
            });
        let __cordl_ret: crate::Unity::Mathematics::bool2 = unsafe {
            method.invoke_unchecked((), (lhs, rhs))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn op_GreaterThanOrEqual_int2_i32_1(
        lhs: crate::Unity::Mathematics::int2,
        rhs: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::bool2> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::Unity::Mathematics::int2 as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::Unity::Mathematics::int2, i32),
                crate::Unity::Mathematics::bool2,
                2usize,
            >("op_GreaterThanOrEqual")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::Unity::Mathematics::int2 as quest_hook::libil2cpp::Type >
                    ::class(), "op_GreaterThanOrEqual", 2usize
                )
            });
        let __cordl_ret: crate::Unity::Mathematics::bool2 = unsafe {
            method.invoke_unchecked((), (lhs, rhs))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn op_GreaterThanOrEqual_int2_int2_0(
        lhs: crate::Unity::Mathematics::int2,
        rhs: crate::Unity::Mathematics::int2,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::bool2> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::Unity::Mathematics::int2 as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::Unity::Mathematics::int2, crate::Unity::Mathematics::int2),
                crate::Unity::Mathematics::bool2,
                2usize,
            >("op_GreaterThanOrEqual")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::Unity::Mathematics::int2 as quest_hook::libil2cpp::Type >
                    ::class(), "op_GreaterThanOrEqual", 2usize
                )
            });
        let __cordl_ret: crate::Unity::Mathematics::bool2 = unsafe {
            method.invoke_unchecked((), (lhs, rhs))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn op_GreaterThan_i32_int2_2(
        lhs: i32,
        rhs: crate::Unity::Mathematics::int2,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::bool2> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::Unity::Mathematics::int2 as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (i32, crate::Unity::Mathematics::int2),
                crate::Unity::Mathematics::bool2,
                2usize,
            >("op_GreaterThan")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::Unity::Mathematics::int2 as quest_hook::libil2cpp::Type >
                    ::class(), "op_GreaterThan", 2usize
                )
            });
        let __cordl_ret: crate::Unity::Mathematics::bool2 = unsafe {
            method.invoke_unchecked((), (lhs, rhs))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn op_GreaterThan_int2_i32_1(
        lhs: crate::Unity::Mathematics::int2,
        rhs: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::bool2> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::Unity::Mathematics::int2 as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::Unity::Mathematics::int2, i32),
                crate::Unity::Mathematics::bool2,
                2usize,
            >("op_GreaterThan")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::Unity::Mathematics::int2 as quest_hook::libil2cpp::Type >
                    ::class(), "op_GreaterThan", 2usize
                )
            });
        let __cordl_ret: crate::Unity::Mathematics::bool2 = unsafe {
            method.invoke_unchecked((), (lhs, rhs))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn op_GreaterThan_int2_int2_0(
        lhs: crate::Unity::Mathematics::int2,
        rhs: crate::Unity::Mathematics::int2,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::bool2> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::Unity::Mathematics::int2 as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::Unity::Mathematics::int2, crate::Unity::Mathematics::int2),
                crate::Unity::Mathematics::bool2,
                2usize,
            >("op_GreaterThan")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::Unity::Mathematics::int2 as quest_hook::libil2cpp::Type >
                    ::class(), "op_GreaterThan", 2usize
                )
            });
        let __cordl_ret: crate::Unity::Mathematics::bool2 = unsafe {
            method.invoke_unchecked((), (lhs, rhs))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn op_Implicit(
        v: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::int2> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::Unity::Mathematics::int2 as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (i32),
                crate::Unity::Mathematics::int2,
                1usize,
            >("op_Implicit")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::Unity::Mathematics::int2 as quest_hook::libil2cpp::Type >
                    ::class(), "op_Implicit", 1usize
                )
            });
        let __cordl_ret: crate::Unity::Mathematics::int2 = unsafe {
            method.invoke_unchecked((), (v))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn op_Increment(
        val: crate::Unity::Mathematics::int2,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::int2> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::Unity::Mathematics::int2 as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::Unity::Mathematics::int2),
                crate::Unity::Mathematics::int2,
                1usize,
            >("op_Increment")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::Unity::Mathematics::int2 as quest_hook::libil2cpp::Type >
                    ::class(), "op_Increment", 1usize
                )
            });
        let __cordl_ret: crate::Unity::Mathematics::int2 = unsafe {
            method.invoke_unchecked((), (val))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn op_Inequality_i32_int2_2(
        lhs: i32,
        rhs: crate::Unity::Mathematics::int2,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::bool2> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::Unity::Mathematics::int2 as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (i32, crate::Unity::Mathematics::int2),
                crate::Unity::Mathematics::bool2,
                2usize,
            >("op_Inequality")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::Unity::Mathematics::int2 as quest_hook::libil2cpp::Type >
                    ::class(), "op_Inequality", 2usize
                )
            });
        let __cordl_ret: crate::Unity::Mathematics::bool2 = unsafe {
            method.invoke_unchecked((), (lhs, rhs))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn op_Inequality_int2_i32_1(
        lhs: crate::Unity::Mathematics::int2,
        rhs: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::bool2> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::Unity::Mathematics::int2 as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::Unity::Mathematics::int2, i32),
                crate::Unity::Mathematics::bool2,
                2usize,
            >("op_Inequality")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::Unity::Mathematics::int2 as quest_hook::libil2cpp::Type >
                    ::class(), "op_Inequality", 2usize
                )
            });
        let __cordl_ret: crate::Unity::Mathematics::bool2 = unsafe {
            method.invoke_unchecked((), (lhs, rhs))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn op_Inequality_int2_int2_0(
        lhs: crate::Unity::Mathematics::int2,
        rhs: crate::Unity::Mathematics::int2,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::bool2> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::Unity::Mathematics::int2 as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::Unity::Mathematics::int2, crate::Unity::Mathematics::int2),
                crate::Unity::Mathematics::bool2,
                2usize,
            >("op_Inequality")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::Unity::Mathematics::int2 as quest_hook::libil2cpp::Type >
                    ::class(), "op_Inequality", 2usize
                )
            });
        let __cordl_ret: crate::Unity::Mathematics::bool2 = unsafe {
            method.invoke_unchecked((), (lhs, rhs))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn op_LeftShift(
        x: crate::Unity::Mathematics::int2,
        n: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::int2> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::Unity::Mathematics::int2 as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::Unity::Mathematics::int2, i32),
                crate::Unity::Mathematics::int2,
                2usize,
            >("op_LeftShift")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::Unity::Mathematics::int2 as quest_hook::libil2cpp::Type >
                    ::class(), "op_LeftShift", 2usize
                )
            });
        let __cordl_ret: crate::Unity::Mathematics::int2 = unsafe {
            method.invoke_unchecked((), (x, n))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn op_LessThanOrEqual_i32_int2_2(
        lhs: i32,
        rhs: crate::Unity::Mathematics::int2,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::bool2> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::Unity::Mathematics::int2 as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (i32, crate::Unity::Mathematics::int2),
                crate::Unity::Mathematics::bool2,
                2usize,
            >("op_LessThanOrEqual")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::Unity::Mathematics::int2 as quest_hook::libil2cpp::Type >
                    ::class(), "op_LessThanOrEqual", 2usize
                )
            });
        let __cordl_ret: crate::Unity::Mathematics::bool2 = unsafe {
            method.invoke_unchecked((), (lhs, rhs))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn op_LessThanOrEqual_int2_i32_1(
        lhs: crate::Unity::Mathematics::int2,
        rhs: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::bool2> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::Unity::Mathematics::int2 as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::Unity::Mathematics::int2, i32),
                crate::Unity::Mathematics::bool2,
                2usize,
            >("op_LessThanOrEqual")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::Unity::Mathematics::int2 as quest_hook::libil2cpp::Type >
                    ::class(), "op_LessThanOrEqual", 2usize
                )
            });
        let __cordl_ret: crate::Unity::Mathematics::bool2 = unsafe {
            method.invoke_unchecked((), (lhs, rhs))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn op_LessThanOrEqual_int2_int2_0(
        lhs: crate::Unity::Mathematics::int2,
        rhs: crate::Unity::Mathematics::int2,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::bool2> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::Unity::Mathematics::int2 as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::Unity::Mathematics::int2, crate::Unity::Mathematics::int2),
                crate::Unity::Mathematics::bool2,
                2usize,
            >("op_LessThanOrEqual")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::Unity::Mathematics::int2 as quest_hook::libil2cpp::Type >
                    ::class(), "op_LessThanOrEqual", 2usize
                )
            });
        let __cordl_ret: crate::Unity::Mathematics::bool2 = unsafe {
            method.invoke_unchecked((), (lhs, rhs))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn op_LessThan_i32_int2_2(
        lhs: i32,
        rhs: crate::Unity::Mathematics::int2,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::bool2> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::Unity::Mathematics::int2 as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (i32, crate::Unity::Mathematics::int2),
                crate::Unity::Mathematics::bool2,
                2usize,
            >("op_LessThan")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::Unity::Mathematics::int2 as quest_hook::libil2cpp::Type >
                    ::class(), "op_LessThan", 2usize
                )
            });
        let __cordl_ret: crate::Unity::Mathematics::bool2 = unsafe {
            method.invoke_unchecked((), (lhs, rhs))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn op_LessThan_int2_i32_1(
        lhs: crate::Unity::Mathematics::int2,
        rhs: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::bool2> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::Unity::Mathematics::int2 as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::Unity::Mathematics::int2, i32),
                crate::Unity::Mathematics::bool2,
                2usize,
            >("op_LessThan")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::Unity::Mathematics::int2 as quest_hook::libil2cpp::Type >
                    ::class(), "op_LessThan", 2usize
                )
            });
        let __cordl_ret: crate::Unity::Mathematics::bool2 = unsafe {
            method.invoke_unchecked((), (lhs, rhs))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn op_LessThan_int2_int2_0(
        lhs: crate::Unity::Mathematics::int2,
        rhs: crate::Unity::Mathematics::int2,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::bool2> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::Unity::Mathematics::int2 as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::Unity::Mathematics::int2, crate::Unity::Mathematics::int2),
                crate::Unity::Mathematics::bool2,
                2usize,
            >("op_LessThan")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::Unity::Mathematics::int2 as quest_hook::libil2cpp::Type >
                    ::class(), "op_LessThan", 2usize
                )
            });
        let __cordl_ret: crate::Unity::Mathematics::bool2 = unsafe {
            method.invoke_unchecked((), (lhs, rhs))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn op_Modulus_i32_int2_2(
        lhs: i32,
        rhs: crate::Unity::Mathematics::int2,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::int2> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::Unity::Mathematics::int2 as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (i32, crate::Unity::Mathematics::int2),
                crate::Unity::Mathematics::int2,
                2usize,
            >("op_Modulus")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::Unity::Mathematics::int2 as quest_hook::libil2cpp::Type >
                    ::class(), "op_Modulus", 2usize
                )
            });
        let __cordl_ret: crate::Unity::Mathematics::int2 = unsafe {
            method.invoke_unchecked((), (lhs, rhs))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn op_Modulus_int2_i32_1(
        lhs: crate::Unity::Mathematics::int2,
        rhs: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::int2> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::Unity::Mathematics::int2 as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::Unity::Mathematics::int2, i32),
                crate::Unity::Mathematics::int2,
                2usize,
            >("op_Modulus")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::Unity::Mathematics::int2 as quest_hook::libil2cpp::Type >
                    ::class(), "op_Modulus", 2usize
                )
            });
        let __cordl_ret: crate::Unity::Mathematics::int2 = unsafe {
            method.invoke_unchecked((), (lhs, rhs))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn op_Modulus_int2_int2_0(
        lhs: crate::Unity::Mathematics::int2,
        rhs: crate::Unity::Mathematics::int2,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::int2> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::Unity::Mathematics::int2 as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::Unity::Mathematics::int2, crate::Unity::Mathematics::int2),
                crate::Unity::Mathematics::int2,
                2usize,
            >("op_Modulus")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::Unity::Mathematics::int2 as quest_hook::libil2cpp::Type >
                    ::class(), "op_Modulus", 2usize
                )
            });
        let __cordl_ret: crate::Unity::Mathematics::int2 = unsafe {
            method.invoke_unchecked((), (lhs, rhs))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn op_Multiply_i32_int2_2(
        lhs: i32,
        rhs: crate::Unity::Mathematics::int2,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::int2> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::Unity::Mathematics::int2 as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (i32, crate::Unity::Mathematics::int2),
                crate::Unity::Mathematics::int2,
                2usize,
            >("op_Multiply")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::Unity::Mathematics::int2 as quest_hook::libil2cpp::Type >
                    ::class(), "op_Multiply", 2usize
                )
            });
        let __cordl_ret: crate::Unity::Mathematics::int2 = unsafe {
            method.invoke_unchecked((), (lhs, rhs))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn op_Multiply_int2_i32_1(
        lhs: crate::Unity::Mathematics::int2,
        rhs: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::int2> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::Unity::Mathematics::int2 as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::Unity::Mathematics::int2, i32),
                crate::Unity::Mathematics::int2,
                2usize,
            >("op_Multiply")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::Unity::Mathematics::int2 as quest_hook::libil2cpp::Type >
                    ::class(), "op_Multiply", 2usize
                )
            });
        let __cordl_ret: crate::Unity::Mathematics::int2 = unsafe {
            method.invoke_unchecked((), (lhs, rhs))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn op_Multiply_int2_int2_0(
        lhs: crate::Unity::Mathematics::int2,
        rhs: crate::Unity::Mathematics::int2,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::int2> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::Unity::Mathematics::int2 as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::Unity::Mathematics::int2, crate::Unity::Mathematics::int2),
                crate::Unity::Mathematics::int2,
                2usize,
            >("op_Multiply")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::Unity::Mathematics::int2 as quest_hook::libil2cpp::Type >
                    ::class(), "op_Multiply", 2usize
                )
            });
        let __cordl_ret: crate::Unity::Mathematics::int2 = unsafe {
            method.invoke_unchecked((), (lhs, rhs))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn op_OnesComplement(
        val: crate::Unity::Mathematics::int2,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::int2> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::Unity::Mathematics::int2 as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::Unity::Mathematics::int2),
                crate::Unity::Mathematics::int2,
                1usize,
            >("op_OnesComplement")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::Unity::Mathematics::int2 as quest_hook::libil2cpp::Type >
                    ::class(), "op_OnesComplement", 1usize
                )
            });
        let __cordl_ret: crate::Unity::Mathematics::int2 = unsafe {
            method.invoke_unchecked((), (val))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn op_RightShift(
        x: crate::Unity::Mathematics::int2,
        n: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::int2> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::Unity::Mathematics::int2 as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::Unity::Mathematics::int2, i32),
                crate::Unity::Mathematics::int2,
                2usize,
            >("op_RightShift")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::Unity::Mathematics::int2 as quest_hook::libil2cpp::Type >
                    ::class(), "op_RightShift", 2usize
                )
            });
        let __cordl_ret: crate::Unity::Mathematics::int2 = unsafe {
            method.invoke_unchecked((), (x, n))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn op_Subtraction_i32_int2_2(
        lhs: i32,
        rhs: crate::Unity::Mathematics::int2,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::int2> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::Unity::Mathematics::int2 as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (i32, crate::Unity::Mathematics::int2),
                crate::Unity::Mathematics::int2,
                2usize,
            >("op_Subtraction")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::Unity::Mathematics::int2 as quest_hook::libil2cpp::Type >
                    ::class(), "op_Subtraction", 2usize
                )
            });
        let __cordl_ret: crate::Unity::Mathematics::int2 = unsafe {
            method.invoke_unchecked((), (lhs, rhs))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn op_Subtraction_int2_i32_1(
        lhs: crate::Unity::Mathematics::int2,
        rhs: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::int2> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::Unity::Mathematics::int2 as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::Unity::Mathematics::int2, i32),
                crate::Unity::Mathematics::int2,
                2usize,
            >("op_Subtraction")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::Unity::Mathematics::int2 as quest_hook::libil2cpp::Type >
                    ::class(), "op_Subtraction", 2usize
                )
            });
        let __cordl_ret: crate::Unity::Mathematics::int2 = unsafe {
            method.invoke_unchecked((), (lhs, rhs))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn op_Subtraction_int2_int2_0(
        lhs: crate::Unity::Mathematics::int2,
        rhs: crate::Unity::Mathematics::int2,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::int2> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::Unity::Mathematics::int2 as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::Unity::Mathematics::int2, crate::Unity::Mathematics::int2),
                crate::Unity::Mathematics::int2,
                2usize,
            >("op_Subtraction")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::Unity::Mathematics::int2 as quest_hook::libil2cpp::Type >
                    ::class(), "op_Subtraction", 2usize
                )
            });
        let __cordl_ret: crate::Unity::Mathematics::int2 = unsafe {
            method.invoke_unchecked((), (lhs, rhs))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn op_UnaryNegation(
        val: crate::Unity::Mathematics::int2,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::int2> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::Unity::Mathematics::int2 as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::Unity::Mathematics::int2),
                crate::Unity::Mathematics::int2,
                1usize,
            >("op_UnaryNegation")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::Unity::Mathematics::int2 as quest_hook::libil2cpp::Type >
                    ::class(), "op_UnaryNegation", 1usize
                )
            });
        let __cordl_ret: crate::Unity::Mathematics::int2 = unsafe {
            method.invoke_unchecked((), (val))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn op_UnaryPlus(
        val: crate::Unity::Mathematics::int2,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::int2> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::Unity::Mathematics::int2 as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::Unity::Mathematics::int2),
                crate::Unity::Mathematics::int2,
                1usize,
            >("op_UnaryPlus")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::Unity::Mathematics::int2 as quest_hook::libil2cpp::Type >
                    ::class(), "op_UnaryPlus", 1usize
                )
            });
        let __cordl_ret: crate::Unity::Mathematics::int2 = unsafe {
            method.invoke_unchecked((), (val))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn set_Item(
        &mut self,
        index: i32,
        value: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::Unity::Mathematics::int2 as quest_hook::libil2cpp::Type>::class()
            .find_method::<(i32, i32), quest_hook::libil2cpp::Void, 2usize>("set_Item")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::Unity::Mathematics::int2 as quest_hook::libil2cpp::Type >
                    ::class(), "set_Item", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (index, value))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn set_xy(
        &mut self,
        value: crate::Unity::Mathematics::int2,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::Unity::Mathematics::int2 as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (crate::Unity::Mathematics::int2),
                quest_hook::libil2cpp::Void,
                1usize,
            >("set_xy")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::Unity::Mathematics::int2 as quest_hook::libil2cpp::Type >
                    ::class(), "set_xy", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (value))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn set_yx(
        &mut self,
        value: crate::Unity::Mathematics::int2,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::Unity::Mathematics::int2 as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (crate::Unity::Mathematics::int2),
                quest_hook::libil2cpp::Void,
                1usize,
            >("set_yx")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::Unity::Mathematics::int2 as quest_hook::libil2cpp::Type >
                    ::class(), "set_yx", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (value))?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Unity+Mathematics+int2")]
impl AsRef<crate::System::IEquatable_1<crate::Unity::Mathematics::int2>>
for crate::Unity::Mathematics::int2 {
    fn as_ref(&self) -> &crate::System::IEquatable_1<crate::Unity::Mathematics::int2> {
        todo!()
    }
}
#[cfg(feature = "Unity+Mathematics+int2")]
impl AsMut<crate::System::IEquatable_1<crate::Unity::Mathematics::int2>>
for crate::Unity::Mathematics::int2 {
    fn as_mut(
        &mut self,
    ) -> &mut crate::System::IEquatable_1<crate::Unity::Mathematics::int2> {
        todo!()
    }
}
#[cfg(feature = "Unity+Mathematics+int2")]
impl AsRef<crate::System::IFormattable> for crate::Unity::Mathematics::int2 {
    fn as_ref(&self) -> &crate::System::IFormattable {
        todo!()
    }
}
#[cfg(feature = "Unity+Mathematics+int2")]
impl AsMut<crate::System::IFormattable> for crate::Unity::Mathematics::int2 {
    fn as_mut(&mut self) -> &mut crate::System::IFormattable {
        todo!()
    }
}
#[cfg(feature = "Unity+Mathematics+int2+DebuggerProxy")]
#[repr(C)]
#[derive(Debug)]
pub struct int2_DebuggerProxy {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub x: i32,
    pub y: i32,
}
#[cfg(feature = "Unity+Mathematics+int2+DebuggerProxy")]
unsafe impl quest_hook::libil2cpp::Type
for crate::Unity::Mathematics::int2_DebuggerProxy {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "Unity.Mathematics";
    const CLASS_NAME: &'static str = "int2/DebuggerProxy";
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
#[cfg(feature = "Unity+Mathematics+int2+DebuggerProxy")]
impl std::ops::Deref for crate::Unity::Mathematics::int2_DebuggerProxy {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Unity+Mathematics+int2+DebuggerProxy")]
impl std::ops::DerefMut for crate::Unity::Mathematics::int2_DebuggerProxy {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Unity+Mathematics+int2+DebuggerProxy")]
impl crate::Unity::Mathematics::int2_DebuggerProxy {
    pub fn New(
        v: crate::Unity::Mathematics::int2,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (v))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        v: crate::Unity::Mathematics::int2,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::Unity::Mathematics::int2_DebuggerProxy as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (crate::Unity::Mathematics::int2),
                quest_hook::libil2cpp::Void,
                1usize,
            >(".ctor")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::Unity::Mathematics::int2_DebuggerProxy as
                    quest_hook::libil2cpp::Type > ::class(), ".ctor", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (v))?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Unity+Mathematics+int2+DebuggerProxy")]
impl quest_hook::libil2cpp::ObjectType
for crate::Unity::Mathematics::int2_DebuggerProxy {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
