#[cfg(feature = "cordl_class_UnityEngine+ProBuilder+MeshOperations+Bevel")]
#[repr(C)]
#[derive(Debug)]
pub struct Bevel {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "cordl_class_UnityEngine+ProBuilder+MeshOperations+Bevel")]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::ProBuilder::MeshOperations::Bevel {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine.ProBuilder.MeshOperations";
    const CLASS_NAME: &'static str = "Bevel";
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
#[cfg(feature = "UnityEngine+ProBuilder+MeshOperations+Bevel")]
impl std::ops::Deref for crate::UnityEngine::ProBuilder::MeshOperations::Bevel {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+ProBuilder+MeshOperations+Bevel")]
impl std::ops::DerefMut for crate::UnityEngine::ProBuilder::MeshOperations::Bevel {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+ProBuilder+MeshOperations+Bevel")]
impl crate::UnityEngine::ProBuilder::MeshOperations::Bevel {
    pub fn BevelEdges(
        mesh: quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::ProBuilderMesh>,
        edges: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IList_1<
                crate::UnityEngine::ProBuilder::Edge,
            >,
        >,
        amount: f32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
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
                                crate::System::Collections::Generic::IList_1<
                                    crate::UnityEngine::ProBuilder::Edge,
                                >,
                            >,
                            f32,
                        ),
                        quest_hook::libil2cpp::Gc<
                            crate::System::Collections::Generic::List_1<
                                quest_hook::libil2cpp::Gc<
                                    crate::UnityEngine::ProBuilder::Face,
                                >,
                            >,
                        >,
                        3usize,
                    >("BevelEdges")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "BevelEdges", 3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::Face>,
            >,
        > = unsafe { cordl_method_info.invoke_unchecked((), (mesh, edges, amount))? };
        Ok(__cordl_ret.into())
    }
    pub fn GetBridgeFaces(
        vertices: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IList_1<
                quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::Vertex>,
            >,
        >,
        left: quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::WingedEdge>,
        right: quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::WingedEdge>,
        holes: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::Dictionary_2<
                i32,
                quest_hook::libil2cpp::Gc<
                    crate::System::Collections::Generic::List_1<
                        crate::UnityEngine::ProBuilder::SimpleTuple_2<
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::ProBuilder::FaceRebuildData,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::System::Collections::Generic::List_1<i32>,
                            >,
                        >,
                    >,
                >,
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
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::System::Collections::Generic::IList_1<
                                    quest_hook::libil2cpp::Gc<
                                        crate::UnityEngine::ProBuilder::Vertex,
                                    >,
                                >,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::ProBuilder::WingedEdge,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::ProBuilder::WingedEdge,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::System::Collections::Generic::Dictionary_2<
                                    i32,
                                    quest_hook::libil2cpp::Gc<
                                        crate::System::Collections::Generic::List_1<
                                            crate::UnityEngine::ProBuilder::SimpleTuple_2<
                                                quest_hook::libil2cpp::Gc<
                                                    crate::UnityEngine::ProBuilder::FaceRebuildData,
                                                >,
                                                quest_hook::libil2cpp::Gc<
                                                    crate::System::Collections::Generic::List_1<i32>,
                                                >,
                                            >,
                                        >,
                                    >,
                                >,
                            >,
                        ),
                        quest_hook::libil2cpp::Gc<
                            crate::System::Collections::Generic::List_1<
                                quest_hook::libil2cpp::Gc<
                                    crate::UnityEngine::ProBuilder::FaceRebuildData,
                                >,
                            >,
                        >,
                        4usize,
                    >("GetBridgeFaces")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "GetBridgeFaces", 4usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                quest_hook::libil2cpp::Gc<
                    crate::UnityEngine::ProBuilder::FaceRebuildData,
                >,
            >,
        > = unsafe {
            cordl_method_info.invoke_unchecked((), (vertices, left, right, holes))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetLeadingEdge(
        wing: quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::WingedEdge>,
        common: i32,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::ProBuilder::Edge> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::ProBuilder::WingedEdge,
                            >,
                            i32,
                        ),
                        crate::UnityEngine::ProBuilder::Edge,
                        2usize,
                    >("GetLeadingEdge")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "GetLeadingEdge", 2usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::ProBuilder::Edge = unsafe {
            cordl_method_info.invoke_unchecked((), (wing, common))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn SlideEdge(
        vertices: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IList_1<
                quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::Vertex>,
            >,
        >,
        we: quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::WingedEdge>,
        amount: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::System::Collections::Generic::IList_1<
                                    quest_hook::libil2cpp::Gc<
                                        crate::UnityEngine::ProBuilder::Vertex,
                                    >,
                                >,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::ProBuilder::WingedEdge,
                            >,
                            f32,
                        ),
                        quest_hook::libil2cpp::Void,
                        3usize,
                    >("SlideEdge")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "SlideEdge", 3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked((), (vertices, we, amount))?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "cordl_class_UnityEngine+ProBuilder+MeshOperations+Bevel")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::ProBuilder::MeshOperations::Bevel {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
