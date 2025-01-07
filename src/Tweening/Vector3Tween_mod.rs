#[cfg(feature = "Tweening+Vector3Tween")]
#[repr(C)]
#[derive(Debug)]
pub struct Vector3Tween {
    __cordl_parent: crate::Tweening::Tween_1<crate::UnityEngine::Vector3>,
}
#[cfg(feature = "Tweening+Vector3Tween")]
unsafe impl quest_hook::libil2cpp::Type for crate::Tweening::Vector3Tween {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "Tweening";
    const CLASS_NAME: &'static str = "Vector3Tween";
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
#[cfg(feature = "Tweening+Vector3Tween")]
impl std::ops::Deref for crate::Tweening::Vector3Tween {
    type Target = crate::Tweening::Tween_1<crate::UnityEngine::Vector3>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Tweening+Vector3Tween")]
impl std::ops::DerefMut for crate::Tweening::Vector3Tween {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Tweening+Vector3Tween")]
impl crate::Tweening::Vector3Tween {
    pub fn GetValue(
        &mut self,
        t: f32,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector3> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Vector3 = __cordl_object
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
    pub fn New_Vector3_Vector3_Action_1_f32_EaseType_f32_1(
        fromValue: crate::UnityEngine::Vector3,
        toValue: crate::UnityEngine::Vector3,
        onUpdate: quest_hook::libil2cpp::Gc<
            crate::System::Action_1<crate::UnityEngine::Vector3>,
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
    pub fn _ctor_Vector3_Vector3_Action_1_f32_EaseType_f32_1(
        &mut self,
        fromValue: crate::UnityEngine::Vector3,
        toValue: crate::UnityEngine::Vector3,
        onUpdate: quest_hook::libil2cpp::Gc<
            crate::System::Action_1<crate::UnityEngine::Vector3>,
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
#[cfg(feature = "Tweening+Vector3Tween")]
impl quest_hook::libil2cpp::ObjectType for crate::Tweening::Vector3Tween {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
