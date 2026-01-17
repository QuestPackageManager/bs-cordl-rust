#[cfg(feature = "cordl_class_OVREnumExtensions")]
#[repr(C)]
#[cfg_attr(feature = "derive_Debug", derive(Debug))]
pub struct OVREnumExtensions {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "cordl_class_OVREnumExtensions")]
unsafe impl quest_hook::libil2cpp::Type for crate::GlobalNamespace::OVREnumExtensions {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "OVREnumExtensions";
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
#[cfg(feature = "OVREnumExtensions")]
impl std::ops::Deref for crate::GlobalNamespace::OVREnumExtensions {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVREnumExtensions")]
impl std::ops::DerefMut for crate::GlobalNamespace::OVREnumExtensions {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVREnumExtensions")]
impl crate::GlobalNamespace::OVREnumExtensions {
    pub fn AsHandType_OVRMesh_MeshType1(
        meshType: crate::GlobalNamespace::OVRMesh_MeshType,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::OVRHand_Hand> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (crate::GlobalNamespace::OVRMesh_MeshType),
                        crate::GlobalNamespace::OVRHand_Hand,
                        1usize,
                    >("AsHandType")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "AsHandType", 1usize
                        )
                    })
            });
        let __cordl_ret: crate::GlobalNamespace::OVRHand_Hand =
            unsafe { cordl_method_info.invoke_unchecked((), (meshType))? };
        Ok(__cordl_ret.into())
    }
    pub fn AsHandType_OVRSkeleton_SkeletonType0(
        skeletonType: crate::GlobalNamespace::OVRSkeleton_SkeletonType,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::OVRHand_Hand> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (crate::GlobalNamespace::OVRSkeleton_SkeletonType),
                        crate::GlobalNamespace::OVRHand_Hand,
                        1usize,
                    >("AsHandType")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "AsHandType", 1usize
                        )
                    })
            });
        let __cordl_ret: crate::GlobalNamespace::OVRHand_Hand =
            unsafe { cordl_method_info.invoke_unchecked((), (skeletonType))? };
        Ok(__cordl_ret.into())
    }
    pub fn AsMeshType_OVRHandSkeletonVersion1(
        hand: crate::GlobalNamespace::OVRHand_Hand,
        version: crate::GlobalNamespace::OVRHandSkeletonVersion,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::OVRMesh_MeshType> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::GlobalNamespace::OVRHand_Hand,
                        crate::GlobalNamespace::OVRHandSkeletonVersion,
                    ), crate::GlobalNamespace::OVRMesh_MeshType, 2usize>(
                        "AsMeshType"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "AsMeshType",
                            2usize
                        )
                    })
            });
        let __cordl_ret: crate::GlobalNamespace::OVRMesh_MeshType =
            unsafe { cordl_method_info.invoke_unchecked((), (hand, version))? };
        Ok(__cordl_ret.into())
    }
    pub fn AsMeshType_OVRHand_Hand0(
        hand: crate::GlobalNamespace::OVRHand_Hand,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::OVRMesh_MeshType> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (crate::GlobalNamespace::OVRHand_Hand),
                        crate::GlobalNamespace::OVRMesh_MeshType,
                        1usize,
                    >("AsMeshType")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "AsMeshType", 1usize
                        )
                    })
            });
        let __cordl_ret: crate::GlobalNamespace::OVRMesh_MeshType =
            unsafe { cordl_method_info.invoke_unchecked((), (hand))? };
        Ok(__cordl_ret.into())
    }
    pub fn AsSkeletonType_OVRHandSkeletonVersion1(
        hand: crate::GlobalNamespace::OVRHand_Hand,
        version: crate::GlobalNamespace::OVRHandSkeletonVersion,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::OVRSkeleton_SkeletonType> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::GlobalNamespace::OVRHand_Hand,
                        crate::GlobalNamespace::OVRHandSkeletonVersion,
                    ), crate::GlobalNamespace::OVRSkeleton_SkeletonType, 2usize>(
                        "AsSkeletonType"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "AsSkeletonType",
                            2usize
                        )
                    })
            });
        let __cordl_ret: crate::GlobalNamespace::OVRSkeleton_SkeletonType =
            unsafe { cordl_method_info.invoke_unchecked((), (hand, version))? };
        Ok(__cordl_ret.into())
    }
    pub fn AsSkeletonType_OVRHand_Hand0(
        hand: crate::GlobalNamespace::OVRHand_Hand,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::OVRSkeleton_SkeletonType> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (crate::GlobalNamespace::OVRHand_Hand),
                        crate::GlobalNamespace::OVRSkeleton_SkeletonType,
                        1usize,
                    >("AsSkeletonType")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "AsSkeletonType", 1usize
                        )
                    })
            });
        let __cordl_ret: crate::GlobalNamespace::OVRSkeleton_SkeletonType =
            unsafe { cordl_method_info.invoke_unchecked((), (hand))? };
        Ok(__cordl_ret.into())
    }
    pub fn IsHand_OVRMesh_MeshType1(
        meshType: crate::GlobalNamespace::OVRMesh_MeshType,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(crate::GlobalNamespace::OVRMesh_MeshType), bool, 1usize>(
                        "IsHand",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "IsHand",
                            1usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe { cordl_method_info.invoke_unchecked((), (meshType))? };
        Ok(__cordl_ret.into())
    }
    pub fn IsHand_OVRSkeleton_SkeletonType0(
        skeletonType: crate::GlobalNamespace::OVRSkeleton_SkeletonType,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (crate::GlobalNamespace::OVRSkeleton_SkeletonType),
                        bool,
                        1usize,
                    >("IsHand")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "IsHand",
                            1usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe { cordl_method_info.invoke_unchecked((), (skeletonType))? };
        Ok(__cordl_ret.into())
    }
    pub fn IsLeft_OVRMesh_MeshType1(
        _cordl_type: crate::GlobalNamespace::OVRMesh_MeshType,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(crate::GlobalNamespace::OVRMesh_MeshType), bool, 1usize>(
                        "IsLeft",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "IsLeft",
                            1usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe { cordl_method_info.invoke_unchecked((), (_cordl_type))? };
        Ok(__cordl_ret.into())
    }
    pub fn IsLeft_OVRSkeleton_SkeletonType0(
        _cordl_type: crate::GlobalNamespace::OVRSkeleton_SkeletonType,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (crate::GlobalNamespace::OVRSkeleton_SkeletonType),
                        bool,
                        1usize,
                    >("IsLeft")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "IsLeft",
                            1usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe { cordl_method_info.invoke_unchecked((), (_cordl_type))? };
        Ok(__cordl_ret.into())
    }
    pub fn IsOVRHandMesh(
        meshType: crate::GlobalNamespace::OVRMesh_MeshType,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(crate::GlobalNamespace::OVRMesh_MeshType), bool, 1usize>(
                        "IsOVRHandMesh",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "IsOVRHandMesh",
                            1usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe { cordl_method_info.invoke_unchecked((), (meshType))? };
        Ok(__cordl_ret.into())
    }
    pub fn IsOVRHandSkeleton(
        skeletonType: crate::GlobalNamespace::OVRSkeleton_SkeletonType,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (crate::GlobalNamespace::OVRSkeleton_SkeletonType),
                        bool,
                        1usize,
                    >("IsOVRHandSkeleton")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "IsOVRHandSkeleton", 1usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe { cordl_method_info.invoke_unchecked((), (skeletonType))? };
        Ok(__cordl_ret.into())
    }
    pub fn IsOpenXRHandMesh(
        meshType: crate::GlobalNamespace::OVRMesh_MeshType,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(crate::GlobalNamespace::OVRMesh_MeshType), bool, 1usize>(
                        "IsOpenXRHandMesh",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "IsOpenXRHandMesh",
                            1usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe { cordl_method_info.invoke_unchecked((), (meshType))? };
        Ok(__cordl_ret.into())
    }
    pub fn IsOpenXRHandSkeleton(
        skeletonType: crate::GlobalNamespace::OVRSkeleton_SkeletonType,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (crate::GlobalNamespace::OVRSkeleton_SkeletonType),
                        bool,
                        1usize,
                    >("IsOpenXRHandSkeleton")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "IsOpenXRHandSkeleton", 1usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe { cordl_method_info.invoke_unchecked((), (skeletonType))? };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "cordl_class_OVREnumExtensions")]
impl quest_hook::libil2cpp::ObjectType for crate::GlobalNamespace::OVREnumExtensions {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
