#[cfg(feature = "cordl_class_UnityEngine+Rendering+SpeedTreeWindManager")]
#[repr(C)]
#[derive(Debug)]
pub struct SpeedTreeWindManager {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "cordl_class_UnityEngine+Rendering+SpeedTreeWindManager")]
unsafe impl quest_hook::libil2cpp::Type for crate::UnityEngine::Rendering::SpeedTreeWindManager {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine.Rendering";
    const CLASS_NAME: &'static str = "SpeedTreeWindManager";
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
#[cfg(feature = "UnityEngine+Rendering+SpeedTreeWindManager")]
impl std::ops::Deref for crate::UnityEngine::Rendering::SpeedTreeWindManager {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Rendering+SpeedTreeWindManager")]
impl std::ops::DerefMut for crate::UnityEngine::Rendering::SpeedTreeWindManager {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Rendering+SpeedTreeWindManager")]
impl crate::UnityEngine::Rendering::SpeedTreeWindManager {
    pub fn UpdateWindAndWriteBufferWindParams(
        renderersID: crate::System::ReadOnlySpan_1<i32>,
        windParams: crate::UnityEngine::Rendering::SpeedTreeWindParamsBufferIterator,
        history: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::System::ReadOnlySpan_1<i32>,
                        crate::UnityEngine::Rendering::SpeedTreeWindParamsBufferIterator,
                        bool,
                    ), quest_hook::libil2cpp::Void, 3usize>(
                        "UpdateWindAndWriteBufferWindParams"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "UpdateWindAndWriteBufferWindParams",
                            3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked((), (renderersID, windParams, history))? };
        Ok(__cordl_ret.into())
    }
    pub fn UpdateWindAndWriteBufferWindParams_Injected(
        renderersID: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::Bindings::ManagedSpanWrapper,
        >,
        windParams: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::Rendering::SpeedTreeWindParamsBufferIterator,
        >,
        history: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        quest_hook::libil2cpp::ByRefMut<
                            crate::UnityEngine::Bindings::ManagedSpanWrapper,
                        >,
                        quest_hook::libil2cpp::ByRefMut<
                            crate::UnityEngine::Rendering::SpeedTreeWindParamsBufferIterator,
                        >,
                        bool,
                    ), quest_hook::libil2cpp::Void, 3usize>(
                        "UpdateWindAndWriteBufferWindParams_Injected",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "UpdateWindAndWriteBufferWindParams_Injected",
                            3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked((), (renderersID, windParams, history))? };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "cordl_class_UnityEngine+Rendering+SpeedTreeWindManager")]
impl quest_hook::libil2cpp::ObjectType for crate::UnityEngine::Rendering::SpeedTreeWindManager {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
