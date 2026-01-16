#[cfg(feature = "cordl_class_TMPro+TMP_MaterialManager")]
#[repr(C)]
#[derive(Debug)]
pub struct TMP_MaterialManager {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "cordl_class_TMPro+TMP_MaterialManager")]
unsafe impl quest_hook::libil2cpp::Type for crate::TMPro::TMP_MaterialManager {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "TMPro";
    const CLASS_NAME: &'static str = "TMP_MaterialManager";
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
#[cfg(feature = "TMPro+TMP_MaterialManager")]
impl std::ops::Deref for crate::TMPro::TMP_MaterialManager {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "TMPro+TMP_MaterialManager")]
impl std::ops::DerefMut for crate::TMPro::TMP_MaterialManager {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "TMPro+TMP_MaterialManager")]
impl crate::TMPro::TMP_MaterialManager {
    #[cfg(feature = "TMPro+TMP_MaterialManager+FallbackMaterial")]
    pub type FallbackMaterial = crate::TMPro::TMP_MaterialManager_FallbackMaterial;
    #[cfg(feature = "TMPro+TMP_MaterialManager+MaskingMaterial")]
    pub type MaskingMaterial = crate::TMPro::TMP_MaterialManager_MaskingMaterial;
    pub fn AddFallbackMaterialReference(
        targetMaterial: quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("AddFallbackMaterialReference")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "AddFallbackMaterialReference", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked((), (targetMaterial))? };
        Ok(__cordl_ret.into())
    }
    pub fn AddMaskingMaterial(
        baseMaterial: quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
        stencilMaterial: quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
        stencilID: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
                        i32,
                    ), quest_hook::libil2cpp::Void, 3usize>("AddMaskingMaterial")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "AddMaskingMaterial",
                            3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked((), (baseMaterial, stencilMaterial, stencilID))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn CleanupFallbackMaterials() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(), quest_hook::libil2cpp::Void, 0usize>(
                        "CleanupFallbackMaterials",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "CleanupFallbackMaterials",
                            0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked((), ())? };
        Ok(__cordl_ret.into())
    }
    pub fn ClearMaterials() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(), quest_hook::libil2cpp::Void, 0usize>("ClearMaterials")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "ClearMaterials",
                            0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked((), ())? };
        Ok(__cordl_ret.into())
    }
    pub fn CopyMaterialPresetProperties(
        source: quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
        destination: quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
                    ), quest_hook::libil2cpp::Void, 2usize>(
                        "CopyMaterialPresetProperties"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "CopyMaterialPresetProperties",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked((), (source, destination))? };
        Ok(__cordl_ret.into())
    }
    pub fn FindRootSortOverrideCanvas(
        start: quest_hook::libil2cpp::Gc<crate::UnityEngine::Transform>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<crate::UnityEngine::Transform>>
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (quest_hook::libil2cpp::Gc<crate::UnityEngine::Transform>),
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Transform>,
                        1usize,
                    >("FindRootSortOverrideCanvas")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "FindRootSortOverrideCanvas", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::Transform> =
            unsafe { cordl_method_info.invoke_unchecked((), (start))? };
        Ok(__cordl_ret.into())
    }
    pub fn GetBaseMaterial(
        stencilMaterial: quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>>
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>),
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
                        1usize,
                    >("GetBaseMaterial")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "GetBaseMaterial", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::Material> =
            unsafe { cordl_method_info.invoke_unchecked((), (stencilMaterial))? };
        Ok(__cordl_ret.into())
    }
    pub fn GetFallbackMaterial_Material1(
        sourceMaterial: quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
        targetMaterial: quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>>
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
                    ), quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>, 2usize>(
                        "GetFallbackMaterial",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "GetFallbackMaterial",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::Material> =
            unsafe { cordl_method_info.invoke_unchecked((), (sourceMaterial, targetMaterial))? };
        Ok(__cordl_ret.into())
    }
    pub fn GetFallbackMaterial_TMP_FontAsset_i32_0(
        fontAsset: quest_hook::libil2cpp::Gc<crate::TMPro::TMP_FontAsset>,
        sourceMaterial: quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
        atlasIndex: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>>
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        quest_hook::libil2cpp::Gc<crate::TMPro::TMP_FontAsset>,
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
                        i32,
                    ), quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>, 3usize>(
                        "GetFallbackMaterial",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "GetFallbackMaterial",
                            3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::Material> = unsafe {
            cordl_method_info.invoke_unchecked((), (fontAsset, sourceMaterial, atlasIndex))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetMaterialForRendering(
        graphic: quest_hook::libil2cpp::Gc<crate::UnityEngine::UI::MaskableGraphic>,
        baseMaterial: quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>>
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::UI::MaskableGraphic>,
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
                    ), quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>, 2usize>(
                        "GetMaterialForRendering",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "GetMaterialForRendering",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::Material> =
            unsafe { cordl_method_info.invoke_unchecked((), (graphic, baseMaterial))? };
        Ok(__cordl_ret.into())
    }
    pub fn GetStencilID(
        obj: quest_hook::libil2cpp::Gc<crate::UnityEngine::GameObject>,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (quest_hook::libil2cpp::Gc<crate::UnityEngine::GameObject>),
                        i32,
                        1usize,
                    >("GetStencilID")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "GetStencilID", 1usize
                        )
                    })
            });
        let __cordl_ret: i32 = unsafe { cordl_method_info.invoke_unchecked((), (obj))? };
        Ok(__cordl_ret.into())
    }
    pub fn GetStencilMaterial(
        baseMaterial: quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
        stencilID: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>>
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>, i32),
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
                        2usize,
                    >("GetStencilMaterial")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "GetStencilMaterial", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::Material> =
            unsafe { cordl_method_info.invoke_unchecked((), (baseMaterial, stencilID))? };
        Ok(__cordl_ret.into())
    }
    pub fn OnPreRender() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(), quest_hook::libil2cpp::Void, 0usize>("OnPreRender")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "OnPreRender",
                            0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked((), ())? };
        Ok(__cordl_ret.into())
    }
    pub fn ReleaseBaseMaterial(
        baseMaterial: quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("ReleaseBaseMaterial")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "ReleaseBaseMaterial", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked((), (baseMaterial))? };
        Ok(__cordl_ret.into())
    }
    pub fn ReleaseFallbackMaterial(
        fallbackMaterial: quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("ReleaseFallbackMaterial")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "ReleaseFallbackMaterial", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked((), (fallbackMaterial))? };
        Ok(__cordl_ret.into())
    }
    pub fn ReleaseStencilMaterial(
        stencilMaterial: quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("ReleaseStencilMaterial")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "ReleaseStencilMaterial", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked((), (stencilMaterial))? };
        Ok(__cordl_ret.into())
    }
    pub fn RemoveFallbackMaterialReference(
        targetMaterial: quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("RemoveFallbackMaterialReference")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "RemoveFallbackMaterialReference", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked((), (targetMaterial))? };
        Ok(__cordl_ret.into())
    }
    pub fn RemoveStencilMaterial(
        stencilMaterial: quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("RemoveStencilMaterial")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "RemoveStencilMaterial", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked((), (stencilMaterial))? };
        Ok(__cordl_ret.into())
    }
    pub fn SetStencil(
        material: quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
        stencilID: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>>
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>, i32),
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
                        2usize,
                    >("SetStencil")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "SetStencil", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::Material> =
            unsafe { cordl_method_info.invoke_unchecked((), (material, stencilID))? };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "cordl_class_TMPro+TMP_MaterialManager")]
impl quest_hook::libil2cpp::ObjectType for crate::TMPro::TMP_MaterialManager {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "cordl_class_TMPro+TMP_MaterialManager+FallbackMaterial")]
#[repr(C)]
#[derive(Debug)]
pub struct TMP_MaterialManager_FallbackMaterial {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub fallbackID: i64,
    pub sourceMaterial: quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
    pub sourceMaterialCRC: i32,
    pub fallbackMaterial: quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
    pub count: i32,
}
#[cfg(feature = "cordl_class_TMPro+TMP_MaterialManager+FallbackMaterial")]
unsafe impl quest_hook::libil2cpp::Type for crate::TMPro::TMP_MaterialManager_FallbackMaterial {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "TMPro";
    const CLASS_NAME: &'static str = "TMP_MaterialManager/FallbackMaterial";
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
#[cfg(feature = "TMPro+TMP_MaterialManager+FallbackMaterial")]
impl std::ops::Deref for crate::TMPro::TMP_MaterialManager_FallbackMaterial {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "TMPro+TMP_MaterialManager+FallbackMaterial")]
impl std::ops::DerefMut for crate::TMPro::TMP_MaterialManager_FallbackMaterial {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "TMPro+TMP_MaterialManager+FallbackMaterial")]
impl crate::TMPro::TMP_MaterialManager_FallbackMaterial {
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self =
            <Self as quest_hook::libil2cpp::Type>::class().instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
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
#[cfg(feature = "cordl_class_TMPro+TMP_MaterialManager+FallbackMaterial")]
impl quest_hook::libil2cpp::ObjectType for crate::TMPro::TMP_MaterialManager_FallbackMaterial {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "cordl_class_TMPro+TMP_MaterialManager+MaskingMaterial")]
#[repr(C)]
#[derive(Debug)]
pub struct TMP_MaterialManager_MaskingMaterial {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub baseMaterial: quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
    pub stencilMaterial: quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
    pub count: i32,
    pub stencilID: i32,
}
#[cfg(feature = "cordl_class_TMPro+TMP_MaterialManager+MaskingMaterial")]
unsafe impl quest_hook::libil2cpp::Type for crate::TMPro::TMP_MaterialManager_MaskingMaterial {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "TMPro";
    const CLASS_NAME: &'static str = "TMP_MaterialManager/MaskingMaterial";
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
#[cfg(feature = "TMPro+TMP_MaterialManager+MaskingMaterial")]
impl std::ops::Deref for crate::TMPro::TMP_MaterialManager_MaskingMaterial {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "TMPro+TMP_MaterialManager+MaskingMaterial")]
impl std::ops::DerefMut for crate::TMPro::TMP_MaterialManager_MaskingMaterial {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "TMPro+TMP_MaterialManager+MaskingMaterial")]
impl crate::TMPro::TMP_MaterialManager_MaskingMaterial {
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self =
            <Self as quest_hook::libil2cpp::Type>::class().instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
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
#[cfg(feature = "cordl_class_TMPro+TMP_MaterialManager+MaskingMaterial")]
impl quest_hook::libil2cpp::ObjectType for crate::TMPro::TMP_MaterialManager_MaskingMaterial {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
