#[cfg(feature = "UnityEngine+ProBuilder+MeshOperations+QuadUtility")]
#[repr(C)]
#[derive(Debug)]
pub struct QuadUtility {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "UnityEngine+ProBuilder+MeshOperations+QuadUtility")]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::ProBuilder::MeshOperations::QuadUtility {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine.ProBuilder.MeshOperations";
    const CLASS_NAME: &'static str = "QuadUtility";
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
#[cfg(feature = "UnityEngine+ProBuilder+MeshOperations+QuadUtility")]
impl std::ops::Deref for crate::UnityEngine::ProBuilder::MeshOperations::QuadUtility {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+ProBuilder+MeshOperations+QuadUtility")]
impl std::ops::DerefMut for crate::UnityEngine::ProBuilder::MeshOperations::QuadUtility {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+ProBuilder+MeshOperations+QuadUtility")]
impl crate::UnityEngine::ProBuilder::MeshOperations::QuadUtility {
    pub fn GetBestQuadConnection(
        wing: quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::WingedEdge>,
        connections: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::Dictionary_2<
                crate::UnityEngine::ProBuilder::EdgeLookup,
                f32,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::Face>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::Gc<
                        crate::UnityEngine::ProBuilder::WingedEdge,
                    >,
                    quest_hook::libil2cpp::Gc<
                        crate::System::Collections::Generic::Dictionary_2<
                            crate::UnityEngine::ProBuilder::EdgeLookup,
                            f32,
                        >,
                    >,
                ),
                quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::Face>,
                2usize,
            >("GetBestQuadConnection")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "GetBestQuadConnection", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::ProBuilder::Face,
        > = unsafe { method.invoke_unchecked((), (wing, connections)) };
        Ok(__cordl_ret.into())
    }
    pub fn GetQuadScore(
        mesh: quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::ProBuilderMesh>,
        left: quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::WingedEdge>,
        right: quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::WingedEdge>,
        normalThreshold: f32,
    ) -> quest_hook::libil2cpp::Result<f32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::Gc<
                        crate::UnityEngine::ProBuilder::ProBuilderMesh,
                    >,
                    quest_hook::libil2cpp::Gc<
                        crate::UnityEngine::ProBuilder::WingedEdge,
                    >,
                    quest_hook::libil2cpp::Gc<
                        crate::UnityEngine::ProBuilder::WingedEdge,
                    >,
                    f32,
                ),
                f32,
                4usize,
            >("GetQuadScore")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "GetQuadScore", 4usize
                )
            });
        let __cordl_ret: f32 = unsafe {
            method.invoke_unchecked((), (mesh, left, right, normalThreshold))
        };
        Ok(__cordl_ret.into())
    }
    pub fn ToQuads(
        mesh: quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::ProBuilderMesh>,
        faces: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IList_1<
                quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::Face>,
            >,
        >,
        smoothing: bool,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::Face>,
            >,
        >,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
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
                    bool,
                ),
                quest_hook::libil2cpp::Gc<
                    crate::System::Collections::Generic::List_1<
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::Face>,
                    >,
                >,
                3usize,
            >("ToQuads")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ToQuads", 3usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::Face>,
            >,
        > = unsafe { method.invoke_unchecked((), (mesh, faces, smoothing)) };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+ProBuilder+MeshOperations+QuadUtility")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::ProBuilder::MeshOperations::QuadUtility {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
