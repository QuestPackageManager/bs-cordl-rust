#[cfg(feature = "cordl_class_Unity+IntegerTime+RationalTimeExtensions")]
#[repr(C)]
#[derive(Debug)]
pub struct RationalTimeExtensions {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "cordl_class_Unity+IntegerTime+RationalTimeExtensions")]
unsafe impl quest_hook::libil2cpp::Type
for crate::Unity::IntegerTime::RationalTimeExtensions {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "Unity.IntegerTime";
    const CLASS_NAME: &'static str = "RationalTimeExtensions";
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
#[cfg(feature = "Unity+IntegerTime+RationalTimeExtensions")]
impl std::ops::Deref for crate::Unity::IntegerTime::RationalTimeExtensions {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Unity+IntegerTime+RationalTimeExtensions")]
impl std::ops::DerefMut for crate::Unity::IntegerTime::RationalTimeExtensions {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Unity+IntegerTime+RationalTimeExtensions")]
impl crate::Unity::IntegerTime::RationalTimeExtensions {
    pub fn Convert(
        _cordl_time: crate::Unity::IntegerTime::RationalTime,
        rate: crate::Unity::IntegerTime::RationalTime_TicksPerSecond,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::IntegerTime::RationalTime> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            crate::Unity::IntegerTime::RationalTime,
                            crate::Unity::IntegerTime::RationalTime_TicksPerSecond,
                        ),
                        crate::Unity::IntegerTime::RationalTime,
                        2usize,
                    >("Convert")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "Convert",
                            2usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::IntegerTime::RationalTime = unsafe {
            cordl_method_info.invoke_unchecked((), (_cordl_time, rate))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Convert_Injected(
        _cordl_time: quest_hook::libil2cpp::ByRefMut<
            crate::Unity::IntegerTime::RationalTime,
        >,
        rate: quest_hook::libil2cpp::ByRefMut<
            crate::Unity::IntegerTime::RationalTime_TicksPerSecond,
        >,
        ret: quest_hook::libil2cpp::ByRefMut<crate::Unity::IntegerTime::RationalTime>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::ByRefMut<
                                crate::Unity::IntegerTime::RationalTime,
                            >,
                            quest_hook::libil2cpp::ByRefMut<
                                crate::Unity::IntegerTime::RationalTime_TicksPerSecond,
                            >,
                            quest_hook::libil2cpp::ByRefMut<
                                crate::Unity::IntegerTime::RationalTime,
                            >,
                        ),
                        quest_hook::libil2cpp::Void,
                        3usize,
                    >("Convert_Injected")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "Convert_Injected", 3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked((), (_cordl_time, rate, ret))?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "cordl_class_Unity+IntegerTime+RationalTimeExtensions")]
impl quest_hook::libil2cpp::ObjectType
for crate::Unity::IntegerTime::RationalTimeExtensions {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
