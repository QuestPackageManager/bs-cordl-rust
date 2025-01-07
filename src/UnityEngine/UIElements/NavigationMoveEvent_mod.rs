#[cfg(feature = "UnityEngine+UIElements+NavigationMoveEvent")]
#[repr(C)]
#[derive(Debug)]
pub struct NavigationMoveEvent {
    __cordl_parent: crate::UnityEngine::UIElements::NavigationEventBase_1<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::NavigationMoveEvent>,
    >,
    pub _direction_k__BackingField: crate::UnityEngine::UIElements::NavigationMoveEvent_Direction,
    pub _move_k__BackingField: crate::UnityEngine::Vector2,
}
#[cfg(feature = "UnityEngine+UIElements+NavigationMoveEvent")]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::UIElements::NavigationMoveEvent {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine.UIElements";
    const CLASS_NAME: &'static str = "NavigationMoveEvent";
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
#[cfg(feature = "UnityEngine+UIElements+NavigationMoveEvent")]
impl std::ops::Deref for crate::UnityEngine::UIElements::NavigationMoveEvent {
    type Target = crate::UnityEngine::UIElements::NavigationEventBase_1<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::NavigationMoveEvent>,
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
    pub fn DetermineMoveDirection(
        x: f32,
        y: f32,
        deadZone: f32,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::UIElements::NavigationMoveEvent_Direction,
    > {
        let __cordl_ret: crate::UnityEngine::UIElements::NavigationMoveEvent_Direction = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("DetermineMoveDirection", (x, y, deadZone))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetPooled_NavigationMoveEvent_Direction_EventModifiers2(
        direction: crate::UnityEngine::UIElements::NavigationMoveEvent_Direction,
        modifiers: crate::UnityEngine::EventModifiers,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::NavigationMoveEvent>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::NavigationMoveEvent,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetPooled", (direction, modifiers))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetPooled_NavigationMoveEvent_Direction_NavigationDeviceType_EventModifiers3(
        direction: crate::UnityEngine::UIElements::NavigationMoveEvent_Direction,
        deviceType: crate::UnityEngine::UIElements::NavigationDeviceType,
        modifiers: crate::UnityEngine::EventModifiers,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::NavigationMoveEvent>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::NavigationMoveEvent,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetPooled", (direction, deviceType, modifiers))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetPooled_Vector2_EventModifiers0(
        moveVector: crate::UnityEngine::Vector2,
        modifiers: crate::UnityEngine::EventModifiers,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::NavigationMoveEvent>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::NavigationMoveEvent,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetPooled", (moveVector, modifiers))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetPooled_Vector2_NavigationDeviceType_EventModifiers1(
        moveVector: crate::UnityEngine::Vector2,
        deviceType: crate::UnityEngine::UIElements::NavigationDeviceType,
        modifiers: crate::UnityEngine::EventModifiers,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::NavigationMoveEvent>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::NavigationMoveEvent,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetPooled", (moveVector, deviceType, modifiers))?;
        Ok(__cordl_ret.into())
    }
    pub fn Init(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Init", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn LocalInit(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("LocalInit", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
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
        Ok(__cordl_ret.into())
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
        Ok(__cordl_ret.into())
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
        Ok(__cordl_ret.into())
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
#[cfg(feature = "UnityEngine+UIElements+NavigationMoveEvent+Direction")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum NavigationMoveEvent_Direction {
    #[default]
    Down = 4i32,
    Left = 1i32,
    Next = 5i32,
    None = 0i32,
    Previous = 6i32,
    Right = 3i32,
    Up = 2i32,
}
#[cfg(feature = "UnityEngine+UIElements+NavigationMoveEvent+Direction")]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::UIElements::NavigationMoveEvent_Direction {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine.UIElements";
    const CLASS_NAME: &'static str = "Direction";
    fn matches_value_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        !ty.is_ref()
            && ty
                .class()
                .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref()
            && ty
                .class()
                .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_value_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        !ty.is_ref()
            && <Self as quest_hook::libil2cpp::Type>::class()
                .is_assignable_from(ty.class())
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref()
            && <Self as quest_hook::libil2cpp::Type>::class()
                .is_assignable_from(ty.class())
    }
}
unsafe impl quest_hook::libil2cpp::Argument
for crate::UnityEngine::UIElements::NavigationMoveEvent_Direction {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
unsafe impl quest_hook::libil2cpp::Parameter
for crate::UnityEngine::UIElements::NavigationMoveEvent_Direction {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_parameter(ty)
    }
    fn from_actual(actual: Self::Actual) -> Self {
        actual
    }
    fn into_actual(self) -> Self::Actual {
        self
    }
}
unsafe impl quest_hook::libil2cpp::Returned
for crate::UnityEngine::UIElements::NavigationMoveEvent_Direction {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_returned(ty)
    }
    fn from_object(object: Option<&mut quest_hook::libil2cpp::Il2CppObject>) -> Self {
        unsafe {
            quest_hook::libil2cpp::raw::unbox(
                quest_hook::libil2cpp::WrapRaw::raw(object.unwrap()),
            )
        }
    }
}
unsafe impl quest_hook::libil2cpp::Return
for crate::UnityEngine::UIElements::NavigationMoveEvent_Direction {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_return(ty)
    }
    fn into_actual(self) -> Self::Actual {
        self
    }
    fn from_actual(actual: Self::Actual) -> Self {
        actual
    }
}
