#[cfg(feature = "UnityEngine+EventSystems+IPointerMoveHandler")]
#[repr(C)]
#[derive(Debug)]
pub struct IPointerMoveHandler {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "UnityEngine+EventSystems+IPointerMoveHandler")]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::EventSystems::IPointerMoveHandler {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine.EventSystems";
    const CLASS_NAME: &'static str = "IPointerMoveHandler";
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
#[cfg(feature = "UnityEngine+EventSystems+IPointerMoveHandler")]
impl std::ops::Deref for crate::UnityEngine::EventSystems::IPointerMoveHandler {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+EventSystems+IPointerMoveHandler")]
impl std::ops::DerefMut for crate::UnityEngine::EventSystems::IPointerMoveHandler {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+EventSystems+IPointerMoveHandler")]
impl crate::UnityEngine::EventSystems::IPointerMoveHandler {
    pub fn OnPointerMove(
        &mut self,
        eventData: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::EventSystems::PointerEventData,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::EventSystems::PointerEventData,
                        >),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("OnPointerMove")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "OnPointerMove", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (eventData))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn from_object_mut(
        object_param: *mut quest_hook::libil2cpp::Il2CppObject,
    ) -> *mut Self {
        unsafe { (object_param as *mut Self) }
    }
}
#[cfg(feature = "UnityEngine+EventSystems+IPointerMoveHandler")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::EventSystems::IPointerMoveHandler {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "UnityEngine+EventSystems+IPointerMoveHandler")]
impl AsRef<crate::UnityEngine::EventSystems::IEventSystemHandler>
for crate::UnityEngine::EventSystems::IPointerMoveHandler {
    fn as_ref(&self) -> &crate::UnityEngine::EventSystems::IEventSystemHandler {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "UnityEngine+EventSystems+IPointerMoveHandler")]
impl AsMut<crate::UnityEngine::EventSystems::IEventSystemHandler>
for crate::UnityEngine::EventSystems::IPointerMoveHandler {
    fn as_mut(&mut self) -> &mut crate::UnityEngine::EventSystems::IEventSystemHandler {
        unsafe { std::mem::transmute(self) }
    }
}
