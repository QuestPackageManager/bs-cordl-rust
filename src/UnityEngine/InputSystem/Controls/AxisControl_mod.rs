#[cfg(feature = "UnityEngine+InputSystem+Controls+AxisControl")]
#[repr(C)]
#[derive(Debug)]
pub struct AxisControl {
    __cordl_parent: crate::UnityEngine::InputSystem::InputControl_1<f32>,
    pub clamp: crate::UnityEngine::InputSystem::Controls::AxisControl_Clamp,
    pub clampMin: f32,
    pub clampMax: f32,
    pub clampConstant: f32,
    pub invert: bool,
    pub normalize: bool,
    pub normalizeMin: f32,
    pub normalizeMax: f32,
    pub normalizeZero: f32,
    pub scale: bool,
    pub scaleFactor: f32,
}
#[cfg(feature = "UnityEngine+InputSystem+Controls+AxisControl")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::InputSystem::Controls::AxisControl
    => "UnityEngine.InputSystem.Controls"."AxisControl"
);
#[cfg(feature = "UnityEngine+InputSystem+Controls+AxisControl")]
impl std::ops::Deref for crate::UnityEngine::InputSystem::Controls::AxisControl {
    type Target = crate::UnityEngine::InputSystem::InputControl_1<f32>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+Controls+AxisControl")]
impl std::ops::DerefMut for crate::UnityEngine::InputSystem::Controls::AxisControl {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+Controls+AxisControl")]
impl crate::UnityEngine::InputSystem::Controls::AxisControl {
    #[cfg(feature = "UnityEngine+InputSystem+Controls+AxisControl+Clamp")]
    pub type Clamp = crate::UnityEngine::InputSystem::Controls::AxisControl_Clamp;
    pub fn Preprocess(&mut self, value: f32) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("Preprocess", (value))?;
        Ok(__cordl_ret)
    }
    pub fn Unpreprocess(&mut self, value: f32) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("Unpreprocess", (value))?;
        Ok(__cordl_ret)
    }
    pub fn ReadUnprocessedValueFromState(
        &mut self,
        statePtr: *mut quest_hook::libil2cpp::Il2CppObject,
    ) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object
            .invoke("ReadUnprocessedValueFromState", (statePtr))?;
        Ok(__cordl_ret)
    }
    pub fn WriteValueIntoState(
        &mut self,
        value: f32,
        statePtr: *mut quest_hook::libil2cpp::Il2CppObject,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("WriteValueIntoState", (value, statePtr))?;
        Ok(__cordl_ret)
    }
    pub fn CalculateOptimizedControlDataType(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::InputSystem::Utilities::FourCC,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::InputSystem::Utilities::FourCC = __cordl_object
            .invoke("CalculateOptimizedControlDataType", ())?;
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
    pub fn FinishSetup(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("FinishSetup", ())?;
        Ok(__cordl_ret)
    }
    pub fn CompareValue(
        &mut self,
        firstStatePtr: *mut quest_hook::libil2cpp::Il2CppObject,
        secondStatePtr: *mut quest_hook::libil2cpp::Il2CppObject,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("CompareValue", (firstStatePtr, secondStatePtr))?;
        Ok(__cordl_ret)
    }
    pub fn EvaluateMagnitude_Il2CppObject0(
        &mut self,
        statePtr: *mut quest_hook::libil2cpp::Il2CppObject,
    ) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("EvaluateMagnitude", (statePtr))?;
        Ok(__cordl_ret)
    }
    pub fn EvaluateMagnitude_f32_1(
        &mut self,
        value: f32,
    ) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("EvaluateMagnitude", (value))?;
        Ok(__cordl_ret)
    }
    pub fn New() -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "UnityEngine+InputSystem+Controls+AxisControl")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::InputSystem::Controls::AxisControl {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "UnityEngine+InputSystem+Controls+AxisControl+Clamp")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AxisControl_Clamp {
    AfterNormalize = 2i32,
    BeforeNormalize = 1i32,
    None = 0i32,
    ToConstantBeforeNormalize = 3i32,
}
#[cfg(feature = "UnityEngine+InputSystem+Controls+AxisControl+Clamp")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::InputSystem::Controls::AxisControl_Clamp =>
    "UnityEngine.InputSystem.Controls"."AxisControl/Clamp"
);
