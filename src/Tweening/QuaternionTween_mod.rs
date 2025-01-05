#[cfg(feature = "Tweening+QuaternionTween")]
#[repr(C)]
#[derive(Debug)]
pub struct QuaternionTween {
    __cordl_parent: crate::Tweening::Tween_1<crate::UnityEngine::Quaternion>,
}
#[cfg(feature = "Tweening+QuaternionTween")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Tweening::QuaternionTween => "Tweening"
    ."QuaternionTween"
);
#[cfg(feature = "Tweening+QuaternionTween")]
impl std::ops::Deref for crate::Tweening::QuaternionTween {
    type Target = crate::Tweening::Tween_1<crate::UnityEngine::Quaternion>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Tweening+QuaternionTween")]
impl std::ops::DerefMut for crate::Tweening::QuaternionTween {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Tweening+QuaternionTween")]
impl crate::Tweening::QuaternionTween {
    pub fn GetValue(
        &mut self,
        t: f32,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Quaternion> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Quaternion = __cordl_object
            .invoke("GetValue", (t))?;
        Ok(__cordl_ret.into())
    }
    pub fn New_0() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn New_Quaternion_Quaternion_Action_1_f32_EaseType_f32_1(
        fromValue: crate::UnityEngine::Quaternion,
        toValue: crate::UnityEngine::Quaternion,
        onUpdate: quest_hook::libil2cpp::Gc<
            crate::System::Action_1<crate::UnityEngine::Quaternion>,
        >,
        duration: f32,
        easeType: crate::GlobalNamespace::EaseType,
        delay: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(
                ".ctor",
                (fromValue, toValue, onUpdate, duration, easeType, delay),
            )?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor_0(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_Quaternion_Quaternion_Action_1_f32_EaseType_f32_1(
        &mut self,
        fromValue: crate::UnityEngine::Quaternion,
        toValue: crate::UnityEngine::Quaternion,
        onUpdate: quest_hook::libil2cpp::Gc<
            crate::System::Action_1<crate::UnityEngine::Quaternion>,
        >,
        duration: f32,
        easeType: crate::GlobalNamespace::EaseType,
        delay: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (fromValue, toValue, onUpdate, duration, easeType, delay))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Tweening+QuaternionTween")]
impl quest_hook::libil2cpp::ObjectType for crate::Tweening::QuaternionTween {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
