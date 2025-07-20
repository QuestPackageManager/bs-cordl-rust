#[cfg(feature = "UnityEngine+ProBuilder+ProBuilderSnapping")]
#[repr(C)]
#[derive(Debug)]
pub struct ProBuilderSnapping {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "UnityEngine+ProBuilder+ProBuilderSnapping")]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::ProBuilder::ProBuilderSnapping {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine.ProBuilder";
    const CLASS_NAME: &'static str = "ProBuilderSnapping";
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
#[cfg(feature = "UnityEngine+ProBuilder+ProBuilderSnapping")]
impl std::ops::Deref for crate::UnityEngine::ProBuilder::ProBuilderSnapping {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+ProBuilder+ProBuilderSnapping")]
impl std::ops::DerefMut for crate::UnityEngine::ProBuilder::ProBuilderSnapping {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+ProBuilder+ProBuilderSnapping")]
impl crate::UnityEngine::ProBuilder::ProBuilderSnapping {
    pub const k_MaxRaySnapDistance: f32 = std::f32::INFINITY;
    pub fn GetSnappingMaskBasedOnNormalVector(
        normal: crate::UnityEngine::Vector3,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector3> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::ProBuilder::ProBuilderSnapping as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::UnityEngine::Vector3),
                crate::UnityEngine::Vector3,
                1usize,
            >("GetSnappingMaskBasedOnNormalVector")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::ProBuilder::ProBuilderSnapping as
                    quest_hook::libil2cpp::Type > ::class(),
                    "GetSnappingMaskBasedOnNormalVector", 1usize
                )
            });
        let __cordl_ret: crate::UnityEngine::Vector3 = unsafe {
            method.invoke_unchecked((), (normal))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn IsCardinalDirection(
        direction: crate::UnityEngine::Vector3,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::ProBuilder::ProBuilderSnapping as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::UnityEngine::Vector3),
                bool,
                1usize,
            >("IsCardinalDirection")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::ProBuilder::ProBuilderSnapping as
                    quest_hook::libil2cpp::Type > ::class(), "IsCardinalDirection",
                    1usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked((), (direction))? };
        Ok(__cordl_ret.into())
    }
    pub fn SnapValueOnRay(
        ray: crate::UnityEngine::Ray,
        distance: f32,
        snap: f32,
        mask: crate::UnityEngine::ProBuilder::Vector3Mask,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector3> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::ProBuilder::ProBuilderSnapping as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::UnityEngine::Ray,
                    f32,
                    f32,
                    crate::UnityEngine::ProBuilder::Vector3Mask,
                ),
                crate::UnityEngine::Vector3,
                4usize,
            >("SnapValueOnRay")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::ProBuilder::ProBuilderSnapping as
                    quest_hook::libil2cpp::Type > ::class(), "SnapValueOnRay", 4usize
                )
            });
        let __cordl_ret: crate::UnityEngine::Vector3 = unsafe {
            method.invoke_unchecked((), (ray, distance, snap, mask))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn SnapVertices(
        mesh: quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::ProBuilderMesh>,
        indexes: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<i32>,
        >,
        snap: crate::UnityEngine::Vector3,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::ProBuilder::ProBuilderSnapping as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::Gc<
                        crate::UnityEngine::ProBuilder::ProBuilderMesh,
                    >,
                    quest_hook::libil2cpp::Gc<
                        crate::System::Collections::Generic::IEnumerable_1<i32>,
                    >,
                    crate::UnityEngine::Vector3,
                ),
                quest_hook::libil2cpp::Void,
                3usize,
            >("SnapVertices")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::ProBuilder::ProBuilderSnapping as
                    quest_hook::libil2cpp::Type > ::class(), "SnapVertices", 3usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (mesh, indexes, snap))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Snap_Vector3_Vector3_1(
        val: crate::UnityEngine::Vector3,
        snap: crate::UnityEngine::Vector3,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector3> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::ProBuilder::ProBuilderSnapping as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::UnityEngine::Vector3, crate::UnityEngine::Vector3),
                crate::UnityEngine::Vector3,
                2usize,
            >("Snap")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::ProBuilder::ProBuilderSnapping as
                    quest_hook::libil2cpp::Type > ::class(), "Snap", 2usize
                )
            });
        let __cordl_ret: crate::UnityEngine::Vector3 = unsafe {
            method.invoke_unchecked((), (val, snap))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Snap_f32_f32_0(val: f32, snap: f32) -> quest_hook::libil2cpp::Result<f32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::ProBuilder::ProBuilderSnapping as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<(f32, f32), f32, 2usize>("Snap")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::ProBuilder::ProBuilderSnapping as
                    quest_hook::libil2cpp::Type > ::class(), "Snap", 2usize
                )
            });
        let __cordl_ret: f32 = unsafe { method.invoke_unchecked((), (val, snap))? };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+ProBuilder+ProBuilderSnapping")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::ProBuilder::ProBuilderSnapping {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
