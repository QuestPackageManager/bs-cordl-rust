#[cfg(
    feature = "UnityEngine+Experimental+Rendering+ScriptableRuntimeReflectionSystemSettings"
)]
#[repr(C)]
#[derive(Debug)]
pub struct ScriptableRuntimeReflectionSystemSettings {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(
    feature = "UnityEngine+Experimental+Rendering+ScriptableRuntimeReflectionSystemSettings"
)]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::Experimental::Rendering::ScriptableRuntimeReflectionSystemSettings {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine.Experimental.Rendering";
    const CLASS_NAME: &'static str = "ScriptableRuntimeReflectionSystemSettings";
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
#[cfg(
    feature = "UnityEngine+Experimental+Rendering+ScriptableRuntimeReflectionSystemSettings"
)]
impl std::ops::Deref
for crate::UnityEngine::Experimental::Rendering::ScriptableRuntimeReflectionSystemSettings {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(
    feature = "UnityEngine+Experimental+Rendering+ScriptableRuntimeReflectionSystemSettings"
)]
impl std::ops::DerefMut
for crate::UnityEngine::Experimental::Rendering::ScriptableRuntimeReflectionSystemSettings {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(
    feature = "UnityEngine+Experimental+Rendering+ScriptableRuntimeReflectionSystemSettings"
)]
impl crate::UnityEngine::Experimental::Rendering::ScriptableRuntimeReflectionSystemSettings {
    pub fn ScriptingDirtyReflectionSystemInstance() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Void,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (),
                        quest_hook::libil2cpp::Void,
                        0usize,
                    >("ScriptingDirtyReflectionSystemInstance")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "ScriptingDirtyReflectionSystemInstance", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked((), ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_Internal_ScriptableRuntimeReflectionSystemSettings_instance() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Experimental::Rendering::ScriptableRuntimeReflectionSystemWrapper,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (),
                        quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::Experimental::Rendering::ScriptableRuntimeReflectionSystemWrapper,
                        >,
                        0usize,
                    >("get_Internal_ScriptableRuntimeReflectionSystemSettings_instance")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "get_Internal_ScriptableRuntimeReflectionSystemSettings_instance",
                            0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Experimental::Rendering::ScriptableRuntimeReflectionSystemWrapper,
        > = unsafe { cordl_method_info.invoke_unchecked((), ())? };
        Ok(__cordl_ret.into())
    }
    pub fn set_Internal_ScriptableRuntimeReflectionSystemSettings_system(
        value: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Experimental::Rendering::IScriptableRuntimeReflectionSystem,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::Experimental::Rendering::IScriptableRuntimeReflectionSystem,
                        >),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("set_Internal_ScriptableRuntimeReflectionSystemSettings_system")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "set_Internal_ScriptableRuntimeReflectionSystemSettings_system",
                            1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked((), (value))?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(
    feature = "UnityEngine+Experimental+Rendering+ScriptableRuntimeReflectionSystemSettings"
)]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::Experimental::Rendering::ScriptableRuntimeReflectionSystemSettings {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
