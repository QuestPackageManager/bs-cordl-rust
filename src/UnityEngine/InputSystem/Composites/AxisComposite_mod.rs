#[cfg(feature = "UnityEngine+InputSystem+Composites+AxisComposite")]
#[repr(C)]
#[derive(Debug)]
pub struct AxisComposite {
    __cordl_parent: crate::UnityEngine::InputSystem::InputBindingComposite_1<f32>,
    pub negative: i32,
    pub positive: i32,
    pub minValue: f32,
    pub maxValue: f32,
    pub whichSideWins: crate::UnityEngine::InputSystem::Composites::AxisComposite_WhichSideWins,
}
#[cfg(feature = "UnityEngine+InputSystem+Composites+AxisComposite")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::InputSystem::Composites::AxisComposite =>
    "UnityEngine.InputSystem.Composites"."AxisComposite"
);
#[cfg(feature = "UnityEngine+InputSystem+Composites+AxisComposite")]
impl std::ops::Deref for crate::UnityEngine::InputSystem::Composites::AxisComposite {
    type Target = crate::UnityEngine::InputSystem::InputBindingComposite_1<f32>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+Composites+AxisComposite")]
impl std::ops::DerefMut for crate::UnityEngine::InputSystem::Composites::AxisComposite {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+Composites+AxisComposite")]
impl crate::UnityEngine::InputSystem::Composites::AxisComposite {
    #[cfg(feature = "UnityEngine+InputSystem+Composites+AxisComposite+WhichSideWins")]
    pub type WhichSideWins = crate::UnityEngine::InputSystem::Composites::AxisComposite_WhichSideWins;
    pub fn EvaluateMagnitude(
        &mut self,
        context: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::InputSystem::InputBindingCompositeContext,
        >,
    ) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("EvaluateMagnitude", (context))?;
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn ReadValue(
        &mut self,
        context: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::InputSystem::InputBindingCompositeContext,
        >,
    ) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("ReadValue", (context))?;
        Ok(__cordl_ret.into())
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
    pub fn get_midPoint(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("get_midPoint", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+InputSystem+Composites+AxisComposite")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::InputSystem::Composites::AxisComposite {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "UnityEngine+InputSystem+Composites+AxisComposite+WhichSideWins")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AxisComposite_WhichSideWins {
    Negative = 2i32,
    Neither = 0i32,
    Positive = 1i32,
}
#[cfg(feature = "UnityEngine+InputSystem+Composites+AxisComposite+WhichSideWins")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::InputSystem::Composites::AxisComposite_WhichSideWins =>
    "UnityEngine.InputSystem.Composites"."AxisComposite/WhichSideWins"
);
