#[cfg(feature = "cordl_class_UnityEngine+ProBuilder+Smoothing")]
#[repr(C)]
#[derive(Debug)]
pub struct Smoothing {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "cordl_class_UnityEngine+ProBuilder+Smoothing")]
unsafe impl quest_hook::libil2cpp::Type for crate::UnityEngine::ProBuilder::Smoothing {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine.ProBuilder";
    const CLASS_NAME: &'static str = "Smoothing";
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
#[cfg(feature = "UnityEngine+ProBuilder+Smoothing")]
impl std::ops::Deref for crate::UnityEngine::ProBuilder::Smoothing {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+ProBuilder+Smoothing")]
impl std::ops::DerefMut for crate::UnityEngine::ProBuilder::Smoothing {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+ProBuilder+Smoothing")]
impl crate::UnityEngine::ProBuilder::Smoothing {
    pub const hardRangeMax: i32 = 42i32;
    pub const hardRangeMin: i32 = 25i32;
    pub const smoothRangeMax: i32 = 24i32;
    pub const smoothRangeMin: i32 = 1i32;
    pub const smoothingGroupNone: i32 = 0i32;
    pub fn ApplySmoothingGroups_Il2CppArray1(
        mesh: quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::ProBuilderMesh>,
        faces: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<
                quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::Face>,
            >,
        >,
        angleThreshold: f32,
        normals: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Vector3>,
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
                                crate::System::Collections::Generic::IEnumerable_1<
                                    quest_hook::libil2cpp::Gc<
                                        crate::UnityEngine::ProBuilder::Face,
                                    >,
                                >,
                            >,
                            f32,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppArray<
                                    crate::UnityEngine::Vector3,
                                >,
                            >,
                        ),
                        quest_hook::libil2cpp::Void,
                        4usize,
                    >("ApplySmoothingGroups")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "ApplySmoothingGroups", 4usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info
                .invoke_unchecked((), (mesh, faces, angleThreshold, normals))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ApplySmoothingGroups_ProBuilderMesh_IEnumerable_1_f32_0(
        mesh: quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::ProBuilderMesh>,
        faces: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<
                quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::Face>,
            >,
        >,
        angleThreshold: f32,
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
                                crate::System::Collections::Generic::IEnumerable_1<
                                    quest_hook::libil2cpp::Gc<
                                        crate::UnityEngine::ProBuilder::Face,
                                    >,
                                >,
                            >,
                            f32,
                        ),
                        quest_hook::libil2cpp::Void,
                        3usize,
                    >("ApplySmoothingGroups")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "ApplySmoothingGroups", 3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked((), (mesh, faces, angleThreshold))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn FindSoftEdgesRecursive(
        normals: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Vector3>,
        >,
        wing: quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::WingedEdge>,
        angleThreshold: f32,
        processed: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::HashSet_1<
                quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::Face>,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
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
                                crate::UnityEngine::ProBuilder::WingedEdge,
                            >,
                            f32,
                            quest_hook::libil2cpp::Gc<
                                crate::System::Collections::Generic::HashSet_1<
                                    quest_hook::libil2cpp::Gc<
                                        crate::UnityEngine::ProBuilder::Face,
                                    >,
                                >,
                            >,
                        ),
                        bool,
                        4usize,
                    >("FindSoftEdgesRecursive")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "FindSoftEdgesRecursive", 4usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            cordl_method_info
                .invoke_unchecked((), (normals, wing, angleThreshold, processed))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetNextUnusedSmoothingGroup(
        start: i32,
        used: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::HashSet_1<i32>,
        >,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            i32,
                            quest_hook::libil2cpp::Gc<
                                crate::System::Collections::Generic::HashSet_1<i32>,
                            >,
                        ),
                        i32,
                        2usize,
                    >("GetNextUnusedSmoothingGroup")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "GetNextUnusedSmoothingGroup", 2usize
                        )
                    })
            });
        let __cordl_ret: i32 = unsafe {
            cordl_method_info.invoke_unchecked((), (start, used))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetUnusedSmoothingGroup(
        mesh: quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::ProBuilderMesh>,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::ProBuilder::ProBuilderMesh,
                        >),
                        i32,
                        1usize,
                    >("GetUnusedSmoothingGroup")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "GetUnusedSmoothingGroup", 1usize
                        )
                    })
            });
        let __cordl_ret: i32 = unsafe {
            cordl_method_info.invoke_unchecked((), (mesh))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn IsSmooth(index: i32) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(i32), bool, 1usize>("IsSmooth")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "IsSmooth", 1usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            cordl_method_info.invoke_unchecked((), (index))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn IsSoftEdge(
        normals: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Vector3>,
        >,
        left: crate::UnityEngine::ProBuilder::EdgeLookup,
        right: crate::UnityEngine::ProBuilder::EdgeLookup,
        threshold: f32,
    ) -> quest_hook::libil2cpp::Result<bool> {
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
                            crate::UnityEngine::ProBuilder::EdgeLookup,
                            crate::UnityEngine::ProBuilder::EdgeLookup,
                            f32,
                        ),
                        bool,
                        4usize,
                    >("IsSoftEdge")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "IsSoftEdge", 4usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            cordl_method_info.invoke_unchecked((), (normals, left, right, threshold))?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "cordl_class_UnityEngine+ProBuilder+Smoothing")]
impl quest_hook::libil2cpp::ObjectType for crate::UnityEngine::ProBuilder::Smoothing {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
