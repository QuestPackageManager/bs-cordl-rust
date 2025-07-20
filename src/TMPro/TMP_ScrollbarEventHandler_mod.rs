#[cfg(feature = "TMPro+TMP_ScrollbarEventHandler")]
#[repr(C)]
#[derive(Debug)]
pub struct TMP_ScrollbarEventHandler {
    __cordl_parent: crate::UnityEngine::MonoBehaviour,
    pub isSelected: bool,
}
#[cfg(feature = "TMPro+TMP_ScrollbarEventHandler")]
unsafe impl quest_hook::libil2cpp::Type for crate::TMPro::TMP_ScrollbarEventHandler {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "TMPro";
    const CLASS_NAME: &'static str = "TMP_ScrollbarEventHandler";
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
#[cfg(feature = "TMPro+TMP_ScrollbarEventHandler")]
impl std::ops::Deref for crate::TMPro::TMP_ScrollbarEventHandler {
    type Target = crate::UnityEngine::MonoBehaviour;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "TMPro+TMP_ScrollbarEventHandler")]
impl std::ops::DerefMut for crate::TMPro::TMP_ScrollbarEventHandler {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "TMPro+TMP_ScrollbarEventHandler")]
impl crate::TMPro::TMP_ScrollbarEventHandler {
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn OnDeselect(
        &mut self,
        eventData: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::EventSystems::BaseEventData,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::EventSystems::BaseEventData,
                        >),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("OnDeselect")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "OnDeselect", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (eventData))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn OnPointerClick(
        &mut self,
        eventData: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::EventSystems::PointerEventData,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::EventSystems::PointerEventData,
                        >),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("OnPointerClick")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "OnPointerClick", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (eventData))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn OnSelect(
        &mut self,
        eventData: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::EventSystems::BaseEventData,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::EventSystems::BaseEventData,
                        >),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("OnSelect")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "OnSelect", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (eventData))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), quest_hook::libil2cpp::Void, 0usize>(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), ".ctor",
                            0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "TMPro+TMP_ScrollbarEventHandler")]
impl quest_hook::libil2cpp::ObjectType for crate::TMPro::TMP_ScrollbarEventHandler {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "TMPro+TMP_ScrollbarEventHandler")]
impl AsRef<crate::UnityEngine::EventSystems::IDeselectHandler>
for crate::TMPro::TMP_ScrollbarEventHandler {
    fn as_ref(&self) -> &crate::UnityEngine::EventSystems::IDeselectHandler {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "TMPro+TMP_ScrollbarEventHandler")]
impl AsMut<crate::UnityEngine::EventSystems::IDeselectHandler>
for crate::TMPro::TMP_ScrollbarEventHandler {
    fn as_mut(&mut self) -> &mut crate::UnityEngine::EventSystems::IDeselectHandler {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "TMPro+TMP_ScrollbarEventHandler")]
impl AsRef<crate::UnityEngine::EventSystems::IEventSystemHandler>
for crate::TMPro::TMP_ScrollbarEventHandler {
    fn as_ref(&self) -> &crate::UnityEngine::EventSystems::IEventSystemHandler {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "TMPro+TMP_ScrollbarEventHandler")]
impl AsMut<crate::UnityEngine::EventSystems::IEventSystemHandler>
for crate::TMPro::TMP_ScrollbarEventHandler {
    fn as_mut(&mut self) -> &mut crate::UnityEngine::EventSystems::IEventSystemHandler {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "TMPro+TMP_ScrollbarEventHandler")]
impl AsRef<crate::UnityEngine::EventSystems::IPointerClickHandler>
for crate::TMPro::TMP_ScrollbarEventHandler {
    fn as_ref(&self) -> &crate::UnityEngine::EventSystems::IPointerClickHandler {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "TMPro+TMP_ScrollbarEventHandler")]
impl AsMut<crate::UnityEngine::EventSystems::IPointerClickHandler>
for crate::TMPro::TMP_ScrollbarEventHandler {
    fn as_mut(&mut self) -> &mut crate::UnityEngine::EventSystems::IPointerClickHandler {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "TMPro+TMP_ScrollbarEventHandler")]
impl AsRef<crate::UnityEngine::EventSystems::ISelectHandler>
for crate::TMPro::TMP_ScrollbarEventHandler {
    fn as_ref(&self) -> &crate::UnityEngine::EventSystems::ISelectHandler {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "TMPro+TMP_ScrollbarEventHandler")]
impl AsMut<crate::UnityEngine::EventSystems::ISelectHandler>
for crate::TMPro::TMP_ScrollbarEventHandler {
    fn as_mut(&mut self) -> &mut crate::UnityEngine::EventSystems::ISelectHandler {
        unsafe { std::mem::transmute(self) }
    }
}
