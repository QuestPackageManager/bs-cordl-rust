#[cfg(
    feature = "cordl_class_UnityEngine+Experimental+Rendering+IScriptableRuntimeReflectionSystem"
)]
#[cfg_attr(feature = "derive_Debug", derive(Debug))]
#[repr(C)]
pub struct IScriptableRuntimeReflectionSystem {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(
    feature = "cordl_class_UnityEngine+Experimental+Rendering+IScriptableRuntimeReflectionSystem"
)]
unsafe impl quest_hook::libil2cpp::Type
    for crate::UnityEngine::Experimental::Rendering::IScriptableRuntimeReflectionSystem
{
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine.Experimental.Rendering";
    const CLASS_NAME: &'static str = "IScriptableRuntimeReflectionSystem";
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
#[cfg(feature = "UnityEngine+Experimental+Rendering+IScriptableRuntimeReflectionSystem")]
impl std::ops::Deref
    for crate::UnityEngine::Experimental::Rendering::IScriptableRuntimeReflectionSystem
{
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Experimental+Rendering+IScriptableRuntimeReflectionSystem")]
impl std::ops::DerefMut
    for crate::UnityEngine::Experimental::Rendering::IScriptableRuntimeReflectionSystem
{
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Experimental+Rendering+IScriptableRuntimeReflectionSystem")]
impl crate::UnityEngine::Experimental::Rendering::IScriptableRuntimeReflectionSystem {
    pub fn TickRealtimeProbes(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), bool, 0usize>("TickRealtimeProbes")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "TickRealtimeProbes",
                            0usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn from_object_mut(object_param: *mut quest_hook::libil2cpp::Il2CppObject) -> *mut Self {
        unsafe { (object_param as *mut Self) }
    }
}
#[cfg(
    feature = "cordl_class_UnityEngine+Experimental+Rendering+IScriptableRuntimeReflectionSystem"
)]
impl quest_hook::libil2cpp::ObjectType
    for crate::UnityEngine::Experimental::Rendering::IScriptableRuntimeReflectionSystem
{
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "UnityEngine+Experimental+Rendering+IScriptableRuntimeReflectionSystem")]
impl AsRef<crate::System::IDisposable>
    for crate::UnityEngine::Experimental::Rendering::IScriptableRuntimeReflectionSystem
{
    fn as_ref(&self) -> &crate::System::IDisposable {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "UnityEngine+Experimental+Rendering+IScriptableRuntimeReflectionSystem")]
impl AsMut<crate::System::IDisposable>
    for crate::UnityEngine::Experimental::Rendering::IScriptableRuntimeReflectionSystem
{
    fn as_mut(&mut self) -> &mut crate::System::IDisposable {
        unsafe { std::mem::transmute(self) }
    }
}
