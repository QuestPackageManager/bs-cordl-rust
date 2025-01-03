#[cfg(feature = "TMPro+ColorTween")]
#[repr(C)]
#[derive(Debug, Clone, Default)]
pub struct ColorTween {
    pub m_Target: quest_hook::libil2cpp::Gc<crate::TMPro::ColorTween_ColorTweenCallback>,
    pub m_StartColor: crate::UnityEngine::Color,
    pub m_TargetColor: crate::UnityEngine::Color,
    pub m_TweenMode: crate::TMPro::ColorTween_ColorTweenMode,
    pub m_Duration: f32,
    pub m_IgnoreTimeScale: bool,
}
#[cfg(feature = "TMPro+ColorTween")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::TMPro::ColorTween => "TMPro"."ColorTween"
);
#[cfg(feature = "TMPro+ColorTween")]
unsafe impl quest_hook::libil2cpp::ThisArgument for crate::TMPro::ColorTween {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "TMPro+ColorTween")]
impl crate::TMPro::ColorTween {
    #[cfg(feature = "TMPro+ColorTween+ColorTweenCallback")]
    pub type ColorTweenCallback = crate::TMPro::ColorTween_ColorTweenCallback;
    #[cfg(feature = "TMPro+ColorTween+ColorTweenMode")]
    pub type ColorTweenMode = crate::TMPro::ColorTween_ColorTweenMode;
    pub fn AddOnChangedCallback(
        &mut self,
        callback: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Events::UnityAction_1<crate::UnityEngine::Color>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "AddOnChangedCallback",
            (callback),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn GetDuration(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "GetDuration",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn GetIgnoreTimescale(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "GetIgnoreTimescale",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn TweenValue(
        &mut self,
        floatPercentage: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "TweenValue",
            (floatPercentage),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn ValidTarget(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "ValidTarget",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_duration(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_duration",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_ignoreTimeScale(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_ignoreTimeScale",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_startColor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Color> {
        let __cordl_ret: crate::UnityEngine::Color = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_startColor",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_targetColor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Color> {
        let __cordl_ret: crate::UnityEngine::Color = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_targetColor",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_tweenMode(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::TMPro::ColorTween_ColorTweenMode> {
        let __cordl_ret: crate::TMPro::ColorTween_ColorTweenMode = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_tweenMode",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn set_duration(
        &mut self,
        value: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_duration",
            (value),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn set_ignoreTimeScale(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_ignoreTimeScale",
            (value),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn set_startColor(
        &mut self,
        value: crate::UnityEngine::Color,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_startColor",
            (value),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn set_targetColor(
        &mut self,
        value: crate::UnityEngine::Color,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_targetColor",
            (value),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn set_tweenMode(
        &mut self,
        value: crate::TMPro::ColorTween_ColorTweenMode,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_tweenMode",
            (value),
        )?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "TMPro+ColorTween")]
impl AsRef<crate::TMPro::ITweenValue> for crate::TMPro::ColorTween {
    fn as_ref(&self) -> &crate::TMPro::ITweenValue {
        todo!()
    }
}
#[cfg(feature = "TMPro+ColorTween")]
impl AsMut<crate::TMPro::ITweenValue> for crate::TMPro::ColorTween {
    fn as_mut(&mut self) -> &mut crate::TMPro::ITweenValue {
        todo!()
    }
}
#[cfg(feature = "TMPro+ColorTween+ColorTweenCallback")]
#[repr(C)]
#[derive(Debug)]
pub struct ColorTween_ColorTweenCallback {
    __cordl_parent: crate::UnityEngine::Events::UnityEvent_1<crate::UnityEngine::Color>,
}
#[cfg(feature = "TMPro+ColorTween+ColorTweenCallback")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::TMPro::ColorTween_ColorTweenCallback => "TMPro"
    ."ColorTween/ColorTweenCallback"
);
#[cfg(feature = "TMPro+ColorTween+ColorTweenCallback")]
impl std::ops::Deref for crate::TMPro::ColorTween_ColorTweenCallback {
    type Target = crate::UnityEngine::Events::UnityEvent_1<crate::UnityEngine::Color>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "TMPro+ColorTween+ColorTweenCallback")]
impl std::ops::DerefMut for crate::TMPro::ColorTween_ColorTweenCallback {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "TMPro+ColorTween+ColorTweenCallback")]
impl crate::TMPro::ColorTween_ColorTweenCallback {
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "TMPro+ColorTween+ColorTweenCallback")]
impl quest_hook::libil2cpp::ObjectType for crate::TMPro::ColorTween_ColorTweenCallback {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "TMPro+ColorTween+ColorTweenMode")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ColorTween_ColorTweenMode {
    All = 0i32,
    Alpha = 2i32,
    RGB = 1i32,
}
#[cfg(feature = "TMPro+ColorTween+ColorTweenMode")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::TMPro::ColorTween_ColorTweenMode => "TMPro"
    ."ColorTween/ColorTweenMode"
);
