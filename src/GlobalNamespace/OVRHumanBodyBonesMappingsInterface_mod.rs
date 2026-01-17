#[cfg(feature = "cordl_class_OVRHumanBodyBonesMappingsInterface")]
#[cfg_attr(feature = "derive_Debug", derive(Debug))]
#[repr(C)]
pub struct OVRHumanBodyBonesMappingsInterface {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "cordl_class_OVRHumanBodyBonesMappingsInterface")]
unsafe impl quest_hook::libil2cpp::Type
    for crate::GlobalNamespace::OVRHumanBodyBonesMappingsInterface
{
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "OVRHumanBodyBonesMappingsInterface";
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
#[cfg(feature = "OVRHumanBodyBonesMappingsInterface")]
impl std::ops::Deref for crate::GlobalNamespace::OVRHumanBodyBonesMappingsInterface {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVRHumanBodyBonesMappingsInterface")]
impl std::ops::DerefMut for crate::GlobalNamespace::OVRHumanBodyBonesMappingsInterface {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVRHumanBodyBonesMappingsInterface")]
impl crate::GlobalNamespace::OVRHumanBodyBonesMappingsInterface {
    pub fn from_object_mut(object_param: *mut quest_hook::libil2cpp::Il2CppObject) -> *mut Self {
        unsafe { (object_param as *mut Self) }
    }
    pub fn get_GetBoneIdToHumanBodyBone(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::Dictionary_2<
                crate::GlobalNamespace::OVRSkeleton_BoneId,
                crate::UnityEngine::HumanBodyBones,
            >,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), quest_hook::libil2cpp::Gc<
                        crate::System::Collections::Generic::Dictionary_2<
                            crate::GlobalNamespace::OVRSkeleton_BoneId,
                            crate::UnityEngine::HumanBodyBones,
                        >,
                    >, 0usize>("get_GetBoneIdToHumanBodyBone")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "get_GetBoneIdToHumanBodyBone",
                            0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::Dictionary_2<
                crate::GlobalNamespace::OVRSkeleton_BoneId,
                crate::UnityEngine::HumanBodyBones,
            >,
        > = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_GetBoneIdToJointPair(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::Dictionary_2<
                crate::GlobalNamespace::OVRSkeleton_BoneId,
                quest_hook::libil2cpp::Gc<
                    crate::System::Tuple_2<
                        crate::GlobalNamespace::OVRSkeleton_BoneId,
                        crate::GlobalNamespace::OVRSkeleton_BoneId,
                    >,
                >,
            >,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), quest_hook::libil2cpp::Gc<
                        crate::System::Collections::Generic::Dictionary_2<
                            crate::GlobalNamespace::OVRSkeleton_BoneId,
                            quest_hook::libil2cpp::Gc<
                                crate::System::Tuple_2<
                                    crate::GlobalNamespace::OVRSkeleton_BoneId,
                                    crate::GlobalNamespace::OVRSkeleton_BoneId,
                                >,
                            >,
                        >,
                    >, 0usize>("get_GetBoneIdToJointPair")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "get_GetBoneIdToJointPair",
                            0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::Dictionary_2<
                crate::GlobalNamespace::OVRSkeleton_BoneId,
                quest_hook::libil2cpp::Gc<
                    crate::System::Tuple_2<
                        crate::GlobalNamespace::OVRSkeleton_BoneId,
                        crate::GlobalNamespace::OVRSkeleton_BoneId,
                    >,
                >,
            >,
        > = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_GetBoneToBodySection(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::Dictionary_2<
                crate::UnityEngine::HumanBodyBones,
                crate::GlobalNamespace::OVRHumanBodyBonesMappings_OVRUnityHumanoidSkeletonRetargeter_BodySection,
            >,
        >,
    >{
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        quest_hook::libil2cpp::Gc<
                            crate::System::Collections::Generic::Dictionary_2<
                                crate::UnityEngine::HumanBodyBones,
                                crate::GlobalNamespace::OVRHumanBodyBonesMappings_OVRUnityHumanoidSkeletonRetargeter_BodySection,
                            >,
                        >,
                        0usize,
                    >("get_GetBoneToBodySection")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "get_GetBoneToBodySection", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::Dictionary_2<
                crate::UnityEngine::HumanBodyBones,
                crate::GlobalNamespace::OVRHumanBodyBonesMappings_OVRUnityHumanoidSkeletonRetargeter_BodySection,
            >,
        > = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_GetBoneToJointPair(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::Dictionary_2<
                crate::UnityEngine::HumanBodyBones,
                quest_hook::libil2cpp::Gc<
                    crate::System::Tuple_2<
                        crate::UnityEngine::HumanBodyBones,
                        crate::UnityEngine::HumanBodyBones,
                    >,
                >,
            >,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), quest_hook::libil2cpp::Gc<
                        crate::System::Collections::Generic::Dictionary_2<
                            crate::UnityEngine::HumanBodyBones,
                            quest_hook::libil2cpp::Gc<
                                crate::System::Tuple_2<
                                    crate::UnityEngine::HumanBodyBones,
                                    crate::UnityEngine::HumanBodyBones,
                                >,
                            >,
                        >,
                    >, 0usize>("get_GetBoneToJointPair")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "get_GetBoneToJointPair",
                            0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::Dictionary_2<
                crate::UnityEngine::HumanBodyBones,
                quest_hook::libil2cpp::Gc<
                    crate::System::Tuple_2<
                        crate::UnityEngine::HumanBodyBones,
                        crate::UnityEngine::HumanBodyBones,
                    >,
                >,
            >,
        > = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_GetFullBodyBoneIdToHumanBodyBone(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::Dictionary_2<
                crate::GlobalNamespace::OVRSkeleton_BoneId,
                crate::UnityEngine::HumanBodyBones,
            >,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), quest_hook::libil2cpp::Gc<
                        crate::System::Collections::Generic::Dictionary_2<
                            crate::GlobalNamespace::OVRSkeleton_BoneId,
                            crate::UnityEngine::HumanBodyBones,
                        >,
                    >, 0usize>("get_GetFullBodyBoneIdToHumanBodyBone")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "get_GetFullBodyBoneIdToHumanBodyBone",
                            0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::Dictionary_2<
                crate::GlobalNamespace::OVRSkeleton_BoneId,
                crate::UnityEngine::HumanBodyBones,
            >,
        > = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_GetFullBodyBoneIdToJointPair(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::Dictionary_2<
                crate::GlobalNamespace::OVRSkeleton_BoneId,
                quest_hook::libil2cpp::Gc<
                    crate::System::Tuple_2<
                        crate::GlobalNamespace::OVRSkeleton_BoneId,
                        crate::GlobalNamespace::OVRSkeleton_BoneId,
                    >,
                >,
            >,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), quest_hook::libil2cpp::Gc<
                        crate::System::Collections::Generic::Dictionary_2<
                            crate::GlobalNamespace::OVRSkeleton_BoneId,
                            quest_hook::libil2cpp::Gc<
                                crate::System::Tuple_2<
                                    crate::GlobalNamespace::OVRSkeleton_BoneId,
                                    crate::GlobalNamespace::OVRSkeleton_BoneId,
                                >,
                            >,
                        >,
                    >, 0usize>("get_GetFullBodyBoneIdToJointPair")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "get_GetFullBodyBoneIdToJointPair",
                            0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::Dictionary_2<
                crate::GlobalNamespace::OVRSkeleton_BoneId,
                quest_hook::libil2cpp::Gc<
                    crate::System::Tuple_2<
                        crate::GlobalNamespace::OVRSkeleton_BoneId,
                        crate::GlobalNamespace::OVRSkeleton_BoneId,
                    >,
                >,
            >,
        > = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "cordl_class_OVRHumanBodyBonesMappingsInterface")]
impl quest_hook::libil2cpp::ObjectType
    for crate::GlobalNamespace::OVRHumanBodyBonesMappingsInterface
{
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
