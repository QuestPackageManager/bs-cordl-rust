#[cfg(feature = "cordl_class_UnityEngine+Rendering+CoreMatrixUtils")]
#[repr(C)]
#[derive(Debug)]
pub struct CoreMatrixUtils {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "cordl_class_UnityEngine+Rendering+CoreMatrixUtils")]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::Rendering::CoreMatrixUtils {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine.Rendering";
    const CLASS_NAME: &'static str = "CoreMatrixUtils";
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
#[cfg(feature = "UnityEngine+Rendering+CoreMatrixUtils")]
impl std::ops::Deref for crate::UnityEngine::Rendering::CoreMatrixUtils {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Rendering+CoreMatrixUtils")]
impl std::ops::DerefMut for crate::UnityEngine::Rendering::CoreMatrixUtils {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Rendering+CoreMatrixUtils")]
impl crate::UnityEngine::Rendering::CoreMatrixUtils {
    pub fn MatrixTimesTranslation(
        inOutMatrix: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Matrix4x4>,
        translation: crate::UnityEngine::Vector3,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::ByRefMut<
                                crate::UnityEngine::Matrix4x4,
                            >,
                            crate::UnityEngine::Vector3,
                        ),
                        quest_hook::libil2cpp::Void,
                        2usize,
                    >("MatrixTimesTranslation")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "MatrixTimesTranslation", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked((), (inOutMatrix, translation))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn MultiplyGenericOrthoMatrix(
        ortho: crate::UnityEngine::Matrix4x4,
        rhs: crate::UnityEngine::Matrix4x4,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Matrix4x4> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (crate::UnityEngine::Matrix4x4, crate::UnityEngine::Matrix4x4),
                        crate::UnityEngine::Matrix4x4,
                        2usize,
                    >("MultiplyGenericOrthoMatrix")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "MultiplyGenericOrthoMatrix", 2usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::Matrix4x4 = unsafe {
            cordl_method_info.invoke_unchecked((), (ortho, rhs))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn MultiplyOrthoMatrix(
        ortho: crate::UnityEngine::Matrix4x4,
        rhs: crate::UnityEngine::Matrix4x4,
        centered: bool,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Matrix4x4> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            crate::UnityEngine::Matrix4x4,
                            crate::UnityEngine::Matrix4x4,
                            bool,
                        ),
                        crate::UnityEngine::Matrix4x4,
                        3usize,
                    >("MultiplyOrthoMatrix")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "MultiplyOrthoMatrix", 3usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::Matrix4x4 = unsafe {
            cordl_method_info.invoke_unchecked((), (ortho, rhs, centered))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn MultiplyOrthoMatrixCentered(
        ortho: crate::UnityEngine::Matrix4x4,
        rhs: crate::UnityEngine::Matrix4x4,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Matrix4x4> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (crate::UnityEngine::Matrix4x4, crate::UnityEngine::Matrix4x4),
                        crate::UnityEngine::Matrix4x4,
                        2usize,
                    >("MultiplyOrthoMatrixCentered")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "MultiplyOrthoMatrixCentered", 2usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::Matrix4x4 = unsafe {
            cordl_method_info.invoke_unchecked((), (ortho, rhs))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn MultiplyPerspectiveMatrix(
        perspective: crate::UnityEngine::Matrix4x4,
        rhs: crate::UnityEngine::Matrix4x4,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Matrix4x4> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (crate::UnityEngine::Matrix4x4, crate::UnityEngine::Matrix4x4),
                        crate::UnityEngine::Matrix4x4,
                        2usize,
                    >("MultiplyPerspectiveMatrix")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "MultiplyPerspectiveMatrix", 2usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::Matrix4x4 = unsafe {
            cordl_method_info.invoke_unchecked((), (perspective, rhs))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn MultiplyProjectionMatrix(
        projMatrix: crate::UnityEngine::Matrix4x4,
        rhs: crate::UnityEngine::Matrix4x4,
        orthoCentered: bool,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Matrix4x4> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            crate::UnityEngine::Matrix4x4,
                            crate::UnityEngine::Matrix4x4,
                            bool,
                        ),
                        crate::UnityEngine::Matrix4x4,
                        3usize,
                    >("MultiplyProjectionMatrix")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "MultiplyProjectionMatrix", 3usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::Matrix4x4 = unsafe {
            cordl_method_info.invoke_unchecked((), (projMatrix, rhs, orthoCentered))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn TranslationTimesMatrix(
        inOutMatrix: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Matrix4x4>,
        translation: crate::UnityEngine::Vector3,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::ByRefMut<
                                crate::UnityEngine::Matrix4x4,
                            >,
                            crate::UnityEngine::Vector3,
                        ),
                        quest_hook::libil2cpp::Void,
                        2usize,
                    >("TranslationTimesMatrix")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "TranslationTimesMatrix", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked((), (inOutMatrix, translation))?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "cordl_class_UnityEngine+Rendering+CoreMatrixUtils")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::Rendering::CoreMatrixUtils {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
