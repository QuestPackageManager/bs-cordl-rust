#[cfg(feature = "UnityEngine+ProBuilder+Shapes+Sphere")]
#[repr(C)]
#[derive(Debug)]
pub struct Sphere {
    __cordl_parent: crate::UnityEngine::ProBuilder::Shapes::Shape,
    pub m_Subdivisions: i32,
    pub m_BottomMostVertexIndex: i32,
    pub m_Smooth: bool,
}
#[cfg(feature = "UnityEngine+ProBuilder+Shapes+Sphere")]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::ProBuilder::Shapes::Sphere {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine.ProBuilder.Shapes";
    const CLASS_NAME: &'static str = "Sphere";
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
#[cfg(feature = "UnityEngine+ProBuilder+Shapes+Sphere")]
impl std::ops::Deref for crate::UnityEngine::ProBuilder::Shapes::Sphere {
    type Target = crate::UnityEngine::ProBuilder::Shapes::Shape;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+ProBuilder+Shapes+Sphere")]
impl std::ops::DerefMut for crate::UnityEngine::ProBuilder::Shapes::Sphere {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+ProBuilder+Shapes+Sphere")]
impl crate::UnityEngine::ProBuilder::Shapes::Sphere {
    pub fn CopyShape(
        &mut self,
        shape: quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::Shapes::Shape>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::ProBuilder::Shapes::Shape,
                        >),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("CopyShape")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "CopyShape", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (shape))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn RebuildMesh(
        &mut self,
        mesh: quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::ProBuilderMesh>,
        _cordl_size: crate::UnityEngine::Vector3,
        rotation: crate::UnityEngine::Quaternion,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Bounds> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::ProBuilder::ProBuilderMesh,
                            >,
                            crate::UnityEngine::Vector3,
                            crate::UnityEngine::Quaternion,
                        ),
                        crate::UnityEngine::Bounds,
                        3usize,
                    >("RebuildMesh")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "RebuildMesh", 3usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::Bounds = unsafe {
            cordl_method_info.invoke_unchecked(self, (mesh, _cordl_size, rotation))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn SubdivideIcosahedron(
        vertices: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Vector3>,
        >,
        radius: f32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Vector3>,
        >,
    > {
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
                            f32,
                        ),
                        quest_hook::libil2cpp::Gc<
                            quest_hook::libil2cpp::Il2CppArray<
                                crate::UnityEngine::Vector3,
                            >,
                        >,
                        2usize,
                    >("SubdivideIcosahedron")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "SubdivideIcosahedron", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Vector3>,
        > = unsafe { cordl_method_info.invoke_unchecked((), (vertices, radius))? };
        Ok(__cordl_ret.into())
    }
    pub fn UpdateBounds(
        &mut self,
        mesh: quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::ProBuilderMesh>,
        _cordl_size: crate::UnityEngine::Vector3,
        rotation: crate::UnityEngine::Quaternion,
        bounds: crate::UnityEngine::Bounds,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Bounds> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::ProBuilder::ProBuilderMesh,
                            >,
                            crate::UnityEngine::Vector3,
                            crate::UnityEngine::Quaternion,
                            crate::UnityEngine::Bounds,
                        ),
                        crate::UnityEngine::Bounds,
                        4usize,
                    >("UpdateBounds")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "UpdateBounds", 4usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::Bounds = unsafe {
            cordl_method_info
                .invoke_unchecked(self, (mesh, _cordl_size, rotation, bounds))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), quest_hook::libil2cpp::Void, 0usize>(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), ".ctor",
                            0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+ProBuilder+Shapes+Sphere")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::ProBuilder::Shapes::Sphere {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
