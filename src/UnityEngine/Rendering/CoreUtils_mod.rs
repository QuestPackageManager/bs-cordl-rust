#[cfg(feature = "cordl_class_UnityEngine+Rendering+CoreUtils")]
#[repr(C)]
#[derive(Debug)]
pub struct CoreUtils {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "cordl_class_UnityEngine+Rendering+CoreUtils")]
unsafe impl quest_hook::libil2cpp::Type for crate::UnityEngine::Rendering::CoreUtils {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine.Rendering";
    const CLASS_NAME: &'static str = "CoreUtils";
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
#[cfg(feature = "UnityEngine+Rendering+CoreUtils")]
impl std::ops::Deref for crate::UnityEngine::Rendering::CoreUtils {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Rendering+CoreUtils")]
impl std::ops::DerefMut for crate::UnityEngine::Rendering::CoreUtils {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Rendering+CoreUtils")]
impl crate::UnityEngine::Rendering::CoreUtils {
    pub const assetCreateMenuPriority1: i32 = 230i32;
    pub const assetCreateMenuPriority2: i32 = 241i32;
    pub const assetCreateMenuPriority3: i32 = 300i32;
    pub const editMenuPriority1: i32 = 320i32;
    pub const editMenuPriority2: i32 = 331i32;
    pub const editMenuPriority3: i32 = 342i32;
    pub const editMenuPriority4: i32 = 353i32;
    pub const gameObjectMenuPriority: i32 = 10i32;
    pub const obsoletePriorityMessage: &'static str = "Use CoreUtils.Priorities instead";
    #[cfg(feature = "UnityEngine+Rendering+CoreUtils+Priorities")]
    pub type Priorities = crate::UnityEngine::Rendering::CoreUtils_Priorities;
    #[cfg(feature = "UnityEngine+Rendering+CoreUtils+Sections")]
    pub type Sections = crate::UnityEngine::Rendering::CoreUtils_Sections;
    pub fn AreAnimatedMaterialsEnabled(
        camera: quest_hook::libil2cpp::Gc<crate::UnityEngine::Camera>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (quest_hook::libil2cpp::Gc<crate::UnityEngine::Camera>),
                        bool,
                        1usize,
                    >("AreAnimatedMaterialsEnabled")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "AreAnimatedMaterialsEnabled", 1usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            cordl_method_info.invoke_unchecked((), (camera))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ArePostProcessesEnabled(
        camera: quest_hook::libil2cpp::Gc<crate::UnityEngine::Camera>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (quest_hook::libil2cpp::Gc<crate::UnityEngine::Camera>),
                        bool,
                        1usize,
                    >("ArePostProcessesEnabled")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "ArePostProcessesEnabled", 1usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            cordl_method_info.invoke_unchecked((), (camera))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn CalculateViewSpaceCorners(
        proj: crate::UnityEngine::Matrix4x4,
        z: f32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Vector3>,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (crate::UnityEngine::Matrix4x4, f32),
                        quest_hook::libil2cpp::Gc<
                            quest_hook::libil2cpp::Il2CppArray<
                                crate::UnityEngine::Vector3,
                            >,
                        >,
                        2usize,
                    >("CalculateViewSpaceCorners")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "CalculateViewSpaceCorners", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Vector3>,
        > = unsafe { cordl_method_info.invoke_unchecked((), (proj, z))? };
        Ok(__cordl_ret.into())
    }
    pub fn ClearCubemap(
        cmd: quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::CommandBuffer>,
        renderTexture: quest_hook::libil2cpp::Gc<crate::UnityEngine::RenderTexture>,
        clearColor: crate::UnityEngine::Color,
        clearMips: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::Rendering::CommandBuffer,
                            >,
                            quest_hook::libil2cpp::Gc<crate::UnityEngine::RenderTexture>,
                            crate::UnityEngine::Color,
                            bool,
                        ),
                        quest_hook::libil2cpp::Void,
                        4usize,
                    >("ClearCubemap")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "ClearCubemap", 4usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info
                .invoke_unchecked((), (cmd, renderTexture, clearColor, clearMips))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ClearRenderTarget(
        cmd: quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::CommandBuffer>,
        clearFlag: crate::UnityEngine::Rendering::ClearFlag,
        clearColor: crate::UnityEngine::Color,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::Rendering::CommandBuffer,
                            >,
                            crate::UnityEngine::Rendering::ClearFlag,
                            crate::UnityEngine::Color,
                        ),
                        quest_hook::libil2cpp::Void,
                        3usize,
                    >("ClearRenderTarget")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "ClearRenderTarget", 3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked((), (cmd, clearFlag, clearColor))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ConvertLinearToActiveColorSpace(
        color: crate::UnityEngine::Color,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Color> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (crate::UnityEngine::Color),
                        crate::UnityEngine::Color,
                        1usize,
                    >("ConvertLinearToActiveColorSpace")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "ConvertLinearToActiveColorSpace", 1usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::Color = unsafe {
            cordl_method_info.invoke_unchecked((), (color))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ConvertSRGBToActiveColorSpace(
        color: crate::UnityEngine::Color,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Color> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (crate::UnityEngine::Color),
                        crate::UnityEngine::Color,
                        1usize,
                    >("ConvertSRGBToActiveColorSpace")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "ConvertSRGBToActiveColorSpace", 1usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::Color = unsafe {
            cordl_method_info.invoke_unchecked((), (color))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn CreateCubeMesh(
        min: crate::UnityEngine::Vector3,
        max: crate::UnityEngine::Vector3,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::Mesh>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (crate::UnityEngine::Vector3, crate::UnityEngine::Vector3),
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Mesh>,
                        2usize,
                    >("CreateCubeMesh")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "CreateCubeMesh", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::Mesh> = unsafe {
            cordl_method_info.invoke_unchecked((), (min, max))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn CreateEngineMaterial_Il2CppString0(
        shaderPath: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>),
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
                        1usize,
                    >("CreateEngineMaterial")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "CreateEngineMaterial", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::Material> = unsafe {
            cordl_method_info.invoke_unchecked((), (shaderPath))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn CreateEngineMaterial_Shader1(
        shader: quest_hook::libil2cpp::Gc<crate::UnityEngine::Shader>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (quest_hook::libil2cpp::Gc<crate::UnityEngine::Shader>),
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
                        1usize,
                    >("CreateEngineMaterial")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "CreateEngineMaterial", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::Material> = unsafe {
            cordl_method_info.invoke_unchecked((), (shader))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Destroy(
        obj: quest_hook::libil2cpp::Gc<crate::UnityEngine::Object>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (quest_hook::libil2cpp::Gc<crate::UnityEngine::Object>),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("Destroy")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "Destroy",
                            1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked((), (obj))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn DivRoundUp(value: i32, divisor: i32) -> quest_hook::libil2cpp::Result<i32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(i32, i32), i32, 2usize>("DivRoundUp")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "DivRoundUp", 2usize
                        )
                    })
            });
        let __cordl_ret: i32 = unsafe {
            cordl_method_info.invoke_unchecked((), (value, divisor))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn DrawFullScreen_CommandBuffer_Il2CppArray_MaterialPropertyBlock_i32_5(
        commandBuffer: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Rendering::CommandBuffer,
        >,
        material: quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
        colorBuffers: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                crate::UnityEngine::Rendering::RenderTargetIdentifier,
            >,
        >,
        properties: quest_hook::libil2cpp::Gc<crate::UnityEngine::MaterialPropertyBlock>,
        shaderPassId: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::Rendering::CommandBuffer,
                            >,
                            quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppArray<
                                    crate::UnityEngine::Rendering::RenderTargetIdentifier,
                                >,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::MaterialPropertyBlock,
                            >,
                            i32,
                        ),
                        quest_hook::libil2cpp::Void,
                        5usize,
                    >("DrawFullScreen")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "DrawFullScreen", 5usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info
                .invoke_unchecked(
                    (),
                    (commandBuffer, material, colorBuffers, properties, shaderPassId),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn DrawFullScreen_CommandBuffer_Il2CppArray_RenderTargetIdentifier_MaterialPropertyBlock_i32_4(
        commandBuffer: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Rendering::CommandBuffer,
        >,
        material: quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
        colorBuffers: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                crate::UnityEngine::Rendering::RenderTargetIdentifier,
            >,
        >,
        depthStencilBuffer: crate::UnityEngine::Rendering::RenderTargetIdentifier,
        properties: quest_hook::libil2cpp::Gc<crate::UnityEngine::MaterialPropertyBlock>,
        shaderPassId: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::Rendering::CommandBuffer,
                            >,
                            quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppArray<
                                    crate::UnityEngine::Rendering::RenderTargetIdentifier,
                                >,
                            >,
                            crate::UnityEngine::Rendering::RenderTargetIdentifier,
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::MaterialPropertyBlock,
                            >,
                            i32,
                        ),
                        quest_hook::libil2cpp::Void,
                        6usize,
                    >("DrawFullScreen")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "DrawFullScreen", 6usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info
                .invoke_unchecked(
                    (),
                    (
                        commandBuffer,
                        material,
                        colorBuffers,
                        depthStencilBuffer,
                        properties,
                        shaderPassId,
                    ),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn DrawFullScreen_CommandBuffer_MaterialPropertyBlock_i32_0(
        commandBuffer: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Rendering::CommandBuffer,
        >,
        material: quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
        properties: quest_hook::libil2cpp::Gc<crate::UnityEngine::MaterialPropertyBlock>,
        shaderPassId: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::Rendering::CommandBuffer,
                            >,
                            quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::MaterialPropertyBlock,
                            >,
                            i32,
                        ),
                        quest_hook::libil2cpp::Void,
                        4usize,
                    >("DrawFullScreen")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "DrawFullScreen", 4usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info
                .invoke_unchecked(
                    (),
                    (commandBuffer, material, properties, shaderPassId),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn DrawFullScreen_CommandBuffer_RenderTargetIdentifier_MaterialPropertyBlock_i32_2(
        commandBuffer: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Rendering::CommandBuffer,
        >,
        material: quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
        colorBuffer: crate::UnityEngine::Rendering::RenderTargetIdentifier,
        properties: quest_hook::libil2cpp::Gc<crate::UnityEngine::MaterialPropertyBlock>,
        shaderPassId: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::Rendering::CommandBuffer,
                            >,
                            quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
                            crate::UnityEngine::Rendering::RenderTargetIdentifier,
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::MaterialPropertyBlock,
                            >,
                            i32,
                        ),
                        quest_hook::libil2cpp::Void,
                        5usize,
                    >("DrawFullScreen")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "DrawFullScreen", 5usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info
                .invoke_unchecked(
                    (),
                    (commandBuffer, material, colorBuffer, properties, shaderPassId),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn DrawFullScreen_CommandBuffer_RenderTargetIdentifier_RenderTargetIdentifier_MaterialPropertyBlock_i32_3(
        commandBuffer: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Rendering::CommandBuffer,
        >,
        material: quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
        colorBuffer: crate::UnityEngine::Rendering::RenderTargetIdentifier,
        depthStencilBuffer: crate::UnityEngine::Rendering::RenderTargetIdentifier,
        properties: quest_hook::libil2cpp::Gc<crate::UnityEngine::MaterialPropertyBlock>,
        shaderPassId: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::Rendering::CommandBuffer,
                            >,
                            quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
                            crate::UnityEngine::Rendering::RenderTargetIdentifier,
                            crate::UnityEngine::Rendering::RenderTargetIdentifier,
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::MaterialPropertyBlock,
                            >,
                            i32,
                        ),
                        quest_hook::libil2cpp::Void,
                        6usize,
                    >("DrawFullScreen")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "DrawFullScreen", 6usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info
                .invoke_unchecked(
                    (),
                    (
                        commandBuffer,
                        material,
                        colorBuffer,
                        depthStencilBuffer,
                        properties,
                        shaderPassId,
                    ),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn DrawFullScreen_RasterCommandBuffer_MaterialPropertyBlock_i32_1(
        commandBuffer: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Rendering::RasterCommandBuffer,
        >,
        material: quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
        properties: quest_hook::libil2cpp::Gc<crate::UnityEngine::MaterialPropertyBlock>,
        shaderPassId: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::Rendering::RasterCommandBuffer,
                            >,
                            quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::MaterialPropertyBlock,
                            >,
                            i32,
                        ),
                        quest_hook::libil2cpp::Void,
                        4usize,
                    >("DrawFullScreen")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "DrawFullScreen", 4usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info
                .invoke_unchecked(
                    (),
                    (commandBuffer, material, properties, shaderPassId),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn DrawRendererList(
        renderContext: crate::UnityEngine::Rendering::ScriptableRenderContext,
        cmd: quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::CommandBuffer>,
        rendererList: crate::UnityEngine::Rendering::RendererList,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            crate::UnityEngine::Rendering::ScriptableRenderContext,
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::Rendering::CommandBuffer,
                            >,
                            crate::UnityEngine::Rendering::RendererList,
                        ),
                        quest_hook::libil2cpp::Void,
                        3usize,
                    >("DrawRendererList")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "DrawRendererList", 3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked((), (renderContext, cmd, rendererList))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn FixupDepthSlice_CubemapFace1(
        depthSlice: i32,
        cubemapFace: crate::UnityEngine::CubemapFace,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (i32, crate::UnityEngine::CubemapFace),
                        i32,
                        2usize,
                    >("FixupDepthSlice")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "FixupDepthSlice", 2usize
                        )
                    })
            });
        let __cordl_ret: i32 = unsafe {
            cordl_method_info.invoke_unchecked((), (depthSlice, cubemapFace))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn FixupDepthSlice_RTHandle0(
        depthSlice: i32,
        buffer: quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::RTHandle>,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            i32,
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::Rendering::RTHandle,
                            >,
                        ),
                        i32,
                        2usize,
                    >("FixupDepthSlice")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "FixupDepthSlice", 2usize
                        )
                    })
            });
        let __cordl_ret: i32 = unsafe {
            cordl_method_info.invoke_unchecked((), (depthSlice, buffer))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetAllAssemblyTypes() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<
                quest_hook::libil2cpp::Gc<crate::System::Type>,
            >,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (),
                        quest_hook::libil2cpp::Gc<
                            crate::System::Collections::Generic::IEnumerable_1<
                                quest_hook::libil2cpp::Gc<crate::System::Type>,
                            >,
                        >,
                        0usize,
                    >("GetAllAssemblyTypes")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "GetAllAssemblyTypes", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<
                quest_hook::libil2cpp::Gc<crate::System::Type>,
            >,
        > = unsafe { cordl_method_info.invoke_unchecked((), ())? };
        Ok(__cordl_ret.into())
    }
    pub fn GetAllTypesDerivedFrom<T>() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<
                quest_hook::libil2cpp::Gc<crate::System::Type>,
            >,
        >,
    >
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (),
                        quest_hook::libil2cpp::Gc<
                            crate::System::Collections::Generic::IEnumerable_1<
                                quest_hook::libil2cpp::Gc<crate::System::Type>,
                            >,
                        >,
                        0usize,
                    >("GetAllTypesDerivedFrom")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "GetAllTypesDerivedFrom", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<
                quest_hook::libil2cpp::Gc<crate::System::Type>,
            >,
        > = unsafe { cordl_method_info.invoke_unchecked((), ())? };
        Ok(__cordl_ret.into())
    }
    pub fn GetCorePath() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (),
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        0usize,
                    >("GetCorePath")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "GetCorePath", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = unsafe { cordl_method_info.invoke_unchecked((), ())? };
        Ok(__cordl_ret.into())
    }
    pub fn GetLastEnumValue<T>() -> quest_hook::libil2cpp::Result<T>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(), T, 0usize>("GetLastEnumValue")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "GetLastEnumValue", 0usize
                        )
                    })
            });
        let __cordl_ret: T = unsafe { cordl_method_info.invoke_unchecked((), ())? };
        Ok(__cordl_ret.into())
    }
    pub fn GetMipCount_f32_1(_cordl_size: f32) -> quest_hook::libil2cpp::Result<i32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(f32), i32, 1usize>("GetMipCount")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "GetMipCount", 1usize
                        )
                    })
            });
        let __cordl_ret: i32 = unsafe {
            cordl_method_info.invoke_unchecked((), (_cordl_size))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetMipCount_i32_0(_cordl_size: i32) -> quest_hook::libil2cpp::Result<i32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(i32), i32, 1usize>("GetMipCount")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "GetMipCount", 1usize
                        )
                    })
            });
        let __cordl_ret: i32 = unsafe {
            cordl_method_info.invoke_unchecked((), (_cordl_size))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetRenderTargetAutoName_GraphicsFormat_Il2CppString__cordl_bool_MSAASamples1(
        width: i32,
        height: i32,
        depth: i32,
        format: crate::UnityEngine::Experimental::Rendering::GraphicsFormat,
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        mips: bool,
        enableMSAA: bool,
        msaaSamples: crate::UnityEngine::Rendering::MSAASamples,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            i32,
                            i32,
                            i32,
                            crate::UnityEngine::Experimental::Rendering::GraphicsFormat,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                            bool,
                            bool,
                            crate::UnityEngine::Rendering::MSAASamples,
                        ),
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        8usize,
                    >("GetRenderTargetAutoName")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "GetRenderTargetAutoName", 8usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = unsafe {
            cordl_method_info
                .invoke_unchecked(
                    (),
                    (width, height, depth, format, name, mips, enableMSAA, msaaSamples),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetRenderTargetAutoName_GraphicsFormat_TextureDimension_Il2CppString__cordl_bool_MSAASamples__cordl_bool__cordl_bool2(
        width: i32,
        height: i32,
        depth: i32,
        format: crate::UnityEngine::Experimental::Rendering::GraphicsFormat,
        dim: crate::UnityEngine::Rendering::TextureDimension,
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        mips: bool,
        enableMSAA: bool,
        msaaSamples: crate::UnityEngine::Rendering::MSAASamples,
        dynamicRes: bool,
        dynamicResExplicit: bool,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            i32,
                            i32,
                            i32,
                            crate::UnityEngine::Experimental::Rendering::GraphicsFormat,
                            crate::UnityEngine::Rendering::TextureDimension,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                            bool,
                            bool,
                            crate::UnityEngine::Rendering::MSAASamples,
                            bool,
                            bool,
                        ),
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        11usize,
                    >("GetRenderTargetAutoName")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "GetRenderTargetAutoName", 11usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = unsafe {
            cordl_method_info
                .invoke_unchecked(
                    (),
                    (
                        width,
                        height,
                        depth,
                        format,
                        dim,
                        name,
                        mips,
                        enableMSAA,
                        msaaSamples,
                        dynamicRes,
                        dynamicResExplicit,
                    ),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetRenderTargetAutoName_Il2CppString_TextureDimension_Il2CppString__cordl_bool_MSAASamples__cordl_bool__cordl_bool3(
        width: i32,
        height: i32,
        depth: i32,
        format: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        dim: crate::UnityEngine::Rendering::TextureDimension,
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        mips: bool,
        enableMSAA: bool,
        msaaSamples: crate::UnityEngine::Rendering::MSAASamples,
        dynamicRes: bool,
        dynamicResExplicit: bool,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            i32,
                            i32,
                            i32,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                            crate::UnityEngine::Rendering::TextureDimension,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                            bool,
                            bool,
                            crate::UnityEngine::Rendering::MSAASamples,
                            bool,
                            bool,
                        ),
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        11usize,
                    >("GetRenderTargetAutoName")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "GetRenderTargetAutoName", 11usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = unsafe {
            cordl_method_info
                .invoke_unchecked(
                    (),
                    (
                        width,
                        height,
                        depth,
                        format,
                        dim,
                        name,
                        mips,
                        enableMSAA,
                        msaaSamples,
                        dynamicRes,
                        dynamicResExplicit,
                    ),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetRenderTargetAutoName_RenderTextureFormat_Il2CppString__cordl_bool_MSAASamples0(
        width: i32,
        height: i32,
        depth: i32,
        format: crate::UnityEngine::RenderTextureFormat,
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        mips: bool,
        enableMSAA: bool,
        msaaSamples: crate::UnityEngine::Rendering::MSAASamples,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            i32,
                            i32,
                            i32,
                            crate::UnityEngine::RenderTextureFormat,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                            bool,
                            bool,
                            crate::UnityEngine::Rendering::MSAASamples,
                        ),
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        8usize,
                    >("GetRenderTargetAutoName")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "GetRenderTargetAutoName", 8usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = unsafe {
            cordl_method_info
                .invoke_unchecked(
                    (),
                    (width, height, depth, format, name, mips, enableMSAA, msaaSamples),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetTextureAutoName_GraphicsFormat1(
        width: i32,
        height: i32,
        format: crate::UnityEngine::Experimental::Rendering::GraphicsFormat,
        dim: crate::UnityEngine::Rendering::TextureDimension,
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        mips: bool,
        depth: i32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            i32,
                            i32,
                            crate::UnityEngine::Experimental::Rendering::GraphicsFormat,
                            crate::UnityEngine::Rendering::TextureDimension,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                            bool,
                            i32,
                        ),
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        7usize,
                    >("GetTextureAutoName")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "GetTextureAutoName", 7usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = unsafe {
            cordl_method_info
                .invoke_unchecked((), (width, height, format, dim, name, mips, depth))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetTextureAutoName_Il2CppString2(
        width: i32,
        height: i32,
        format: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        dim: crate::UnityEngine::Rendering::TextureDimension,
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        mips: bool,
        depth: i32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            i32,
                            i32,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                            crate::UnityEngine::Rendering::TextureDimension,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                            bool,
                            i32,
                        ),
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        7usize,
                    >("GetTextureAutoName")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "GetTextureAutoName", 7usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = unsafe {
            cordl_method_info
                .invoke_unchecked((), (width, height, format, dim, name, mips, depth))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetTextureAutoName_TextureFormat0(
        width: i32,
        height: i32,
        format: crate::UnityEngine::TextureFormat,
        dim: crate::UnityEngine::Rendering::TextureDimension,
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        mips: bool,
        depth: i32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            i32,
                            i32,
                            crate::UnityEngine::TextureFormat,
                            crate::UnityEngine::Rendering::TextureDimension,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                            bool,
                            i32,
                        ),
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        7usize,
                    >("GetTextureAutoName")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "GetTextureAutoName", 7usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = unsafe {
            cordl_method_info
                .invoke_unchecked((), (width, height, format, dim, name, mips, depth))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetTextureHash(
        texture: quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture>,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture>),
                        i32,
                        1usize,
                    >("GetTextureHash")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "GetTextureHash", 1usize
                        )
                    })
            });
        let __cordl_ret: i32 = unsafe {
            cordl_method_info.invoke_unchecked((), (texture))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn HasFlag<T>(mask: T, flag: T) -> quest_hook::libil2cpp::Result<bool>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(T, T), bool, 2usize>("HasFlag")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "HasFlag",
                            2usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            cordl_method_info.invoke_unchecked((), (mask, flag))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn IsLightOverlapDebugEnabled(
        camera: quest_hook::libil2cpp::Gc<crate::UnityEngine::Camera>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (quest_hook::libil2cpp::Gc<crate::UnityEngine::Camera>),
                        bool,
                        1usize,
                    >("IsLightOverlapDebugEnabled")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "IsLightOverlapDebugEnabled", 1usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            cordl_method_info.invoke_unchecked((), (camera))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn IsSceneFilteringEnabled() -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(), bool, 0usize>("IsSceneFilteringEnabled")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "IsSceneFilteringEnabled", 0usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe { cordl_method_info.invoke_unchecked((), ())? };
        Ok(__cordl_ret.into())
    }
    pub fn IsSceneLightingDisabled(
        camera: quest_hook::libil2cpp::Gc<crate::UnityEngine::Camera>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (quest_hook::libil2cpp::Gc<crate::UnityEngine::Camera>),
                        bool,
                        1usize,
                    >("IsSceneLightingDisabled")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "IsSceneLightingDisabled", 1usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            cordl_method_info.invoke_unchecked((), (camera))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn IsSceneViewFogEnabled(
        camera: quest_hook::libil2cpp::Gc<crate::UnityEngine::Camera>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (quest_hook::libil2cpp::Gc<crate::UnityEngine::Camera>),
                        bool,
                        1usize,
                    >("IsSceneViewFogEnabled")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "IsSceneViewFogEnabled", 1usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            cordl_method_info.invoke_unchecked((), (camera))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn IsSceneViewPrefabStageContextHidden() -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (),
                        bool,
                        0usize,
                    >("IsSceneViewPrefabStageContextHidden")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "IsSceneViewPrefabStageContextHidden", 0usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe { cordl_method_info.invoke_unchecked((), ())? };
        Ok(__cordl_ret.into())
    }
    pub fn PreviousPowerOfTwo(_cordl_size: i32) -> quest_hook::libil2cpp::Result<i32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(i32), i32, 1usize>("PreviousPowerOfTwo")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "PreviousPowerOfTwo", 1usize
                        )
                    })
            });
        let __cordl_ret: i32 = unsafe {
            cordl_method_info.invoke_unchecked((), (_cordl_size))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn SafeRelease_ComputeBuffer1(
        buffer: quest_hook::libil2cpp::Gc<crate::UnityEngine::ComputeBuffer>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (quest_hook::libil2cpp::Gc<crate::UnityEngine::ComputeBuffer>),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("SafeRelease")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "SafeRelease", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked((), (buffer))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn SafeRelease_GraphicsBuffer0(
        buffer: quest_hook::libil2cpp::Gc<crate::UnityEngine::GraphicsBuffer>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (quest_hook::libil2cpp::Gc<crate::UnityEngine::GraphicsBuffer>),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("SafeRelease")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "SafeRelease", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked((), (buffer))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn SetKeyword_BaseCommandBuffer_Il2CppString__cordl_bool2(
        cmd: quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::BaseCommandBuffer>,
        keyword: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        state: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::Rendering::BaseCommandBuffer,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                            bool,
                        ),
                        quest_hook::libil2cpp::Void,
                        3usize,
                    >("SetKeyword")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "SetKeyword", 3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked((), (cmd, keyword, state))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn SetKeyword_CommandBuffer_ComputeShader_Il2CppString__cordl_bool1(
        cmd: quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::CommandBuffer>,
        cs: quest_hook::libil2cpp::Gc<crate::UnityEngine::ComputeShader>,
        keyword: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        state: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::Rendering::CommandBuffer,
                            >,
                            quest_hook::libil2cpp::Gc<crate::UnityEngine::ComputeShader>,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                            bool,
                        ),
                        quest_hook::libil2cpp::Void,
                        4usize,
                    >("SetKeyword")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "SetKeyword", 4usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked((), (cmd, cs, keyword, state))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn SetKeyword_CommandBuffer_Il2CppString__cordl_bool0(
        cmd: quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::CommandBuffer>,
        keyword: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        state: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::Rendering::CommandBuffer,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                            bool,
                        ),
                        quest_hook::libil2cpp::Void,
                        3usize,
                    >("SetKeyword")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "SetKeyword", 3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked((), (cmd, keyword, state))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn SetKeyword_ComputeShader_Il2CppString__cordl_bool5(
        cs: quest_hook::libil2cpp::Gc<crate::UnityEngine::ComputeShader>,
        keyword: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        state: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<crate::UnityEngine::ComputeShader>,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                            bool,
                        ),
                        quest_hook::libil2cpp::Void,
                        3usize,
                    >("SetKeyword")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "SetKeyword", 3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked((), (cs, keyword, state))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn SetKeyword_Material_Il2CppString__cordl_bool3(
        material: quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
        keyword: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        state: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                            bool,
                        ),
                        quest_hook::libil2cpp::Void,
                        3usize,
                    >("SetKeyword")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "SetKeyword", 3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked((), (material, keyword, state))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn SetKeyword_Material_LocalKeyword__cordl_bool4(
        material: quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
        keyword: crate::UnityEngine::Rendering::LocalKeyword,
        state: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
                            crate::UnityEngine::Rendering::LocalKeyword,
                            bool,
                        ),
                        quest_hook::libil2cpp::Void,
                        3usize,
                    >("SetKeyword")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "SetKeyword", 3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked((), (material, keyword, state))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn SetRenderTarget_Il2CppArray_RTHandle24(
        cmd: quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::CommandBuffer>,
        colorBuffers: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                crate::UnityEngine::Rendering::RenderTargetIdentifier,
            >,
        >,
        depthBuffer: quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::RTHandle>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::Rendering::CommandBuffer,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppArray<
                                    crate::UnityEngine::Rendering::RenderTargetIdentifier,
                                >,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::Rendering::RTHandle,
                            >,
                        ),
                        quest_hook::libil2cpp::Void,
                        3usize,
                    >("SetRenderTarget")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "SetRenderTarget", 3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked((), (cmd, colorBuffers, depthBuffer))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn SetRenderTarget_Il2CppArray_RTHandle_ClearFlag25(
        cmd: quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::CommandBuffer>,
        colorBuffers: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                crate::UnityEngine::Rendering::RenderTargetIdentifier,
            >,
        >,
        depthBuffer: quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::RTHandle>,
        clearFlag: crate::UnityEngine::Rendering::ClearFlag,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::Rendering::CommandBuffer,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppArray<
                                    crate::UnityEngine::Rendering::RenderTargetIdentifier,
                                >,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::Rendering::RTHandle,
                            >,
                            crate::UnityEngine::Rendering::ClearFlag,
                        ),
                        quest_hook::libil2cpp::Void,
                        4usize,
                    >("SetRenderTarget")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "SetRenderTarget", 4usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info
                .invoke_unchecked((), (cmd, colorBuffers, depthBuffer, clearFlag))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn SetRenderTarget_Il2CppArray_RTHandle_ClearFlag_Color26(
        cmd: quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::CommandBuffer>,
        colorBuffers: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                crate::UnityEngine::Rendering::RenderTargetIdentifier,
            >,
        >,
        depthBuffer: quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::RTHandle>,
        clearFlag: crate::UnityEngine::Rendering::ClearFlag,
        clearColor: crate::UnityEngine::Color,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::Rendering::CommandBuffer,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppArray<
                                    crate::UnityEngine::Rendering::RenderTargetIdentifier,
                                >,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::Rendering::RTHandle,
                            >,
                            crate::UnityEngine::Rendering::ClearFlag,
                            crate::UnityEngine::Color,
                        ),
                        quest_hook::libil2cpp::Void,
                        5usize,
                    >("SetRenderTarget")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "SetRenderTarget", 5usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info
                .invoke_unchecked(
                    (),
                    (cmd, colorBuffers, depthBuffer, clearFlag, clearColor),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn SetRenderTarget_Il2CppArray_RenderTargetIdentifier5(
        cmd: quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::CommandBuffer>,
        colorBuffers: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                crate::UnityEngine::Rendering::RenderTargetIdentifier,
            >,
        >,
        depthBuffer: crate::UnityEngine::Rendering::RenderTargetIdentifier,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::Rendering::CommandBuffer,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppArray<
                                    crate::UnityEngine::Rendering::RenderTargetIdentifier,
                                >,
                            >,
                            crate::UnityEngine::Rendering::RenderTargetIdentifier,
                        ),
                        quest_hook::libil2cpp::Void,
                        3usize,
                    >("SetRenderTarget")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "SetRenderTarget", 3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked((), (cmd, colorBuffers, depthBuffer))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn SetRenderTarget_Il2CppArray_RenderTargetIdentifier_ClearFlag6(
        cmd: quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::CommandBuffer>,
        colorBuffers: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                crate::UnityEngine::Rendering::RenderTargetIdentifier,
            >,
        >,
        depthBuffer: crate::UnityEngine::Rendering::RenderTargetIdentifier,
        clearFlag: crate::UnityEngine::Rendering::ClearFlag,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::Rendering::CommandBuffer,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppArray<
                                    crate::UnityEngine::Rendering::RenderTargetIdentifier,
                                >,
                            >,
                            crate::UnityEngine::Rendering::RenderTargetIdentifier,
                            crate::UnityEngine::Rendering::ClearFlag,
                        ),
                        quest_hook::libil2cpp::Void,
                        4usize,
                    >("SetRenderTarget")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "SetRenderTarget", 4usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info
                .invoke_unchecked((), (cmd, colorBuffers, depthBuffer, clearFlag))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn SetRenderTarget_Il2CppArray_RenderTargetIdentifier_ClearFlag_Color7(
        cmd: quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::CommandBuffer>,
        colorBuffers: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                crate::UnityEngine::Rendering::RenderTargetIdentifier,
            >,
        >,
        depthBuffer: crate::UnityEngine::Rendering::RenderTargetIdentifier,
        clearFlag: crate::UnityEngine::Rendering::ClearFlag,
        clearColor: crate::UnityEngine::Color,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::Rendering::CommandBuffer,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppArray<
                                    crate::UnityEngine::Rendering::RenderTargetIdentifier,
                                >,
                            >,
                            crate::UnityEngine::Rendering::RenderTargetIdentifier,
                            crate::UnityEngine::Rendering::ClearFlag,
                            crate::UnityEngine::Color,
                        ),
                        quest_hook::libil2cpp::Void,
                        5usize,
                    >("SetRenderTarget")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "SetRenderTarget", 5usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info
                .invoke_unchecked(
                    (),
                    (cmd, colorBuffers, depthBuffer, clearFlag, clearColor),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn SetRenderTarget_RTHandle_ClearFlag_Color_i32_CubemapFace_i32_17(
        cmd: quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::CommandBuffer>,
        buffer: quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::RTHandle>,
        clearFlag: crate::UnityEngine::Rendering::ClearFlag,
        clearColor: crate::UnityEngine::Color,
        miplevel: i32,
        cubemapFace: crate::UnityEngine::CubemapFace,
        depthSlice: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::Rendering::CommandBuffer,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::Rendering::RTHandle,
                            >,
                            crate::UnityEngine::Rendering::ClearFlag,
                            crate::UnityEngine::Color,
                            i32,
                            crate::UnityEngine::CubemapFace,
                            i32,
                        ),
                        quest_hook::libil2cpp::Void,
                        7usize,
                    >("SetRenderTarget")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "SetRenderTarget", 7usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info
                .invoke_unchecked(
                    (),
                    (
                        cmd,
                        buffer,
                        clearFlag,
                        clearColor,
                        miplevel,
                        cubemapFace,
                        depthSlice,
                    ),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn SetRenderTarget_RTHandle_ClearFlag_i32_CubemapFace_i32_18(
        cmd: quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::CommandBuffer>,
        buffer: quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::RTHandle>,
        clearFlag: crate::UnityEngine::Rendering::ClearFlag,
        miplevel: i32,
        cubemapFace: crate::UnityEngine::CubemapFace,
        depthSlice: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::Rendering::CommandBuffer,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::Rendering::RTHandle,
                            >,
                            crate::UnityEngine::Rendering::ClearFlag,
                            i32,
                            crate::UnityEngine::CubemapFace,
                            i32,
                        ),
                        quest_hook::libil2cpp::Void,
                        6usize,
                    >("SetRenderTarget")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "SetRenderTarget", 6usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info
                .invoke_unchecked(
                    (),
                    (cmd, buffer, clearFlag, miplevel, cubemapFace, depthSlice),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn SetRenderTarget_RTHandle_RTHandle_ClearFlag_Color_i32_CubemapFace_i32_21(
        cmd: quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::CommandBuffer>,
        colorBuffer: quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::RTHandle>,
        depthBuffer: quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::RTHandle>,
        clearFlag: crate::UnityEngine::Rendering::ClearFlag,
        clearColor: crate::UnityEngine::Color,
        miplevel: i32,
        cubemapFace: crate::UnityEngine::CubemapFace,
        depthSlice: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::Rendering::CommandBuffer,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::Rendering::RTHandle,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::Rendering::RTHandle,
                            >,
                            crate::UnityEngine::Rendering::ClearFlag,
                            crate::UnityEngine::Color,
                            i32,
                            crate::UnityEngine::CubemapFace,
                            i32,
                        ),
                        quest_hook::libil2cpp::Void,
                        8usize,
                    >("SetRenderTarget")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "SetRenderTarget", 8usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info
                .invoke_unchecked(
                    (),
                    (
                        cmd,
                        colorBuffer,
                        depthBuffer,
                        clearFlag,
                        clearColor,
                        miplevel,
                        cubemapFace,
                        depthSlice,
                    ),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn SetRenderTarget_RTHandle_RTHandle_ClearFlag_i32_CubemapFace_i32_20(
        cmd: quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::CommandBuffer>,
        colorBuffer: quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::RTHandle>,
        depthBuffer: quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::RTHandle>,
        clearFlag: crate::UnityEngine::Rendering::ClearFlag,
        miplevel: i32,
        cubemapFace: crate::UnityEngine::CubemapFace,
        depthSlice: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::Rendering::CommandBuffer,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::Rendering::RTHandle,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::Rendering::RTHandle,
                            >,
                            crate::UnityEngine::Rendering::ClearFlag,
                            i32,
                            crate::UnityEngine::CubemapFace,
                            i32,
                        ),
                        quest_hook::libil2cpp::Void,
                        7usize,
                    >("SetRenderTarget")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "SetRenderTarget", 7usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info
                .invoke_unchecked(
                    (),
                    (
                        cmd,
                        colorBuffer,
                        depthBuffer,
                        clearFlag,
                        miplevel,
                        cubemapFace,
                        depthSlice,
                    ),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn SetRenderTarget_RTHandle_RTHandle_i32_CubemapFace_i32_19(
        cmd: quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::CommandBuffer>,
        colorBuffer: quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::RTHandle>,
        depthBuffer: quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::RTHandle>,
        miplevel: i32,
        cubemapFace: crate::UnityEngine::CubemapFace,
        depthSlice: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::Rendering::CommandBuffer,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::Rendering::RTHandle,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::Rendering::RTHandle,
                            >,
                            i32,
                            crate::UnityEngine::CubemapFace,
                            i32,
                        ),
                        quest_hook::libil2cpp::Void,
                        6usize,
                    >("SetRenderTarget")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "SetRenderTarget", 6usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info
                .invoke_unchecked(
                    (),
                    (cmd, colorBuffer, depthBuffer, miplevel, cubemapFace, depthSlice),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn SetRenderTarget_RTHandle_RenderBufferLoadAction_RenderBufferStoreAction_ClearFlag_Color_i32_CubemapFace_i32_22(
        cmd: quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::CommandBuffer>,
        buffer: quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::RTHandle>,
        loadAction: crate::UnityEngine::Rendering::RenderBufferLoadAction,
        storeAction: crate::UnityEngine::Rendering::RenderBufferStoreAction,
        clearFlag: crate::UnityEngine::Rendering::ClearFlag,
        clearColor: crate::UnityEngine::Color,
        miplevel: i32,
        cubemapFace: crate::UnityEngine::CubemapFace,
        depthSlice: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::Rendering::CommandBuffer,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::Rendering::RTHandle,
                            >,
                            crate::UnityEngine::Rendering::RenderBufferLoadAction,
                            crate::UnityEngine::Rendering::RenderBufferStoreAction,
                            crate::UnityEngine::Rendering::ClearFlag,
                            crate::UnityEngine::Color,
                            i32,
                            crate::UnityEngine::CubemapFace,
                            i32,
                        ),
                        quest_hook::libil2cpp::Void,
                        9usize,
                    >("SetRenderTarget")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "SetRenderTarget", 9usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info
                .invoke_unchecked(
                    (),
                    (
                        cmd,
                        buffer,
                        loadAction,
                        storeAction,
                        clearFlag,
                        clearColor,
                        miplevel,
                        cubemapFace,
                        depthSlice,
                    ),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn SetRenderTarget_RTHandle_RenderBufferLoadAction_RenderBufferStoreAction_RTHandle_RenderBufferLoadAction_RenderBufferStoreAction_ClearFlag_Color_i32_CubemapFace_i32_23(
        cmd: quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::CommandBuffer>,
        colorBuffer: quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::RTHandle>,
        colorLoadAction: crate::UnityEngine::Rendering::RenderBufferLoadAction,
        colorStoreAction: crate::UnityEngine::Rendering::RenderBufferStoreAction,
        depthBuffer: quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::RTHandle>,
        depthLoadAction: crate::UnityEngine::Rendering::RenderBufferLoadAction,
        depthStoreAction: crate::UnityEngine::Rendering::RenderBufferStoreAction,
        clearFlag: crate::UnityEngine::Rendering::ClearFlag,
        clearColor: crate::UnityEngine::Color,
        miplevel: i32,
        cubemapFace: crate::UnityEngine::CubemapFace,
        depthSlice: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::Rendering::CommandBuffer,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::Rendering::RTHandle,
                            >,
                            crate::UnityEngine::Rendering::RenderBufferLoadAction,
                            crate::UnityEngine::Rendering::RenderBufferStoreAction,
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::Rendering::RTHandle,
                            >,
                            crate::UnityEngine::Rendering::RenderBufferLoadAction,
                            crate::UnityEngine::Rendering::RenderBufferStoreAction,
                            crate::UnityEngine::Rendering::ClearFlag,
                            crate::UnityEngine::Color,
                            i32,
                            crate::UnityEngine::CubemapFace,
                            i32,
                        ),
                        quest_hook::libil2cpp::Void,
                        12usize,
                    >("SetRenderTarget")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "SetRenderTarget", 12usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info
                .invoke_unchecked(
                    (),
                    (
                        cmd,
                        colorBuffer,
                        colorLoadAction,
                        colorStoreAction,
                        depthBuffer,
                        depthLoadAction,
                        depthStoreAction,
                        clearFlag,
                        clearColor,
                        miplevel,
                        cubemapFace,
                        depthSlice,
                    ),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn SetRenderTarget_RenderTargetIdentifier_ClearFlag_Color_i32_CubemapFace_i32_0(
        cmd: quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::CommandBuffer>,
        buffer: crate::UnityEngine::Rendering::RenderTargetIdentifier,
        clearFlag: crate::UnityEngine::Rendering::ClearFlag,
        clearColor: crate::UnityEngine::Color,
        miplevel: i32,
        cubemapFace: crate::UnityEngine::CubemapFace,
        depthSlice: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::Rendering::CommandBuffer,
                            >,
                            crate::UnityEngine::Rendering::RenderTargetIdentifier,
                            crate::UnityEngine::Rendering::ClearFlag,
                            crate::UnityEngine::Color,
                            i32,
                            crate::UnityEngine::CubemapFace,
                            i32,
                        ),
                        quest_hook::libil2cpp::Void,
                        7usize,
                    >("SetRenderTarget")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "SetRenderTarget", 7usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info
                .invoke_unchecked(
                    (),
                    (
                        cmd,
                        buffer,
                        clearFlag,
                        clearColor,
                        miplevel,
                        cubemapFace,
                        depthSlice,
                    ),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn SetRenderTarget_RenderTargetIdentifier_ClearFlag_i32_CubemapFace_i32_1(
        cmd: quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::CommandBuffer>,
        buffer: crate::UnityEngine::Rendering::RenderTargetIdentifier,
        clearFlag: crate::UnityEngine::Rendering::ClearFlag,
        miplevel: i32,
        cubemapFace: crate::UnityEngine::CubemapFace,
        depthSlice: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::Rendering::CommandBuffer,
                            >,
                            crate::UnityEngine::Rendering::RenderTargetIdentifier,
                            crate::UnityEngine::Rendering::ClearFlag,
                            i32,
                            crate::UnityEngine::CubemapFace,
                            i32,
                        ),
                        quest_hook::libil2cpp::Void,
                        6usize,
                    >("SetRenderTarget")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "SetRenderTarget", 6usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info
                .invoke_unchecked(
                    (),
                    (cmd, buffer, clearFlag, miplevel, cubemapFace, depthSlice),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn SetRenderTarget_RenderTargetIdentifier_RenderBufferLoadAction_RenderBufferStoreAction_ClearFlag11(
        cmd: quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::CommandBuffer>,
        buffer: crate::UnityEngine::Rendering::RenderTargetIdentifier,
        loadAction: crate::UnityEngine::Rendering::RenderBufferLoadAction,
        storeAction: crate::UnityEngine::Rendering::RenderBufferStoreAction,
        clearFlag: crate::UnityEngine::Rendering::ClearFlag,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::Rendering::CommandBuffer,
                            >,
                            crate::UnityEngine::Rendering::RenderTargetIdentifier,
                            crate::UnityEngine::Rendering::RenderBufferLoadAction,
                            crate::UnityEngine::Rendering::RenderBufferStoreAction,
                            crate::UnityEngine::Rendering::ClearFlag,
                        ),
                        quest_hook::libil2cpp::Void,
                        5usize,
                    >("SetRenderTarget")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "SetRenderTarget", 5usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info
                .invoke_unchecked((), (cmd, buffer, loadAction, storeAction, clearFlag))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn SetRenderTarget_RenderTargetIdentifier_RenderBufferLoadAction_RenderBufferStoreAction_ClearFlag_Color8(
        cmd: quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::CommandBuffer>,
        buffer: crate::UnityEngine::Rendering::RenderTargetIdentifier,
        loadAction: crate::UnityEngine::Rendering::RenderBufferLoadAction,
        storeAction: crate::UnityEngine::Rendering::RenderBufferStoreAction,
        clearFlag: crate::UnityEngine::Rendering::ClearFlag,
        clearColor: crate::UnityEngine::Color,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::Rendering::CommandBuffer,
                            >,
                            crate::UnityEngine::Rendering::RenderTargetIdentifier,
                            crate::UnityEngine::Rendering::RenderBufferLoadAction,
                            crate::UnityEngine::Rendering::RenderBufferStoreAction,
                            crate::UnityEngine::Rendering::ClearFlag,
                            crate::UnityEngine::Color,
                        ),
                        quest_hook::libil2cpp::Void,
                        6usize,
                    >("SetRenderTarget")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "SetRenderTarget", 6usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info
                .invoke_unchecked(
                    (),
                    (cmd, buffer, loadAction, storeAction, clearFlag, clearColor),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn SetRenderTarget_RenderTargetIdentifier_RenderBufferLoadAction_RenderBufferStoreAction_ClearFlag_Color_i32_CubemapFace_i32_10(
        cmd: quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::CommandBuffer>,
        buffer: crate::UnityEngine::Rendering::RenderTargetIdentifier,
        loadAction: crate::UnityEngine::Rendering::RenderBufferLoadAction,
        storeAction: crate::UnityEngine::Rendering::RenderBufferStoreAction,
        clearFlag: crate::UnityEngine::Rendering::ClearFlag,
        clearColor: crate::UnityEngine::Color,
        miplevel: i32,
        cubemapFace: crate::UnityEngine::CubemapFace,
        depthSlice: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::Rendering::CommandBuffer,
                            >,
                            crate::UnityEngine::Rendering::RenderTargetIdentifier,
                            crate::UnityEngine::Rendering::RenderBufferLoadAction,
                            crate::UnityEngine::Rendering::RenderBufferStoreAction,
                            crate::UnityEngine::Rendering::ClearFlag,
                            crate::UnityEngine::Color,
                            i32,
                            crate::UnityEngine::CubemapFace,
                            i32,
                        ),
                        quest_hook::libil2cpp::Void,
                        9usize,
                    >("SetRenderTarget")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "SetRenderTarget", 9usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info
                .invoke_unchecked(
                    (),
                    (
                        cmd,
                        buffer,
                        loadAction,
                        storeAction,
                        clearFlag,
                        clearColor,
                        miplevel,
                        cubemapFace,
                        depthSlice,
                    ),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn SetRenderTarget_RenderTargetIdentifier_RenderBufferLoadAction_RenderBufferStoreAction_RenderBufferLoadAction_RenderBufferStoreAction_ClearFlag_Color15(
        cmd: quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::CommandBuffer>,
        buffer: crate::UnityEngine::Rendering::RenderTargetIdentifier,
        colorLoadAction: crate::UnityEngine::Rendering::RenderBufferLoadAction,
        colorStoreAction: crate::UnityEngine::Rendering::RenderBufferStoreAction,
        depthLoadAction: crate::UnityEngine::Rendering::RenderBufferLoadAction,
        depthStoreAction: crate::UnityEngine::Rendering::RenderBufferStoreAction,
        clearFlag: crate::UnityEngine::Rendering::ClearFlag,
        clearColor: crate::UnityEngine::Color,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::Rendering::CommandBuffer,
                            >,
                            crate::UnityEngine::Rendering::RenderTargetIdentifier,
                            crate::UnityEngine::Rendering::RenderBufferLoadAction,
                            crate::UnityEngine::Rendering::RenderBufferStoreAction,
                            crate::UnityEngine::Rendering::RenderBufferLoadAction,
                            crate::UnityEngine::Rendering::RenderBufferStoreAction,
                            crate::UnityEngine::Rendering::ClearFlag,
                            crate::UnityEngine::Color,
                        ),
                        quest_hook::libil2cpp::Void,
                        8usize,
                    >("SetRenderTarget")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "SetRenderTarget", 8usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info
                .invoke_unchecked(
                    (),
                    (
                        cmd,
                        buffer,
                        colorLoadAction,
                        colorStoreAction,
                        depthLoadAction,
                        depthStoreAction,
                        clearFlag,
                        clearColor,
                    ),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn SetRenderTarget_RenderTargetIdentifier_RenderBufferLoadAction_RenderBufferStoreAction_RenderTargetIdentifier_RenderBufferLoadAction_RenderBufferStoreAction_ClearFlag16(
        cmd: quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::CommandBuffer>,
        colorBuffer: crate::UnityEngine::Rendering::RenderTargetIdentifier,
        colorLoadAction: crate::UnityEngine::Rendering::RenderBufferLoadAction,
        colorStoreAction: crate::UnityEngine::Rendering::RenderBufferStoreAction,
        depthBuffer: crate::UnityEngine::Rendering::RenderTargetIdentifier,
        depthLoadAction: crate::UnityEngine::Rendering::RenderBufferLoadAction,
        depthStoreAction: crate::UnityEngine::Rendering::RenderBufferStoreAction,
        clearFlag: crate::UnityEngine::Rendering::ClearFlag,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::Rendering::CommandBuffer,
                            >,
                            crate::UnityEngine::Rendering::RenderTargetIdentifier,
                            crate::UnityEngine::Rendering::RenderBufferLoadAction,
                            crate::UnityEngine::Rendering::RenderBufferStoreAction,
                            crate::UnityEngine::Rendering::RenderTargetIdentifier,
                            crate::UnityEngine::Rendering::RenderBufferLoadAction,
                            crate::UnityEngine::Rendering::RenderBufferStoreAction,
                            crate::UnityEngine::Rendering::ClearFlag,
                        ),
                        quest_hook::libil2cpp::Void,
                        8usize,
                    >("SetRenderTarget")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "SetRenderTarget", 8usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info
                .invoke_unchecked(
                    (),
                    (
                        cmd,
                        colorBuffer,
                        colorLoadAction,
                        colorStoreAction,
                        depthBuffer,
                        depthLoadAction,
                        depthStoreAction,
                        clearFlag,
                    ),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn SetRenderTarget_RenderTargetIdentifier_RenderBufferLoadAction_RenderBufferStoreAction_RenderTargetIdentifier_RenderBufferLoadAction_RenderBufferStoreAction_ClearFlag_Color12(
        cmd: quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::CommandBuffer>,
        colorBuffer: crate::UnityEngine::Rendering::RenderTargetIdentifier,
        colorLoadAction: crate::UnityEngine::Rendering::RenderBufferLoadAction,
        colorStoreAction: crate::UnityEngine::Rendering::RenderBufferStoreAction,
        depthBuffer: crate::UnityEngine::Rendering::RenderTargetIdentifier,
        depthLoadAction: crate::UnityEngine::Rendering::RenderBufferLoadAction,
        depthStoreAction: crate::UnityEngine::Rendering::RenderBufferStoreAction,
        clearFlag: crate::UnityEngine::Rendering::ClearFlag,
        clearColor: crate::UnityEngine::Color,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::Rendering::CommandBuffer,
                            >,
                            crate::UnityEngine::Rendering::RenderTargetIdentifier,
                            crate::UnityEngine::Rendering::RenderBufferLoadAction,
                            crate::UnityEngine::Rendering::RenderBufferStoreAction,
                            crate::UnityEngine::Rendering::RenderTargetIdentifier,
                            crate::UnityEngine::Rendering::RenderBufferLoadAction,
                            crate::UnityEngine::Rendering::RenderBufferStoreAction,
                            crate::UnityEngine::Rendering::ClearFlag,
                            crate::UnityEngine::Color,
                        ),
                        quest_hook::libil2cpp::Void,
                        9usize,
                    >("SetRenderTarget")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "SetRenderTarget", 9usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info
                .invoke_unchecked(
                    (),
                    (
                        cmd,
                        colorBuffer,
                        colorLoadAction,
                        colorStoreAction,
                        depthBuffer,
                        depthLoadAction,
                        depthStoreAction,
                        clearFlag,
                        clearColor,
                    ),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn SetRenderTarget_RenderTargetIdentifier_RenderBufferLoadAction_RenderBufferStoreAction_RenderTargetIdentifier_RenderBufferLoadAction_RenderBufferStoreAction_ClearFlag_Color_i32_CubemapFace_i32_14(
        cmd: quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::CommandBuffer>,
        colorBuffer: crate::UnityEngine::Rendering::RenderTargetIdentifier,
        colorLoadAction: crate::UnityEngine::Rendering::RenderBufferLoadAction,
        colorStoreAction: crate::UnityEngine::Rendering::RenderBufferStoreAction,
        depthBuffer: crate::UnityEngine::Rendering::RenderTargetIdentifier,
        depthLoadAction: crate::UnityEngine::Rendering::RenderBufferLoadAction,
        depthStoreAction: crate::UnityEngine::Rendering::RenderBufferStoreAction,
        clearFlag: crate::UnityEngine::Rendering::ClearFlag,
        clearColor: crate::UnityEngine::Color,
        miplevel: i32,
        cubemapFace: crate::UnityEngine::CubemapFace,
        depthSlice: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::Rendering::CommandBuffer,
                            >,
                            crate::UnityEngine::Rendering::RenderTargetIdentifier,
                            crate::UnityEngine::Rendering::RenderBufferLoadAction,
                            crate::UnityEngine::Rendering::RenderBufferStoreAction,
                            crate::UnityEngine::Rendering::RenderTargetIdentifier,
                            crate::UnityEngine::Rendering::RenderBufferLoadAction,
                            crate::UnityEngine::Rendering::RenderBufferStoreAction,
                            crate::UnityEngine::Rendering::ClearFlag,
                            crate::UnityEngine::Color,
                            i32,
                            crate::UnityEngine::CubemapFace,
                            i32,
                        ),
                        quest_hook::libil2cpp::Void,
                        12usize,
                    >("SetRenderTarget")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "SetRenderTarget", 12usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info
                .invoke_unchecked(
                    (),
                    (
                        cmd,
                        colorBuffer,
                        colorLoadAction,
                        colorStoreAction,
                        depthBuffer,
                        depthLoadAction,
                        depthStoreAction,
                        clearFlag,
                        clearColor,
                        miplevel,
                        cubemapFace,
                        depthSlice,
                    ),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn SetRenderTarget_RenderTargetIdentifier_RenderBufferLoadAction_RenderBufferStoreAction_RenderTargetIdentifier_RenderBufferLoadAction_RenderBufferStoreAction_i32_CubemapFace_i32_13(
        cmd: quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::CommandBuffer>,
        colorBuffer: crate::UnityEngine::Rendering::RenderTargetIdentifier,
        colorLoadAction: crate::UnityEngine::Rendering::RenderBufferLoadAction,
        colorStoreAction: crate::UnityEngine::Rendering::RenderBufferStoreAction,
        depthBuffer: crate::UnityEngine::Rendering::RenderTargetIdentifier,
        depthLoadAction: crate::UnityEngine::Rendering::RenderBufferLoadAction,
        depthStoreAction: crate::UnityEngine::Rendering::RenderBufferStoreAction,
        miplevel: i32,
        cubemapFace: crate::UnityEngine::CubemapFace,
        depthSlice: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::Rendering::CommandBuffer,
                            >,
                            crate::UnityEngine::Rendering::RenderTargetIdentifier,
                            crate::UnityEngine::Rendering::RenderBufferLoadAction,
                            crate::UnityEngine::Rendering::RenderBufferStoreAction,
                            crate::UnityEngine::Rendering::RenderTargetIdentifier,
                            crate::UnityEngine::Rendering::RenderBufferLoadAction,
                            crate::UnityEngine::Rendering::RenderBufferStoreAction,
                            i32,
                            crate::UnityEngine::CubemapFace,
                            i32,
                        ),
                        quest_hook::libil2cpp::Void,
                        10usize,
                    >("SetRenderTarget")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "SetRenderTarget", 10usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info
                .invoke_unchecked(
                    (),
                    (
                        cmd,
                        colorBuffer,
                        colorLoadAction,
                        colorStoreAction,
                        depthBuffer,
                        depthLoadAction,
                        depthStoreAction,
                        miplevel,
                        cubemapFace,
                        depthSlice,
                    ),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn SetRenderTarget_RenderTargetIdentifier_RenderBufferLoadAction_RenderBufferStoreAction_i32_CubemapFace_i32_9(
        cmd: quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::CommandBuffer>,
        buffer: crate::UnityEngine::Rendering::RenderTargetIdentifier,
        loadAction: crate::UnityEngine::Rendering::RenderBufferLoadAction,
        storeAction: crate::UnityEngine::Rendering::RenderBufferStoreAction,
        miplevel: i32,
        cubemapFace: crate::UnityEngine::CubemapFace,
        depthSlice: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::Rendering::CommandBuffer,
                            >,
                            crate::UnityEngine::Rendering::RenderTargetIdentifier,
                            crate::UnityEngine::Rendering::RenderBufferLoadAction,
                            crate::UnityEngine::Rendering::RenderBufferStoreAction,
                            i32,
                            crate::UnityEngine::CubemapFace,
                            i32,
                        ),
                        quest_hook::libil2cpp::Void,
                        7usize,
                    >("SetRenderTarget")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "SetRenderTarget", 7usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info
                .invoke_unchecked(
                    (),
                    (
                        cmd,
                        buffer,
                        loadAction,
                        storeAction,
                        miplevel,
                        cubemapFace,
                        depthSlice,
                    ),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn SetRenderTarget_RenderTargetIdentifier_RenderTargetIdentifier_ClearFlag_Color_i32_CubemapFace_i32_4(
        cmd: quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::CommandBuffer>,
        colorBuffer: crate::UnityEngine::Rendering::RenderTargetIdentifier,
        depthBuffer: crate::UnityEngine::Rendering::RenderTargetIdentifier,
        clearFlag: crate::UnityEngine::Rendering::ClearFlag,
        clearColor: crate::UnityEngine::Color,
        miplevel: i32,
        cubemapFace: crate::UnityEngine::CubemapFace,
        depthSlice: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::Rendering::CommandBuffer,
                            >,
                            crate::UnityEngine::Rendering::RenderTargetIdentifier,
                            crate::UnityEngine::Rendering::RenderTargetIdentifier,
                            crate::UnityEngine::Rendering::ClearFlag,
                            crate::UnityEngine::Color,
                            i32,
                            crate::UnityEngine::CubemapFace,
                            i32,
                        ),
                        quest_hook::libil2cpp::Void,
                        8usize,
                    >("SetRenderTarget")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "SetRenderTarget", 8usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info
                .invoke_unchecked(
                    (),
                    (
                        cmd,
                        colorBuffer,
                        depthBuffer,
                        clearFlag,
                        clearColor,
                        miplevel,
                        cubemapFace,
                        depthSlice,
                    ),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn SetRenderTarget_RenderTargetIdentifier_RenderTargetIdentifier_ClearFlag_i32_CubemapFace_i32_3(
        cmd: quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::CommandBuffer>,
        colorBuffer: crate::UnityEngine::Rendering::RenderTargetIdentifier,
        depthBuffer: crate::UnityEngine::Rendering::RenderTargetIdentifier,
        clearFlag: crate::UnityEngine::Rendering::ClearFlag,
        miplevel: i32,
        cubemapFace: crate::UnityEngine::CubemapFace,
        depthSlice: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::Rendering::CommandBuffer,
                            >,
                            crate::UnityEngine::Rendering::RenderTargetIdentifier,
                            crate::UnityEngine::Rendering::RenderTargetIdentifier,
                            crate::UnityEngine::Rendering::ClearFlag,
                            i32,
                            crate::UnityEngine::CubemapFace,
                            i32,
                        ),
                        quest_hook::libil2cpp::Void,
                        7usize,
                    >("SetRenderTarget")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "SetRenderTarget", 7usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info
                .invoke_unchecked(
                    (),
                    (
                        cmd,
                        colorBuffer,
                        depthBuffer,
                        clearFlag,
                        miplevel,
                        cubemapFace,
                        depthSlice,
                    ),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn SetRenderTarget_RenderTargetIdentifier_RenderTargetIdentifier_i32_CubemapFace_i32_2(
        cmd: quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::CommandBuffer>,
        colorBuffer: crate::UnityEngine::Rendering::RenderTargetIdentifier,
        depthBuffer: crate::UnityEngine::Rendering::RenderTargetIdentifier,
        miplevel: i32,
        cubemapFace: crate::UnityEngine::CubemapFace,
        depthSlice: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::Rendering::CommandBuffer,
                            >,
                            crate::UnityEngine::Rendering::RenderTargetIdentifier,
                            crate::UnityEngine::Rendering::RenderTargetIdentifier,
                            i32,
                            crate::UnityEngine::CubemapFace,
                            i32,
                        ),
                        quest_hook::libil2cpp::Void,
                        6usize,
                    >("SetRenderTarget")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "SetRenderTarget", 6usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info
                .invoke_unchecked(
                    (),
                    (cmd, colorBuffer, depthBuffer, miplevel, cubemapFace, depthSlice),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn SetViewport(
        cmd: quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::CommandBuffer>,
        target: quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::RTHandle>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::Rendering::CommandBuffer,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::Rendering::RTHandle,
                            >,
                        ),
                        quest_hook::libil2cpp::Void,
                        2usize,
                    >("SetViewport")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "SetViewport", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked((), (cmd, target))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn SetViewportAndClear(
        cmd: quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::CommandBuffer>,
        buffer: quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::RTHandle>,
        clearFlag: crate::UnityEngine::Rendering::ClearFlag,
        clearColor: crate::UnityEngine::Color,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::Rendering::CommandBuffer,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::Rendering::RTHandle,
                            >,
                            crate::UnityEngine::Rendering::ClearFlag,
                            crate::UnityEngine::Color,
                        ),
                        quest_hook::libil2cpp::Void,
                        4usize,
                    >("SetViewportAndClear")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "SetViewportAndClear", 4usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked((), (cmd, buffer, clearFlag, clearColor))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Swap<T>(
        a: quest_hook::libil2cpp::ByRefMut<T>,
        b: quest_hook::libil2cpp::ByRefMut<T>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::ByRefMut<T>,
                            quest_hook::libil2cpp::ByRefMut<T>,
                        ),
                        quest_hook::libil2cpp::Void,
                        2usize,
                    >("Swap")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "Swap",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked((), (a, b))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_blackCubeTexture() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::Cubemap>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (),
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Cubemap>,
                        0usize,
                    >("get_blackCubeTexture")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "get_blackCubeTexture", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::Cubemap> = unsafe {
            cordl_method_info.invoke_unchecked((), ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_blackVolumeTexture() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture3D>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (),
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture3D>,
                        0usize,
                    >("get_blackVolumeTexture")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "get_blackVolumeTexture", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture3D> = unsafe {
            cordl_method_info.invoke_unchecked((), ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_emptyBuffer() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::GraphicsBuffer>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (),
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::GraphicsBuffer>,
                        0usize,
                    >("get_emptyBuffer")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "get_emptyBuffer", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::GraphicsBuffer> = unsafe {
            cordl_method_info.invoke_unchecked((), ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_emptyUAV() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::RenderTexture>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (),
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::RenderTexture>,
                        0usize,
                    >("get_emptyUAV")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "get_emptyUAV", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::RenderTexture> = unsafe {
            cordl_method_info.invoke_unchecked((), ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_magentaCubeTexture() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::Cubemap>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (),
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Cubemap>,
                        0usize,
                    >("get_magentaCubeTexture")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "get_magentaCubeTexture", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::Cubemap> = unsafe {
            cordl_method_info.invoke_unchecked((), ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_magentaCubeTextureArray() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::CubemapArray>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (),
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::CubemapArray>,
                        0usize,
                    >("get_magentaCubeTextureArray")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "get_magentaCubeTextureArray", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::CubemapArray> = unsafe {
            cordl_method_info.invoke_unchecked((), ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_whiteCubeTexture() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::Cubemap>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (),
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Cubemap>,
                        0usize,
                    >("get_whiteCubeTexture")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "get_whiteCubeTexture", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::Cubemap> = unsafe {
            cordl_method_info.invoke_unchecked((), ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_whiteVolumeTexture() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture3D>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (),
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture3D>,
                        0usize,
                    >("get_whiteVolumeTexture")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "get_whiteVolumeTexture", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture3D> = unsafe {
            cordl_method_info.invoke_unchecked((), ())?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "cordl_class_UnityEngine+Rendering+CoreUtils")]
impl quest_hook::libil2cpp::ObjectType for crate::UnityEngine::Rendering::CoreUtils {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "cordl_class_UnityEngine+Rendering+CoreUtils+Priorities")]
#[repr(C)]
#[derive(Debug)]
pub struct CoreUtils_Priorities {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "cordl_class_UnityEngine+Rendering+CoreUtils+Priorities")]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::Rendering::CoreUtils_Priorities {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine.Rendering";
    const CLASS_NAME: &'static str = "CoreUtils/Priorities";
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
#[cfg(feature = "UnityEngine+Rendering+CoreUtils+Priorities")]
impl std::ops::Deref for crate::UnityEngine::Rendering::CoreUtils_Priorities {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Rendering+CoreUtils+Priorities")]
impl std::ops::DerefMut for crate::UnityEngine::Rendering::CoreUtils_Priorities {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Rendering+CoreUtils+Priorities")]
impl crate::UnityEngine::Rendering::CoreUtils_Priorities {
    pub const assetsCreateRenderingMenuPriority: i32 = 308i32;
    pub const assetsCreateShaderMenuPriority: i32 = 83i32;
    pub const editMenuPriority: i32 = 320i32;
    pub const gameObjectMenuPriority: i32 = 10i32;
    pub const srpLensFlareMenuPriority: i32 = 303i32;
}
#[cfg(feature = "cordl_class_UnityEngine+Rendering+CoreUtils+Priorities")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::Rendering::CoreUtils_Priorities {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "cordl_class_UnityEngine+Rendering+CoreUtils+Sections")]
#[repr(C)]
#[derive(Debug)]
pub struct CoreUtils_Sections {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "cordl_class_UnityEngine+Rendering+CoreUtils+Sections")]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::Rendering::CoreUtils_Sections {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine.Rendering";
    const CLASS_NAME: &'static str = "CoreUtils/Sections";
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
#[cfg(feature = "UnityEngine+Rendering+CoreUtils+Sections")]
impl std::ops::Deref for crate::UnityEngine::Rendering::CoreUtils_Sections {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Rendering+CoreUtils+Sections")]
impl std::ops::DerefMut for crate::UnityEngine::Rendering::CoreUtils_Sections {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Rendering+CoreUtils+Sections")]
impl crate::UnityEngine::Rendering::CoreUtils_Sections {
    pub const section1: i32 = 10000i32;
    pub const section2: i32 = 20000i32;
    pub const section3: i32 = 30000i32;
    pub const section4: i32 = 40000i32;
    pub const section5: i32 = 50000i32;
    pub const section6: i32 = 60000i32;
    pub const section7: i32 = 70000i32;
    pub const section8: i32 = 80000i32;
}
#[cfg(feature = "cordl_class_UnityEngine+Rendering+CoreUtils+Sections")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::Rendering::CoreUtils_Sections {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
