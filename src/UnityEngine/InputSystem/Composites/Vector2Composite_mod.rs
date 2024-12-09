#[cfg(feature = "UnityEngine+InputSystem+Composites+Vector2Composite")]
#[repr(C)]
#[derive(Debug)]
pub struct Vector2Composite {
    __cordl_parent: crate::UnityEngine::InputSystem::InputBindingComposite_1<
        crate::UnityEngine::Vector2,
    >,
    pub up: i32,
    pub down: i32,
    pub left: i32,
    pub right: i32,
    pub normalize: bool,
    pub mode: crate::UnityEngine::InputSystem::Composites::Vector2Composite_Mode,
}
#[cfg(feature = "UnityEngine+InputSystem+Composites+Vector2Composite")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::InputSystem::Composites::Vector2Composite =>
    "UnityEngine.InputSystem.Composites"."Vector2Composite"
);
#[cfg(feature = "UnityEngine+InputSystem+Composites+Vector2Composite")]
impl std::ops::Deref for crate::UnityEngine::InputSystem::Composites::Vector2Composite {
    type Target = crate::UnityEngine::InputSystem::InputBindingComposite_1<
        crate::UnityEngine::Vector2,
    >;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+Composites+Vector2Composite")]
impl std::ops::DerefMut
for crate::UnityEngine::InputSystem::Composites::Vector2Composite {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+Composites+Vector2Composite")]
impl crate::UnityEngine::InputSystem::Composites::Vector2Composite {
    #[cfg(feature = "UnityEngine+InputSystem+Composites+Vector2Composite+Mode")]
    pub type Mode = crate::UnityEngine::InputSystem::Composites::Vector2Composite_Mode;
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
        Ok(__cordl_ret)
    }
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
    pub fn ReadValue(
        &mut self,
        context: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::InputSystem::InputBindingCompositeContext,
        >,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector2> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Vector2 = __cordl_object
            .invoke("ReadValue", (context))?;
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
}
#[cfg(feature = "UnityEngine+InputSystem+Composites+Vector2Composite")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::InputSystem::Composites::Vector2Composite {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "UnityEngine+InputSystem+Composites+Vector2Composite+Mode")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Vector2Composite_Mode {
    Analog = 2i32,
    Digital = 1i32,
    DigitalNormalized = 0i32,
}
#[cfg(feature = "UnityEngine+InputSystem+Composites+Vector2Composite+Mode")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::InputSystem::Composites::Vector2Composite_Mode =>
    "UnityEngine.InputSystem.Composites"."Vector2Composite/Mode"
);
