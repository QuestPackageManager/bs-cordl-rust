#[cfg(feature = "UnityEngine+ProBuilder+TransformUtility")]
#[repr(C)]
#[derive(Debug)]
pub struct TransformUtility {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "UnityEngine+ProBuilder+TransformUtility")]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::ProBuilder::TransformUtility {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine.ProBuilder";
    const CLASS_NAME: &'static str = "TransformUtility";
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
#[cfg(feature = "UnityEngine+ProBuilder+TransformUtility")]
impl std::ops::Deref for crate::UnityEngine::ProBuilder::TransformUtility {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+ProBuilder+TransformUtility")]
impl std::ops::DerefMut for crate::UnityEngine::ProBuilder::TransformUtility {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+ProBuilder+TransformUtility")]
impl crate::UnityEngine::ProBuilder::TransformUtility {
    pub fn InverseTransformVertex(
        transform: quest_hook::libil2cpp::Gc<crate::UnityEngine::Transform>,
        vertex: quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::Vertex>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::Vertex>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<crate::UnityEngine::Transform>,
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::ProBuilder::Vertex,
                            >,
                        ),
                        quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::ProBuilder::Vertex,
                        >,
                        2usize,
                    >("InverseTransformVertex")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "InverseTransformVertex", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::ProBuilder::Vertex,
        > = unsafe { cordl_method_info.invoke_unchecked((), (transform, vertex))? };
        Ok(__cordl_ret.into())
    }
    pub fn ReparentChildren(
        t: quest_hook::libil2cpp::Gc<crate::UnityEngine::Transform>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (quest_hook::libil2cpp::Gc<crate::UnityEngine::Transform>),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("ReparentChildren")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "ReparentChildren", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked((), (t))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn TransformVertex(
        transform: quest_hook::libil2cpp::Gc<crate::UnityEngine::Transform>,
        vertex: quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::Vertex>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::Vertex>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<crate::UnityEngine::Transform>,
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::ProBuilder::Vertex,
                            >,
                        ),
                        quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::ProBuilder::Vertex,
                        >,
                        2usize,
                    >("TransformVertex")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "TransformVertex", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::ProBuilder::Vertex,
        > = unsafe { cordl_method_info.invoke_unchecked((), (transform, vertex))? };
        Ok(__cordl_ret.into())
    }
    pub fn UnparentChildren(
        t: quest_hook::libil2cpp::Gc<crate::UnityEngine::Transform>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (quest_hook::libil2cpp::Gc<crate::UnityEngine::Transform>),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("UnparentChildren")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "UnparentChildren", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked((), (t))?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+ProBuilder+TransformUtility")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::ProBuilder::TransformUtility {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
