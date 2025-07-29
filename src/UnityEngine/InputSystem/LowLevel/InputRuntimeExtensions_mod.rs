#[cfg(feature = "cordl_class_UnityEngine+InputSystem+LowLevel+InputRuntimeExtensions")]
#[repr(C)]
#[derive(Debug)]
pub struct InputRuntimeExtensions {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "cordl_class_UnityEngine+InputSystem+LowLevel+InputRuntimeExtensions")]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::InputSystem::LowLevel::InputRuntimeExtensions {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine.InputSystem.LowLevel";
    const CLASS_NAME: &'static str = "InputRuntimeExtensions";
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
#[cfg(feature = "UnityEngine+InputSystem+LowLevel+InputRuntimeExtensions")]
impl std::ops::Deref
for crate::UnityEngine::InputSystem::LowLevel::InputRuntimeExtensions {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+LowLevel+InputRuntimeExtensions")]
impl std::ops::DerefMut
for crate::UnityEngine::InputSystem::LowLevel::InputRuntimeExtensions {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+LowLevel+InputRuntimeExtensions")]
impl crate::UnityEngine::InputSystem::LowLevel::InputRuntimeExtensions {
    pub fn DeviceCommand<TCommand>(
        runtime: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::LowLevel::IInputRuntime,
        >,
        deviceId: i32,
        command: quest_hook::libil2cpp::ByRefMut<TCommand>,
    ) -> quest_hook::libil2cpp::Result<i64>
    where
        TCommand: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::InputSystem::LowLevel::IInputRuntime,
                            >,
                            i32,
                            quest_hook::libil2cpp::ByRefMut<TCommand>,
                        ),
                        i64,
                        3usize,
                    >("DeviceCommand")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "DeviceCommand", 3usize
                        )
                    })
            });
        let __cordl_ret: i64 = unsafe {
            cordl_method_info.invoke_unchecked((), (runtime, deviceId, command))?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "cordl_class_UnityEngine+InputSystem+LowLevel+InputRuntimeExtensions")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::InputSystem::LowLevel::InputRuntimeExtensions {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
