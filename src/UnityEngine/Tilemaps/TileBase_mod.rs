#[cfg(feature = "UnityEngine+Tilemaps+TileBase")]
#[repr(C)]
#[derive(Debug)]
pub struct TileBase {
    __cordl_parent: crate::UnityEngine::ScriptableObject,
}
#[cfg(feature = "UnityEngine+Tilemaps+TileBase")]
unsafe impl quest_hook::libil2cpp::Type for crate::UnityEngine::Tilemaps::TileBase {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine.Tilemaps";
    const CLASS_NAME: &'static str = "TileBase";
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
#[cfg(feature = "UnityEngine+Tilemaps+TileBase")]
impl std::ops::Deref for crate::UnityEngine::Tilemaps::TileBase {
    type Target = crate::UnityEngine::ScriptableObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Tilemaps+TileBase")]
impl std::ops::DerefMut for crate::UnityEngine::Tilemaps::TileBase {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Tilemaps+TileBase")]
impl crate::UnityEngine::Tilemaps::TileBase {
    pub fn GetTileAnimationData(
        &mut self,
        position: crate::UnityEngine::Vector3Int,
        tilemap: quest_hook::libil2cpp::Gc<crate::UnityEngine::Tilemaps::ITilemap>,
        tileAnimationData: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::Tilemaps::TileAnimationData,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (
                            crate::UnityEngine::Vector3Int,
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::Tilemaps::ITilemap,
                            >,
                            quest_hook::libil2cpp::ByRefMut<
                                crate::UnityEngine::Tilemaps::TileAnimationData,
                            >,
                        ),
                        bool,
                        3usize,
                    >("GetTileAnimationData")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "GetTileAnimationData", 3usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            method.invoke_unchecked(self, (position, tilemap, tileAnimationData))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetTileAnimationDataNoRef(
        &mut self,
        position: crate::UnityEngine::Vector3Int,
        tilemap: quest_hook::libil2cpp::Gc<crate::UnityEngine::Tilemaps::ITilemap>,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Tilemaps::TileAnimationData> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (
                            crate::UnityEngine::Vector3Int,
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::Tilemaps::ITilemap,
                            >,
                        ),
                        crate::UnityEngine::Tilemaps::TileAnimationData,
                        2usize,
                    >("GetTileAnimationDataNoRef")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "GetTileAnimationDataNoRef", 2usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::Tilemaps::TileAnimationData = unsafe {
            method.invoke_unchecked(self, (position, tilemap))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetTileAnimationDataRef(
        &mut self,
        position: crate::UnityEngine::Vector3Int,
        tilemap: quest_hook::libil2cpp::Gc<crate::UnityEngine::Tilemaps::ITilemap>,
        tileAnimationData: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::Tilemaps::TileAnimationData,
        >,
        hasAnimation: quest_hook::libil2cpp::ByRefMut<bool>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (
                            crate::UnityEngine::Vector3Int,
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::Tilemaps::ITilemap,
                            >,
                            quest_hook::libil2cpp::ByRefMut<
                                crate::UnityEngine::Tilemaps::TileAnimationData,
                            >,
                            quest_hook::libil2cpp::ByRefMut<bool>,
                        ),
                        quest_hook::libil2cpp::Void,
                        4usize,
                    >("GetTileAnimationDataRef")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "GetTileAnimationDataRef", 4usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method
                .invoke_unchecked(
                    self,
                    (position, tilemap, tileAnimationData, hasAnimation),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetTileData(
        &mut self,
        position: crate::UnityEngine::Vector3Int,
        tilemap: quest_hook::libil2cpp::Gc<crate::UnityEngine::Tilemaps::ITilemap>,
        tileData: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Tilemaps::TileData>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (
                            crate::UnityEngine::Vector3Int,
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::Tilemaps::ITilemap,
                            >,
                            quest_hook::libil2cpp::ByRefMut<
                                crate::UnityEngine::Tilemaps::TileData,
                            >,
                        ),
                        quest_hook::libil2cpp::Void,
                        3usize,
                    >("GetTileData")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "GetTileData", 3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (position, tilemap, tileData))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetTileDataNoRef(
        &mut self,
        position: crate::UnityEngine::Vector3Int,
        tilemap: quest_hook::libil2cpp::Gc<crate::UnityEngine::Tilemaps::ITilemap>,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Tilemaps::TileData> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (
                            crate::UnityEngine::Vector3Int,
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::Tilemaps::ITilemap,
                            >,
                        ),
                        crate::UnityEngine::Tilemaps::TileData,
                        2usize,
                    >("GetTileDataNoRef")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "GetTileDataNoRef", 2usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::Tilemaps::TileData = unsafe {
            method.invoke_unchecked(self, (position, tilemap))?
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
    pub fn RefreshTile(
        &mut self,
        position: crate::UnityEngine::Vector3Int,
        tilemap: quest_hook::libil2cpp::Gc<crate::UnityEngine::Tilemaps::ITilemap>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (
                            crate::UnityEngine::Vector3Int,
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::Tilemaps::ITilemap,
                            >,
                        ),
                        quest_hook::libil2cpp::Void,
                        2usize,
                    >("RefreshTile")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "RefreshTile", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (position, tilemap))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn StartUp(
        &mut self,
        position: crate::UnityEngine::Vector3Int,
        tilemap: quest_hook::libil2cpp::Gc<crate::UnityEngine::Tilemaps::ITilemap>,
        go: quest_hook::libil2cpp::Gc<crate::UnityEngine::GameObject>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (
                            crate::UnityEngine::Vector3Int,
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::Tilemaps::ITilemap,
                            >,
                            quest_hook::libil2cpp::Gc<crate::UnityEngine::GameObject>,
                        ),
                        bool,
                        3usize,
                    >("StartUp")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "StartUp", 3usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            method.invoke_unchecked(self, (position, tilemap, go))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn StartUpRef(
        &mut self,
        position: crate::UnityEngine::Vector3Int,
        tilemap: quest_hook::libil2cpp::Gc<crate::UnityEngine::Tilemaps::ITilemap>,
        go: quest_hook::libil2cpp::Gc<crate::UnityEngine::GameObject>,
        startUpInvokedByUser: quest_hook::libil2cpp::ByRefMut<bool>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (
                            crate::UnityEngine::Vector3Int,
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::Tilemaps::ITilemap,
                            >,
                            quest_hook::libil2cpp::Gc<crate::UnityEngine::GameObject>,
                            quest_hook::libil2cpp::ByRefMut<bool>,
                        ),
                        quest_hook::libil2cpp::Void,
                        4usize,
                    >("StartUpRef")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "StartUpRef", 4usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (position, tilemap, go, startUpInvokedByUser))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<(), quest_hook::libil2cpp::Void, 0usize>(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), ".ctor", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+Tilemaps+TileBase")]
impl quest_hook::libil2cpp::ObjectType for crate::UnityEngine::Tilemaps::TileBase {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
