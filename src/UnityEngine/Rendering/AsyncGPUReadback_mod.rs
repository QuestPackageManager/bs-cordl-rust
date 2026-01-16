#[cfg(feature = "cordl_class_UnityEngine+Rendering+AsyncGPUReadback")]
#[repr(C)]
#[derive(Debug)]
pub struct AsyncGPUReadback {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "cordl_class_UnityEngine+Rendering+AsyncGPUReadback")]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::Rendering::AsyncGPUReadback {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine.Rendering";
    const CLASS_NAME: &'static str = "AsyncGPUReadback";
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
#[cfg(feature = "UnityEngine+Rendering+AsyncGPUReadback")]
impl std::ops::Deref for crate::UnityEngine::Rendering::AsyncGPUReadback {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Rendering+AsyncGPUReadback")]
impl std::ops::DerefMut for crate::UnityEngine::Rendering::AsyncGPUReadback {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Rendering+AsyncGPUReadback")]
impl crate::UnityEngine::Rendering::AsyncGPUReadback {
    pub fn Request_Action_1_0(
        src: quest_hook::libil2cpp::Gc<crate::UnityEngine::GraphicsBuffer>,
        callback: quest_hook::libil2cpp::Gc<
            crate::System::Action_1<
                crate::UnityEngine::Rendering::AsyncGPUReadbackRequest,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::Rendering::AsyncGPUReadbackRequest,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::GraphicsBuffer,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::System::Action_1<
                                    crate::UnityEngine::Rendering::AsyncGPUReadbackRequest,
                                >,
                            >,
                        ),
                        crate::UnityEngine::Rendering::AsyncGPUReadbackRequest,
                        2usize,
                    >("Request")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "Request",
                            2usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::Rendering::AsyncGPUReadbackRequest = unsafe {
            cordl_method_info.invoke_unchecked((), (src, callback))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Request_Internal_GraphicsBuffer_1(
        buffer: quest_hook::libil2cpp::Gc<crate::UnityEngine::GraphicsBuffer>,
        data: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::Rendering::AsyncGPUReadbackRequest,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::GraphicsBuffer,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppObject,
                            >,
                        ),
                        crate::UnityEngine::Rendering::AsyncGPUReadbackRequest,
                        2usize,
                    >("Request_Internal_GraphicsBuffer_1")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "Request_Internal_GraphicsBuffer_1", 2usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::Rendering::AsyncGPUReadbackRequest = unsafe {
            cordl_method_info.invoke_unchecked((), (buffer, data))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Request_Internal_GraphicsBuffer_1_Injected(
        buffer: crate::System::IntPtr,
        data: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        ret: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::Rendering::AsyncGPUReadbackRequest,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            crate::System::IntPtr,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppObject,
                            >,
                            quest_hook::libil2cpp::ByRefMut<
                                crate::UnityEngine::Rendering::AsyncGPUReadbackRequest,
                            >,
                        ),
                        quest_hook::libil2cpp::Void,
                        3usize,
                    >("Request_Internal_GraphicsBuffer_1_Injected")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "Request_Internal_GraphicsBuffer_1_Injected", 3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked((), (buffer, data, ret))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Request_Internal_GraphicsBuffer_2(
        src: quest_hook::libil2cpp::Gc<crate::UnityEngine::GraphicsBuffer>,
        _cordl_size: i32,
        offset: i32,
        data: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::Rendering::AsyncGPUReadbackRequest,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::GraphicsBuffer,
                            >,
                            i32,
                            i32,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppObject,
                            >,
                        ),
                        crate::UnityEngine::Rendering::AsyncGPUReadbackRequest,
                        4usize,
                    >("Request_Internal_GraphicsBuffer_2")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "Request_Internal_GraphicsBuffer_2", 4usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::Rendering::AsyncGPUReadbackRequest = unsafe {
            cordl_method_info.invoke_unchecked((), (src, _cordl_size, offset, data))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Request_Internal_GraphicsBuffer_2_Injected(
        src: crate::System::IntPtr,
        _cordl_size: i32,
        offset: i32,
        data: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        ret: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::Rendering::AsyncGPUReadbackRequest,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            crate::System::IntPtr,
                            i32,
                            i32,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppObject,
                            >,
                            quest_hook::libil2cpp::ByRefMut<
                                crate::UnityEngine::Rendering::AsyncGPUReadbackRequest,
                            >,
                        ),
                        quest_hook::libil2cpp::Void,
                        5usize,
                    >("Request_Internal_GraphicsBuffer_2_Injected")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "Request_Internal_GraphicsBuffer_2_Injected", 5usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info
                .invoke_unchecked((), (src, _cordl_size, offset, data, ret))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Request_i32_i32_Action_1_1(
        src: quest_hook::libil2cpp::Gc<crate::UnityEngine::GraphicsBuffer>,
        _cordl_size: i32,
        offset: i32,
        callback: quest_hook::libil2cpp::Gc<
            crate::System::Action_1<
                crate::UnityEngine::Rendering::AsyncGPUReadbackRequest,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::Rendering::AsyncGPUReadbackRequest,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::GraphicsBuffer,
                            >,
                            i32,
                            i32,
                            quest_hook::libil2cpp::Gc<
                                crate::System::Action_1<
                                    crate::UnityEngine::Rendering::AsyncGPUReadbackRequest,
                                >,
                            >,
                        ),
                        crate::UnityEngine::Rendering::AsyncGPUReadbackRequest,
                        4usize,
                    >("Request")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "Request",
                            4usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::Rendering::AsyncGPUReadbackRequest = unsafe {
            cordl_method_info.invoke_unchecked((), (src, _cordl_size, offset, callback))?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "cordl_class_UnityEngine+Rendering+AsyncGPUReadback")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::Rendering::AsyncGPUReadback {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
