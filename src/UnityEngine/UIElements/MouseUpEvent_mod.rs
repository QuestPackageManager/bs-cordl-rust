#[cfg(feature = "UnityEngine+UIElements+MouseUpEvent")]
#[repr(C)]
#[derive(Debug)]
pub struct MouseUpEvent {
    __cordl_parent: crate::UnityEngine::UIElements::MouseEventBase_1<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::MouseUpEvent>,
    >,
}
#[cfg(feature = "UnityEngine+UIElements+MouseUpEvent")]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::UIElements::MouseUpEvent {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine.UIElements";
    const CLASS_NAME: &'static str = "MouseUpEvent";
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
#[cfg(feature = "UnityEngine+UIElements+MouseUpEvent")]
impl std::ops::Deref for crate::UnityEngine::UIElements::MouseUpEvent {
    type Target = crate::UnityEngine::UIElements::MouseEventBase_1<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::MouseUpEvent>,
    >;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+MouseUpEvent")]
impl std::ops::DerefMut for crate::UnityEngine::UIElements::MouseUpEvent {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+MouseUpEvent")]
impl crate::UnityEngine::UIElements::MouseUpEvent {
    pub fn GetPooled_PointerCancelEvent2(
        pointerEvent: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::PointerCancelEvent,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::MouseUpEvent>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::MouseUpEvent,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetPooled", (pointerEvent))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetPooled_PointerMoveEvent1(
        pointerEvent: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::PointerMoveEvent,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::MouseUpEvent>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::MouseUpEvent,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetPooled", (pointerEvent))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetPooled_PointerUpEvent0(
        pointerEvent: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::PointerUpEvent,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::MouseUpEvent>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::MouseUpEvent,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetPooled", (pointerEvent))?;
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
    pub fn MakeFromPointerEvent(
        pointerEvent: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::IPointerEvent,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::MouseUpEvent>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::MouseUpEvent,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("MakeFromPointerEvent", (pointerEvent))?;
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
}
#[cfg(feature = "UnityEngine+UIElements+MouseUpEvent")]
impl quest_hook::libil2cpp::ObjectType for crate::UnityEngine::UIElements::MouseUpEvent {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
