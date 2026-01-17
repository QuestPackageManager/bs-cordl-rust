#[cfg(feature = "cordl_class_UnityEngine+Rendering+GPUDrivenCallbacks")]
#[repr(C)]
#[cfg_attr(feature = "derive_Debug", derive(Debug))]
pub struct GPUDrivenCallbacks {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "cordl_class_UnityEngine+Rendering+GPUDrivenCallbacks")]
unsafe impl quest_hook::libil2cpp::Type for crate::UnityEngine::Rendering::GPUDrivenCallbacks {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine.Rendering";
    const CLASS_NAME: &'static str = "GPUDrivenCallbacks";
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
#[cfg(feature = "UnityEngine+Rendering+GPUDrivenCallbacks")]
impl std::ops::Deref for crate::UnityEngine::Rendering::GPUDrivenCallbacks {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Rendering+GPUDrivenCallbacks")]
impl std::ops::DerefMut for crate::UnityEngine::Rendering::GPUDrivenCallbacks {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Rendering+GPUDrivenCallbacks")]
impl crate::UnityEngine::Rendering::GPUDrivenCallbacks {
    pub fn InvokeGPUDrivenLODGroupDataNativeCallback(
        callback: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Rendering::GPUDrivenLODGroupDataNativeCallback,
        >,
        lodGroupDataNative: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::Rendering::GPUDrivenLODGroupDataNative,
        >,
        target: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Rendering::GPUDrivenLODGroupDataCallback,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::Rendering::GPUDrivenLODGroupDataNativeCallback,
                        >,
                        quest_hook::libil2cpp::ByRefMut<
                            crate::UnityEngine::Rendering::GPUDrivenLODGroupDataNative,
                        >,
                        quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::Rendering::GPUDrivenLODGroupDataCallback,
                        >,
                    ), quest_hook::libil2cpp::Void, 3usize>(
                        "InvokeGPUDrivenLODGroupDataNativeCallback",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "InvokeGPUDrivenLODGroupDataNativeCallback",
                            3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked((), (callback, lodGroupDataNative, target))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn InvokeGPUDrivenRendererDataNativeCallback(
        callback: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Rendering::GPUDrivenRendererDataNativeCallback,
        >,
        rendererDataNative: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::Rendering::GPUDrivenRendererGroupDataNative,
        >,
        meshes: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                quest_hook::libil2cpp::Gc<crate::UnityEngine::Mesh>,
            >,
        >,
        materials: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
            >,
        >,
        target: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Rendering::GPUDrivenRendererDataCallback,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::Rendering::GPUDrivenRendererDataNativeCallback,
                        >,
                        quest_hook::libil2cpp::ByRefMut<
                            crate::UnityEngine::Rendering::GPUDrivenRendererGroupDataNative,
                        >,
                        quest_hook::libil2cpp::Gc<
                            crate::System::Collections::Generic::List_1<
                                quest_hook::libil2cpp::Gc<crate::UnityEngine::Mesh>,
                            >,
                        >,
                        quest_hook::libil2cpp::Gc<
                            crate::System::Collections::Generic::List_1<
                                quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
                            >,
                        >,
                        quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::Rendering::GPUDrivenRendererDataCallback,
                        >,
                    ), quest_hook::libil2cpp::Void, 5usize>(
                        "InvokeGPUDrivenRendererDataNativeCallback",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "InvokeGPUDrivenRendererDataNativeCallback",
                            5usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(
                (),
                (callback, rendererDataNative, meshes, materials, target),
            )?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "cordl_class_UnityEngine+Rendering+GPUDrivenCallbacks")]
impl quest_hook::libil2cpp::ObjectType for crate::UnityEngine::Rendering::GPUDrivenCallbacks {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
