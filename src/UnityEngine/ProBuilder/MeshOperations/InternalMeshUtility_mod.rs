#[cfg(feature = "cordl_class_UnityEngine+ProBuilder+MeshOperations+InternalMeshUtility")]
#[repr(C)]
#[derive(Debug)]
pub struct InternalMeshUtility {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "cordl_class_UnityEngine+ProBuilder+MeshOperations+InternalMeshUtility")]
unsafe impl quest_hook::libil2cpp::Type
    for crate::UnityEngine::ProBuilder::MeshOperations::InternalMeshUtility
{
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine.ProBuilder.MeshOperations";
    const CLASS_NAME: &'static str = "InternalMeshUtility";
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
#[cfg(feature = "UnityEngine+ProBuilder+MeshOperations+InternalMeshUtility")]
impl std::ops::Deref for crate::UnityEngine::ProBuilder::MeshOperations::InternalMeshUtility {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+ProBuilder+MeshOperations+InternalMeshUtility")]
impl std::ops::DerefMut for crate::UnityEngine::ProBuilder::MeshOperations::InternalMeshUtility {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+ProBuilder+MeshOperations+InternalMeshUtility")]
impl crate::UnityEngine::ProBuilder::MeshOperations::InternalMeshUtility {
    pub fn AverageNormalWithIndexes(
        shared: quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::SharedVertex>,
        all: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<i32>>,
        norm: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IList_1<crate::UnityEngine::Vector3>,
        >,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector3> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::SharedVertex>,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<i32>>,
                        quest_hook::libil2cpp::Gc<
                            crate::System::Collections::Generic::IList_1<
                                crate::UnityEngine::Vector3,
                            >,
                        >,
                    ), crate::UnityEngine::Vector3, 3usize>(
                        "AverageNormalWithIndexes"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "AverageNormalWithIndexes",
                            3usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::Vector3 =
            unsafe { cordl_method_info.invoke_unchecked((), (shared, all, norm))? };
        Ok(__cordl_ret.into())
    }
    pub fn CreateMeshWithTransform(
        t: quest_hook::libil2cpp::Gc<crate::UnityEngine::Transform>,
        preserveFaces: bool,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::ProBuilderMesh>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (quest_hook::libil2cpp::Gc<crate::UnityEngine::Transform>, bool),
                        quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::ProBuilder::ProBuilderMesh,
                        >,
                        2usize,
                    >("CreateMeshWithTransform")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "CreateMeshWithTransform", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::ProBuilderMesh> =
            unsafe { cordl_method_info.invoke_unchecked((), (t, preserveFaces))? };
        Ok(__cordl_ret.into())
    }
    pub fn FilterUnusedSubmeshIndexes(
        mesh: quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::ProBuilderMesh>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::ProBuilder::ProBuilderMesh,
                        >),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("FilterUnusedSubmeshIndexes")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "FilterUnusedSubmeshIndexes", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked((), (mesh))? };
        Ok(__cordl_ret.into())
    }
    pub fn ResetPbObjectWithMeshFilter(
        pb: quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::ProBuilderMesh>,
        preserveFaces: bool,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::ProBuilderMesh>,
                        bool,
                    ), bool, 2usize>("ResetPbObjectWithMeshFilter")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "ResetPbObjectWithMeshFilter",
                            2usize
                        )
                    })
            });
        let __cordl_ret: bool =
            unsafe { cordl_method_info.invoke_unchecked((), (pb, preserveFaces))? };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "cordl_class_UnityEngine+ProBuilder+MeshOperations+InternalMeshUtility")]
impl quest_hook::libil2cpp::ObjectType
    for crate::UnityEngine::ProBuilder::MeshOperations::InternalMeshUtility
{
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
