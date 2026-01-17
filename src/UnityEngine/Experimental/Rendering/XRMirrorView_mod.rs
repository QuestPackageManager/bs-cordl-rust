#[cfg(feature = "cordl_class_UnityEngine+Experimental+Rendering+XRMirrorView")]
#[repr(C)]
#[derive(Debug)]
pub struct XRMirrorView {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "cordl_class_UnityEngine+Experimental+Rendering+XRMirrorView")]
unsafe impl quest_hook::libil2cpp::Type
    for crate::UnityEngine::Experimental::Rendering::XRMirrorView
{
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine.Experimental.Rendering";
    const CLASS_NAME: &'static str = "XRMirrorView";
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
#[cfg(feature = "UnityEngine+Experimental+Rendering+XRMirrorView")]
impl std::ops::Deref for crate::UnityEngine::Experimental::Rendering::XRMirrorView {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Experimental+Rendering+XRMirrorView")]
impl std::ops::DerefMut for crate::UnityEngine::Experimental::Rendering::XRMirrorView {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Experimental+Rendering+XRMirrorView")]
impl crate::UnityEngine::Experimental::Rendering::XRMirrorView {
    pub fn RenderMirrorView(
        cmd: quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::CommandBuffer>,
        camera: quest_hook::libil2cpp::Gc<crate::UnityEngine::Camera>,
        mat: quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
        display: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::CommandBuffer>,
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Camera>,
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                    ), quest_hook::libil2cpp::Void, 4usize>("RenderMirrorView")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "RenderMirrorView",
                            4usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked((), (cmd, camera, mat, display))? };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "cordl_class_UnityEngine+Experimental+Rendering+XRMirrorView")]
impl quest_hook::libil2cpp::ObjectType
    for crate::UnityEngine::Experimental::Rendering::XRMirrorView
{
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
