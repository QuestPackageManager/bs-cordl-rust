#[cfg(feature = "cordl_class_UnityEngine+Rendering+SphericalHarmonicsL2Utils")]
#[repr(C)]
#[derive(Debug)]
pub struct SphericalHarmonicsL2Utils {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "cordl_class_UnityEngine+Rendering+SphericalHarmonicsL2Utils")]
unsafe impl quest_hook::libil2cpp::Type
    for crate::UnityEngine::Rendering::SphericalHarmonicsL2Utils
{
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine.Rendering";
    const CLASS_NAME: &'static str = "SphericalHarmonicsL2Utils";
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
#[cfg(feature = "UnityEngine+Rendering+SphericalHarmonicsL2Utils")]
impl std::ops::Deref for crate::UnityEngine::Rendering::SphericalHarmonicsL2Utils {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Rendering+SphericalHarmonicsL2Utils")]
impl std::ops::DerefMut for crate::UnityEngine::Rendering::SphericalHarmonicsL2Utils {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Rendering+SphericalHarmonicsL2Utils")]
impl crate::UnityEngine::Rendering::SphericalHarmonicsL2Utils {
    pub fn GetCoefficient(
        sh: crate::UnityEngine::Rendering::SphericalHarmonicsL2,
        index: i32,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector3> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (crate::UnityEngine::Rendering::SphericalHarmonicsL2, i32),
                        crate::UnityEngine::Vector3,
                        2usize,
                    >("GetCoefficient")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "GetCoefficient", 2usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::Vector3 =
            unsafe { cordl_method_info.invoke_unchecked((), (sh, index))? };
        Ok(__cordl_ret.into())
    }
    pub fn GetL1(
        sh: crate::UnityEngine::Rendering::SphericalHarmonicsL2,
        L1_R: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector3>,
        L1_G: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector3>,
        L1_B: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector3>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::UnityEngine::Rendering::SphericalHarmonicsL2,
                        quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector3>,
                        quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector3>,
                        quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector3>,
                    ), quest_hook::libil2cpp::Void, 4usize>("GetL1")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "GetL1",
                            4usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked((), (sh, L1_R, L1_G, L1_B))? };
        Ok(__cordl_ret.into())
    }
    pub fn GetL2(
        sh: crate::UnityEngine::Rendering::SphericalHarmonicsL2,
        L2_0: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector3>,
        L2_1: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector3>,
        L2_2: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector3>,
        L2_3: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector3>,
        L2_4: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector3>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::UnityEngine::Rendering::SphericalHarmonicsL2,
                        quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector3>,
                        quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector3>,
                        quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector3>,
                        quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector3>,
                        quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector3>,
                    ), quest_hook::libil2cpp::Void, 6usize>("GetL2")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "GetL2",
                            6usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked((), (sh, L2_0, L2_1, L2_2, L2_3, L2_4))? };
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self =
            <Self as quest_hook::libil2cpp::Type>::class().instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn SetCoefficient(
        sh: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Rendering::SphericalHarmonicsL2>,
        index: i32,
        coefficient: crate::UnityEngine::Vector3,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        quest_hook::libil2cpp::ByRefMut<
                            crate::UnityEngine::Rendering::SphericalHarmonicsL2,
                        >,
                        i32,
                        crate::UnityEngine::Vector3,
                    ), quest_hook::libil2cpp::Void, 3usize>("SetCoefficient")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "SetCoefficient",
                            3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked((), (sh, index, coefficient))? };
        Ok(__cordl_ret.into())
    }
    pub fn SetL0(
        sh: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Rendering::SphericalHarmonicsL2>,
        L0: crate::UnityEngine::Vector3,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        quest_hook::libil2cpp::ByRefMut<
                            crate::UnityEngine::Rendering::SphericalHarmonicsL2,
                        >,
                        crate::UnityEngine::Vector3,
                    ), quest_hook::libil2cpp::Void, 2usize>("SetL0")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "SetL0",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked((), (sh, L0))? };
        Ok(__cordl_ret.into())
    }
    pub fn SetL1(
        sh: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Rendering::SphericalHarmonicsL2>,
        L1_R: crate::UnityEngine::Vector3,
        L1_G: crate::UnityEngine::Vector3,
        L1_B: crate::UnityEngine::Vector3,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        quest_hook::libil2cpp::ByRefMut<
                            crate::UnityEngine::Rendering::SphericalHarmonicsL2,
                        >,
                        crate::UnityEngine::Vector3,
                        crate::UnityEngine::Vector3,
                        crate::UnityEngine::Vector3,
                    ), quest_hook::libil2cpp::Void, 4usize>("SetL1")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "SetL1",
                            4usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked((), (sh, L1_R, L1_G, L1_B))? };
        Ok(__cordl_ret.into())
    }
    pub fn SetL1B(
        sh: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Rendering::SphericalHarmonicsL2>,
        L1_B: crate::UnityEngine::Vector3,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        quest_hook::libil2cpp::ByRefMut<
                            crate::UnityEngine::Rendering::SphericalHarmonicsL2,
                        >,
                        crate::UnityEngine::Vector3,
                    ), quest_hook::libil2cpp::Void, 2usize>("SetL1B")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "SetL1B",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked((), (sh, L1_B))? };
        Ok(__cordl_ret.into())
    }
    pub fn SetL1G(
        sh: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Rendering::SphericalHarmonicsL2>,
        L1_G: crate::UnityEngine::Vector3,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        quest_hook::libil2cpp::ByRefMut<
                            crate::UnityEngine::Rendering::SphericalHarmonicsL2,
                        >,
                        crate::UnityEngine::Vector3,
                    ), quest_hook::libil2cpp::Void, 2usize>("SetL1G")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "SetL1G",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked((), (sh, L1_G))? };
        Ok(__cordl_ret.into())
    }
    pub fn SetL1R(
        sh: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Rendering::SphericalHarmonicsL2>,
        L1_R: crate::UnityEngine::Vector3,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        quest_hook::libil2cpp::ByRefMut<
                            crate::UnityEngine::Rendering::SphericalHarmonicsL2,
                        >,
                        crate::UnityEngine::Vector3,
                    ), quest_hook::libil2cpp::Void, 2usize>("SetL1R")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "SetL1R",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked((), (sh, L1_R))? };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(&mut self) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), quest_hook::libil2cpp::Void, 0usize>(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            ".ctor",
                            0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "cordl_class_UnityEngine+Rendering+SphericalHarmonicsL2Utils")]
impl quest_hook::libil2cpp::ObjectType
    for crate::UnityEngine::Rendering::SphericalHarmonicsL2Utils
{
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
