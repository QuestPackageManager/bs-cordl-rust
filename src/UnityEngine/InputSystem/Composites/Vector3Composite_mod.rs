#[cfg(feature = "UnityEngine+InputSystem+Composites+Vector3Composite+Mode")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Vector3Composite_Mode {
    Analog = 0i32,
    Digital = 2i32,
    DigitalNormalized = 1i32,
}
#[cfg(feature = "UnityEngine+InputSystem+Composites+Vector3Composite+Mode")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::InputSystem::Composites::Vector3Composite_Mode =>
    "UnityEngine.InputSystem.Composites"."Vector3Composite/Mode"
);
#[cfg(feature = "UnityEngine+InputSystem+Composites+Vector3Composite")]
#[repr(C)]
#[derive(Debug)]
pub struct Vector3Composite {
    __cordl_parent: crate::UnityEngine::InputSystem::InputBindingComposite_1<
        crate::UnityEngine::Vector3,
    >,
    pub up: i32,
    pub down: i32,
    pub left: i32,
    pub right: i32,
    pub forward: i32,
    pub backward: i32,
    pub mode: crate::UnityEngine::InputSystem::Composites::Vector3Composite_Mode,
}
#[cfg(feature = "UnityEngine+InputSystem+Composites+Vector3Composite")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::InputSystem::Composites::Vector3Composite =>
    "UnityEngine.InputSystem.Composites"."Vector3Composite"
);
#[cfg(feature = "UnityEngine+InputSystem+Composites+Vector3Composite")]
impl std::ops::Deref for crate::UnityEngine::InputSystem::Composites::Vector3Composite {
    type Target = crate::UnityEngine::InputSystem::InputBindingComposite_1<
        crate::UnityEngine::Vector3,
    >;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+Composites+Vector3Composite")]
impl std::ops::DerefMut
for crate::UnityEngine::InputSystem::Composites::Vector3Composite {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+Composites+Vector3Composite")]
impl crate::UnityEngine::InputSystem::Composites::Vector3Composite {
    #[cfg(feature = "UnityEngine+InputSystem+Composites+Vector3Composite+Mode")]
    pub type Mode = crate::UnityEngine::InputSystem::Composites::Vector3Composite_Mode;
    pub fn ReadValue(
        &mut self,
        context: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::InputSystem::InputBindingCompositeContext,
        >,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector3> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Vector3 = __cordl_object
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
    pub fn New() -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "UnityEngine+InputSystem+Composites+Vector3Composite")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::InputSystem::Composites::Vector3Composite {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
