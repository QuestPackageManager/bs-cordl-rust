#[cfg(feature = "UnityEngine+Screen")]
#[repr(C)]
#[derive(Debug)]
pub struct Screen {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "UnityEngine+Screen")]
unsafe impl quest_hook::libil2cpp::Type for crate::UnityEngine::Screen {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine";
    const CLASS_NAME: &'static str = "Screen";
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
#[cfg(feature = "UnityEngine+Screen")]
impl std::ops::Deref for crate::UnityEngine::Screen {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Screen")]
impl std::ops::DerefMut for crate::UnityEngine::Screen {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Screen")]
impl crate::UnityEngine::Screen {
    pub fn GetScreenOrientation() -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::ScreenOrientation,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::Screen as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (),
                crate::UnityEngine::ScreenOrientation,
                0usize,
            >("GetScreenOrientation")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::Screen as quest_hook::libil2cpp::Type >
                    ::class(), "GetScreenOrientation", 0usize
                )
            });
        let __cordl_ret: crate::UnityEngine::ScreenOrientation = unsafe {
            method.invoke_unchecked((), ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn SetResolution_FullScreenMode_RefreshRate0(
        width: i32,
        height: i32,
        fullscreenMode: crate::UnityEngine::FullScreenMode,
        preferredRefreshRate: crate::UnityEngine::RefreshRate,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::Screen as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    i32,
                    i32,
                    crate::UnityEngine::FullScreenMode,
                    crate::UnityEngine::RefreshRate,
                ),
                quest_hook::libil2cpp::Void,
                4usize,
            >("SetResolution")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::Screen as quest_hook::libil2cpp::Type >
                    ::class(), "SetResolution", 4usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method
                .invoke_unchecked(
                    (),
                    (width, height, fullscreenMode, preferredRefreshRate),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn SetResolution_Injected(
        width: i32,
        height: i32,
        fullscreenMode: crate::UnityEngine::FullScreenMode,
        preferredRefreshRate: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::RefreshRate,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::Screen as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    i32,
                    i32,
                    crate::UnityEngine::FullScreenMode,
                    quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::RefreshRate>,
                ),
                quest_hook::libil2cpp::Void,
                4usize,
            >("SetResolution_Injected")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::Screen as quest_hook::libil2cpp::Type >
                    ::class(), "SetResolution_Injected", 4usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method
                .invoke_unchecked(
                    (),
                    (width, height, fullscreenMode, preferredRefreshRate),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn SetResolution__cordl_bool2(
        width: i32,
        height: i32,
        fullscreen: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::Screen as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (i32, i32, bool),
                quest_hook::libil2cpp::Void,
                3usize,
            >("SetResolution")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::Screen as quest_hook::libil2cpp::Type >
                    ::class(), "SetResolution", 3usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (width, height, fullscreen))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn SetResolution__cordl_bool_i32_1(
        width: i32,
        height: i32,
        fullscreen: bool,
        preferredRefreshRate: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::Screen as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (i32, i32, bool, i32),
                quest_hook::libil2cpp::Void,
                4usize,
            >("SetResolution")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::Screen as quest_hook::libil2cpp::Type >
                    ::class(), "SetResolution", 4usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method
                .invoke_unchecked((), (width, height, fullscreen, preferredRefreshRate))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_currentResolution() -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::Resolution,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::Screen as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (),
                crate::UnityEngine::Resolution,
                0usize,
            >("get_currentResolution")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::Screen as quest_hook::libil2cpp::Type >
                    ::class(), "get_currentResolution", 0usize
                )
            });
        let __cordl_ret: crate::UnityEngine::Resolution = unsafe {
            method.invoke_unchecked((), ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_currentResolution_Injected(
        ret: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Resolution>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::Screen as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Resolution>),
                quest_hook::libil2cpp::Void,
                1usize,
            >("get_currentResolution_Injected")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::Screen as quest_hook::libil2cpp::Type >
                    ::class(), "get_currentResolution_Injected", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (ret))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_dpi() -> quest_hook::libil2cpp::Result<f32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::Screen as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<(), f32, 0usize>("get_dpi")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::Screen as quest_hook::libil2cpp::Type >
                    ::class(), "get_dpi", 0usize
                )
            });
        let __cordl_ret: f32 = unsafe { method.invoke_unchecked((), ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_fullScreen() -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::Screen as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<(), bool, 0usize>("get_fullScreen")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::Screen as quest_hook::libil2cpp::Type >
                    ::class(), "get_fullScreen", 0usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked((), ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_height() -> quest_hook::libil2cpp::Result<i32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::Screen as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<(), i32, 0usize>("get_height")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::Screen as quest_hook::libil2cpp::Type >
                    ::class(), "get_height", 0usize
                )
            });
        let __cordl_ret: i32 = unsafe { method.invoke_unchecked((), ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_orientation() -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::ScreenOrientation,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::Screen as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (),
                crate::UnityEngine::ScreenOrientation,
                0usize,
            >("get_orientation")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::Screen as quest_hook::libil2cpp::Type >
                    ::class(), "get_orientation", 0usize
                )
            });
        let __cordl_ret: crate::UnityEngine::ScreenOrientation = unsafe {
            method.invoke_unchecked((), ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_resolutions() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Resolution>,
        >,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::Screen as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (),
                quest_hook::libil2cpp::Gc<
                    quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Resolution>,
                >,
                0usize,
            >("get_resolutions")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::Screen as quest_hook::libil2cpp::Type >
                    ::class(), "get_resolutions", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Resolution>,
        > = unsafe { method.invoke_unchecked((), ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_width() -> quest_hook::libil2cpp::Result<i32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::Screen as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<(), i32, 0usize>("get_width")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::Screen as quest_hook::libil2cpp::Type >
                    ::class(), "get_width", 0usize
                )
            });
        let __cordl_ret: i32 = unsafe { method.invoke_unchecked((), ())? };
        Ok(__cordl_ret.into())
    }
    pub fn set_fullScreen(
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::Screen as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (bool),
                quest_hook::libil2cpp::Void,
                1usize,
            >("set_fullScreen")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::Screen as quest_hook::libil2cpp::Type >
                    ::class(), "set_fullScreen", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (value))?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+Screen")]
impl quest_hook::libil2cpp::ObjectType for crate::UnityEngine::Screen {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
