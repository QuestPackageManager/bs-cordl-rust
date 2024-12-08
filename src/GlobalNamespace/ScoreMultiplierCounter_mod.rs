#[cfg(feature = "ScoreMultiplierCounter+MultiplierEventType")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ScoreMultiplierCounter_MultiplierEventType {
    Negative = 2i32,
    Neutral = 1i32,
    Positive = 0i32,
}
#[cfg(feature = "ScoreMultiplierCounter+MultiplierEventType")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::ScoreMultiplierCounter_MultiplierEventType => ""
    ."ScoreMultiplierCounter/MultiplierEventType"
);
#[cfg(feature = "ScoreMultiplierCounter")]
#[repr(C)]
#[derive(Debug)]
pub struct ScoreMultiplierCounter {
    __cordl_parent: crate::System::Object,
    pub _multiplier: i32,
    pub _multiplierIncreaseProgress: i32,
    pub _multiplierIncreaseMaxProgress: i32,
}
#[cfg(feature = "ScoreMultiplierCounter")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for ScoreMultiplierCounter => ""."ScoreMultiplierCounter"
);
#[cfg(feature = "ScoreMultiplierCounter")]
impl std::ops::Deref for ScoreMultiplierCounter {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "ScoreMultiplierCounter")]
impl std::ops::DerefMut for ScoreMultiplierCounter {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "ScoreMultiplierCounter")]
impl ScoreMultiplierCounter {
    #[cfg(feature = "ScoreMultiplierCounter+MultiplierEventType")]
    pub type MultiplierEventType = crate::GlobalNamespace::ScoreMultiplierCounter_MultiplierEventType;
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
    pub fn ProcessMultiplierEvent(
        &mut self,
        multiplierEventType: crate::GlobalNamespace::ScoreMultiplierCounter_MultiplierEventType,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("ProcessMultiplierEvent", (multiplierEventType))?;
        Ok(__cordl_ret)
    }
    pub fn Reset(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Reset", ())?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_multiplier(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_multiplier", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_normalizedProgress(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("get_normalizedProgress", ())?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "ScoreMultiplierCounter")]
impl quest_hook::libil2cpp::ObjectType for ScoreMultiplierCounter {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
