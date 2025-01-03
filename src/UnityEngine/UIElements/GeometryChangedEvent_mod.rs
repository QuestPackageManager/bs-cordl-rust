#[cfg(feature = "UnityEngine+UIElements+GeometryChangedEvent")]
#[repr(C)]
#[derive(Debug)]
pub struct GeometryChangedEvent {
    __cordl_parent: crate::UnityEngine::UIElements::EventBase_1<
        *mut crate::UnityEngine::UIElements::GeometryChangedEvent,
    >,
    pub _oldRect_k__BackingField: crate::UnityEngine::Rect,
    pub _newRect_k__BackingField: crate::UnityEngine::Rect,
    pub _layoutPass_k__BackingField: i32,
}
#[cfg(feature = "UnityEngine+UIElements+GeometryChangedEvent")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::UIElements::GeometryChangedEvent =>
    "UnityEngine.UIElements"."GeometryChangedEvent"
);
#[cfg(feature = "UnityEngine+UIElements+GeometryChangedEvent")]
impl std::ops::Deref for crate::UnityEngine::UIElements::GeometryChangedEvent {
    type Target = crate::UnityEngine::UIElements::EventBase_1<
        *mut crate::UnityEngine::UIElements::GeometryChangedEvent,
    >;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+GeometryChangedEvent")]
impl std::ops::DerefMut for crate::UnityEngine::UIElements::GeometryChangedEvent {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+GeometryChangedEvent")]
impl crate::UnityEngine::UIElements::GeometryChangedEvent {
    #[cfg(feature = "UnityEngine+UIElements+GeometryChangedEvent+__c")]
    pub type __c = crate::UnityEngine::UIElements::GeometryChangedEvent___c;
    pub fn GetPooled(
        oldRect: crate::UnityEngine::Rect,
        newRect: crate::UnityEngine::Rect,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::GeometryChangedEvent>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::GeometryChangedEvent,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetPooled", (oldRect, newRect))?;
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
    pub fn get_layoutPass(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_layoutPass", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_newRect(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Rect> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Rect = __cordl_object
            .invoke("get_newRect", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_oldRect(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Rect> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Rect = __cordl_object
            .invoke("get_oldRect", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn set_layoutPass(
        &mut self,
        value: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_layoutPass", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_newRect(
        &mut self,
        value: crate::UnityEngine::Rect,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_newRect", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_oldRect(
        &mut self,
        value: crate::UnityEngine::Rect,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_oldRect", (value))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+UIElements+GeometryChangedEvent")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::UIElements::GeometryChangedEvent {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
