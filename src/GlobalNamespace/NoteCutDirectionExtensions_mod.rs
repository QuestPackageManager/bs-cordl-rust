#[cfg(feature = "NoteCutDirectionExtensions")]
#[repr(C)]
#[derive(Debug)]
pub struct NoteCutDirectionExtensions {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "NoteCutDirectionExtensions")]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::NoteCutDirectionExtensions {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "NoteCutDirectionExtensions";
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
#[cfg(feature = "NoteCutDirectionExtensions")]
impl std::ops::Deref for crate::GlobalNamespace::NoteCutDirectionExtensions {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "NoteCutDirectionExtensions")]
impl std::ops::DerefMut for crate::GlobalNamespace::NoteCutDirectionExtensions {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "NoteCutDirectionExtensions")]
impl crate::GlobalNamespace::NoteCutDirectionExtensions {
    pub fn Direction(
        cutDirection: crate::GlobalNamespace::NoteCutDirection,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector2> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (crate::GlobalNamespace::NoteCutDirection),
                        crate::UnityEngine::Vector2,
                        1usize,
                    >("Direction")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "Direction", 1usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::Vector2 = unsafe {
            cordl_method_info.invoke_unchecked((), (cutDirection))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn IsMainDirection(
        cutDirection: crate::GlobalNamespace::NoteCutDirection,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (crate::GlobalNamespace::NoteCutDirection),
                        bool,
                        1usize,
                    >("IsMainDirection")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "IsMainDirection", 1usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            cordl_method_info.invoke_unchecked((), (cutDirection))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn IsOnSamePlane(
        noteCutDirection1: crate::GlobalNamespace::NoteCutDirection,
        noteCutDirection2: crate::GlobalNamespace::NoteCutDirection,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            crate::GlobalNamespace::NoteCutDirection,
                            crate::GlobalNamespace::NoteCutDirection,
                        ),
                        bool,
                        2usize,
                    >("IsOnSamePlane")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "IsOnSamePlane", 2usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            cordl_method_info
                .invoke_unchecked((), (noteCutDirection1, noteCutDirection2))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn MainNoteCutDirectionFromCutDirAngle(
        angle: f32,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::NoteCutDirection> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (f32),
                        crate::GlobalNamespace::NoteCutDirection,
                        1usize,
                    >("MainNoteCutDirectionFromCutDirAngle")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "MainNoteCutDirectionFromCutDirAngle", 1usize
                        )
                    })
            });
        let __cordl_ret: crate::GlobalNamespace::NoteCutDirection = unsafe {
            cordl_method_info.invoke_unchecked((), (angle))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Mirrored(
        cutDirection: crate::GlobalNamespace::NoteCutDirection,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::NoteCutDirection> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (crate::GlobalNamespace::NoteCutDirection),
                        crate::GlobalNamespace::NoteCutDirection,
                        1usize,
                    >("Mirrored")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "Mirrored", 1usize
                        )
                    })
            });
        let __cordl_ret: crate::GlobalNamespace::NoteCutDirection = unsafe {
            cordl_method_info.invoke_unchecked((), (cutDirection))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn NoteCutDirectionFromDirection(
        direction: crate::UnityEngine::Vector3,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::NoteCutDirection> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (crate::UnityEngine::Vector3),
                        crate::GlobalNamespace::NoteCutDirection,
                        1usize,
                    >("NoteCutDirectionFromDirection")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "NoteCutDirectionFromDirection", 1usize
                        )
                    })
            });
        let __cordl_ret: crate::GlobalNamespace::NoteCutDirection = unsafe {
            cordl_method_info.invoke_unchecked((), (direction))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Opposite(
        cutDirection: crate::GlobalNamespace::NoteCutDirection,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::NoteCutDirection> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (crate::GlobalNamespace::NoteCutDirection),
                        crate::GlobalNamespace::NoteCutDirection,
                        1usize,
                    >("Opposite")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "Opposite", 1usize
                        )
                    })
            });
        let __cordl_ret: crate::GlobalNamespace::NoteCutDirection = unsafe {
            cordl_method_info.invoke_unchecked((), (cutDirection))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Rotation(
        cutDirection: crate::GlobalNamespace::NoteCutDirection,
        offset: f32,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Quaternion> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (crate::GlobalNamespace::NoteCutDirection, f32),
                        crate::UnityEngine::Quaternion,
                        2usize,
                    >("Rotation")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "Rotation", 2usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::Quaternion = unsafe {
            cordl_method_info.invoke_unchecked((), (cutDirection, offset))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn RotationAngle(
        cutDirection: crate::GlobalNamespace::NoteCutDirection,
    ) -> quest_hook::libil2cpp::Result<f32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (crate::GlobalNamespace::NoteCutDirection),
                        f32,
                        1usize,
                    >("RotationAngle")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "RotationAngle", 1usize
                        )
                    })
            });
        let __cordl_ret: f32 = unsafe {
            cordl_method_info.invoke_unchecked((), (cutDirection))?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "NoteCutDirectionExtensions")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::NoteCutDirectionExtensions {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
