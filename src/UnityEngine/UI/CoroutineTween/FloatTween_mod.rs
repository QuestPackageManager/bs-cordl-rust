#[cfg(feature = "UnityEngine+UI+CoroutineTween+FloatTween")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct FloatTween {
    pub m_Target: *mut crate::UnityEngine::UI::CoroutineTween::FloatTween_FloatTweenCallback,
    pub m_StartValue: f32,
    pub m_TargetValue: f32,
    pub m_Duration: f32,
    pub m_IgnoreTimeScale: bool,
}
#[cfg(feature = "UnityEngine+UI+CoroutineTween+FloatTween")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::UI::CoroutineTween::FloatTween =>
    "UnityEngine.UI.CoroutineTween"."FloatTween"
);
#[cfg(feature = "UnityEngine+UI+CoroutineTween+FloatTween")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::UI::CoroutineTween::FloatTween {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+UI+CoroutineTween+FloatTween")]
impl crate::UnityEngine::UI::CoroutineTween::FloatTween {
    #[cfg(feature = "UnityEngine+UI+CoroutineTween+FloatTween+FloatTweenCallback")]
    pub type FloatTweenCallback = crate::UnityEngine::UI::CoroutineTween::FloatTween_FloatTweenCallback;
    pub fn AddOnChangedCallback(
        &mut self,
        callback: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Events::UnityAction_1<f32>,
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
    pub fn get_startValue(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_startValue",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_targetValue(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_targetValue",
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
    pub fn set_startValue(
        &mut self,
        value: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_startValue",
            (value),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn set_targetValue(
        &mut self,
        value: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_targetValue",
            (value),
        )?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+UI+CoroutineTween+FloatTween")]
impl AsRef<crate::UnityEngine::UI::CoroutineTween::ITweenValue>
for crate::UnityEngine::UI::CoroutineTween::FloatTween {
    fn as_ref(&self) -> &crate::UnityEngine::UI::CoroutineTween::ITweenValue {
        todo!()
    }
}
#[cfg(feature = "UnityEngine+UI+CoroutineTween+FloatTween")]
impl AsMut<crate::UnityEngine::UI::CoroutineTween::ITweenValue>
for crate::UnityEngine::UI::CoroutineTween::FloatTween {
    fn as_mut(&mut self) -> &mut crate::UnityEngine::UI::CoroutineTween::ITweenValue {
        todo!()
    }
}
#[cfg(feature = "UnityEngine+UI+CoroutineTween+FloatTween+FloatTweenCallback")]
#[repr(C)]
#[derive(Debug)]
pub struct FloatTween_FloatTweenCallback {
    __cordl_parent: crate::UnityEngine::Events::UnityEvent_1<f32>,
}
#[cfg(feature = "UnityEngine+UI+CoroutineTween+FloatTween+FloatTweenCallback")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::UI::CoroutineTween::FloatTween_FloatTweenCallback =>
    "UnityEngine.UI.CoroutineTween"."FloatTween/FloatTweenCallback"
);
#[cfg(feature = "UnityEngine+UI+CoroutineTween+FloatTween+FloatTweenCallback")]
impl std::ops::Deref
for crate::UnityEngine::UI::CoroutineTween::FloatTween_FloatTweenCallback {
    type Target = crate::UnityEngine::Events::UnityEvent_1<f32>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UI+CoroutineTween+FloatTween+FloatTweenCallback")]
impl std::ops::DerefMut
for crate::UnityEngine::UI::CoroutineTween::FloatTween_FloatTweenCallback {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UI+CoroutineTween+FloatTween+FloatTweenCallback")]
impl crate::UnityEngine::UI::CoroutineTween::FloatTween_FloatTweenCallback {
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
#[cfg(feature = "UnityEngine+UI+CoroutineTween+FloatTween+FloatTweenCallback")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::UI::CoroutineTween::FloatTween_FloatTweenCallback {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
