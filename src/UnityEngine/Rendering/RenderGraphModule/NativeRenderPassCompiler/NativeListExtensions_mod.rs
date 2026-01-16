#[cfg(
    feature = "cordl_class_UnityEngine+Rendering+RenderGraphModule+NativeRenderPassCompiler+NativeListExtensions"
)]
#[repr(C)]
#[derive(Debug)]
pub struct NativeListExtensions {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(
    feature = "cordl_class_UnityEngine+Rendering+RenderGraphModule+NativeRenderPassCompiler+NativeListExtensions"
)]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::Rendering::RenderGraphModule::NativeRenderPassCompiler::NativeListExtensions {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine.Rendering.RenderGraphModule.NativeRenderPassCompiler";
    const CLASS_NAME: &'static str = "NativeListExtensions";
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
    feature = "UnityEngine+Rendering+RenderGraphModule+NativeRenderPassCompiler+NativeListExtensions"
)]
impl std::ops::Deref
for crate::UnityEngine::Rendering::RenderGraphModule::NativeRenderPassCompiler::NativeListExtensions {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(
    feature = "UnityEngine+Rendering+RenderGraphModule+NativeRenderPassCompiler+NativeListExtensions"
)]
impl std::ops::DerefMut
for crate::UnityEngine::Rendering::RenderGraphModule::NativeRenderPassCompiler::NativeListExtensions {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(
    feature = "UnityEngine+Rendering+RenderGraphModule+NativeRenderPassCompiler+NativeListExtensions"
)]
impl
    crate::UnityEngine::Rendering::RenderGraphModule::NativeRenderPassCompiler::NativeListExtensions
{
    pub fn LastIndex<T>(
        list: quest_hook::libil2cpp::ByRefMut<crate::Unity::Collections::NativeList_1<T>>,
    ) -> quest_hook::libil2cpp::Result<i32>
    where
        T: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (quest_hook::libil2cpp::ByRefMut<
                            crate::Unity::Collections::NativeList_1<T>,
                        >),
                        i32,
                        1usize,
                    >("LastIndex")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "LastIndex", 1usize
                        )
                    })
            });
        let __cordl_ret: i32 = unsafe { cordl_method_info.invoke_unchecked((), (list))? };
        Ok(__cordl_ret.into())
    }
    pub fn MakeReadOnlySpan<T>(
        list: quest_hook::libil2cpp::ByRefMut<crate::Unity::Collections::NativeList_1<T>>,
        first: i32,
        numElements: i32,
    ) -> quest_hook::libil2cpp::Result<crate::System::ReadOnlySpan_1<T>>
    where
        T: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        quest_hook::libil2cpp::ByRefMut<crate::Unity::Collections::NativeList_1<T>>,
                        i32,
                        i32,
                    ), crate::System::ReadOnlySpan_1<T>, 3usize>(
                        "MakeReadOnlySpan"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "MakeReadOnlySpan",
                            3usize
                        )
                    })
            });
        let __cordl_ret: crate::System::ReadOnlySpan_1<T> =
            unsafe { cordl_method_info.invoke_unchecked((), (list, first, numElements))? };
        Ok(__cordl_ret.into())
    }
}
#[cfg(
    feature = "cordl_class_UnityEngine+Rendering+RenderGraphModule+NativeRenderPassCompiler+NativeListExtensions"
)]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::Rendering::RenderGraphModule::NativeRenderPassCompiler::NativeListExtensions {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
