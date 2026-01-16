#[cfg(feature = "cordl_class_UnityEngine+Rendering+LODGroupRenderingUtils")]
#[repr(C)]
#[derive(Debug)]
pub struct LODGroupRenderingUtils {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "cordl_class_UnityEngine+Rendering+LODGroupRenderingUtils")]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::Rendering::LODGroupRenderingUtils {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine.Rendering";
    const CLASS_NAME: &'static str = "LODGroupRenderingUtils";
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
#[cfg(feature = "UnityEngine+Rendering+LODGroupRenderingUtils")]
impl std::ops::Deref for crate::UnityEngine::Rendering::LODGroupRenderingUtils {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Rendering+LODGroupRenderingUtils")]
impl std::ops::DerefMut for crate::UnityEngine::Rendering::LODGroupRenderingUtils {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Rendering+LODGroupRenderingUtils")]
impl crate::UnityEngine::Rendering::LODGroupRenderingUtils {
    pub fn CalculateFOVHalfAngle(
        fieldOfView: f32,
    ) -> quest_hook::libil2cpp::Result<f32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(f32), f32, 1usize>("CalculateFOVHalfAngle")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "CalculateFOVHalfAngle", 1usize
                        )
                    })
            });
        let __cordl_ret: f32 = unsafe {
            cordl_method_info.invoke_unchecked((), (fieldOfView))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn CalculateLODDistance(
        relativeScreenHeight: f32,
        _cordl_size: f32,
    ) -> quest_hook::libil2cpp::Result<f32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (f32, f32),
                        f32,
                        2usize,
                    >("CalculateLODDistance")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "CalculateLODDistance", 2usize
                        )
                    })
            });
        let __cordl_ret: f32 = unsafe {
            cordl_method_info.invoke_unchecked((), (relativeScreenHeight, _cordl_size))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn CalculatePerspectiveDistance(
        objPosition: crate::UnityEngine::Vector3,
        camPosition: crate::UnityEngine::Vector3,
        sqrScreenRelativeMetric: f32,
    ) -> quest_hook::libil2cpp::Result<f32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (crate::UnityEngine::Vector3, crate::UnityEngine::Vector3, f32),
                        f32,
                        3usize,
                    >("CalculatePerspectiveDistance")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "CalculatePerspectiveDistance", 3usize
                        )
                    })
            });
        let __cordl_ret: f32 = unsafe {
            cordl_method_info
                .invoke_unchecked(
                    (),
                    (objPosition, camPosition, sqrScreenRelativeMetric),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn CalculateScreenRelativeMetric(
        lodParams: crate::UnityEngine::Rendering::LODParameters,
        lodBias: f32,
    ) -> quest_hook::libil2cpp::Result<f32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (crate::UnityEngine::Rendering::LODParameters, f32),
                        f32,
                        2usize,
                    >("CalculateScreenRelativeMetric")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "CalculateScreenRelativeMetric", 2usize
                        )
                    })
            });
        let __cordl_ret: f32 = unsafe {
            cordl_method_info.invoke_unchecked((), (lodParams, lodBias))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn CalculateSqrPerspectiveDistance(
        objPosition: crate::UnityEngine::Vector3,
        camPosition: crate::UnityEngine::Vector3,
        sqrScreenRelativeMetric: f32,
    ) -> quest_hook::libil2cpp::Result<f32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (crate::UnityEngine::Vector3, crate::UnityEngine::Vector3, f32),
                        f32,
                        3usize,
                    >("CalculateSqrPerspectiveDistance")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "CalculateSqrPerspectiveDistance", 3usize
                        )
                    })
            });
        let __cordl_ret: f32 = unsafe {
            cordl_method_info
                .invoke_unchecked(
                    (),
                    (objPosition, camPosition, sqrScreenRelativeMetric),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetWorldReferencePoint(
        lodGroup: quest_hook::libil2cpp::Gc<crate::UnityEngine::LODGroup>,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector3> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (quest_hook::libil2cpp::Gc<crate::UnityEngine::LODGroup>),
                        crate::UnityEngine::Vector3,
                        1usize,
                    >("GetWorldReferencePoint")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "GetWorldReferencePoint", 1usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::Vector3 = unsafe {
            cordl_method_info.invoke_unchecked((), (lodGroup))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetWorldSpaceScale(
        lodGroup: quest_hook::libil2cpp::Gc<crate::UnityEngine::LODGroup>,
    ) -> quest_hook::libil2cpp::Result<f32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (quest_hook::libil2cpp::Gc<crate::UnityEngine::LODGroup>),
                        f32,
                        1usize,
                    >("GetWorldSpaceScale")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "GetWorldSpaceScale", 1usize
                        )
                    })
            });
        let __cordl_ret: f32 = unsafe {
            cordl_method_info.invoke_unchecked((), (lodGroup))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetWorldSpaceSize(
        lodGroup: quest_hook::libil2cpp::Gc<crate::UnityEngine::LODGroup>,
    ) -> quest_hook::libil2cpp::Result<f32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (quest_hook::libil2cpp::Gc<crate::UnityEngine::LODGroup>),
                        f32,
                        1usize,
                    >("GetWorldSpaceSize")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "GetWorldSpaceSize", 1usize
                        )
                    })
            });
        let __cordl_ret: f32 = unsafe {
            cordl_method_info.invoke_unchecked((), (lodGroup))?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "cordl_class_UnityEngine+Rendering+LODGroupRenderingUtils")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::Rendering::LODGroupRenderingUtils {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
