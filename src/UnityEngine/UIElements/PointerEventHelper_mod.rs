#[cfg(feature = "UnityEngine+UIElements+PointerEventHelper")]
#[repr(C)]
#[derive(Debug)]
pub struct PointerEventHelper {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "UnityEngine+UIElements+PointerEventHelper")]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::UIElements::PointerEventHelper {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine.UIElements";
    const CLASS_NAME: &'static str = "PointerEventHelper";
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
#[cfg(feature = "UnityEngine+UIElements+PointerEventHelper")]
impl std::ops::Deref for crate::UnityEngine::UIElements::PointerEventHelper {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+PointerEventHelper")]
impl std::ops::DerefMut for crate::UnityEngine::UIElements::PointerEventHelper {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+PointerEventHelper")]
impl crate::UnityEngine::UIElements::PointerEventHelper {
    pub fn GetPooled(
        eventType: crate::UnityEngine::EventType,
        mousePosition: crate::UnityEngine::Vector3,
        delta: crate::UnityEngine::Vector2,
        button: i32,
        clickCount: i32,
        modifiers: crate::UnityEngine::EventModifiers,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::EventBase>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::UIElements::PointerEventHelper as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::UnityEngine::EventType,
                    crate::UnityEngine::Vector3,
                    crate::UnityEngine::Vector2,
                    i32,
                    i32,
                    crate::UnityEngine::EventModifiers,
                ),
                quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::EventBase>,
                6usize,
            >("GetPooled")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::UIElements::PointerEventHelper as
                    quest_hook::libil2cpp::Type > ::class(), "GetPooled", 6usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::EventBase,
        > = unsafe {
            method
                .invoke_unchecked(
                    (),
                    (eventType, mousePosition, delta, button, clickCount, modifiers),
                )?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+UIElements+PointerEventHelper")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::UIElements::PointerEventHelper {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
