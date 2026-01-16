#[cfg(feature = "cordl_class_UnityEngine+Rendering+RenderGraphModule+IComputeRenderGraphBuilder")]
#[derive(Debug)]
#[repr(C)]
pub struct IComputeRenderGraphBuilder {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "cordl_class_UnityEngine+Rendering+RenderGraphModule+IComputeRenderGraphBuilder")]
unsafe impl quest_hook::libil2cpp::Type
    for crate::UnityEngine::Rendering::RenderGraphModule::IComputeRenderGraphBuilder
{
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine.Rendering.RenderGraphModule";
    const CLASS_NAME: &'static str = "IComputeRenderGraphBuilder";
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
#[cfg(feature = "UnityEngine+Rendering+RenderGraphModule+IComputeRenderGraphBuilder")]
impl std::ops::Deref
    for crate::UnityEngine::Rendering::RenderGraphModule::IComputeRenderGraphBuilder
{
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Rendering+RenderGraphModule+IComputeRenderGraphBuilder")]
impl std::ops::DerefMut
    for crate::UnityEngine::Rendering::RenderGraphModule::IComputeRenderGraphBuilder
{
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Rendering+RenderGraphModule+IComputeRenderGraphBuilder")]
impl crate::UnityEngine::Rendering::RenderGraphModule::IComputeRenderGraphBuilder {
    pub fn SetRenderFunc<PassData>(
        &mut self,
        renderFunc: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Rendering::RenderGraphModule::BaseRenderFunc_2<
                PassData,
                quest_hook::libil2cpp::Gc<
                    crate::UnityEngine::Rendering::RenderGraphModule::ComputeGraphContext,
                >,
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
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::Rendering::RenderGraphModule::BaseRenderFunc_2<
                                PassData,
                                quest_hook::libil2cpp::Gc<
                                    crate::UnityEngine::Rendering::RenderGraphModule::ComputeGraphContext,
                                >,
                            >,
                        >),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("SetRenderFunc")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "SetRenderFunc", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (renderFunc))? };
        Ok(__cordl_ret.into())
    }
    pub fn from_object_mut(object_param: *mut quest_hook::libil2cpp::Il2CppObject) -> *mut Self {
        unsafe { (object_param as *mut Self) }
    }
}
#[cfg(feature = "cordl_class_UnityEngine+Rendering+RenderGraphModule+IComputeRenderGraphBuilder")]
impl quest_hook::libil2cpp::ObjectType
    for crate::UnityEngine::Rendering::RenderGraphModule::IComputeRenderGraphBuilder
{
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "UnityEngine+Rendering+RenderGraphModule+IComputeRenderGraphBuilder")]
impl AsRef<crate::System::IDisposable>
    for crate::UnityEngine::Rendering::RenderGraphModule::IComputeRenderGraphBuilder
{
    fn as_ref(&self) -> &crate::System::IDisposable {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "UnityEngine+Rendering+RenderGraphModule+IComputeRenderGraphBuilder")]
impl AsMut<crate::System::IDisposable>
    for crate::UnityEngine::Rendering::RenderGraphModule::IComputeRenderGraphBuilder
{
    fn as_mut(&mut self) -> &mut crate::System::IDisposable {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "UnityEngine+Rendering+RenderGraphModule+IComputeRenderGraphBuilder")]
impl AsRef<crate::UnityEngine::Rendering::RenderGraphModule::IBaseRenderGraphBuilder>
    for crate::UnityEngine::Rendering::RenderGraphModule::IComputeRenderGraphBuilder
{
    fn as_ref(&self) -> &crate::UnityEngine::Rendering::RenderGraphModule::IBaseRenderGraphBuilder {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "UnityEngine+Rendering+RenderGraphModule+IComputeRenderGraphBuilder")]
impl AsMut<crate::UnityEngine::Rendering::RenderGraphModule::IBaseRenderGraphBuilder>
    for crate::UnityEngine::Rendering::RenderGraphModule::IComputeRenderGraphBuilder
{
    fn as_mut(
        &mut self,
    ) -> &mut crate::UnityEngine::Rendering::RenderGraphModule::IBaseRenderGraphBuilder {
        unsafe { std::mem::transmute(self) }
    }
}
