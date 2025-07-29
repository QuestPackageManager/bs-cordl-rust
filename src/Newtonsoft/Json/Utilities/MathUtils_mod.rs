#[cfg(feature = "cordl_class_Newtonsoft+Json+Utilities+MathUtils")]
#[repr(C)]
#[derive(Debug)]
pub struct MathUtils {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "cordl_class_Newtonsoft+Json+Utilities+MathUtils")]
unsafe impl quest_hook::libil2cpp::Type
for crate::Newtonsoft::Json::Utilities::MathUtils {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "Newtonsoft.Json.Utilities";
    const CLASS_NAME: &'static str = "MathUtils";
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
#[cfg(feature = "cordl_class_Newtonsoft+Json+Utilities+MathUtils")]
impl std::ops::Deref for crate::Newtonsoft::Json::Utilities::MathUtils {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "cordl_class_Newtonsoft+Json+Utilities+MathUtils")]
impl std::ops::DerefMut for crate::Newtonsoft::Json::Utilities::MathUtils {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Newtonsoft+Json+Utilities+MathUtils")]
impl crate::Newtonsoft::Json::Utilities::MathUtils {
    pub fn ApproxEquals(d1: f64, d2: f64) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(f64, f64), bool, 2usize>("ApproxEquals")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "ApproxEquals", 2usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            cordl_method_info.invoke_unchecked((), (d1, d2))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn IntLength(i: u64) -> quest_hook::libil2cpp::Result<i32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(u64), i32, 1usize>("IntLength")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "IntLength", 1usize
                        )
                    })
            });
        let __cordl_ret: i32 = unsafe { cordl_method_info.invoke_unchecked((), (i))? };
        Ok(__cordl_ret.into())
    }
    pub fn IntToHex(n: i32) -> quest_hook::libil2cpp::Result<char> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(i32), char, 1usize>("IntToHex")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "IntToHex", 1usize
                        )
                    })
            });
        let __cordl_ret: char = unsafe { cordl_method_info.invoke_unchecked((), (n))? };
        Ok(__cordl_ret.into())
    }
    pub fn Max_Nullable_1_Nullable_1_0(
        val1: crate::System::Nullable_1<i32>,
        val2: crate::System::Nullable_1<i32>,
    ) -> quest_hook::libil2cpp::Result<crate::System::Nullable_1<i32>> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (crate::System::Nullable_1<i32>, crate::System::Nullable_1<i32>),
                        crate::System::Nullable_1<i32>,
                        2usize,
                    >("Max")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "Max",
                            2usize
                        )
                    })
            });
        let __cordl_ret: crate::System::Nullable_1<i32> = unsafe {
            cordl_method_info.invoke_unchecked((), (val1, val2))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Max_Nullable_1_Nullable_1_1(
        val1: crate::System::Nullable_1<f64>,
        val2: crate::System::Nullable_1<f64>,
    ) -> quest_hook::libil2cpp::Result<crate::System::Nullable_1<f64>> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (crate::System::Nullable_1<f64>, crate::System::Nullable_1<f64>),
                        crate::System::Nullable_1<f64>,
                        2usize,
                    >("Max")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "Max",
                            2usize
                        )
                    })
            });
        let __cordl_ret: crate::System::Nullable_1<f64> = unsafe {
            cordl_method_info.invoke_unchecked((), (val1, val2))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Min(
        val1: crate::System::Nullable_1<i32>,
        val2: crate::System::Nullable_1<i32>,
    ) -> quest_hook::libil2cpp::Result<crate::System::Nullable_1<i32>> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (crate::System::Nullable_1<i32>, crate::System::Nullable_1<i32>),
                        crate::System::Nullable_1<i32>,
                        2usize,
                    >("Min")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "Min",
                            2usize
                        )
                    })
            });
        let __cordl_ret: crate::System::Nullable_1<i32> = unsafe {
            cordl_method_info.invoke_unchecked((), (val1, val2))?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "cordl_class_Newtonsoft+Json+Utilities+MathUtils")]
impl quest_hook::libil2cpp::ObjectType
for crate::Newtonsoft::Json::Utilities::MathUtils {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
