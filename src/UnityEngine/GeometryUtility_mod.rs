#[cfg(feature = "cordl_class_UnityEngine+GeometryUtility")]
#[repr(C)]
#[derive(Debug)]
pub struct GeometryUtility {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "cordl_class_UnityEngine+GeometryUtility")]
unsafe impl quest_hook::libil2cpp::Type for crate::UnityEngine::GeometryUtility {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine";
    const CLASS_NAME: &'static str = "GeometryUtility";
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
#[cfg(feature = "UnityEngine+GeometryUtility")]
impl std::ops::Deref for crate::UnityEngine::GeometryUtility {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+GeometryUtility")]
impl std::ops::DerefMut for crate::UnityEngine::GeometryUtility {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+GeometryUtility")]
impl crate::UnityEngine::GeometryUtility {
    pub fn CalculateFrustumPlanes_Camera0(
        camera: quest_hook::libil2cpp::Gc<crate::UnityEngine::Camera>,
        planes: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Plane>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Camera>,
                        quest_hook::libil2cpp::Gc<
                            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Plane>,
                        >,
                    ), quest_hook::libil2cpp::Void, 2usize>(
                        "CalculateFrustumPlanes"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "CalculateFrustumPlanes",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked((), (camera, planes))? };
        Ok(__cordl_ret.into())
    }
    pub fn CalculateFrustumPlanes_Matrix4x4_1(
        worldToProjectionMatrix: crate::UnityEngine::Matrix4x4,
        planes: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Plane>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::UnityEngine::Matrix4x4,
                        quest_hook::libil2cpp::Gc<
                            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Plane>,
                        >,
                    ), quest_hook::libil2cpp::Void, 2usize>(
                        "CalculateFrustumPlanes"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "CalculateFrustumPlanes",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked((), (worldToProjectionMatrix, planes))? };
        Ok(__cordl_ret.into())
    }
    pub fn Internal_ExtractPlanes(
        planes: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<
                quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Plane>,
            >,
        >,
        worldToProjectionMatrix: crate::UnityEngine::Matrix4x4,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        quest_hook::libil2cpp::ByRefMut<
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Plane>,
                            >,
                        >,
                        crate::UnityEngine::Matrix4x4,
                    ), quest_hook::libil2cpp::Void, 2usize>(
                        "Internal_ExtractPlanes"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Internal_ExtractPlanes",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked((), (planes, worldToProjectionMatrix))? };
        Ok(__cordl_ret.into())
    }
    pub fn Internal_ExtractPlanes_Injected(
        planes: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::Bindings::BlittableArrayWrapper,
        >,
        worldToProjectionMatrix: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Matrix4x4>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        quest_hook::libil2cpp::ByRefMut<
                            crate::UnityEngine::Bindings::BlittableArrayWrapper,
                        >,
                        quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Matrix4x4>,
                    ), quest_hook::libil2cpp::Void, 2usize>(
                        "Internal_ExtractPlanes_Injected"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Internal_ExtractPlanes_Injected",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked((), (planes, worldToProjectionMatrix))? };
        Ok(__cordl_ret.into())
    }
    pub fn TestPlanesAABB(
        planes: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Plane>,
        >,
        bounds: crate::UnityEngine::Bounds,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        quest_hook::libil2cpp::Gc<
                            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Plane>,
                        >,
                        crate::UnityEngine::Bounds,
                    ), bool, 2usize>("TestPlanesAABB")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "TestPlanesAABB",
                            2usize
                        )
                    })
            });
        let __cordl_ret: bool =
            unsafe { cordl_method_info.invoke_unchecked((), (planes, bounds))? };
        Ok(__cordl_ret.into())
    }
    pub fn TestPlanesAABB_Injected(
        planes: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Bindings::ManagedSpanWrapper>,
        bounds: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Bounds>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        quest_hook::libil2cpp::ByRefMut<
                            crate::UnityEngine::Bindings::ManagedSpanWrapper,
                        >,
                        quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Bounds>,
                    ), bool, 2usize>("TestPlanesAABB_Injected")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "TestPlanesAABB_Injected",
                            2usize
                        )
                    })
            });
        let __cordl_ret: bool =
            unsafe { cordl_method_info.invoke_unchecked((), (planes, bounds))? };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "cordl_class_UnityEngine+GeometryUtility")]
impl quest_hook::libil2cpp::ObjectType for crate::UnityEngine::GeometryUtility {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
