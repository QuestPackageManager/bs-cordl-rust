#[cfg(feature = "cordl_class_UnityEngine+NumericFieldDraggerUtility")]
#[repr(C)]
#[derive(Debug)]
pub struct NumericFieldDraggerUtility {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "cordl_class_UnityEngine+NumericFieldDraggerUtility")]
unsafe impl quest_hook::libil2cpp::Type for crate::UnityEngine::NumericFieldDraggerUtility {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine";
    const CLASS_NAME: &'static str = "NumericFieldDraggerUtility";
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.class()
            .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
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
#[cfg(feature = "UnityEngine+NumericFieldDraggerUtility")]
impl std::ops::Deref for crate::UnityEngine::NumericFieldDraggerUtility {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+NumericFieldDraggerUtility")]
impl std::ops::DerefMut for crate::UnityEngine::NumericFieldDraggerUtility {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+NumericFieldDraggerUtility")]
impl crate::UnityEngine::NumericFieldDraggerUtility {
    pub fn Acceleration(
        shiftPressed: bool,
        altPressed: bool,
    ) -> quest_hook::libil2cpp::Result<f32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(bool, bool), f32, 2usize>("Acceleration")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Acceleration",
                            2usize
                        )
                    })
            });
        let __cordl_ret: f32 =
            unsafe { cordl_method_info.invoke_unchecked((), (shiftPressed, altPressed))? };
        Ok(__cordl_ret.into())
    }
    pub fn CalculateFloatDragSensitivity_f64_0(value: f64) -> quest_hook::libil2cpp::Result<f64> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(f64), f64, 1usize>("CalculateFloatDragSensitivity")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "CalculateFloatDragSensitivity",
                            1usize
                        )
                    })
            });
        let __cordl_ret: f64 = unsafe { cordl_method_info.invoke_unchecked((), (value))? };
        Ok(__cordl_ret.into())
    }
    pub fn CalculateFloatDragSensitivity_f64_f64_1(
        value: f64,
        minValue: f64,
        maxValue: f64,
    ) -> quest_hook::libil2cpp::Result<f64> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(f64, f64, f64), f64, 3usize>(
                        "CalculateFloatDragSensitivity",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "CalculateFloatDragSensitivity",
                            3usize
                        )
                    })
            });
        let __cordl_ret: f64 =
            unsafe { cordl_method_info.invoke_unchecked((), (value, minValue, maxValue))? };
        Ok(__cordl_ret.into())
    }
    pub fn CalculateIntDragSensitivity_f64_2(value: f64) -> quest_hook::libil2cpp::Result<f64> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(f64), f64, 1usize>("CalculateIntDragSensitivity")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "CalculateIntDragSensitivity",
                            1usize
                        )
                    })
            });
        let __cordl_ret: f64 = unsafe { cordl_method_info.invoke_unchecked((), (value))? };
        Ok(__cordl_ret.into())
    }
    pub fn CalculateIntDragSensitivity_i64_0(value: i64) -> quest_hook::libil2cpp::Result<i64> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(i64), i64, 1usize>("CalculateIntDragSensitivity")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "CalculateIntDragSensitivity",
                            1usize
                        )
                    })
            });
        let __cordl_ret: i64 = unsafe { cordl_method_info.invoke_unchecked((), (value))? };
        Ok(__cordl_ret.into())
    }
    pub fn CalculateIntDragSensitivity_i64_i64_i64_3(
        value: i64,
        minValue: i64,
        maxValue: i64,
    ) -> quest_hook::libil2cpp::Result<i64> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(i64, i64, i64), i64, 3usize>(
                        "CalculateIntDragSensitivity",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "CalculateIntDragSensitivity",
                            3usize
                        )
                    })
            });
        let __cordl_ret: i64 =
            unsafe { cordl_method_info.invoke_unchecked((), (value, minValue, maxValue))? };
        Ok(__cordl_ret.into())
    }
    pub fn CalculateIntDragSensitivity_u64_1(value: u64) -> quest_hook::libil2cpp::Result<u64> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(u64), u64, 1usize>("CalculateIntDragSensitivity")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "CalculateIntDragSensitivity",
                            1usize
                        )
                    })
            });
        let __cordl_ret: u64 = unsafe { cordl_method_info.invoke_unchecked((), (value))? };
        Ok(__cordl_ret.into())
    }
    pub fn NiceDelta(
        deviceDelta: crate::UnityEngine::Vector2,
        acceleration: f32,
    ) -> quest_hook::libil2cpp::Result<f32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(crate::UnityEngine::Vector2, f32), f32, 2usize>(
                        "NiceDelta",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "NiceDelta",
                            2usize
                        )
                    })
            });
        let __cordl_ret: f32 =
            unsafe { cordl_method_info.invoke_unchecked((), (deviceDelta, acceleration))? };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "cordl_class_UnityEngine+NumericFieldDraggerUtility")]
impl quest_hook::libil2cpp::ObjectType for crate::UnityEngine::NumericFieldDraggerUtility {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
