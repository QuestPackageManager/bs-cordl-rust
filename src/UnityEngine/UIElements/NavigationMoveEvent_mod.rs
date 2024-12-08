#[cfg(feature = "UnityEngine+UIElements+NavigationMoveEvent+Direction")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NavigationMoveEvent_Direction {
    Down = 4i32,
    Left = 1i32,
    Next = 5i32,
    None = 0i32,
    Previous = 6i32,
    Right = 3i32,
    Up = 2i32,
}
#[cfg(feature = "UnityEngine+UIElements+NavigationMoveEvent+Direction")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::UIElements::NavigationMoveEvent_Direction => "UnityEngine.UIElements"
    ."NavigationMoveEvent/Direction"
);
#[cfg(feature = "UnityEngine+UIElements+NavigationMoveEvent")]
#[repr(C)]
#[derive(Debug)]
pub struct NavigationMoveEvent {
    __cordl_parent: crate::UnityEngine::UIElements::NavigationEventBase_1<
        *mut crate::UnityEngine::UIElements::NavigationMoveEvent,
    >,
    pub _direction_k__BackingField: crate::UnityEngine::UIElements::NavigationMoveEvent_Direction,
    pub _move_k__BackingField: crate::UnityEngine::Vector2,
}
#[cfg(feature = "UnityEngine+UIElements+NavigationMoveEvent")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::UIElements::NavigationMoveEvent =>
    "UnityEngine.UIElements"."NavigationMoveEvent"
);
#[cfg(feature = "UnityEngine+UIElements+NavigationMoveEvent")]
impl std::ops::Deref for crate::UnityEngine::UIElements::NavigationMoveEvent {
    type Target = crate::UnityEngine::UIElements::NavigationEventBase_1<
        *mut crate::UnityEngine::UIElements::NavigationMoveEvent,
    >;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+NavigationMoveEvent")]
impl std::ops::DerefMut for crate::UnityEngine::UIElements::NavigationMoveEvent {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+NavigationMoveEvent")]
impl crate::UnityEngine::UIElements::NavigationMoveEvent {
    #[cfg(feature = "UnityEngine+UIElements+NavigationMoveEvent+Direction")]
    pub type Direction = crate::UnityEngine::UIElements::NavigationMoveEvent_Direction;
    #[cfg(feature = "UnityEngine+UIElements+NavigationMoveEvent+__c")]
    pub type __c = crate::UnityEngine::UIElements::NavigationMoveEvent___c;
    pub fn get_direction(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::UIElements::NavigationMoveEvent_Direction,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::UIElements::NavigationMoveEvent_Direction = __cordl_object
            .invoke("get_direction", ())?;
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
    pub fn LocalInit(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("LocalInit", ())?;
        Ok(__cordl_ret)
    }
    pub fn set_move(
        &mut self,
        value: crate::UnityEngine::Vector2,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_move", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_direction(
        &mut self,
        value: crate::UnityEngine::UIElements::NavigationMoveEvent_Direction,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_direction", (value))?;
        Ok(__cordl_ret)
    }
    pub fn Init(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Init", ())?;
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
#[cfg(feature = "UnityEngine+UIElements+NavigationMoveEvent")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::UIElements::NavigationMoveEvent {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
