#[cfg(feature = "Tweening+Vector2Tween")]
#[repr(C)]
#[derive(Debug)]
pub struct Vector2Tween {
    __cordl_parent: crate::Tweening::Tween_1<crate::UnityEngine::Vector2>,
}
#[cfg(feature = "Tweening+Vector2Tween")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Tweening::Vector2Tween => "Tweening"
    ."Vector2Tween"
);
#[cfg(feature = "Tweening+Vector2Tween")]
impl std::ops::Deref for crate::Tweening::Vector2Tween {
    type Target = crate::Tweening::Tween_1<crate::UnityEngine::Vector2>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Tweening+Vector2Tween")]
impl std::ops::DerefMut for crate::Tweening::Vector2Tween {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Tweening+Vector2Tween")]
impl crate::Tweening::Vector2Tween {
    pub fn GetValue(
        &mut self,
        t: f32,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector2> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Vector2 = __cordl_object
            .invoke("GetValue", (t))?;
        Ok(__cordl_ret)
    }
    pub fn New_0() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
    pub fn New_Vector2_Vector2_Action_1_f32_EaseType_f32_1(
        fromValue: crate::UnityEngine::Vector2,
        toValue: crate::UnityEngine::Vector2,
        onUpdate: *mut crate::System::Action_1<crate::UnityEngine::Vector2>,
        duration: f32,
        easeType: EaseType,
        delay: f32,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(
                ".ctor",
                (fromValue, toValue, onUpdate, duration, easeType, delay),
            )?;
        Ok(__cordl_object)
    }
    pub fn _ctor_0(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_Vector2_Vector2_Action_1_f32_EaseType_f32_1(
        &mut self,
        fromValue: crate::UnityEngine::Vector2,
        toValue: crate::UnityEngine::Vector2,
        onUpdate: *mut crate::System::Action_1<crate::UnityEngine::Vector2>,
        duration: f32,
        easeType: EaseType,
        delay: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (fromValue, toValue, onUpdate, duration, easeType, delay))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "Tweening+Vector2Tween")]
impl quest_hook::libil2cpp::ObjectType for crate::Tweening::Vector2Tween {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
