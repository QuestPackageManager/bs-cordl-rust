#[cfg(feature = "UnityEngine+Screen")]
#[repr(C)]
#[derive(Debug)]
pub struct Screen {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "UnityEngine+Screen")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::Screen => "UnityEngine"."Screen"
);
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
        let __cordl_ret: crate::UnityEngine::ScreenOrientation = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetScreenOrientation", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn SetResolution_FullScreenMode_RefreshRate0(
        width: i32,
        height: i32,
        fullscreenMode: crate::UnityEngine::FullScreenMode,
        preferredRefreshRate: crate::UnityEngine::RefreshRate,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "SetResolution",
                (width, height, fullscreenMode, preferredRefreshRate),
            )?;
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
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "SetResolution_Injected",
                (width, height, fullscreenMode, preferredRefreshRate),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn SetResolution__cordl_bool2(
        width: i32,
        height: i32,
        fullscreen: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("SetResolution", (width, height, fullscreen))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetResolution__cordl_bool_i32_1(
        width: i32,
        height: i32,
        fullscreen: bool,
        preferredRefreshRate: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("SetResolution", (width, height, fullscreen, preferredRefreshRate))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_currentResolution() -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::Resolution,
    > {
        let __cordl_ret: crate::UnityEngine::Resolution = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_currentResolution", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_currentResolution_Injected(
        ret: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Resolution>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_currentResolution_Injected", (ret))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_dpi() -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_dpi", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_fullScreen() -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_fullScreen", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_height() -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_height", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_orientation() -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::ScreenOrientation,
    > {
        let __cordl_ret: crate::UnityEngine::ScreenOrientation = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_orientation", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_resolutions() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Resolution>,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Resolution>,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_resolutions", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_width() -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_width", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn set_fullScreen(
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("set_fullScreen", (value))?;
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
