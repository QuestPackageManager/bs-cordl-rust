#[cfg(feature = "Tweening+IntTween")]
#[repr(C)]
#[derive(Debug)]
pub struct IntTween {
    __cordl_parent: crate::Tweening::Tween_1<i32>,
    pub _roundingType: crate::Tweening::IntTween_RoundingType,
}
#[cfg(feature = "Tweening+IntTween")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Tweening::IntTween => "Tweening"."IntTween"
);
#[cfg(feature = "Tweening+IntTween")]
impl std::ops::Deref for crate::Tweening::IntTween {
    type Target = crate::Tweening::Tween_1<i32>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Tweening+IntTween")]
impl std::ops::DerefMut for crate::Tweening::IntTween {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Tweening+IntTween")]
impl crate::Tweening::IntTween {
    #[cfg(feature = "Tweening+IntTween+RoundingType")]
    pub type RoundingType = crate::Tweening::IntTween_RoundingType;
    pub fn GetValue(&mut self, t: f32) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("GetValue", (t))?;
        Ok(__cordl_ret.into())
    }
    pub fn New_0() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn New_i32_i32_Action_1_f32_EaseType_IntTween_RoundingType_f32_1(
        fromValue: i32,
        toValue: i32,
        onUpdate: quest_hook::libil2cpp::Gc<crate::System::Action_1<i32>>,
        duration: f32,
        easeType: crate::GlobalNamespace::EaseType,
        roundingType: crate::Tweening::IntTween_RoundingType,
        delay: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(
                ".ctor",
                (fromValue, toValue, onUpdate, duration, easeType, roundingType, delay),
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
    pub fn _ctor_i32_i32_Action_1_f32_EaseType_IntTween_RoundingType_f32_1(
        &mut self,
        fromValue: i32,
        toValue: i32,
        onUpdate: quest_hook::libil2cpp::Gc<crate::System::Action_1<i32>>,
        duration: f32,
        easeType: crate::GlobalNamespace::EaseType,
        roundingType: crate::Tweening::IntTween_RoundingType,
        delay: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                ".ctor",
                (fromValue, toValue, onUpdate, duration, easeType, roundingType, delay),
            )?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Tweening+IntTween")]
impl quest_hook::libil2cpp::ObjectType for crate::Tweening::IntTween {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "Tweening+IntTween+RoundingType")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum IntTween_RoundingType {
    #[default]
    Ceil = 2i32,
    Floor = 0i32,
    Round = 1i32,
}
#[cfg(feature = "Tweening+IntTween+RoundingType")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::Tweening::IntTween_RoundingType => "Tweening"
    ."IntTween/RoundingType"
);
