#[cfg(feature = "cordl_class_UnityEngine+Rendering+CameraCaptureBridge")]
#[repr(C)]
#[derive(Debug)]
pub struct CameraCaptureBridge {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "cordl_class_UnityEngine+Rendering+CameraCaptureBridge")]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::Rendering::CameraCaptureBridge {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine.Rendering";
    const CLASS_NAME: &'static str = "CameraCaptureBridge";
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
#[cfg(feature = "UnityEngine+Rendering+CameraCaptureBridge")]
impl std::ops::Deref for crate::UnityEngine::Rendering::CameraCaptureBridge {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Rendering+CameraCaptureBridge")]
impl std::ops::DerefMut for crate::UnityEngine::Rendering::CameraCaptureBridge {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Rendering+CameraCaptureBridge")]
impl crate::UnityEngine::Rendering::CameraCaptureBridge {
    #[cfg(feature = "UnityEngine+Rendering+CameraCaptureBridge+CameraEntry")]
    pub type CameraEntry = crate::UnityEngine::Rendering::CameraCaptureBridge_CameraEntry;
    pub fn AddCaptureAction(
        camera: quest_hook::libil2cpp::Gc<crate::UnityEngine::Camera>,
        action: quest_hook::libil2cpp::Gc<
            crate::System::Action_2<
                crate::UnityEngine::Rendering::RenderTargetIdentifier,
                quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::CommandBuffer>,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<crate::UnityEngine::Camera>,
                            quest_hook::libil2cpp::Gc<
                                crate::System::Action_2<
                                    crate::UnityEngine::Rendering::RenderTargetIdentifier,
                                    quest_hook::libil2cpp::Gc<
                                        crate::UnityEngine::Rendering::CommandBuffer,
                                    >,
                                >,
                            >,
                        ),
                        quest_hook::libil2cpp::Void,
                        2usize,
                    >("AddCaptureAction")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "AddCaptureAction", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked((), (camera, action))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetCachedCaptureActionsEnumerator(
        camera: quest_hook::libil2cpp::Gc<crate::UnityEngine::Camera>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerator_1<
                quest_hook::libil2cpp::Gc<
                    crate::System::Action_2<
                        crate::UnityEngine::Rendering::RenderTargetIdentifier,
                        quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::Rendering::CommandBuffer,
                        >,
                    >,
                >,
            >,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (quest_hook::libil2cpp::Gc<crate::UnityEngine::Camera>),
                        quest_hook::libil2cpp::Gc<
                            crate::System::Collections::Generic::IEnumerator_1<
                                quest_hook::libil2cpp::Gc<
                                    crate::System::Action_2<
                                        crate::UnityEngine::Rendering::RenderTargetIdentifier,
                                        quest_hook::libil2cpp::Gc<
                                            crate::UnityEngine::Rendering::CommandBuffer,
                                        >,
                                    >,
                                >,
                            >,
                        >,
                        1usize,
                    >("GetCachedCaptureActionsEnumerator")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "GetCachedCaptureActionsEnumerator", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerator_1<
                quest_hook::libil2cpp::Gc<
                    crate::System::Action_2<
                        crate::UnityEngine::Rendering::RenderTargetIdentifier,
                        quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::Rendering::CommandBuffer,
                        >,
                    >,
                >,
            >,
        > = unsafe { cordl_method_info.invoke_unchecked((), (camera))? };
        Ok(__cordl_ret.into())
    }
    pub fn GetCaptureActions(
        camera: quest_hook::libil2cpp::Gc<crate::UnityEngine::Camera>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerator_1<
                quest_hook::libil2cpp::Gc<
                    crate::System::Action_2<
                        crate::UnityEngine::Rendering::RenderTargetIdentifier,
                        quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::Rendering::CommandBuffer,
                        >,
                    >,
                >,
            >,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (quest_hook::libil2cpp::Gc<crate::UnityEngine::Camera>),
                        quest_hook::libil2cpp::Gc<
                            crate::System::Collections::Generic::IEnumerator_1<
                                quest_hook::libil2cpp::Gc<
                                    crate::System::Action_2<
                                        crate::UnityEngine::Rendering::RenderTargetIdentifier,
                                        quest_hook::libil2cpp::Gc<
                                            crate::UnityEngine::Rendering::CommandBuffer,
                                        >,
                                    >,
                                >,
                            >,
                        >,
                        1usize,
                    >("GetCaptureActions")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "GetCaptureActions", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerator_1<
                quest_hook::libil2cpp::Gc<
                    crate::System::Action_2<
                        crate::UnityEngine::Rendering::RenderTargetIdentifier,
                        quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::Rendering::CommandBuffer,
                        >,
                    >,
                >,
            >,
        > = unsafe { cordl_method_info.invoke_unchecked((), (camera))? };
        Ok(__cordl_ret.into())
    }
    pub fn RemoveCaptureAction(
        camera: quest_hook::libil2cpp::Gc<crate::UnityEngine::Camera>,
        action: quest_hook::libil2cpp::Gc<
            crate::System::Action_2<
                crate::UnityEngine::Rendering::RenderTargetIdentifier,
                quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::CommandBuffer>,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<crate::UnityEngine::Camera>,
                            quest_hook::libil2cpp::Gc<
                                crate::System::Action_2<
                                    crate::UnityEngine::Rendering::RenderTargetIdentifier,
                                    quest_hook::libil2cpp::Gc<
                                        crate::UnityEngine::Rendering::CommandBuffer,
                                    >,
                                >,
                            >,
                        ),
                        quest_hook::libil2cpp::Void,
                        2usize,
                    >("RemoveCaptureAction")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "RemoveCaptureAction", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked((), (camera, action))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_enabled() -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(), bool, 0usize>("get_enabled")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "get_enabled", 0usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe { cordl_method_info.invoke_unchecked((), ())? };
        Ok(__cordl_ret.into())
    }
    pub fn set_enabled(
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (bool),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("set_enabled")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "set_enabled", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked((), (value))?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "cordl_class_UnityEngine+Rendering+CameraCaptureBridge")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::Rendering::CameraCaptureBridge {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "cordl_class_UnityEngine+Rendering+CameraCaptureBridge+CameraEntry")]
#[repr(C)]
#[derive(Debug)]
pub struct CameraCaptureBridge_CameraEntry {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub actions: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::HashSet_1<
            quest_hook::libil2cpp::Gc<
                crate::System::Action_2<
                    crate::UnityEngine::Rendering::RenderTargetIdentifier,
                    quest_hook::libil2cpp::Gc<
                        crate::UnityEngine::Rendering::CommandBuffer,
                    >,
                >,
            >,
        >,
    >,
    pub cachedEnumerator: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::IEnumerator_1<
            quest_hook::libil2cpp::Gc<
                crate::System::Action_2<
                    crate::UnityEngine::Rendering::RenderTargetIdentifier,
                    quest_hook::libil2cpp::Gc<
                        crate::UnityEngine::Rendering::CommandBuffer,
                    >,
                >,
            >,
        >,
    >,
}
#[cfg(feature = "cordl_class_UnityEngine+Rendering+CameraCaptureBridge+CameraEntry")]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::Rendering::CameraCaptureBridge_CameraEntry {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine.Rendering";
    const CLASS_NAME: &'static str = "CameraCaptureBridge/CameraEntry";
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
#[cfg(feature = "UnityEngine+Rendering+CameraCaptureBridge+CameraEntry")]
impl std::ops::Deref for crate::UnityEngine::Rendering::CameraCaptureBridge_CameraEntry {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Rendering+CameraCaptureBridge+CameraEntry")]
impl std::ops::DerefMut
for crate::UnityEngine::Rendering::CameraCaptureBridge_CameraEntry {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Rendering+CameraCaptureBridge+CameraEntry")]
impl crate::UnityEngine::Rendering::CameraCaptureBridge_CameraEntry {
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), quest_hook::libil2cpp::Void, 0usize>(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), ".ctor",
                            0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "cordl_class_UnityEngine+Rendering+CameraCaptureBridge+CameraEntry")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::Rendering::CameraCaptureBridge_CameraEntry {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
