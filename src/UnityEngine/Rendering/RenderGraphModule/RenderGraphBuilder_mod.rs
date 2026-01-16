#[cfg(feature = "cordl_class_UnityEngine+Rendering+RenderGraphModule+RenderGraphBuilder")]
#[derive(Debug, Clone, Default, PartialEq)]
#[repr(C)]
pub struct RenderGraphBuilder {
    pub m_RenderPass: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::Rendering::RenderGraphModule::RenderGraphPass,
    >,
    pub m_Resources: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::Rendering::RenderGraphModule::RenderGraphResourceRegistry,
    >,
    pub m_RenderGraph:
        quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::RenderGraphModule::RenderGraph>,
    pub m_Disposed: bool,
}
#[cfg(feature = "cordl_class_UnityEngine+Rendering+RenderGraphModule+RenderGraphBuilder")]
unsafe impl quest_hook::libil2cpp::Type
    for crate::UnityEngine::Rendering::RenderGraphModule::RenderGraphBuilder
{
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine.Rendering.RenderGraphModule";
    const CLASS_NAME: &'static str = "RenderGraphBuilder";
    fn matches_value_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        !ty.is_ref()
            && ty
                .class()
                .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref()
            && ty
                .class()
                .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_value_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        !ty.is_ref()
            && <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref() && <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
}
#[cfg(feature = "cordl_class_UnityEngine+Rendering+RenderGraphModule+RenderGraphBuilder")]
unsafe impl quest_hook::libil2cpp::Argument
    for crate::UnityEngine::Rendering::RenderGraphModule::RenderGraphBuilder
{
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "cordl_class_UnityEngine+Rendering+RenderGraphModule+RenderGraphBuilder")]
unsafe impl quest_hook::libil2cpp::Parameter
    for crate::UnityEngine::Rendering::RenderGraphModule::RenderGraphBuilder
{
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_parameter(ty)
    }
    fn from_actual(actual: <Self as quest_hook::libil2cpp::Parameter>::Actual) -> Self {
        actual
    }
    fn into_actual(self) -> <Self as quest_hook::libil2cpp::Parameter>::Actual {
        self
    }
}
#[cfg(feature = "cordl_class_UnityEngine+Rendering+RenderGraphModule+RenderGraphBuilder")]
unsafe impl quest_hook::libil2cpp::Returned
    for crate::UnityEngine::Rendering::RenderGraphModule::RenderGraphBuilder
{
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_returned(ty)
    }
    fn from_object(object: Option<&mut quest_hook::libil2cpp::Il2CppObject>) -> Self {
        unsafe {
            quest_hook::libil2cpp::raw::unbox(quest_hook::libil2cpp::WrapRaw::raw(object.unwrap()))
        }
    }
}
#[cfg(feature = "cordl_class_UnityEngine+Rendering+RenderGraphModule+RenderGraphBuilder")]
unsafe impl quest_hook::libil2cpp::Return
    for crate::UnityEngine::Rendering::RenderGraphModule::RenderGraphBuilder
{
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_return(ty)
    }
    fn into_actual(self) -> <Self as quest_hook::libil2cpp::Return>::Actual {
        self
    }
    fn from_actual(actual: <Self as quest_hook::libil2cpp::Return>::Actual) -> Self {
        actual
    }
}
#[cfg(feature = "cordl_class_UnityEngine+Rendering+RenderGraphModule+RenderGraphBuilder")]
unsafe impl quest_hook::libil2cpp::ThisArgument
    for crate::UnityEngine::Rendering::RenderGraphModule::RenderGraphBuilder
{
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+Rendering+RenderGraphModule+RenderGraphBuilder")]
impl crate::UnityEngine::Rendering::RenderGraphModule::RenderGraphBuilder {
    pub fn AllowPassCulling(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(bool), quest_hook::libil2cpp::Void, 1usize>("AllowPassCulling")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "AllowPassCulling",
                            1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (value))? };
        Ok(__cordl_ret.into())
    }
    pub fn AllowRendererListCulling(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(bool), quest_hook::libil2cpp::Void, 1usize>(
                        "AllowRendererListCulling",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "AllowRendererListCulling",
                            1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (value))? };
        Ok(__cordl_ret.into())
    }
    pub fn CheckResource(
        &mut self,
        res: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::Rendering::RenderGraphModule::ResourceHandle,
        >,
        checkTransientReadWrite: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::ByRefMut<
                            crate::UnityEngine::Rendering::RenderGraphModule::ResourceHandle,
                        >,
                        bool,
                    ), quest_hook::libil2cpp::Void, 2usize>("CheckResource")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "CheckResource",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (res, checkTransientReadWrite))? };
        Ok(__cordl_ret.into())
    }
    pub fn CreateTransientBuffer_ByRefMut0(
        &mut self,
        desc: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::Rendering::RenderGraphModule::BufferDesc,
        >,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Rendering::RenderGraphModule::BufferHandle>
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(quest_hook::libil2cpp::ByRefMut<
                        crate::UnityEngine::Rendering::RenderGraphModule::BufferDesc,
                    >), crate::UnityEngine::Rendering::RenderGraphModule::BufferHandle, 1usize>(
                        "CreateTransientBuffer",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "CreateTransientBuffer",
                            1usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::Rendering::RenderGraphModule::BufferHandle =
            unsafe { cordl_method_info.invoke_unchecked(self, (desc))? };
        Ok(__cordl_ret.into())
    }
    pub fn CreateTransientBuffer_ByRefMut1(
        &mut self,
        graphicsbuffer: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::Rendering::RenderGraphModule::BufferHandle,
        >,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Rendering::RenderGraphModule::BufferHandle>
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(quest_hook::libil2cpp::ByRefMut<
                        crate::UnityEngine::Rendering::RenderGraphModule::BufferHandle,
                    >), crate::UnityEngine::Rendering::RenderGraphModule::BufferHandle, 1usize>(
                        "CreateTransientBuffer",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "CreateTransientBuffer",
                            1usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::Rendering::RenderGraphModule::BufferHandle =
            unsafe { cordl_method_info.invoke_unchecked(self, (graphicsbuffer))? };
        Ok(__cordl_ret.into())
    }
    pub fn CreateTransientTexture_ByRefMut0(
        &mut self,
        desc: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::Rendering::RenderGraphModule::TextureDesc,
        >,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::Rendering::RenderGraphModule::TextureHandle,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(quest_hook::libil2cpp::ByRefMut<
                        crate::UnityEngine::Rendering::RenderGraphModule::TextureDesc,
                    >), crate::UnityEngine::Rendering::RenderGraphModule::TextureHandle, 1usize>(
                        "CreateTransientTexture",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "CreateTransientTexture",
                            1usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::Rendering::RenderGraphModule::TextureHandle =
            unsafe { cordl_method_info.invoke_unchecked(self, (desc))? };
        Ok(__cordl_ret.into())
    }
    pub fn CreateTransientTexture_ByRefMut1(
        &mut self,
        texture: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::Rendering::RenderGraphModule::TextureHandle,
        >,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::Rendering::RenderGraphModule::TextureHandle,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(quest_hook::libil2cpp::ByRefMut<
                        crate::UnityEngine::Rendering::RenderGraphModule::TextureHandle,
                    >), crate::UnityEngine::Rendering::RenderGraphModule::TextureHandle, 1usize>(
                        "CreateTransientTexture",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "CreateTransientTexture",
                            1usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::Rendering::RenderGraphModule::TextureHandle =
            unsafe { cordl_method_info.invoke_unchecked(self, (texture))? };
        Ok(__cordl_ret.into())
    }
    pub fn DependsOn(
        &mut self,
        input: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::Rendering::RenderGraphModule::RendererListHandle,
        >,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::Rendering::RenderGraphModule::RendererListHandle,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::ByRefMut<
                            crate::UnityEngine::Rendering::RenderGraphModule::RendererListHandle,
                        >),
                        crate::UnityEngine::Rendering::RenderGraphModule::RendererListHandle,
                        1usize,
                    >("DependsOn")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "DependsOn", 1usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::Rendering::RenderGraphModule::RendererListHandle =
            unsafe { cordl_method_info.invoke_unchecked(self, (input))? };
        Ok(__cordl_ret.into())
    }
    pub fn Dispose_0(&mut self) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), quest_hook::libil2cpp::Void, 0usize>("Dispose")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Dispose",
                            0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn Dispose__cordl_bool1(
        &mut self,
        disposing: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(bool), quest_hook::libil2cpp::Void, 1usize>("Dispose")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Dispose",
                            1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (disposing))? };
        Ok(__cordl_ret.into())
    }
    pub fn EnableAsyncCompute(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(bool), quest_hook::libil2cpp::Void, 1usize>(
                        "EnableAsyncCompute",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "EnableAsyncCompute",
                            1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (value))? };
        Ok(__cordl_ret.into())
    }
    pub fn EnableFoveatedRasterization(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(bool), quest_hook::libil2cpp::Void, 1usize>(
                        "EnableFoveatedRasterization",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "EnableFoveatedRasterization",
                            1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (value))? };
        Ok(__cordl_ret.into())
    }
    pub fn GenerateDebugData(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(bool), quest_hook::libil2cpp::Void, 1usize>("GenerateDebugData")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "GenerateDebugData",
                            1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (value))? };
        Ok(__cordl_ret.into())
    }
    pub fn ReadBuffer(
        &mut self,
        input: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::Rendering::RenderGraphModule::BufferHandle,
        >,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Rendering::RenderGraphModule::BufferHandle>
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(quest_hook::libil2cpp::ByRefMut<
                        crate::UnityEngine::Rendering::RenderGraphModule::BufferHandle,
                    >), crate::UnityEngine::Rendering::RenderGraphModule::BufferHandle, 1usize>(
                        "ReadBuffer",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "ReadBuffer",
                            1usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::Rendering::RenderGraphModule::BufferHandle =
            unsafe { cordl_method_info.invoke_unchecked(self, (input))? };
        Ok(__cordl_ret.into())
    }
    pub fn ReadRayTracingAccelerationStructure(
        &mut self,
        input: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::Rendering::RenderGraphModule::RayTracingAccelerationStructureHandle,
        >,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::Rendering::RenderGraphModule::RayTracingAccelerationStructureHandle,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::ByRefMut<
                            crate::UnityEngine::Rendering::RenderGraphModule::RayTracingAccelerationStructureHandle,
                        >),
                        crate::UnityEngine::Rendering::RenderGraphModule::RayTracingAccelerationStructureHandle,
                        1usize,
                    >("ReadRayTracingAccelerationStructure")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "ReadRayTracingAccelerationStructure", 1usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::Rendering::RenderGraphModule::RayTracingAccelerationStructureHandle = unsafe {
            cordl_method_info.invoke_unchecked(self, (input))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ReadTexture(
        &mut self,
        input: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::Rendering::RenderGraphModule::TextureHandle,
        >,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::Rendering::RenderGraphModule::TextureHandle,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(quest_hook::libil2cpp::ByRefMut<
                        crate::UnityEngine::Rendering::RenderGraphModule::TextureHandle,
                    >), crate::UnityEngine::Rendering::RenderGraphModule::TextureHandle, 1usize>(
                        "ReadTexture",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "ReadTexture",
                            1usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::Rendering::RenderGraphModule::TextureHandle =
            unsafe { cordl_method_info.invoke_unchecked(self, (input))? };
        Ok(__cordl_ret.into())
    }
    pub fn ReadWriteTexture(
        &mut self,
        input: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::Rendering::RenderGraphModule::TextureHandle,
        >,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::Rendering::RenderGraphModule::TextureHandle,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(quest_hook::libil2cpp::ByRefMut<
                        crate::UnityEngine::Rendering::RenderGraphModule::TextureHandle,
                    >), crate::UnityEngine::Rendering::RenderGraphModule::TextureHandle, 1usize>(
                        "ReadWriteTexture",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "ReadWriteTexture",
                            1usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::Rendering::RenderGraphModule::TextureHandle =
            unsafe { cordl_method_info.invoke_unchecked(self, (input))? };
        Ok(__cordl_ret.into())
    }
    pub fn SetRenderFunc<PassData>(
        &mut self,
        renderFunc: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Rendering::RenderGraphModule::BaseRenderFunc_2<
                PassData,
                crate::UnityEngine::Rendering::RenderGraphModule::RenderGraphContext,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        PassData: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(quest_hook::libil2cpp::Gc<
                        crate::UnityEngine::Rendering::RenderGraphModule::BaseRenderFunc_2<
                            PassData,
                            crate::UnityEngine::Rendering::RenderGraphModule::RenderGraphContext,
                        >,
                    >), quest_hook::libil2cpp::Void, 1usize>("SetRenderFunc")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "SetRenderFunc",
                            1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (renderFunc))? };
        Ok(__cordl_ret.into())
    }
    pub fn UseColorBuffer(
        &mut self,
        input: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::Rendering::RenderGraphModule::TextureHandle,
        >,
        index: i32,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::Rendering::RenderGraphModule::TextureHandle,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::ByRefMut<
                            crate::UnityEngine::Rendering::RenderGraphModule::TextureHandle,
                        >,
                        i32,
                    ), crate::UnityEngine::Rendering::RenderGraphModule::TextureHandle, 2usize>(
                        "UseColorBuffer",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "UseColorBuffer",
                            2usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::Rendering::RenderGraphModule::TextureHandle =
            unsafe { cordl_method_info.invoke_unchecked(self, (input, index))? };
        Ok(__cordl_ret.into())
    }
    pub fn UseDepthBuffer(
        &mut self,
        input: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::Rendering::RenderGraphModule::TextureHandle,
        >,
        flags: crate::UnityEngine::Rendering::RenderGraphModule::DepthAccess,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::Rendering::RenderGraphModule::TextureHandle,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::ByRefMut<
                            crate::UnityEngine::Rendering::RenderGraphModule::TextureHandle,
                        >,
                        crate::UnityEngine::Rendering::RenderGraphModule::DepthAccess,
                    ), crate::UnityEngine::Rendering::RenderGraphModule::TextureHandle, 2usize>(
                        "UseDepthBuffer",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "UseDepthBuffer",
                            2usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::Rendering::RenderGraphModule::TextureHandle =
            unsafe { cordl_method_info.invoke_unchecked(self, (input, flags))? };
        Ok(__cordl_ret.into())
    }
    pub fn UseRendererList(
        &mut self,
        input: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::Rendering::RenderGraphModule::RendererListHandle,
        >,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::Rendering::RenderGraphModule::RendererListHandle,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::ByRefMut<
                            crate::UnityEngine::Rendering::RenderGraphModule::RendererListHandle,
                        >),
                        crate::UnityEngine::Rendering::RenderGraphModule::RendererListHandle,
                        1usize,
                    >("UseRendererList")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "UseRendererList", 1usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::Rendering::RenderGraphModule::RendererListHandle =
            unsafe { cordl_method_info.invoke_unchecked(self, (input))? };
        Ok(__cordl_ret.into())
    }
    pub fn WriteBuffer(
        &mut self,
        input: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::Rendering::RenderGraphModule::BufferHandle,
        >,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Rendering::RenderGraphModule::BufferHandle>
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(quest_hook::libil2cpp::ByRefMut<
                        crate::UnityEngine::Rendering::RenderGraphModule::BufferHandle,
                    >), crate::UnityEngine::Rendering::RenderGraphModule::BufferHandle, 1usize>(
                        "WriteBuffer",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "WriteBuffer",
                            1usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::Rendering::RenderGraphModule::BufferHandle =
            unsafe { cordl_method_info.invoke_unchecked(self, (input))? };
        Ok(__cordl_ret.into())
    }
    pub fn WriteRayTracingAccelerationStructure(
        &mut self,
        input: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::Rendering::RenderGraphModule::RayTracingAccelerationStructureHandle,
        >,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::Rendering::RenderGraphModule::RayTracingAccelerationStructureHandle,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::ByRefMut<
                            crate::UnityEngine::Rendering::RenderGraphModule::RayTracingAccelerationStructureHandle,
                        >),
                        crate::UnityEngine::Rendering::RenderGraphModule::RayTracingAccelerationStructureHandle,
                        1usize,
                    >("WriteRayTracingAccelerationStructure")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "WriteRayTracingAccelerationStructure", 1usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::Rendering::RenderGraphModule::RayTracingAccelerationStructureHandle = unsafe {
            cordl_method_info.invoke_unchecked(self, (input))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn WriteTexture(
        &mut self,
        input: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::Rendering::RenderGraphModule::TextureHandle,
        >,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::Rendering::RenderGraphModule::TextureHandle,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(quest_hook::libil2cpp::ByRefMut<
                        crate::UnityEngine::Rendering::RenderGraphModule::TextureHandle,
                    >), crate::UnityEngine::Rendering::RenderGraphModule::TextureHandle, 1usize>(
                        "WriteTexture",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "WriteTexture",
                            1usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::Rendering::RenderGraphModule::TextureHandle =
            unsafe { cordl_method_info.invoke_unchecked(self, (input))? };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        renderPass: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Rendering::RenderGraphModule::RenderGraphPass,
        >,
        resources: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Rendering::RenderGraphModule::RenderGraphResourceRegistry,
        >,
        renderGraph: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Rendering::RenderGraphModule::RenderGraph,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::Rendering::RenderGraphModule::RenderGraphPass,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::Rendering::RenderGraphModule::RenderGraphResourceRegistry,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::Rendering::RenderGraphModule::RenderGraph,
                            >,
                        ),
                        quest_hook::libil2cpp::Void,
                        3usize,
                    >(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), ".ctor",
                            3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (renderPass, resources, renderGraph))?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+Rendering+RenderGraphModule+RenderGraphBuilder")]
impl AsRef<crate::System::IDisposable>
    for crate::UnityEngine::Rendering::RenderGraphModule::RenderGraphBuilder
{
    fn as_ref(&self) -> &crate::System::IDisposable {
        todo!()
    }
}
#[cfg(feature = "UnityEngine+Rendering+RenderGraphModule+RenderGraphBuilder")]
impl AsMut<crate::System::IDisposable>
    for crate::UnityEngine::Rendering::RenderGraphModule::RenderGraphBuilder
{
    fn as_mut(&mut self) -> &mut crate::System::IDisposable {
        todo!()
    }
}
