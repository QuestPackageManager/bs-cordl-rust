#[cfg(feature = "cordl_class_UnityEngine+Rendering+ProbeVolumePositioning")]
#[repr(C)]
#[derive(Debug)]
pub struct ProbeVolumePositioning {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "cordl_class_UnityEngine+Rendering+ProbeVolumePositioning")]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::Rendering::ProbeVolumePositioning {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine.Rendering";
    const CLASS_NAME: &'static str = "ProbeVolumePositioning";
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
#[cfg(feature = "UnityEngine+Rendering+ProbeVolumePositioning")]
impl std::ops::Deref for crate::UnityEngine::Rendering::ProbeVolumePositioning {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Rendering+ProbeVolumePositioning")]
impl std::ops::DerefMut for crate::UnityEngine::Rendering::ProbeVolumePositioning {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Rendering+ProbeVolumePositioning")]
impl crate::UnityEngine::Rendering::ProbeVolumePositioning {
    pub fn OBBAABBIntersect(
        a: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::Rendering::ProbeReferenceVolume_Volume,
        >,
        b: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Bounds>,
        aAABB: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Bounds>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::ByRefMut<
                                crate::UnityEngine::Rendering::ProbeReferenceVolume_Volume,
                            >,
                            quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Bounds>,
                            quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Bounds>,
                        ),
                        bool,
                        3usize,
                    >("OBBAABBIntersect")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "OBBAABBIntersect", 3usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            cordl_method_info.invoke_unchecked((), (a, b, aAABB))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn OBBContains(
        obb: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::Rendering::ProbeReferenceVolume_Volume,
        >,
        point: crate::UnityEngine::Vector3,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::ByRefMut<
                                crate::UnityEngine::Rendering::ProbeReferenceVolume_Volume,
                            >,
                            crate::UnityEngine::Vector3,
                        ),
                        bool,
                        2usize,
                    >("OBBContains")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "OBBContains", 2usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            cordl_method_info.invoke_unchecked((), (obb, point))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn OBBIntersect(
        a: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::Rendering::ProbeReferenceVolume_Volume,
        >,
        b: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::Rendering::ProbeReferenceVolume_Volume,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::ByRefMut<
                                crate::UnityEngine::Rendering::ProbeReferenceVolume_Volume,
                            >,
                            quest_hook::libil2cpp::ByRefMut<
                                crate::UnityEngine::Rendering::ProbeReferenceVolume_Volume,
                            >,
                        ),
                        bool,
                        2usize,
                    >("OBBIntersect")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "OBBIntersect", 2usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            cordl_method_info.invoke_unchecked((), (a, b))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ProjectAABB(
        corners: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<
                quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Vector3>,
            >,
        >,
        axis: crate::UnityEngine::Vector3,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector2> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::ByRefMut<
                                quest_hook::libil2cpp::Gc<
                                    quest_hook::libil2cpp::Il2CppArray<
                                        crate::UnityEngine::Vector3,
                                    >,
                                >,
                            >,
                            crate::UnityEngine::Vector3,
                        ),
                        crate::UnityEngine::Vector2,
                        2usize,
                    >("ProjectAABB")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "ProjectAABB", 2usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::Vector2 = unsafe {
            cordl_method_info.invoke_unchecked((), (corners, axis))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ProjectOBB(
        a: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::Rendering::ProbeReferenceVolume_Volume,
        >,
        axis: crate::UnityEngine::Vector3,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector2> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::ByRefMut<
                                crate::UnityEngine::Rendering::ProbeReferenceVolume_Volume,
                            >,
                            crate::UnityEngine::Vector3,
                        ),
                        crate::UnityEngine::Vector2,
                        2usize,
                    >("ProjectOBB")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "ProjectOBB", 2usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::Vector2 = unsafe {
            cordl_method_info.invoke_unchecked((), (a, axis))?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "cordl_class_UnityEngine+Rendering+ProbeVolumePositioning")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::Rendering::ProbeVolumePositioning {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
