#[cfg(feature = "System+Numerics+NumericsHelpers")]
#[repr(C)]
#[derive(Debug)]
pub struct NumericsHelpers {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "System+Numerics+NumericsHelpers")]
unsafe impl quest_hook::libil2cpp::Type for crate::System::Numerics::NumericsHelpers {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "System.Numerics";
    const CLASS_NAME: &'static str = "NumericsHelpers";
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
#[cfg(feature = "System+Numerics+NumericsHelpers")]
impl std::ops::Deref for crate::System::Numerics::NumericsHelpers {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Numerics+NumericsHelpers")]
impl std::ops::DerefMut for crate::System::Numerics::NumericsHelpers {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Numerics+NumericsHelpers")]
impl crate::System::Numerics::NumericsHelpers {
    pub fn Abs(a: i32) -> quest_hook::libil2cpp::Result<u32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<(i32), u32, 1usize>("Abs")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "Abs", 1usize
                )
            });
        let __cordl_ret: u32 = unsafe { method.invoke_unchecked((), (a)) };
        Ok(__cordl_ret.into())
    }
    pub fn CbitHighZero_u32_0(u: u32) -> quest_hook::libil2cpp::Result<i32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<(u32), i32, 1usize>("CbitHighZero")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "CbitHighZero", 1usize
                )
            });
        let __cordl_ret: i32 = unsafe { method.invoke_unchecked((), (u)) };
        Ok(__cordl_ret.into())
    }
    pub fn CbitHighZero_u64_1(uu: u64) -> quest_hook::libil2cpp::Result<i32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<(u64), i32, 1usize>("CbitHighZero")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "CbitHighZero", 1usize
                )
            });
        let __cordl_ret: i32 = unsafe { method.invoke_unchecked((), (uu)) };
        Ok(__cordl_ret.into())
    }
    pub fn CombineHash_i32_i32_1(
        n1: i32,
        n2: i32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<(i32, i32), i32, 2usize>("CombineHash")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "CombineHash", 2usize
                )
            });
        let __cordl_ret: i32 = unsafe { method.invoke_unchecked((), (n1, n2)) };
        Ok(__cordl_ret.into())
    }
    pub fn CombineHash_u32_u32_0(
        u1: u32,
        u2: u32,
    ) -> quest_hook::libil2cpp::Result<u32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<(u32, u32), u32, 2usize>("CombineHash")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "CombineHash", 2usize
                )
            });
        let __cordl_ret: u32 = unsafe { method.invoke_unchecked((), (u1, u2)) };
        Ok(__cordl_ret.into())
    }
    pub fn DangerousMakeTwosComplement(
        d: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>),
                quest_hook::libil2cpp::Void,
                1usize,
            >("DangerousMakeTwosComplement")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "DangerousMakeTwosComplement", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (d))
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetDoubleFromParts(
        sign: i32,
        exp: i32,
        man: u64,
    ) -> quest_hook::libil2cpp::Result<f64> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<(i32, i32, u64), f64, 3usize>("GetDoubleFromParts")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "GetDoubleFromParts", 3usize
                )
            });
        let __cordl_ret: f64 = unsafe { method.invoke_unchecked((), (sign, exp, man)) };
        Ok(__cordl_ret.into())
    }
    pub fn GetDoubleParts(
        dbl: f64,
        sign: quest_hook::libil2cpp::ByRefMut<i32>,
        exp: quest_hook::libil2cpp::ByRefMut<i32>,
        man: quest_hook::libil2cpp::ByRefMut<u64>,
        fFinite: quest_hook::libil2cpp::ByRefMut<bool>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    f64,
                    quest_hook::libil2cpp::ByRefMut<i32>,
                    quest_hook::libil2cpp::ByRefMut<i32>,
                    quest_hook::libil2cpp::ByRefMut<u64>,
                    quest_hook::libil2cpp::ByRefMut<bool>,
                ),
                quest_hook::libil2cpp::Void,
                5usize,
            >("GetDoubleParts")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "GetDoubleParts", 5usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (dbl, sign, exp, man, fFinite))
        };
        Ok(__cordl_ret.into())
    }
    pub fn MakeUlong(uHi: u32, uLo: u32) -> quest_hook::libil2cpp::Result<u64> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<(u32, u32), u64, 2usize>("MakeUlong")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "MakeUlong", 2usize
                )
            });
        let __cordl_ret: u64 = unsafe { method.invoke_unchecked((), (uHi, uLo)) };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Numerics+NumericsHelpers")]
impl quest_hook::libil2cpp::ObjectType for crate::System::Numerics::NumericsHelpers {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
