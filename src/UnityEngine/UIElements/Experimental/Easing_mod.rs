#[cfg(feature = "UnityEngine+UIElements+Experimental+Easing")]
#[repr(C)]
#[derive(Debug)]
pub struct Easing {
    __cordl_parent: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
}
#[cfg(feature = "UnityEngine+UIElements+Experimental+Easing")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::UIElements::Experimental::Easing =>
    "UnityEngine.UIElements.Experimental"."Easing"
);
#[cfg(feature = "UnityEngine+UIElements+Experimental+Easing")]
impl std::ops::Deref for crate::UnityEngine::UIElements::Experimental::Easing {
    type Target = quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+Experimental+Easing")]
impl std::ops::DerefMut for crate::UnityEngine::UIElements::Experimental::Easing {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+Experimental+Easing")]
impl crate::UnityEngine::UIElements::Experimental::Easing {
    pub fn InBack(t: f32) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("InBack", (t))?;
        Ok(__cordl_ret.into())
    }
    pub fn InBounce(t: f32) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("InBounce", (t))?;
        Ok(__cordl_ret.into())
    }
    pub fn InCirc(t: f32) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("InCirc", (t))?;
        Ok(__cordl_ret.into())
    }
    pub fn InCubic(t: f32) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("InCubic", (t))?;
        Ok(__cordl_ret.into())
    }
    pub fn InElastic(t: f32) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("InElastic", (t))?;
        Ok(__cordl_ret.into())
    }
    pub fn InOutBack(t: f32) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("InOutBack", (t))?;
        Ok(__cordl_ret.into())
    }
    pub fn InOutBounce(t: f32) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("InOutBounce", (t))?;
        Ok(__cordl_ret.into())
    }
    pub fn InOutCirc(t: f32) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("InOutCirc", (t))?;
        Ok(__cordl_ret.into())
    }
    pub fn InOutCubic(t: f32) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("InOutCubic", (t))?;
        Ok(__cordl_ret.into())
    }
    pub fn InOutElastic(t: f32) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("InOutElastic", (t))?;
        Ok(__cordl_ret.into())
    }
    pub fn InOutPower(t: f32, power: i32) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("InOutPower", (t, power))?;
        Ok(__cordl_ret.into())
    }
    pub fn InOutQuad(t: f32) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("InOutQuad", (t))?;
        Ok(__cordl_ret.into())
    }
    pub fn InOutSine(t: f32) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("InOutSine", (t))?;
        Ok(__cordl_ret.into())
    }
    pub fn InPower(t: f32, power: i32) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("InPower", (t, power))?;
        Ok(__cordl_ret.into())
    }
    pub fn InQuad(t: f32) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("InQuad", (t))?;
        Ok(__cordl_ret.into())
    }
    pub fn InSine(t: f32) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("InSine", (t))?;
        Ok(__cordl_ret.into())
    }
    pub fn Linear(t: f32) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Linear", (t))?;
        Ok(__cordl_ret.into())
    }
    pub fn OutBack(t: f32) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("OutBack", (t))?;
        Ok(__cordl_ret.into())
    }
    pub fn OutBounce(t: f32) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("OutBounce", (t))?;
        Ok(__cordl_ret.into())
    }
    pub fn OutCirc(t: f32) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("OutCirc", (t))?;
        Ok(__cordl_ret.into())
    }
    pub fn OutCubic(t: f32) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("OutCubic", (t))?;
        Ok(__cordl_ret.into())
    }
    pub fn OutElastic(t: f32) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("OutElastic", (t))?;
        Ok(__cordl_ret.into())
    }
    pub fn OutPower(t: f32, power: i32) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("OutPower", (t, power))?;
        Ok(__cordl_ret.into())
    }
    pub fn OutQuad(t: f32) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("OutQuad", (t))?;
        Ok(__cordl_ret.into())
    }
    pub fn OutSine(t: f32) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("OutSine", (t))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+UIElements+Experimental+Easing")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::UIElements::Experimental::Easing {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
