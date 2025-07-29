#[cfg(feature = "cordl_class_UnityEngine+ProBuilder+MeshHandles")]
#[repr(C)]
#[derive(Debug)]
pub struct MeshHandles {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "cordl_class_UnityEngine+ProBuilder+MeshHandles")]
unsafe impl quest_hook::libil2cpp::Type for crate::UnityEngine::ProBuilder::MeshHandles {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine.ProBuilder";
    const CLASS_NAME: &'static str = "MeshHandles";
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
#[cfg(feature = "UnityEngine+ProBuilder+MeshHandles")]
impl std::ops::Deref for crate::UnityEngine::ProBuilder::MeshHandles {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+ProBuilder+MeshHandles")]
impl std::ops::DerefMut for crate::UnityEngine::ProBuilder::MeshHandles {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+ProBuilder+MeshHandles")]
impl crate::UnityEngine::ProBuilder::MeshHandles {
    pub fn CreateEdgeBillboardMesh_ICollection_1_1(
        mesh: quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::ProBuilderMesh>,
        target: quest_hook::libil2cpp::Gc<crate::UnityEngine::Mesh>,
        edges: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::ICollection_1<
                crate::UnityEngine::ProBuilder::Edge,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::ProBuilder::ProBuilderMesh,
                            >,
                            quest_hook::libil2cpp::Gc<crate::UnityEngine::Mesh>,
                            quest_hook::libil2cpp::Gc<
                                crate::System::Collections::Generic::ICollection_1<
                                    crate::UnityEngine::ProBuilder::Edge,
                                >,
                            >,
                        ),
                        quest_hook::libil2cpp::Void,
                        3usize,
                    >("CreateEdgeBillboardMesh")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "CreateEdgeBillboardMesh", 3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked((), (mesh, target, edges))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn CreateEdgeBillboardMesh_ProBuilderMesh_Mesh0(
        mesh: quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::ProBuilderMesh>,
        target: quest_hook::libil2cpp::Gc<crate::UnityEngine::Mesh>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::ProBuilder::ProBuilderMesh,
                            >,
                            quest_hook::libil2cpp::Gc<crate::UnityEngine::Mesh>,
                        ),
                        quest_hook::libil2cpp::Void,
                        2usize,
                    >("CreateEdgeBillboardMesh")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "CreateEdgeBillboardMesh", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked((), (mesh, target))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn CreateEdgeMesh_Il2CppArray1(
        mesh: quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::ProBuilderMesh>,
        target: quest_hook::libil2cpp::Gc<crate::UnityEngine::Mesh>,
        edges: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::ProBuilder::Edge>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::ProBuilder::ProBuilderMesh,
                            >,
                            quest_hook::libil2cpp::Gc<crate::UnityEngine::Mesh>,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppArray<
                                    crate::UnityEngine::ProBuilder::Edge,
                                >,
                            >,
                        ),
                        quest_hook::libil2cpp::Void,
                        3usize,
                    >("CreateEdgeMesh")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "CreateEdgeMesh", 3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked((), (mesh, target, edges))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn CreateEdgeMesh_ProBuilderMesh_Mesh0(
        mesh: quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::ProBuilderMesh>,
        target: quest_hook::libil2cpp::Gc<crate::UnityEngine::Mesh>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::ProBuilder::ProBuilderMesh,
                            >,
                            quest_hook::libil2cpp::Gc<crate::UnityEngine::Mesh>,
                        ),
                        quest_hook::libil2cpp::Void,
                        2usize,
                    >("CreateEdgeMesh")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "CreateEdgeMesh", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked((), (mesh, target))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn CreateFaceMesh(
        mesh: quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::ProBuilderMesh>,
        target: quest_hook::libil2cpp::Gc<crate::UnityEngine::Mesh>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::ProBuilder::ProBuilderMesh,
                            >,
                            quest_hook::libil2cpp::Gc<crate::UnityEngine::Mesh>,
                        ),
                        quest_hook::libil2cpp::Void,
                        2usize,
                    >("CreateFaceMesh")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "CreateFaceMesh", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked((), (mesh, target))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn CreateFaceMeshFromFaces(
        mesh: quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::ProBuilderMesh>,
        faces: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IList_1<
                quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::Face>,
            >,
        >,
        target: quest_hook::libil2cpp::Gc<crate::UnityEngine::Mesh>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::ProBuilder::ProBuilderMesh,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::System::Collections::Generic::IList_1<
                                    quest_hook::libil2cpp::Gc<
                                        crate::UnityEngine::ProBuilder::Face,
                                    >,
                                >,
                            >,
                            quest_hook::libil2cpp::Gc<crate::UnityEngine::Mesh>,
                        ),
                        quest_hook::libil2cpp::Void,
                        3usize,
                    >("CreateFaceMeshFromFaces")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "CreateFaceMeshFromFaces", 3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked((), (mesh, faces, target))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn CreatePointBillboardMesh_IList_1_Mesh1(
        positions: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IList_1<crate::UnityEngine::Vector3>,
        >,
        indexes: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IList_1<i32>,
        >,
        target: quest_hook::libil2cpp::Gc<crate::UnityEngine::Mesh>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::System::Collections::Generic::IList_1<
                                    crate::UnityEngine::Vector3,
                                >,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::System::Collections::Generic::IList_1<i32>,
                            >,
                            quest_hook::libil2cpp::Gc<crate::UnityEngine::Mesh>,
                        ),
                        quest_hook::libil2cpp::Void,
                        3usize,
                    >("CreatePointBillboardMesh")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "CreatePointBillboardMesh", 3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked((), (positions, indexes, target))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn CreatePointBillboardMesh_Mesh0(
        positions: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IList_1<crate::UnityEngine::Vector3>,
        >,
        target: quest_hook::libil2cpp::Gc<crate::UnityEngine::Mesh>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::System::Collections::Generic::IList_1<
                                    crate::UnityEngine::Vector3,
                                >,
                            >,
                            quest_hook::libil2cpp::Gc<crate::UnityEngine::Mesh>,
                        ),
                        quest_hook::libil2cpp::Void,
                        2usize,
                    >("CreatePointBillboardMesh")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "CreatePointBillboardMesh", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked((), (positions, target))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn CreatePointMesh(
        positions: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Vector3>,
        >,
        indexes: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IList_1<i32>,
        >,
        target: quest_hook::libil2cpp::Gc<crate::UnityEngine::Mesh>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppArray<
                                    crate::UnityEngine::Vector3,
                                >,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::System::Collections::Generic::IList_1<i32>,
                            >,
                            quest_hook::libil2cpp::Gc<crate::UnityEngine::Mesh>,
                        ),
                        quest_hook::libil2cpp::Void,
                        3usize,
                    >("CreatePointMesh")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "CreatePointMesh", 3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked((), (positions, indexes, target))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn CreateVertexMesh_IList_1_1(
        mesh: quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::ProBuilderMesh>,
        target: quest_hook::libil2cpp::Gc<crate::UnityEngine::Mesh>,
        indexes: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IList_1<i32>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::ProBuilder::ProBuilderMesh,
                            >,
                            quest_hook::libil2cpp::Gc<crate::UnityEngine::Mesh>,
                            quest_hook::libil2cpp::Gc<
                                crate::System::Collections::Generic::IList_1<i32>,
                            >,
                        ),
                        quest_hook::libil2cpp::Void,
                        3usize,
                    >("CreateVertexMesh")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "CreateVertexMesh", 3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked((), (mesh, target, indexes))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn CreateVertexMesh_ProBuilderMesh_Mesh0(
        mesh: quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::ProBuilderMesh>,
        target: quest_hook::libil2cpp::Gc<crate::UnityEngine::Mesh>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::ProBuilder::ProBuilderMesh,
                            >,
                            quest_hook::libil2cpp::Gc<crate::UnityEngine::Mesh>,
                        ),
                        quest_hook::libil2cpp::Void,
                        2usize,
                    >("CreateVertexMesh")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "CreateVertexMesh", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked((), (mesh, target))?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "cordl_class_UnityEngine+ProBuilder+MeshHandles")]
impl quest_hook::libil2cpp::ObjectType for crate::UnityEngine::ProBuilder::MeshHandles {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
