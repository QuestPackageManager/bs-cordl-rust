#[cfg(feature = "cordl_class_UnityEngine+Rendering+RayTracingShader")]
#[repr(C)]
#[derive(Debug)]
pub struct RayTracingShader {
    __cordl_parent: crate::UnityEngine::Object,
}
#[cfg(feature = "cordl_class_UnityEngine+Rendering+RayTracingShader")]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::Rendering::RayTracingShader {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine.Rendering";
    const CLASS_NAME: &'static str = "RayTracingShader";
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
#[cfg(feature = "UnityEngine+Rendering+RayTracingShader")]
impl std::ops::Deref for crate::UnityEngine::Rendering::RayTracingShader {
    type Target = crate::UnityEngine::Object;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Rendering+RayTracingShader")]
impl std::ops::DerefMut for crate::UnityEngine::Rendering::RayTracingShader {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Rendering+RayTracingShader")]
impl crate::UnityEngine::Rendering::RayTracingShader {}
#[cfg(feature = "cordl_class_UnityEngine+Rendering+RayTracingShader")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::Rendering::RayTracingShader {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
