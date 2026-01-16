#[cfg(feature = "cordl_class_UnityEngine+Rendering+TileLayoutUtils")]
#[repr(C)]
#[derive(Debug)]
pub struct TileLayoutUtils {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "cordl_class_UnityEngine+Rendering+TileLayoutUtils")]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::Rendering::TileLayoutUtils {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine.Rendering";
    const CLASS_NAME: &'static str = "TileLayoutUtils";
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
#[cfg(feature = "UnityEngine+Rendering+TileLayoutUtils")]
impl std::ops::Deref for crate::UnityEngine::Rendering::TileLayoutUtils {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Rendering+TileLayoutUtils")]
impl std::ops::DerefMut for crate::UnityEngine::Rendering::TileLayoutUtils {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Rendering+TileLayoutUtils")]
impl crate::UnityEngine::Rendering::TileLayoutUtils {
    pub fn TryLayoutByCol(
        src: crate::UnityEngine::RectInt,
        tileSize: u32,
        main: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::RectInt>,
        other: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::RectInt>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            crate::UnityEngine::RectInt,
                            u32,
                            quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::RectInt>,
                            quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::RectInt>,
                        ),
                        bool,
                        4usize,
                    >("TryLayoutByCol")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "TryLayoutByCol", 4usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            cordl_method_info.invoke_unchecked((), (src, tileSize, main, other))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn TryLayoutByRow(
        src: crate::UnityEngine::RectInt,
        tileSize: u32,
        main: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::RectInt>,
        other: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::RectInt>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            crate::UnityEngine::RectInt,
                            u32,
                            quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::RectInt>,
                            quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::RectInt>,
                        ),
                        bool,
                        4usize,
                    >("TryLayoutByRow")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "TryLayoutByRow", 4usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            cordl_method_info.invoke_unchecked((), (src, tileSize, main, other))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn TryLayoutByTiles(
        src: crate::UnityEngine::RectInt,
        tileSize: u32,
        main: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::RectInt>,
        topRow: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::RectInt>,
        rightCol: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::RectInt>,
        topRight: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::RectInt>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            crate::UnityEngine::RectInt,
                            u32,
                            quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::RectInt>,
                            quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::RectInt>,
                            quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::RectInt>,
                            quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::RectInt>,
                        ),
                        bool,
                        6usize,
                    >("TryLayoutByTiles")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "TryLayoutByTiles", 6usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            cordl_method_info
                .invoke_unchecked((), (src, tileSize, main, topRow, rightCol, topRight))?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "cordl_class_UnityEngine+Rendering+TileLayoutUtils")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::Rendering::TileLayoutUtils {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
