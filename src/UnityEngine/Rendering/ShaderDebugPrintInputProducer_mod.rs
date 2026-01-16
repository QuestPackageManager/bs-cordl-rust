#[cfg(feature = "cordl_class_UnityEngine+Rendering+ShaderDebugPrintInputProducer")]
#[repr(C)]
#[derive(Debug)]
pub struct ShaderDebugPrintInputProducer {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "cordl_class_UnityEngine+Rendering+ShaderDebugPrintInputProducer")]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::Rendering::ShaderDebugPrintInputProducer {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine.Rendering";
    const CLASS_NAME: &'static str = "ShaderDebugPrintInputProducer";
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
#[cfg(feature = "UnityEngine+Rendering+ShaderDebugPrintInputProducer")]
impl std::ops::Deref for crate::UnityEngine::Rendering::ShaderDebugPrintInputProducer {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Rendering+ShaderDebugPrintInputProducer")]
impl std::ops::DerefMut
for crate::UnityEngine::Rendering::ShaderDebugPrintInputProducer {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Rendering+ShaderDebugPrintInputProducer")]
impl crate::UnityEngine::Rendering::ShaderDebugPrintInputProducer {
    pub fn Get() -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::Rendering::ShaderDebugPrintInput,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (),
                        crate::UnityEngine::Rendering::ShaderDebugPrintInput,
                        0usize,
                    >("Get")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "Get",
                            0usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::Rendering::ShaderDebugPrintInput = unsafe {
            cordl_method_info.invoke_unchecked((), ())?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "cordl_class_UnityEngine+Rendering+ShaderDebugPrintInputProducer")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::Rendering::ShaderDebugPrintInputProducer {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
