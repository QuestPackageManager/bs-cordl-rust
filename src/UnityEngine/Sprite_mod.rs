#[cfg(feature = "UnityEngine+Sprite")]
#[repr(C)]
#[derive(Debug)]
pub struct Sprite {
    __cordl_parent: crate::UnityEngine::Object,
}
#[cfg(feature = "UnityEngine+Sprite")]
unsafe impl quest_hook::libil2cpp::Type for crate::UnityEngine::Sprite {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine";
    const CLASS_NAME: &'static str = "Sprite";
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
#[cfg(feature = "UnityEngine+Sprite")]
impl std::ops::Deref for crate::UnityEngine::Sprite {
    type Target = crate::UnityEngine::Object;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Sprite")]
impl std::ops::DerefMut for crate::UnityEngine::Sprite {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Sprite")]
impl crate::UnityEngine::Sprite {
    pub fn CreateSprite(
        texture: quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture2D>,
        rect: crate::UnityEngine::Rect,
        pivot: crate::UnityEngine::Vector2,
        pixelsPerUnit: f32,
        extrude: u32,
        meshType: crate::UnityEngine::SpriteMeshType,
        border: crate::UnityEngine::Vector4,
        generateFallbackPhysicsShape: bool,
        secondaryTexture: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                crate::UnityEngine::SecondarySpriteTexture,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::Sprite>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture2D>,
                            crate::UnityEngine::Rect,
                            crate::UnityEngine::Vector2,
                            f32,
                            u32,
                            crate::UnityEngine::SpriteMeshType,
                            crate::UnityEngine::Vector4,
                            bool,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppArray<
                                    crate::UnityEngine::SecondarySpriteTexture,
                                >,
                            >,
                        ),
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Sprite>,
                        9usize,
                    >("CreateSprite")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "CreateSprite", 9usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::Sprite> = unsafe {
            cordl_method_info
                .invoke_unchecked(
                    (),
                    (
                        texture,
                        rect,
                        pivot,
                        pixelsPerUnit,
                        extrude,
                        meshType,
                        border,
                        generateFallbackPhysicsShape,
                        secondaryTexture,
                    ),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn CreateSpriteWithoutTextureScripting(
        rect: crate::UnityEngine::Rect,
        pivot: crate::UnityEngine::Vector2,
        pixelsToUnits: f32,
        texture: quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture2D>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::Sprite>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            crate::UnityEngine::Rect,
                            crate::UnityEngine::Vector2,
                            f32,
                            quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture2D>,
                        ),
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Sprite>,
                        4usize,
                    >("CreateSpriteWithoutTextureScripting")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "CreateSpriteWithoutTextureScripting", 4usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::Sprite> = unsafe {
            cordl_method_info
                .invoke_unchecked((), (rect, pivot, pixelsToUnits, texture))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn CreateSpriteWithoutTextureScripting_Injected(
        rect: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Rect>,
        pivot: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector2>,
        pixelsToUnits: f32,
        texture: quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture2D>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::Sprite>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Rect>,
                            quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector2>,
                            f32,
                            quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture2D>,
                        ),
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Sprite>,
                        4usize,
                    >("CreateSpriteWithoutTextureScripting_Injected")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "CreateSpriteWithoutTextureScripting_Injected", 4usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::Sprite> = unsafe {
            cordl_method_info
                .invoke_unchecked((), (rect, pivot, pixelsToUnits, texture))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn CreateSprite_Injected(
        texture: quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture2D>,
        rect: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Rect>,
        pivot: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector2>,
        pixelsPerUnit: f32,
        extrude: u32,
        meshType: crate::UnityEngine::SpriteMeshType,
        border: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector4>,
        generateFallbackPhysicsShape: bool,
        secondaryTexture: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                crate::UnityEngine::SecondarySpriteTexture,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::Sprite>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture2D>,
                            quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Rect>,
                            quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector2>,
                            f32,
                            u32,
                            crate::UnityEngine::SpriteMeshType,
                            quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector4>,
                            bool,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppArray<
                                    crate::UnityEngine::SecondarySpriteTexture,
                                >,
                            >,
                        ),
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Sprite>,
                        9usize,
                    >("CreateSprite_Injected")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "CreateSprite_Injected", 9usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::Sprite> = unsafe {
            cordl_method_info
                .invoke_unchecked(
                    (),
                    (
                        texture,
                        rect,
                        pivot,
                        pixelsPerUnit,
                        extrude,
                        meshType,
                        border,
                        generateFallbackPhysicsShape,
                        secondaryTexture,
                    ),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Create_Rect_Vector2_f32_1(
        rect: crate::UnityEngine::Rect,
        pivot: crate::UnityEngine::Vector2,
        pixelsToUnits: f32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::Sprite>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (crate::UnityEngine::Rect, crate::UnityEngine::Vector2, f32),
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Sprite>,
                        3usize,
                    >("Create")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "Create",
                            3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::Sprite> = unsafe {
            cordl_method_info.invoke_unchecked((), (rect, pivot, pixelsToUnits))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Create_Rect_Vector2_f32_Texture2D0(
        rect: crate::UnityEngine::Rect,
        pivot: crate::UnityEngine::Vector2,
        pixelsToUnits: f32,
        texture: quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture2D>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::Sprite>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            crate::UnityEngine::Rect,
                            crate::UnityEngine::Vector2,
                            f32,
                            quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture2D>,
                        ),
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Sprite>,
                        4usize,
                    >("Create")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "Create",
                            4usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::Sprite> = unsafe {
            cordl_method_info
                .invoke_unchecked((), (rect, pivot, pixelsToUnits, texture))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Create_Texture2D_Rect_Vector2_8(
        texture: quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture2D>,
        rect: crate::UnityEngine::Rect,
        pivot: crate::UnityEngine::Vector2,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::Sprite>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture2D>,
                            crate::UnityEngine::Rect,
                            crate::UnityEngine::Vector2,
                        ),
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Sprite>,
                        3usize,
                    >("Create")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "Create",
                            3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::Sprite> = unsafe {
            cordl_method_info.invoke_unchecked((), (texture, rect, pivot))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Create_Texture2D_Rect_Vector2_f32_7(
        texture: quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture2D>,
        rect: crate::UnityEngine::Rect,
        pivot: crate::UnityEngine::Vector2,
        pixelsPerUnit: f32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::Sprite>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture2D>,
                            crate::UnityEngine::Rect,
                            crate::UnityEngine::Vector2,
                            f32,
                        ),
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Sprite>,
                        4usize,
                    >("Create")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "Create",
                            4usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::Sprite> = unsafe {
            cordl_method_info
                .invoke_unchecked((), (texture, rect, pivot, pixelsPerUnit))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Create_Texture2D_Rect_Vector2_f32_u32_6(
        texture: quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture2D>,
        rect: crate::UnityEngine::Rect,
        pivot: crate::UnityEngine::Vector2,
        pixelsPerUnit: f32,
        extrude: u32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::Sprite>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture2D>,
                            crate::UnityEngine::Rect,
                            crate::UnityEngine::Vector2,
                            f32,
                            u32,
                        ),
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Sprite>,
                        5usize,
                    >("Create")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "Create",
                            5usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::Sprite> = unsafe {
            cordl_method_info
                .invoke_unchecked((), (texture, rect, pivot, pixelsPerUnit, extrude))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Create_Texture2D_Rect_Vector2_f32_u32_SpriteMeshType5(
        texture: quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture2D>,
        rect: crate::UnityEngine::Rect,
        pivot: crate::UnityEngine::Vector2,
        pixelsPerUnit: f32,
        extrude: u32,
        meshType: crate::UnityEngine::SpriteMeshType,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::Sprite>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture2D>,
                            crate::UnityEngine::Rect,
                            crate::UnityEngine::Vector2,
                            f32,
                            u32,
                            crate::UnityEngine::SpriteMeshType,
                        ),
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Sprite>,
                        6usize,
                    >("Create")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "Create",
                            6usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::Sprite> = unsafe {
            cordl_method_info
                .invoke_unchecked(
                    (),
                    (texture, rect, pivot, pixelsPerUnit, extrude, meshType),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Create_Texture2D_Rect_Vector2_f32_u32_SpriteMeshType_Vector4_4(
        texture: quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture2D>,
        rect: crate::UnityEngine::Rect,
        pivot: crate::UnityEngine::Vector2,
        pixelsPerUnit: f32,
        extrude: u32,
        meshType: crate::UnityEngine::SpriteMeshType,
        border: crate::UnityEngine::Vector4,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::Sprite>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture2D>,
                            crate::UnityEngine::Rect,
                            crate::UnityEngine::Vector2,
                            f32,
                            u32,
                            crate::UnityEngine::SpriteMeshType,
                            crate::UnityEngine::Vector4,
                        ),
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Sprite>,
                        7usize,
                    >("Create")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "Create",
                            7usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::Sprite> = unsafe {
            cordl_method_info
                .invoke_unchecked(
                    (),
                    (texture, rect, pivot, pixelsPerUnit, extrude, meshType, border),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Create_Texture2D_Rect_Vector2_f32_u32_SpriteMeshType_Vector4__cordl_bool2(
        texture: quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture2D>,
        rect: crate::UnityEngine::Rect,
        pivot: crate::UnityEngine::Vector2,
        pixelsPerUnit: f32,
        extrude: u32,
        meshType: crate::UnityEngine::SpriteMeshType,
        border: crate::UnityEngine::Vector4,
        generateFallbackPhysicsShape: bool,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::Sprite>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture2D>,
                            crate::UnityEngine::Rect,
                            crate::UnityEngine::Vector2,
                            f32,
                            u32,
                            crate::UnityEngine::SpriteMeshType,
                            crate::UnityEngine::Vector4,
                            bool,
                        ),
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Sprite>,
                        8usize,
                    >("Create")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "Create",
                            8usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::Sprite> = unsafe {
            cordl_method_info
                .invoke_unchecked(
                    (),
                    (
                        texture,
                        rect,
                        pivot,
                        pixelsPerUnit,
                        extrude,
                        meshType,
                        border,
                        generateFallbackPhysicsShape,
                    ),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Create_Texture2D_Rect_Vector2_f32_u32_SpriteMeshType_Vector4__cordl_bool_Il2CppArray3(
        texture: quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture2D>,
        rect: crate::UnityEngine::Rect,
        pivot: crate::UnityEngine::Vector2,
        pixelsPerUnit: f32,
        extrude: u32,
        meshType: crate::UnityEngine::SpriteMeshType,
        border: crate::UnityEngine::Vector4,
        generateFallbackPhysicsShape: bool,
        secondaryTextures: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                crate::UnityEngine::SecondarySpriteTexture,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::Sprite>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture2D>,
                            crate::UnityEngine::Rect,
                            crate::UnityEngine::Vector2,
                            f32,
                            u32,
                            crate::UnityEngine::SpriteMeshType,
                            crate::UnityEngine::Vector4,
                            bool,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppArray<
                                    crate::UnityEngine::SecondarySpriteTexture,
                                >,
                            >,
                        ),
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Sprite>,
                        9usize,
                    >("Create")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "Create",
                            9usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::Sprite> = unsafe {
            cordl_method_info
                .invoke_unchecked(
                    (),
                    (
                        texture,
                        rect,
                        pivot,
                        pixelsPerUnit,
                        extrude,
                        meshType,
                        border,
                        generateFallbackPhysicsShape,
                        secondaryTextures,
                    ),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetInnerUVs(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector4> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        crate::UnityEngine::Vector4,
                        0usize,
                    >("GetInnerUVs")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "GetInnerUVs", 0usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::Vector4 = unsafe {
            cordl_method_info.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetInnerUVs_Injected(
        &mut self,
        ret: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector4>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector4>),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("GetInnerUVs_Injected")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "GetInnerUVs_Injected", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (ret))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetOuterUVs(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector4> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        crate::UnityEngine::Vector4,
                        0usize,
                    >("GetOuterUVs")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "GetOuterUVs", 0usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::Vector4 = unsafe {
            cordl_method_info.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetOuterUVs_Injected(
        &mut self,
        ret: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector4>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector4>),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("GetOuterUVs_Injected")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "GetOuterUVs_Injected", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (ret))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetPacked(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), i32, 0usize>("GetPacked")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "GetPacked", 0usize
                        )
                    })
            });
        let __cordl_ret: i32 = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn GetPackingMode(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), i32, 0usize>("GetPackingMode")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "GetPackingMode", 0usize
                        )
                    })
            });
        let __cordl_ret: i32 = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn GetPackingRotation(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), i32, 0usize>("GetPackingRotation")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "GetPackingRotation", 0usize
                        )
                    })
            });
        let __cordl_ret: i32 = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn GetPadding(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector4> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), crate::UnityEngine::Vector4, 0usize>("GetPadding")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "GetPadding", 0usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::Vector4 = unsafe {
            cordl_method_info.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetPadding_Injected(
        &mut self,
        ret: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector4>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector4>),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("GetPadding_Injected")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "GetPadding_Injected", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (ret))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetPhysicsShape(
        &mut self,
        shapeIdx: i32,
        physicsShape: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<crate::UnityEngine::Vector2>,
        >,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            i32,
                            quest_hook::libil2cpp::Gc<
                                crate::System::Collections::Generic::List_1<
                                    crate::UnityEngine::Vector2,
                                >,
                            >,
                        ),
                        i32,
                        2usize,
                    >("GetPhysicsShape")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "GetPhysicsShape", 2usize
                        )
                    })
            });
        let __cordl_ret: i32 = unsafe {
            cordl_method_info.invoke_unchecked(self, (shapeIdx, physicsShape))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetPhysicsShapeCount(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), i32, 0usize>("GetPhysicsShapeCount")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "GetPhysicsShapeCount", 0usize
                        )
                    })
            });
        let __cordl_ret: i32 = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn GetPhysicsShapeImpl(
        sprite: quest_hook::libil2cpp::Gc<crate::UnityEngine::Sprite>,
        shapeIdx: i32,
        physicsShape: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<crate::UnityEngine::Vector2>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<crate::UnityEngine::Sprite>,
                            i32,
                            quest_hook::libil2cpp::Gc<
                                crate::System::Collections::Generic::List_1<
                                    crate::UnityEngine::Vector2,
                                >,
                            >,
                        ),
                        quest_hook::libil2cpp::Void,
                        3usize,
                    >("GetPhysicsShapeImpl")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "GetPhysicsShapeImpl", 3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked((), (sprite, shapeIdx, physicsShape))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetPhysicsShapePointCount(
        &mut self,
        shapeIdx: i32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(i32), i32, 1usize>("GetPhysicsShapePointCount")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "GetPhysicsShapePointCount", 1usize
                        )
                    })
            });
        let __cordl_ret: i32 = unsafe {
            cordl_method_info.invoke_unchecked(self, (shapeIdx))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetSecondaryTexture(
        &mut self,
        index: i32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture2D>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (i32),
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture2D>,
                        1usize,
                    >("GetSecondaryTexture")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "GetSecondaryTexture", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture2D> = unsafe {
            cordl_method_info.invoke_unchecked(self, (index))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetSecondaryTextureCount(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), i32, 0usize>("GetSecondaryTextureCount")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "GetSecondaryTextureCount", 0usize
                        )
                    })
            });
        let __cordl_ret: i32 = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn GetSecondaryTextures(
        &mut self,
        secondaryTexture: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                crate::UnityEngine::SecondarySpriteTexture,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<
                            quest_hook::libil2cpp::Il2CppArray<
                                crate::UnityEngine::SecondarySpriteTexture,
                            >,
                        >),
                        i32,
                        1usize,
                    >("GetSecondaryTextures")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "GetSecondaryTextures", 1usize
                        )
                    })
            });
        let __cordl_ret: i32 = unsafe {
            cordl_method_info.invoke_unchecked(self, (secondaryTexture))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetTextureRect(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Rect> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        crate::UnityEngine::Rect,
                        0usize,
                    >("GetTextureRect")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "GetTextureRect", 0usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::Rect = unsafe {
            cordl_method_info.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetTextureRectOffset(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector2> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        crate::UnityEngine::Vector2,
                        0usize,
                    >("GetTextureRectOffset")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "GetTextureRectOffset", 0usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::Vector2 = unsafe {
            cordl_method_info.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetTextureRectOffset_Injected(
        &mut self,
        ret: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector2>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector2>),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("GetTextureRectOffset_Injected")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "GetTextureRectOffset_Injected", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (ret))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetTextureRect_Injected(
        &mut self,
        ret: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Rect>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Rect>),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("GetTextureRect_Injected")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "GetTextureRect_Injected", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (ret))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Internal_GetPhysicsShapePointCount(
        &mut self,
        shapeIdx: i32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (i32),
                        i32,
                        1usize,
                    >("Internal_GetPhysicsShapePointCount")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "Internal_GetPhysicsShapePointCount", 1usize
                        )
                    })
            });
        let __cordl_ret: i32 = unsafe {
            cordl_method_info.invoke_unchecked(self, (shapeIdx))?
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
    pub fn OverrideGeometry(
        &mut self,
        vertices: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Vector2>,
        >,
        triangles: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u16>>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppArray<
                                    crate::UnityEngine::Vector2,
                                >,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppArray<u16>,
                            >,
                        ),
                        quest_hook::libil2cpp::Void,
                        2usize,
                    >("OverrideGeometry")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "OverrideGeometry", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (vertices, triangles))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn OverridePhysicsShapeCount(
        sprite: quest_hook::libil2cpp::Gc<crate::UnityEngine::Sprite>,
        physicsShapeCount: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (quest_hook::libil2cpp::Gc<crate::UnityEngine::Sprite>, i32),
                        quest_hook::libil2cpp::Void,
                        2usize,
                    >("OverridePhysicsShapeCount")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "OverridePhysicsShapeCount", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked((), (sprite, physicsShapeCount))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn OverridePhysicsShape_IList_1_0(
        &mut self,
        physicsShapes: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IList_1<
                quest_hook::libil2cpp::Gc<
                    quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Vector2>,
                >,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<
                            crate::System::Collections::Generic::IList_1<
                                quest_hook::libil2cpp::Gc<
                                    quest_hook::libil2cpp::Il2CppArray<
                                        crate::UnityEngine::Vector2,
                                    >,
                                >,
                            >,
                        >),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("OverridePhysicsShape")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "OverridePhysicsShape", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (physicsShapes))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn OverridePhysicsShape_Sprite_Il2CppArray_i32_1(
        sprite: quest_hook::libil2cpp::Gc<crate::UnityEngine::Sprite>,
        physicsShape: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Vector2>,
        >,
        idx: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<crate::UnityEngine::Sprite>,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppArray<
                                    crate::UnityEngine::Vector2,
                                >,
                            >,
                            i32,
                        ),
                        quest_hook::libil2cpp::Void,
                        3usize,
                    >("OverridePhysicsShape")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "OverridePhysicsShape", 3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked((), (sprite, physicsShape, idx))?
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
    pub fn get_associatedAlphaSplitTexture(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture2D>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture2D>,
                        0usize,
                    >("get_associatedAlphaSplitTexture")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "get_associatedAlphaSplitTexture", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture2D> = unsafe {
            cordl_method_info.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_border(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector4> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), crate::UnityEngine::Vector4, 0usize>("get_border")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "get_border", 0usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::Vector4 = unsafe {
            cordl_method_info.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_border_Injected(
        &mut self,
        ret: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector4>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector4>),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("get_border_Injected")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "get_border_Injected", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (ret))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_bounds(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Bounds> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), crate::UnityEngine::Bounds, 0usize>("get_bounds")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "get_bounds", 0usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::Bounds = unsafe {
            cordl_method_info.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_bounds_Injected(
        &mut self,
        ret: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Bounds>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Bounds>),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("get_bounds_Injected")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "get_bounds_Injected", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (ret))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_packed(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), bool, 0usize>("get_packed")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "get_packed", 0usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_packingMode(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::SpritePackingMode> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        crate::UnityEngine::SpritePackingMode,
                        0usize,
                    >("get_packingMode")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "get_packingMode", 0usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::SpritePackingMode = unsafe {
            cordl_method_info.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_packingRotation(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::SpritePackingRotation> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        crate::UnityEngine::SpritePackingRotation,
                        0usize,
                    >("get_packingRotation")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "get_packingRotation", 0usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::SpritePackingRotation = unsafe {
            cordl_method_info.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_pivot(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector2> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), crate::UnityEngine::Vector2, 0usize>("get_pivot")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "get_pivot", 0usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::Vector2 = unsafe {
            cordl_method_info.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_pivot_Injected(
        &mut self,
        ret: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector2>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector2>),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("get_pivot_Injected")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "get_pivot_Injected", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (ret))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_pixelsPerUnit(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), f32, 0usize>("get_pixelsPerUnit")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "get_pixelsPerUnit", 0usize
                        )
                    })
            });
        let __cordl_ret: f32 = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_rect(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Rect> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), crate::UnityEngine::Rect, 0usize>("get_rect")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "get_rect", 0usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::Rect = unsafe {
            cordl_method_info.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_rect_Injected(
        &mut self,
        ret: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Rect>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Rect>),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("get_rect_Injected")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "get_rect_Injected", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (ret))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_spriteAtlasTextureScale(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), f32, 0usize>("get_spriteAtlasTextureScale")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "get_spriteAtlasTextureScale", 0usize
                        )
                    })
            });
        let __cordl_ret: f32 = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_texture(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture2D>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture2D>,
                        0usize,
                    >("get_texture")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "get_texture", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture2D> = unsafe {
            cordl_method_info.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_textureRect(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Rect> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        crate::UnityEngine::Rect,
                        0usize,
                    >("get_textureRect")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "get_textureRect", 0usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::Rect = unsafe {
            cordl_method_info.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_textureRectOffset(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector2> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        crate::UnityEngine::Vector2,
                        0usize,
                    >("get_textureRectOffset")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "get_textureRectOffset", 0usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::Vector2 = unsafe {
            cordl_method_info.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_triangles(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u16>>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        quest_hook::libil2cpp::Gc<
                            quest_hook::libil2cpp::Il2CppArray<u16>,
                        >,
                        0usize,
                    >("get_triangles")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "get_triangles", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<u16>,
        > = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_uv(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Vector2>,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        quest_hook::libil2cpp::Gc<
                            quest_hook::libil2cpp::Il2CppArray<
                                crate::UnityEngine::Vector2,
                            >,
                        >,
                        0usize,
                    >("get_uv")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "get_uv",
                            0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Vector2>,
        > = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_vertices(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Vector2>,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        quest_hook::libil2cpp::Gc<
                            quest_hook::libil2cpp::Il2CppArray<
                                crate::UnityEngine::Vector2,
                            >,
                        >,
                        0usize,
                    >("get_vertices")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "get_vertices", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Vector2>,
        > = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+Sprite")]
impl quest_hook::libil2cpp::ObjectType for crate::UnityEngine::Sprite {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
