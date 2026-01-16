#[cfg(
    feature = "cordl_class_UnityEngine+Rendering+RenderGraphModule+NativeRenderPassCompiler+PassFragmentData"
)]
#[derive(Debug, Clone, Default, PartialEq)]
#[repr(C)]
pub struct PassFragmentData {
    pub resource: crate::UnityEngine::Rendering::RenderGraphModule::ResourceHandle,
    pub accessFlags: crate::UnityEngine::Rendering::RenderGraphModule::AccessFlags,
    pub mipLevel: i32,
    pub depthSlice: i32,
}
#[cfg(
    feature = "cordl_class_UnityEngine+Rendering+RenderGraphModule+NativeRenderPassCompiler+PassFragmentData"
)]
unsafe impl quest_hook::libil2cpp::Type
    for crate::UnityEngine::Rendering::RenderGraphModule::NativeRenderPassCompiler::PassFragmentData
{
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str =
        "UnityEngine.Rendering.RenderGraphModule.NativeRenderPassCompiler";
    const CLASS_NAME: &'static str = "PassFragmentData";
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
#[cfg(
    feature = "cordl_class_UnityEngine+Rendering+RenderGraphModule+NativeRenderPassCompiler+PassFragmentData"
)]
unsafe impl quest_hook::libil2cpp::Argument
    for crate::UnityEngine::Rendering::RenderGraphModule::NativeRenderPassCompiler::PassFragmentData
{
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(
    feature = "cordl_class_UnityEngine+Rendering+RenderGraphModule+NativeRenderPassCompiler+PassFragmentData"
)]
unsafe impl quest_hook::libil2cpp::Parameter
    for crate::UnityEngine::Rendering::RenderGraphModule::NativeRenderPassCompiler::PassFragmentData
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
#[cfg(
    feature = "cordl_class_UnityEngine+Rendering+RenderGraphModule+NativeRenderPassCompiler+PassFragmentData"
)]
unsafe impl quest_hook::libil2cpp::Returned
    for crate::UnityEngine::Rendering::RenderGraphModule::NativeRenderPassCompiler::PassFragmentData
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
#[cfg(
    feature = "cordl_class_UnityEngine+Rendering+RenderGraphModule+NativeRenderPassCompiler+PassFragmentData"
)]
unsafe impl quest_hook::libil2cpp::Return
    for crate::UnityEngine::Rendering::RenderGraphModule::NativeRenderPassCompiler::PassFragmentData
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
#[cfg(
    feature = "cordl_class_UnityEngine+Rendering+RenderGraphModule+NativeRenderPassCompiler+PassFragmentData"
)]
unsafe impl quest_hook::libil2cpp::ThisArgument
    for crate::UnityEngine::Rendering::RenderGraphModule::NativeRenderPassCompiler::PassFragmentData
{
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(
    feature = "UnityEngine+Rendering+RenderGraphModule+NativeRenderPassCompiler+PassFragmentData"
)]
impl crate::UnityEngine::Rendering::RenderGraphModule::NativeRenderPassCompiler::PassFragmentData {
    pub fn GetHashCode(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), i32, 0usize>("GetHashCode")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "GetHashCode",
                            0usize
                        )
                    })
            });
        let __cordl_ret: i32 = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn SameSubResource(
        x: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::Rendering::RenderGraphModule::NativeRenderPassCompiler::PassFragmentData,
        >,
        y: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::Rendering::RenderGraphModule::NativeRenderPassCompiler::PassFragmentData,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::ByRefMut<
                                crate::UnityEngine::Rendering::RenderGraphModule::NativeRenderPassCompiler::PassFragmentData,
                            >,
                            quest_hook::libil2cpp::ByRefMut<
                                crate::UnityEngine::Rendering::RenderGraphModule::NativeRenderPassCompiler::PassFragmentData,
                            >,
                        ),
                        bool,
                        2usize,
                    >("SameSubResource")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "SameSubResource", 2usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe { cordl_method_info.invoke_unchecked((), (x, y))? };
        Ok(__cordl_ret.into())
    }
}
