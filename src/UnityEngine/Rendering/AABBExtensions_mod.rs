#[cfg(feature = "cordl_class_UnityEngine+Rendering+AABBExtensions")]
#[repr(C)]
#[derive(Debug)]
pub struct AABBExtensions {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "cordl_class_UnityEngine+Rendering+AABBExtensions")]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::Rendering::AABBExtensions {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine.Rendering";
    const CLASS_NAME: &'static str = "AABBExtensions";
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
#[cfg(feature = "UnityEngine+Rendering+AABBExtensions")]
impl std::ops::Deref for crate::UnityEngine::Rendering::AABBExtensions {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Rendering+AABBExtensions")]
impl std::ops::DerefMut for crate::UnityEngine::Rendering::AABBExtensions {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Rendering+AABBExtensions")]
impl crate::UnityEngine::Rendering::AABBExtensions {
    pub fn ToAABB(
        bounds: crate::UnityEngine::Bounds,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Rendering::AABB> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (crate::UnityEngine::Bounds),
                        crate::UnityEngine::Rendering::AABB,
                        1usize,
                    >("ToAABB")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "ToAABB",
                            1usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::Rendering::AABB = unsafe {
            cordl_method_info.invoke_unchecked((), (bounds))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ToBounds(
        aabb: crate::UnityEngine::Rendering::AABB,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Bounds> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (crate::UnityEngine::Rendering::AABB),
                        crate::UnityEngine::Bounds,
                        1usize,
                    >("ToBounds")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "ToBounds", 1usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::Bounds = unsafe {
            cordl_method_info.invoke_unchecked((), (aabb))?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "cordl_class_UnityEngine+Rendering+AABBExtensions")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::Rendering::AABBExtensions {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
