#[cfg(feature = "cordl_class_OVRExtensions")]
#[repr(C)]
#[derive(Debug)]
pub struct OVRExtensions {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "cordl_class_OVRExtensions")]
unsafe impl quest_hook::libil2cpp::Type for crate::GlobalNamespace::OVRExtensions {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "OVRExtensions";
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
#[cfg(feature = "OVRExtensions")]
impl std::ops::Deref for crate::GlobalNamespace::OVRExtensions {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVRExtensions")]
impl std::ops::DerefMut for crate::GlobalNamespace::OVRExtensions {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVRExtensions")]
impl crate::GlobalNamespace::OVRExtensions {
    pub fn ConvertToHMDMatrix34(
        m: crate::UnityEngine::Matrix4x4,
    ) -> quest_hook::libil2cpp::Result<crate::OVR::OpenVR::HmdMatrix34_t> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (crate::UnityEngine::Matrix4x4),
                        crate::OVR::OpenVR::HmdMatrix34_t,
                        1usize,
                    >("ConvertToHMDMatrix34")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "ConvertToHMDMatrix34", 1usize
                        )
                    })
            });
        let __cordl_ret: crate::OVR::OpenVR::HmdMatrix34_t = unsafe {
            cordl_method_info.invoke_unchecked((), (m))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn CopyFrom(
        gradient: quest_hook::libil2cpp::Gc<crate::UnityEngine::Gradient>,
        otherGradient: quest_hook::libil2cpp::Gc<crate::UnityEngine::Gradient>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<crate::UnityEngine::Gradient>,
                            quest_hook::libil2cpp::Gc<crate::UnityEngine::Gradient>,
                        ),
                        quest_hook::libil2cpp::Void,
                        2usize,
                    >("CopyFrom")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "CopyFrom", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked((), (gradient, otherGradient))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Equals(
        gradient: quest_hook::libil2cpp::Gc<crate::UnityEngine::Gradient>,
        otherGradient: quest_hook::libil2cpp::Gc<crate::UnityEngine::Gradient>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<crate::UnityEngine::Gradient>,
                            quest_hook::libil2cpp::Gc<crate::UnityEngine::Gradient>,
                        ),
                        bool,
                        2usize,
                    >("Equals")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "Equals",
                            2usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            cordl_method_info.invoke_unchecked((), (gradient, otherGradient))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn FindChildRecursive(
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::Transform>,
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::Transform>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<crate::UnityEngine::Transform>,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                        ),
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Transform>,
                        2usize,
                    >("FindChildRecursive")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "FindChildRecursive", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::Transform> = unsafe {
            cordl_method_info.invoke_unchecked((), (parent, name))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn FromColorf(
        c: crate::GlobalNamespace::OVRPlugin_Colorf,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Color> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (crate::GlobalNamespace::OVRPlugin_Colorf),
                        crate::UnityEngine::Color,
                        1usize,
                    >("FromColorf")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "FromColorf", 1usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::Color = unsafe {
            cordl_method_info.invoke_unchecked((), (c))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn FromFlippedXQuatf(
        q: crate::GlobalNamespace::OVRPlugin_Quatf,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Quaternion> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (crate::GlobalNamespace::OVRPlugin_Quatf),
                        crate::UnityEngine::Quaternion,
                        1usize,
                    >("FromFlippedXQuatf")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "FromFlippedXQuatf", 1usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::Quaternion = unsafe {
            cordl_method_info.invoke_unchecked((), (q))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn FromFlippedXVector2f(
        v: crate::GlobalNamespace::OVRPlugin_Vector2f,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector2> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (crate::GlobalNamespace::OVRPlugin_Vector2f),
                        crate::UnityEngine::Vector2,
                        1usize,
                    >("FromFlippedXVector2f")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "FromFlippedXVector2f", 1usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::Vector2 = unsafe {
            cordl_method_info.invoke_unchecked((), (v))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn FromFlippedXVector3f(
        v: crate::GlobalNamespace::OVRPlugin_Vector3f,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector3> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (crate::GlobalNamespace::OVRPlugin_Vector3f),
                        crate::UnityEngine::Vector3,
                        1usize,
                    >("FromFlippedXVector3f")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "FromFlippedXVector3f", 1usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::Vector3 = unsafe {
            cordl_method_info.invoke_unchecked((), (v))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn FromFlippedZQuatf(
        q: crate::GlobalNamespace::OVRPlugin_Quatf,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Quaternion> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (crate::GlobalNamespace::OVRPlugin_Quatf),
                        crate::UnityEngine::Quaternion,
                        1usize,
                    >("FromFlippedZQuatf")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "FromFlippedZQuatf", 1usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::Quaternion = unsafe {
            cordl_method_info.invoke_unchecked((), (q))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn FromFlippedZVector3f(
        v: crate::GlobalNamespace::OVRPlugin_Vector3f,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector3> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (crate::GlobalNamespace::OVRPlugin_Vector3f),
                        crate::UnityEngine::Vector3,
                        1usize,
                    >("FromFlippedZVector3f")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "FromFlippedZVector3f", 1usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::Vector3 = unsafe {
            cordl_method_info.invoke_unchecked((), (v))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn FromOVRPose(
        t: quest_hook::libil2cpp::Gc<crate::UnityEngine::Transform>,
        pose: crate::GlobalNamespace::OVRPose,
        isLocal: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<crate::UnityEngine::Transform>,
                            crate::GlobalNamespace::OVRPose,
                            bool,
                        ),
                        quest_hook::libil2cpp::Void,
                        3usize,
                    >("FromOVRPose")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "FromOVRPose", 3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked((), (t, pose, isLocal))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn FromQuatf(
        q: crate::GlobalNamespace::OVRPlugin_Quatf,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Quaternion> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (crate::GlobalNamespace::OVRPlugin_Quatf),
                        crate::UnityEngine::Quaternion,
                        1usize,
                    >("FromQuatf")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "FromQuatf", 1usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::Quaternion = unsafe {
            cordl_method_info.invoke_unchecked((), (q))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn FromSize3f(
        v: crate::GlobalNamespace::OVRPlugin_Size3f,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector3> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (crate::GlobalNamespace::OVRPlugin_Size3f),
                        crate::UnityEngine::Vector3,
                        1usize,
                    >("FromSize3f")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "FromSize3f", 1usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::Vector3 = unsafe {
            cordl_method_info.invoke_unchecked((), (v))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn FromSizef(
        v: crate::GlobalNamespace::OVRPlugin_Sizef,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector2> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (crate::GlobalNamespace::OVRPlugin_Sizef),
                        crate::UnityEngine::Vector2,
                        1usize,
                    >("FromSizef")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "FromSizef", 1usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::Vector2 = unsafe {
            cordl_method_info.invoke_unchecked((), (v))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn FromVector2f(
        v: crate::GlobalNamespace::OVRPlugin_Vector2f,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector2> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (crate::GlobalNamespace::OVRPlugin_Vector2f),
                        crate::UnityEngine::Vector2,
                        1usize,
                    >("FromVector2f")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "FromVector2f", 1usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::Vector2 = unsafe {
            cordl_method_info.invoke_unchecked((), (v))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn FromVector3f(
        v: crate::GlobalNamespace::OVRPlugin_Vector3f,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector3> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (crate::GlobalNamespace::OVRPlugin_Vector3f),
                        crate::UnityEngine::Vector3,
                        1usize,
                    >("FromVector3f")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "FromVector3f", 1usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::Vector3 = unsafe {
            cordl_method_info.invoke_unchecked((), (v))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn FromVector4f(
        v: crate::GlobalNamespace::OVRPlugin_Vector4f,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector4> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (crate::GlobalNamespace::OVRPlugin_Vector4f),
                        crate::UnityEngine::Vector4,
                        1usize,
                    >("FromVector4f")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "FromVector4f", 1usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::Vector4 = unsafe {
            cordl_method_info.invoke_unchecked((), (v))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ToColorf(
        c: crate::UnityEngine::Color,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::OVRPlugin_Colorf> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (crate::UnityEngine::Color),
                        crate::GlobalNamespace::OVRPlugin_Colorf,
                        1usize,
                    >("ToColorf")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "ToColorf", 1usize
                        )
                    })
            });
        let __cordl_ret: crate::GlobalNamespace::OVRPlugin_Colorf = unsafe {
            cordl_method_info.invoke_unchecked((), (c))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ToFlippedXQuatf(
        q: crate::UnityEngine::Quaternion,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::OVRPlugin_Quatf> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (crate::UnityEngine::Quaternion),
                        crate::GlobalNamespace::OVRPlugin_Quatf,
                        1usize,
                    >("ToFlippedXQuatf")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "ToFlippedXQuatf", 1usize
                        )
                    })
            });
        let __cordl_ret: crate::GlobalNamespace::OVRPlugin_Quatf = unsafe {
            cordl_method_info.invoke_unchecked((), (q))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ToFlippedXVector3f(
        v: crate::UnityEngine::Vector3,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::OVRPlugin_Vector3f> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (crate::UnityEngine::Vector3),
                        crate::GlobalNamespace::OVRPlugin_Vector3f,
                        1usize,
                    >("ToFlippedXVector3f")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "ToFlippedXVector3f", 1usize
                        )
                    })
            });
        let __cordl_ret: crate::GlobalNamespace::OVRPlugin_Vector3f = unsafe {
            cordl_method_info.invoke_unchecked((), (v))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ToFlippedZQuatf(
        q: crate::UnityEngine::Quaternion,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::OVRPlugin_Quatf> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (crate::UnityEngine::Quaternion),
                        crate::GlobalNamespace::OVRPlugin_Quatf,
                        1usize,
                    >("ToFlippedZQuatf")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "ToFlippedZQuatf", 1usize
                        )
                    })
            });
        let __cordl_ret: crate::GlobalNamespace::OVRPlugin_Quatf = unsafe {
            cordl_method_info.invoke_unchecked((), (q))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ToFlippedZVector3f(
        v: crate::UnityEngine::Vector3,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::OVRPlugin_Vector3f> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (crate::UnityEngine::Vector3),
                        crate::GlobalNamespace::OVRPlugin_Vector3f,
                        1usize,
                    >("ToFlippedZVector3f")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "ToFlippedZVector3f", 1usize
                        )
                    })
            });
        let __cordl_ret: crate::GlobalNamespace::OVRPlugin_Vector3f = unsafe {
            cordl_method_info.invoke_unchecked((), (v))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ToFrustum(
        f: crate::GlobalNamespace::OVRPlugin_Frustumf,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::OVRTracker_Frustum> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (crate::GlobalNamespace::OVRPlugin_Frustumf),
                        crate::GlobalNamespace::OVRTracker_Frustum,
                        1usize,
                    >("ToFrustum")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "ToFrustum", 1usize
                        )
                    })
            });
        let __cordl_ret: crate::GlobalNamespace::OVRTracker_Frustum = unsafe {
            cordl_method_info.invoke_unchecked((), (f))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ToHeadSpacePose_OVRPose0(
        trackingSpacePose: crate::GlobalNamespace::OVRPose,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::OVRPose> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (crate::GlobalNamespace::OVRPose),
                        crate::GlobalNamespace::OVRPose,
                        1usize,
                    >("ToHeadSpacePose")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "ToHeadSpacePose", 1usize
                        )
                    })
            });
        let __cordl_ret: crate::GlobalNamespace::OVRPose = unsafe {
            cordl_method_info.invoke_unchecked((), (trackingSpacePose))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ToHeadSpacePose_Transform_Camera1(
        transform: quest_hook::libil2cpp::Gc<crate::UnityEngine::Transform>,
        camera: quest_hook::libil2cpp::Gc<crate::UnityEngine::Camera>,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::OVRPose> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<crate::UnityEngine::Transform>,
                            quest_hook::libil2cpp::Gc<crate::UnityEngine::Camera>,
                        ),
                        crate::GlobalNamespace::OVRPose,
                        2usize,
                    >("ToHeadSpacePose")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "ToHeadSpacePose", 2usize
                        )
                    })
            });
        let __cordl_ret: crate::GlobalNamespace::OVRPose = unsafe {
            cordl_method_info.invoke_unchecked((), (transform, camera))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ToNativeArray<T>(
        enumerable: quest_hook::libil2cpp::Gc<T>,
        allocator: crate::Unity::Collections::Allocator,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Collections::NativeArray_1<T>>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<T>,
                            crate::Unity::Collections::Allocator,
                        ),
                        crate::Unity::Collections::NativeArray_1<T>,
                        2usize,
                    >("ToNativeArray")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "ToNativeArray", 2usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Collections::NativeArray_1<T> = unsafe {
            cordl_method_info.invoke_unchecked((), (enumerable, allocator))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ToNonAlloc<T>(
        enumerable: quest_hook::libil2cpp::Gc<T>,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::OVREnumerable_1<T>>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (quest_hook::libil2cpp::Gc<T>),
                        crate::GlobalNamespace::OVREnumerable_1<T>,
                        1usize,
                    >("ToNonAlloc")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "ToNonAlloc", 1usize
                        )
                    })
            });
        let __cordl_ret: crate::GlobalNamespace::OVREnumerable_1<T> = unsafe {
            cordl_method_info.invoke_unchecked((), (enumerable))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ToOVRPose_OVRPlugin_Posef1(
        p: crate::GlobalNamespace::OVRPlugin_Posef,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::OVRPose> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (crate::GlobalNamespace::OVRPlugin_Posef),
                        crate::GlobalNamespace::OVRPose,
                        1usize,
                    >("ToOVRPose")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "ToOVRPose", 1usize
                        )
                    })
            });
        let __cordl_ret: crate::GlobalNamespace::OVRPose = unsafe {
            cordl_method_info.invoke_unchecked((), (p))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ToOVRPose_Transform__cordl_bool0(
        t: quest_hook::libil2cpp::Gc<crate::UnityEngine::Transform>,
        isLocal: bool,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::OVRPose> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (quest_hook::libil2cpp::Gc<crate::UnityEngine::Transform>, bool),
                        crate::GlobalNamespace::OVRPose,
                        2usize,
                    >("ToOVRPose")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "ToOVRPose", 2usize
                        )
                    })
            });
        let __cordl_ret: crate::GlobalNamespace::OVRPose = unsafe {
            cordl_method_info.invoke_unchecked((), (t, isLocal))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ToQuatf(
        q: crate::UnityEngine::Quaternion,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::OVRPlugin_Quatf> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (crate::UnityEngine::Quaternion),
                        crate::GlobalNamespace::OVRPlugin_Quatf,
                        1usize,
                    >("ToQuatf")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "ToQuatf",
                            1usize
                        )
                    })
            });
        let __cordl_ret: crate::GlobalNamespace::OVRPlugin_Quatf = unsafe {
            cordl_method_info.invoke_unchecked((), (q))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ToSize3f(
        v: crate::UnityEngine::Vector3,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::OVRPlugin_Size3f> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (crate::UnityEngine::Vector3),
                        crate::GlobalNamespace::OVRPlugin_Size3f,
                        1usize,
                    >("ToSize3f")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "ToSize3f", 1usize
                        )
                    })
            });
        let __cordl_ret: crate::GlobalNamespace::OVRPlugin_Size3f = unsafe {
            cordl_method_info.invoke_unchecked((), (v))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ToSizef(
        v: crate::UnityEngine::Vector2,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::OVRPlugin_Sizef> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (crate::UnityEngine::Vector2),
                        crate::GlobalNamespace::OVRPlugin_Sizef,
                        1usize,
                    >("ToSizef")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "ToSizef",
                            1usize
                        )
                    })
            });
        let __cordl_ret: crate::GlobalNamespace::OVRPlugin_Sizef = unsafe {
            cordl_method_info.invoke_unchecked((), (v))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ToSpaceStorageLocation(
        storageLocation: crate::GlobalNamespace::OVRSpace_StorageLocation,
    ) -> quest_hook::libil2cpp::Result<
        crate::GlobalNamespace::OVRPlugin_SpaceStorageLocation,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (crate::GlobalNamespace::OVRSpace_StorageLocation),
                        crate::GlobalNamespace::OVRPlugin_SpaceStorageLocation,
                        1usize,
                    >("ToSpaceStorageLocation")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "ToSpaceStorageLocation", 1usize
                        )
                    })
            });
        let __cordl_ret: crate::GlobalNamespace::OVRPlugin_SpaceStorageLocation = unsafe {
            cordl_method_info.invoke_unchecked((), (storageLocation))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ToTrackingSpacePose(
        transform: quest_hook::libil2cpp::Gc<crate::UnityEngine::Transform>,
        camera: quest_hook::libil2cpp::Gc<crate::UnityEngine::Camera>,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::OVRPose> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<crate::UnityEngine::Transform>,
                            quest_hook::libil2cpp::Gc<crate::UnityEngine::Camera>,
                        ),
                        crate::GlobalNamespace::OVRPose,
                        2usize,
                    >("ToTrackingSpacePose")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "ToTrackingSpacePose", 2usize
                        )
                    })
            });
        let __cordl_ret: crate::GlobalNamespace::OVRPose = unsafe {
            cordl_method_info.invoke_unchecked((), (transform, camera))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ToVector2f(
        v: crate::UnityEngine::Vector2,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::OVRPlugin_Vector2f> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (crate::UnityEngine::Vector2),
                        crate::GlobalNamespace::OVRPlugin_Vector2f,
                        1usize,
                    >("ToVector2f")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "ToVector2f", 1usize
                        )
                    })
            });
        let __cordl_ret: crate::GlobalNamespace::OVRPlugin_Vector2f = unsafe {
            cordl_method_info.invoke_unchecked((), (v))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ToVector3f(
        v: crate::UnityEngine::Vector3,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::OVRPlugin_Vector3f> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (crate::UnityEngine::Vector3),
                        crate::GlobalNamespace::OVRPlugin_Vector3f,
                        1usize,
                    >("ToVector3f")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "ToVector3f", 1usize
                        )
                    })
            });
        let __cordl_ret: crate::GlobalNamespace::OVRPlugin_Vector3f = unsafe {
            cordl_method_info.invoke_unchecked((), (v))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ToVector4f(
        v: crate::UnityEngine::Vector4,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::OVRPlugin_Vector4f> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (crate::UnityEngine::Vector4),
                        crate::GlobalNamespace::OVRPlugin_Vector4f,
                        1usize,
                    >("ToVector4f")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "ToVector4f", 1usize
                        )
                    })
            });
        let __cordl_ret: crate::GlobalNamespace::OVRPlugin_Vector4f = unsafe {
            cordl_method_info.invoke_unchecked((), (v))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ToWorldSpacePose_Camera1(
        trackingSpacePose: crate::GlobalNamespace::OVRPose,
        mainCamera: quest_hook::libil2cpp::Gc<crate::UnityEngine::Camera>,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::OVRPose> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            crate::GlobalNamespace::OVRPose,
                            quest_hook::libil2cpp::Gc<crate::UnityEngine::Camera>,
                        ),
                        crate::GlobalNamespace::OVRPose,
                        2usize,
                    >("ToWorldSpacePose")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "ToWorldSpacePose", 2usize
                        )
                    })
            });
        let __cordl_ret: crate::GlobalNamespace::OVRPose = unsafe {
            cordl_method_info.invoke_unchecked((), (trackingSpacePose, mainCamera))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ToWorldSpacePose_OVRPose0(
        trackingSpacePose: crate::GlobalNamespace::OVRPose,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::OVRPose> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (crate::GlobalNamespace::OVRPose),
                        crate::GlobalNamespace::OVRPose,
                        1usize,
                    >("ToWorldSpacePose")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "ToWorldSpacePose", 1usize
                        )
                    })
            });
        let __cordl_ret: crate::GlobalNamespace::OVRPose = unsafe {
            cordl_method_info.invoke_unchecked((), (trackingSpacePose))?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "cordl_class_OVRExtensions")]
impl quest_hook::libil2cpp::ObjectType for crate::GlobalNamespace::OVRExtensions {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
