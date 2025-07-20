#[cfg(feature = "UnityEngine+UIElements+GeometryChangedEvent")]
#[repr(C)]
#[derive(Debug)]
pub struct GeometryChangedEvent {
    __cordl_parent: crate::UnityEngine::UIElements::EventBase_1<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::GeometryChangedEvent>,
    >,
    pub _oldRect_k__BackingField: crate::UnityEngine::Rect,
    pub _newRect_k__BackingField: crate::UnityEngine::Rect,
    pub _layoutPass_k__BackingField: i32,
}
#[cfg(feature = "UnityEngine+UIElements+GeometryChangedEvent")]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::UIElements::GeometryChangedEvent {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine.UIElements";
    const CLASS_NAME: &'static str = "GeometryChangedEvent";
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
#[cfg(feature = "UnityEngine+UIElements+GeometryChangedEvent")]
impl std::ops::Deref for crate::UnityEngine::UIElements::GeometryChangedEvent {
    type Target = crate::UnityEngine::UIElements::EventBase_1<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::GeometryChangedEvent>,
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
    pub fn GetPooled(
        oldRect: crate::UnityEngine::Rect,
        newRect: crate::UnityEngine::Rect,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::GeometryChangedEvent>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::UIElements::GeometryChangedEvent as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::UnityEngine::Rect, crate::UnityEngine::Rect),
                quest_hook::libil2cpp::Gc<
                    crate::UnityEngine::UIElements::GeometryChangedEvent,
                >,
                2usize,
            >("GetPooled")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::UIElements::GeometryChangedEvent as
                    quest_hook::libil2cpp::Type > ::class(), "GetPooled", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::GeometryChangedEvent,
        > = unsafe { method.invoke_unchecked((), (oldRect, newRect))? };
        Ok(__cordl_ret.into())
    }
    pub fn Init(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::UIElements::GeometryChangedEvent as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), quest_hook::libil2cpp::Void, 0usize>("Init")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::UIElements::GeometryChangedEvent as
                    quest_hook::libil2cpp::Type > ::class(), "Init", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn LocalInit(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::UIElements::GeometryChangedEvent as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), quest_hook::libil2cpp::Void, 0usize>("LocalInit")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::UIElements::GeometryChangedEvent as
                    quest_hook::libil2cpp::Type > ::class(), "LocalInit", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())?
        };
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
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::UIElements::GeometryChangedEvent as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), quest_hook::libil2cpp::Void, 0usize>(".ctor")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::UIElements::GeometryChangedEvent as
                    quest_hook::libil2cpp::Type > ::class(), ".ctor", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_layoutPass(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::UIElements::GeometryChangedEvent as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), i32, 0usize>("get_layoutPass")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::UIElements::GeometryChangedEvent as
                    quest_hook::libil2cpp::Type > ::class(), "get_layoutPass", 0usize
                )
            });
        let __cordl_ret: i32 = unsafe { method.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_newRect(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Rect> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::UIElements::GeometryChangedEvent as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), crate::UnityEngine::Rect, 0usize>("get_newRect")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::UIElements::GeometryChangedEvent as
                    quest_hook::libil2cpp::Type > ::class(), "get_newRect", 0usize
                )
            });
        let __cordl_ret: crate::UnityEngine::Rect = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_oldRect(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Rect> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::UIElements::GeometryChangedEvent as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), crate::UnityEngine::Rect, 0usize>("get_oldRect")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::UIElements::GeometryChangedEvent as
                    quest_hook::libil2cpp::Type > ::class(), "get_oldRect", 0usize
                )
            });
        let __cordl_ret: crate::UnityEngine::Rect = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn set_layoutPass(
        &mut self,
        value: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::UIElements::GeometryChangedEvent as quest_hook::libil2cpp::Type>::class()
            .find_method::<(i32), quest_hook::libil2cpp::Void, 1usize>("set_layoutPass")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::UIElements::GeometryChangedEvent as
                    quest_hook::libil2cpp::Type > ::class(), "set_layoutPass", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (value))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn set_newRect(
        &mut self,
        value: crate::UnityEngine::Rect,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::UIElements::GeometryChangedEvent as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (crate::UnityEngine::Rect),
                quest_hook::libil2cpp::Void,
                1usize,
            >("set_newRect")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::UIElements::GeometryChangedEvent as
                    quest_hook::libil2cpp::Type > ::class(), "set_newRect", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (value))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn set_oldRect(
        &mut self,
        value: crate::UnityEngine::Rect,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::UIElements::GeometryChangedEvent as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (crate::UnityEngine::Rect),
                quest_hook::libil2cpp::Void,
                1usize,
            >("set_oldRect")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::UIElements::GeometryChangedEvent as
                    quest_hook::libil2cpp::Type > ::class(), "set_oldRect", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (value))?
        };
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
