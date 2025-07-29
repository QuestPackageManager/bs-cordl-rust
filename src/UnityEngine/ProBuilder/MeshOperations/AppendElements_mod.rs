#[cfg(feature = "cordl_class_UnityEngine+ProBuilder+MeshOperations+AppendElements")]
#[repr(C)]
#[derive(Debug)]
pub struct AppendElements {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "cordl_class_UnityEngine+ProBuilder+MeshOperations+AppendElements")]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::ProBuilder::MeshOperations::AppendElements {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine.ProBuilder.MeshOperations";
    const CLASS_NAME: &'static str = "AppendElements";
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
#[cfg(feature = "UnityEngine+ProBuilder+MeshOperations+AppendElements")]
impl std::ops::Deref for crate::UnityEngine::ProBuilder::MeshOperations::AppendElements {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+ProBuilder+MeshOperations+AppendElements")]
impl std::ops::DerefMut
for crate::UnityEngine::ProBuilder::MeshOperations::AppendElements {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+ProBuilder+MeshOperations+AppendElements")]
impl crate::UnityEngine::ProBuilder::MeshOperations::AppendElements {
    pub fn AppendFace(
        mesh: quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::ProBuilderMesh>,
        positions: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Vector3>,
        >,
        colors: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Color>,
        >,
        uv0s: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Vector2>,
        >,
        uv2s: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Vector4>,
        >,
        uv3s: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Vector4>,
        >,
        face: quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::Face>,
        common: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<i32>>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::Face>,
    > {
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
                                quest_hook::libil2cpp::Il2CppArray<
                                    crate::UnityEngine::Vector3,
                                >,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppArray<
                                    crate::UnityEngine::Color,
                                >,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppArray<
                                    crate::UnityEngine::Vector2,
                                >,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppArray<
                                    crate::UnityEngine::Vector4,
                                >,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppArray<
                                    crate::UnityEngine::Vector4,
                                >,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::ProBuilder::Face,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppArray<i32>,
                            >,
                        ),
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::Face>,
                        8usize,
                    >("AppendFace")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "AppendFace", 8usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::ProBuilder::Face,
        > = unsafe {
            cordl_method_info
                .invoke_unchecked(
                    (),
                    (mesh, positions, colors, uv0s, uv2s, uv3s, face, common),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn AppendFaces(
        mesh: quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::ProBuilderMesh>,
        positions: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<
                    quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Vector3>,
                >,
            >,
        >,
        colors: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<
                    quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Color>,
                >,
            >,
        >,
        uvs: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<
                    quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Vector2>,
                >,
            >,
        >,
        faces: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::Face>,
            >,
        >,
        shared: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<i32>>,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::Face>,
            >,
        >,
    > {
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
                                quest_hook::libil2cpp::Il2CppArray<
                                    quest_hook::libil2cpp::Gc<
                                        quest_hook::libil2cpp::Il2CppArray<
                                            crate::UnityEngine::Vector3,
                                        >,
                                    >,
                                >,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppArray<
                                    quest_hook::libil2cpp::Gc<
                                        quest_hook::libil2cpp::Il2CppArray<
                                            crate::UnityEngine::Color,
                                        >,
                                    >,
                                >,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppArray<
                                    quest_hook::libil2cpp::Gc<
                                        quest_hook::libil2cpp::Il2CppArray<
                                            crate::UnityEngine::Vector2,
                                        >,
                                    >,
                                >,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppArray<
                                    quest_hook::libil2cpp::Gc<
                                        crate::UnityEngine::ProBuilder::Face,
                                    >,
                                >,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppArray<
                                    quest_hook::libil2cpp::Gc<
                                        quest_hook::libil2cpp::Il2CppArray<i32>,
                                    >,
                                >,
                            >,
                        ),
                        quest_hook::libil2cpp::Gc<
                            quest_hook::libil2cpp::Il2CppArray<
                                quest_hook::libil2cpp::Gc<
                                    crate::UnityEngine::ProBuilder::Face,
                                >,
                            >,
                        >,
                        6usize,
                    >("AppendFaces")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "AppendFaces", 6usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::Face>,
            >,
        > = unsafe {
            cordl_method_info
                .invoke_unchecked((), (mesh, positions, colors, uvs, faces, shared))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn AppendVerticesToEdge_Edge0(
        mesh: quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::ProBuilderMesh>,
        edge: crate::UnityEngine::ProBuilder::Edge,
        count: i32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                crate::UnityEngine::ProBuilder::Edge,
            >,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::ProBuilder::ProBuilderMesh,
                            >,
                            crate::UnityEngine::ProBuilder::Edge,
                            i32,
                        ),
                        quest_hook::libil2cpp::Gc<
                            crate::System::Collections::Generic::List_1<
                                crate::UnityEngine::ProBuilder::Edge,
                            >,
                        >,
                        3usize,
                    >("AppendVerticesToEdge")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "AppendVerticesToEdge", 3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                crate::UnityEngine::ProBuilder::Edge,
            >,
        > = unsafe { cordl_method_info.invoke_unchecked((), (mesh, edge, count))? };
        Ok(__cordl_ret.into())
    }
    pub fn AppendVerticesToEdge_IList_1_1(
        mesh: quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::ProBuilderMesh>,
        edges: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IList_1<
                crate::UnityEngine::ProBuilder::Edge,
            >,
        >,
        count: i32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                crate::UnityEngine::ProBuilder::Edge,
            >,
        >,
    > {
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
                                    crate::UnityEngine::ProBuilder::Edge,
                                >,
                            >,
                            i32,
                        ),
                        quest_hook::libil2cpp::Gc<
                            crate::System::Collections::Generic::List_1<
                                crate::UnityEngine::ProBuilder::Edge,
                            >,
                        >,
                        3usize,
                    >("AppendVerticesToEdge")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "AppendVerticesToEdge", 3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                crate::UnityEngine::ProBuilder::Edge,
            >,
        > = unsafe { cordl_method_info.invoke_unchecked((), (mesh, edges, count))? };
        Ok(__cordl_ret.into())
    }
    pub fn AppendVerticesToFace_ProBuilderMesh_Face_Il2CppArray0(
        mesh: quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::ProBuilderMesh>,
        face: quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::Face>,
        points: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Vector3>,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::Face>,
    > {
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
                                crate::UnityEngine::ProBuilder::Face,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppArray<
                                    crate::UnityEngine::Vector3,
                                >,
                            >,
                        ),
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::Face>,
                        3usize,
                    >("AppendVerticesToFace")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "AppendVerticesToFace", 3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::ProBuilder::Face,
        > = unsafe { cordl_method_info.invoke_unchecked((), (mesh, face, points))? };
        Ok(__cordl_ret.into())
    }
    pub fn AppendVerticesToFace__cordl_bool1(
        mesh: quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::ProBuilderMesh>,
        face: quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::Face>,
        points: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Vector3>,
        >,
        insertOnEdge: bool,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::Face>,
    > {
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
                                crate::UnityEngine::ProBuilder::Face,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppArray<
                                    crate::UnityEngine::Vector3,
                                >,
                            >,
                            bool,
                        ),
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::Face>,
                        4usize,
                    >("AppendVerticesToFace")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "AppendVerticesToFace", 4usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::ProBuilder::Face,
        > = unsafe {
            cordl_method_info.invoke_unchecked((), (mesh, face, points, insertOnEdge))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Bridge(
        mesh: quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::ProBuilderMesh>,
        a: crate::UnityEngine::ProBuilder::Edge,
        b: crate::UnityEngine::ProBuilder::Edge,
        allowNonManifoldGeometry: bool,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::Face>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::ProBuilder::ProBuilderMesh,
                            >,
                            crate::UnityEngine::ProBuilder::Edge,
                            crate::UnityEngine::ProBuilder::Edge,
                            bool,
                        ),
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::Face>,
                        4usize,
                    >("Bridge")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "Bridge",
                            4usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::ProBuilder::Face,
        > = unsafe {
            cordl_method_info
                .invoke_unchecked((), (mesh, a, b, allowNonManifoldGeometry))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ClearAndRefreshMesh(
        mesh: quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::ProBuilderMesh>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::ProBuilder::ProBuilderMesh,
                        >),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("ClearAndRefreshMesh")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "ClearAndRefreshMesh", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked((), (mesh))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn CreatePolygon(
        mesh: quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::ProBuilderMesh>,
        indexes: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IList_1<i32>,
        >,
        unordered: bool,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::Face>,
    > {
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
                                crate::System::Collections::Generic::IList_1<i32>,
                            >,
                            bool,
                        ),
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::Face>,
                        3usize,
                    >("CreatePolygon")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "CreatePolygon", 3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::ProBuilder::Face,
        > = unsafe {
            cordl_method_info.invoke_unchecked((), (mesh, indexes, unordered))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn CreatePolygonWithHole(
        mesh: quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::ProBuilderMesh>,
        indexes: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IList_1<i32>,
        >,
        holes: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IList_1<
                quest_hook::libil2cpp::Gc<
                    crate::System::Collections::Generic::IList_1<i32>,
                >,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::Face>,
    > {
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
                                crate::System::Collections::Generic::IList_1<i32>,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::System::Collections::Generic::IList_1<
                                    quest_hook::libil2cpp::Gc<
                                        crate::System::Collections::Generic::IList_1<i32>,
                                    >,
                                >,
                            >,
                        ),
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::Face>,
                        3usize,
                    >("CreatePolygonWithHole")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "CreatePolygonWithHole", 3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::ProBuilder::Face,
        > = unsafe { cordl_method_info.invoke_unchecked((), (mesh, indexes, holes))? };
        Ok(__cordl_ret.into())
    }
    pub fn CreateShapeFromPolygon_PolyShape0(
        poly: quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::PolyShape>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::ActionResult>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::ProBuilder::PolyShape,
                        >),
                        quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::ProBuilder::ActionResult,
                        >,
                        1usize,
                    >("CreateShapeFromPolygon")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "CreateShapeFromPolygon", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::ProBuilder::ActionResult,
        > = unsafe { cordl_method_info.invoke_unchecked((), (poly))? };
        Ok(__cordl_ret.into())
    }
    pub fn CreateShapeFromPolygon_ProBuilderMesh_IList_1_f32__cordl_bool1(
        mesh: quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::ProBuilderMesh>,
        points: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IList_1<crate::UnityEngine::Vector3>,
        >,
        extrude: f32,
        flipNormals: bool,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::ActionResult>,
    > {
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
                                    crate::UnityEngine::Vector3,
                                >,
                            >,
                            f32,
                            bool,
                        ),
                        quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::ProBuilder::ActionResult,
                        >,
                        4usize,
                    >("CreateShapeFromPolygon")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "CreateShapeFromPolygon", 4usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::ProBuilder::ActionResult,
        > = unsafe {
            cordl_method_info.invoke_unchecked((), (mesh, points, extrude, flipNormals))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn CreateShapeFromPolygon_ProBuilderMesh_IList_1_f32__cordl_bool_IList_1_3(
        mesh: quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::ProBuilderMesh>,
        points: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IList_1<crate::UnityEngine::Vector3>,
        >,
        extrude: f32,
        flipNormals: bool,
        holePoints: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IList_1<
                quest_hook::libil2cpp::Gc<
                    crate::System::Collections::Generic::IList_1<
                        crate::UnityEngine::Vector3,
                    >,
                >,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::ActionResult>,
    > {
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
                                    crate::UnityEngine::Vector3,
                                >,
                            >,
                            f32,
                            bool,
                            quest_hook::libil2cpp::Gc<
                                crate::System::Collections::Generic::IList_1<
                                    quest_hook::libil2cpp::Gc<
                                        crate::System::Collections::Generic::IList_1<
                                            crate::UnityEngine::Vector3,
                                        >,
                                    >,
                                >,
                            >,
                        ),
                        quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::ProBuilder::ActionResult,
                        >,
                        5usize,
                    >("CreateShapeFromPolygon")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "CreateShapeFromPolygon", 5usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::ProBuilder::ActionResult,
        > = unsafe {
            cordl_method_info
                .invoke_unchecked((), (mesh, points, extrude, flipNormals, holePoints))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn CreateShapeFromPolygon_ProBuilderMesh_IList_1_f32__cordl_bool_Vector3_IList_1_2(
        mesh: quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::ProBuilderMesh>,
        points: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IList_1<crate::UnityEngine::Vector3>,
        >,
        extrude: f32,
        flipNormals: bool,
        cameraLookAt: crate::UnityEngine::Vector3,
        holePoints: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IList_1<
                quest_hook::libil2cpp::Gc<
                    crate::System::Collections::Generic::IList_1<
                        crate::UnityEngine::Vector3,
                    >,
                >,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::ActionResult>,
    > {
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
                                    crate::UnityEngine::Vector3,
                                >,
                            >,
                            f32,
                            bool,
                            crate::UnityEngine::Vector3,
                            quest_hook::libil2cpp::Gc<
                                crate::System::Collections::Generic::IList_1<
                                    quest_hook::libil2cpp::Gc<
                                        crate::System::Collections::Generic::IList_1<
                                            crate::UnityEngine::Vector3,
                                        >,
                                    >,
                                >,
                            >,
                        ),
                        quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::ProBuilder::ActionResult,
                        >,
                        6usize,
                    >("CreateShapeFromPolygon")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "CreateShapeFromPolygon", 6usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::ProBuilder::ActionResult,
        > = unsafe {
            cordl_method_info
                .invoke_unchecked(
                    (),
                    (mesh, points, extrude, flipNormals, cameraLookAt, holePoints),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn DuplicateAndFlip(
        mesh: quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::ProBuilderMesh>,
        faces: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::Face>,
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
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppArray<
                                    quest_hook::libil2cpp::Gc<
                                        crate::UnityEngine::ProBuilder::Face,
                                    >,
                                >,
                            >,
                        ),
                        quest_hook::libil2cpp::Void,
                        2usize,
                    >("DuplicateAndFlip")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "DuplicateAndFlip", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked((), (mesh, faces))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn FaceWithVertices(
        vertices: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::Vertex>,
            >,
        >,
        unordered: bool,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::FaceRebuildData>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::System::Collections::Generic::List_1<
                                    quest_hook::libil2cpp::Gc<
                                        crate::UnityEngine::ProBuilder::Vertex,
                                    >,
                                >,
                            >,
                            bool,
                        ),
                        quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::ProBuilder::FaceRebuildData,
                        >,
                        2usize,
                    >("FaceWithVertices")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "FaceWithVertices", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::ProBuilder::FaceRebuildData,
        > = unsafe { cordl_method_info.invoke_unchecked((), (vertices, unordered))? };
        Ok(__cordl_ret.into())
    }
    pub fn FaceWithVerticesAndHole(
        borderVertices: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::Vertex>,
            >,
        >,
        holes: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                quest_hook::libil2cpp::Gc<
                    crate::System::Collections::Generic::List_1<
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::Vertex>,
                    >,
                >,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::FaceRebuildData>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::System::Collections::Generic::List_1<
                                    quest_hook::libil2cpp::Gc<
                                        crate::UnityEngine::ProBuilder::Vertex,
                                    >,
                                >,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::System::Collections::Generic::List_1<
                                    quest_hook::libil2cpp::Gc<
                                        crate::System::Collections::Generic::List_1<
                                            quest_hook::libil2cpp::Gc<
                                                crate::UnityEngine::ProBuilder::Vertex,
                                            >,
                                        >,
                                    >,
                                >,
                            >,
                        ),
                        quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::ProBuilder::FaceRebuildData,
                        >,
                        2usize,
                    >("FaceWithVerticesAndHole")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "FaceWithVerticesAndHole", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::ProBuilder::FaceRebuildData,
        > = unsafe { cordl_method_info.invoke_unchecked((), (borderVertices, holes))? };
        Ok(__cordl_ret.into())
    }
    pub fn InsertVertexInFace(
        mesh: quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::ProBuilderMesh>,
        face: quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::Face>,
        point: crate::UnityEngine::Vector3,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::Face>,
            >,
        >,
    > {
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
                                crate::UnityEngine::ProBuilder::Face,
                            >,
                            crate::UnityEngine::Vector3,
                        ),
                        quest_hook::libil2cpp::Gc<
                            quest_hook::libil2cpp::Il2CppArray<
                                quest_hook::libil2cpp::Gc<
                                    crate::UnityEngine::ProBuilder::Face,
                                >,
                            >,
                        >,
                        3usize,
                    >("InsertVertexInFace")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "InsertVertexInFace", 3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::Face>,
            >,
        > = unsafe { cordl_method_info.invoke_unchecked((), (mesh, face, point))? };
        Ok(__cordl_ret.into())
    }
    pub fn InsertVertexInMesh(
        mesh: quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::ProBuilderMesh>,
        point: crate::UnityEngine::Vector3,
        normal: crate::UnityEngine::Vector3,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::Vertex>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::ProBuilder::ProBuilderMesh,
                            >,
                            crate::UnityEngine::Vector3,
                            crate::UnityEngine::Vector3,
                        ),
                        quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::ProBuilder::Vertex,
                        >,
                        3usize,
                    >("InsertVertexInMesh")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "InsertVertexInMesh", 3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::ProBuilder::Vertex,
        > = unsafe { cordl_method_info.invoke_unchecked((), (mesh, point, normal))? };
        Ok(__cordl_ret.into())
    }
    pub fn InsertVertexOnEdge(
        mesh: quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::ProBuilderMesh>,
        originalEdge: crate::UnityEngine::ProBuilder::Edge,
        point: crate::UnityEngine::Vector3,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::Vertex>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::ProBuilder::ProBuilderMesh,
                            >,
                            crate::UnityEngine::ProBuilder::Edge,
                            crate::UnityEngine::Vector3,
                        ),
                        quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::ProBuilder::Vertex,
                        >,
                        3usize,
                    >("InsertVertexOnEdge")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "InsertVertexOnEdge", 3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::ProBuilder::Vertex,
        > = unsafe {
            cordl_method_info.invoke_unchecked((), (mesh, originalEdge, point))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn TentCapWithVertices(
        path: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::Vertex>,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                quest_hook::libil2cpp::Gc<
                    crate::UnityEngine::ProBuilder::FaceRebuildData,
                >,
            >,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (quest_hook::libil2cpp::Gc<
                            crate::System::Collections::Generic::List_1<
                                quest_hook::libil2cpp::Gc<
                                    crate::UnityEngine::ProBuilder::Vertex,
                                >,
                            >,
                        >),
                        quest_hook::libil2cpp::Gc<
                            crate::System::Collections::Generic::List_1<
                                quest_hook::libil2cpp::Gc<
                                    crate::UnityEngine::ProBuilder::FaceRebuildData,
                                >,
                            >,
                        >,
                        1usize,
                    >("TentCapWithVertices")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "TentCapWithVertices", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                quest_hook::libil2cpp::Gc<
                    crate::UnityEngine::ProBuilder::FaceRebuildData,
                >,
            >,
        > = unsafe { cordl_method_info.invoke_unchecked((), (path))? };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "cordl_class_UnityEngine+ProBuilder+MeshOperations+AppendElements")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::ProBuilder::MeshOperations::AppendElements {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
