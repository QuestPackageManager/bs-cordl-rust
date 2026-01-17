#[cfg(
    feature = "cordl_class_UnityEngine+Rendering+RenderGraphModule+NativeRenderPassCompiler+ResourceVersionedData"
)]
#[derive(Debug, Clone, Default, PartialEq)]
#[repr(C)]
pub struct ResourceVersionedData {
    pub written: bool,
    pub writePassId: i32,
    pub numReaders: i32,
}
#[cfg(
    feature = "cordl_class_UnityEngine+Rendering+RenderGraphModule+NativeRenderPassCompiler+ResourceVersionedData"
)]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::Rendering::RenderGraphModule::NativeRenderPassCompiler::ResourceVersionedData {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine.Rendering.RenderGraphModule.NativeRenderPassCompiler";
    const CLASS_NAME: &'static str = "ResourceVersionedData";
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
            && <Self as quest_hook::libil2cpp::Type>::class()
                .is_assignable_from(ty.class())
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref()
            && <Self as quest_hook::libil2cpp::Type>::class()
                .is_assignable_from(ty.class())
    }
}
#[cfg(
    feature = "cordl_class_UnityEngine+Rendering+RenderGraphModule+NativeRenderPassCompiler+ResourceVersionedData"
)]
unsafe impl quest_hook::libil2cpp::Argument
for crate::UnityEngine::Rendering::RenderGraphModule::NativeRenderPassCompiler::ResourceVersionedData {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(
    feature = "cordl_class_UnityEngine+Rendering+RenderGraphModule+NativeRenderPassCompiler+ResourceVersionedData"
)]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::UnityEngine::Rendering::RenderGraphModule::NativeRenderPassCompiler::ResourceVersionedData {
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
    feature = "cordl_class_UnityEngine+Rendering+RenderGraphModule+NativeRenderPassCompiler+ResourceVersionedData"
)]
unsafe impl quest_hook::libil2cpp::Returned
for crate::UnityEngine::Rendering::RenderGraphModule::NativeRenderPassCompiler::ResourceVersionedData {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_returned(ty)
    }
    fn from_object(object: Option<&mut quest_hook::libil2cpp::Il2CppObject>) -> Self {
        unsafe {
            quest_hook::libil2cpp::raw::unbox(
                quest_hook::libil2cpp::WrapRaw::raw(object.unwrap()),
            )
        }
    }
}
#[cfg(
    feature = "cordl_class_UnityEngine+Rendering+RenderGraphModule+NativeRenderPassCompiler+ResourceVersionedData"
)]
unsafe impl quest_hook::libil2cpp::Return
for crate::UnityEngine::Rendering::RenderGraphModule::NativeRenderPassCompiler::ResourceVersionedData {
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
    feature = "cordl_class_UnityEngine+Rendering+RenderGraphModule+NativeRenderPassCompiler+ResourceVersionedData"
)]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::Rendering::RenderGraphModule::NativeRenderPassCompiler::ResourceVersionedData {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(
    feature = "UnityEngine+Rendering+RenderGraphModule+NativeRenderPassCompiler+ResourceVersionedData"
)]
impl crate::UnityEngine::Rendering::RenderGraphModule::NativeRenderPassCompiler::ResourceVersionedData {
    pub fn RegisterReadingPass(
        &mut self,
        ctx: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        h: crate::UnityEngine::Rendering::RenderGraphModule::ResourceHandle,
        passId: i32,
        index: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppObject,
                            >,
                            crate::UnityEngine::Rendering::RenderGraphModule::ResourceHandle,
                            i32,
                            i32,
                        ),
                        quest_hook::libil2cpp::Void,
                        4usize,
                    >("RegisterReadingPass")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "RegisterReadingPass", 4usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (ctx, h, passId, index))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn RemoveReadingPass(
        &mut self,
        ctx: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        h: crate::UnityEngine::Rendering::RenderGraphModule::ResourceHandle,
        passId: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppObject,
                            >,
                            crate::UnityEngine::Rendering::RenderGraphModule::ResourceHandle,
                            i32,
                        ),
                        quest_hook::libil2cpp::Void,
                        3usize,
                    >("RemoveReadingPass")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "RemoveReadingPass", 3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (ctx, h, passId))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn SetWritingPass(
        &mut self,
        ctx: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        h: crate::UnityEngine::Rendering::RenderGraphModule::ResourceHandle,
        passId: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppObject,
                            >,
                            crate::UnityEngine::Rendering::RenderGraphModule::ResourceHandle,
                            i32,
                        ),
                        quest_hook::libil2cpp::Void,
                        3usize,
                    >("SetWritingPass")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "SetWritingPass", 3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (ctx, h, passId))?
        };
        Ok(__cordl_ret.into())
    }
}
