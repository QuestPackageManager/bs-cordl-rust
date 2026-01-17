#[cfg(feature = "cordl_class_FixedUpdateVector3SmoothValue")]
#[repr(C)]
#[cfg_attr(feature = "derive_Debug", derive(Debug))]
pub struct FixedUpdateVector3SmoothValue {
    __cordl_parent: crate::GlobalNamespace::FixedUpdateSmoothValue_1<crate::UnityEngine::Vector3>,
}
#[cfg(feature = "cordl_class_FixedUpdateVector3SmoothValue")]
unsafe impl quest_hook::libil2cpp::Type for crate::GlobalNamespace::FixedUpdateVector3SmoothValue {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "FixedUpdateVector3SmoothValue";
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
#[cfg(feature = "FixedUpdateVector3SmoothValue")]
impl std::ops::Deref for crate::GlobalNamespace::FixedUpdateVector3SmoothValue {
    type Target = crate::GlobalNamespace::FixedUpdateSmoothValue_1<crate::UnityEngine::Vector3>;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "FixedUpdateVector3SmoothValue")]
impl std::ops::DerefMut for crate::GlobalNamespace::FixedUpdateVector3SmoothValue {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "FixedUpdateVector3SmoothValue")]
impl crate::GlobalNamespace::FixedUpdateVector3SmoothValue {
    pub fn Interpolate(
        &mut self,
        value0: crate::UnityEngine::Vector3,
        value1: crate::UnityEngine::Vector3,
        t: f32,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector3> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        crate::UnityEngine::Vector3,
                        crate::UnityEngine::Vector3,
                        f32,
                    ), crate::UnityEngine::Vector3, 3usize>("Interpolate")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Interpolate",
                            3usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::Vector3 =
            unsafe { cordl_method_info.invoke_unchecked(self, (value0, value1, t))? };
        Ok(__cordl_ret.into())
    }
    pub fn New(smooth: f32) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self =
            <Self as quest_hook::libil2cpp::Type>::class().instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (smooth))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        smooth: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(f32), quest_hook::libil2cpp::Void, 1usize>(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            ".ctor",
                            1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (smooth))? };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "cordl_class_FixedUpdateVector3SmoothValue")]
impl quest_hook::libil2cpp::ObjectType for crate::GlobalNamespace::FixedUpdateVector3SmoothValue {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
