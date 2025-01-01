#[cfg(feature = "NoVRInputModule")]
#[repr(C)]
#[derive(Debug)]
pub struct NoVRInputModule {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub onProcessMousePressEvent: *mut crate::System::Action_1<
        *mut crate::UnityEngine::GameObject,
    >,
    pub pointerDidClickEvent: *mut crate::System::Action_1<
        *mut crate::UnityEngine::EventSystems::PointerEventData,
    >,
}
#[cfg(feature = "NoVRInputModule")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::NoVRInputModule => ""
    ."NoVRInputModule"
);
#[cfg(feature = "NoVRInputModule")]
impl std::ops::Deref for crate::GlobalNamespace::NoVRInputModule {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "NoVRInputModule")]
impl std::ops::DerefMut for crate::GlobalNamespace::NoVRInputModule {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "NoVRInputModule")]
impl crate::GlobalNamespace::NoVRInputModule {
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
    pub fn add_onProcessMousePressEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::System::Action_1<*mut crate::UnityEngine::GameObject>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_onProcessMousePressEvent", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn add_pointerDidClickEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::System::Action_1<
                *mut crate::UnityEngine::EventSystems::PointerEventData,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_pointerDidClickEvent", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn remove_onProcessMousePressEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::System::Action_1<*mut crate::UnityEngine::GameObject>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_onProcessMousePressEvent", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn remove_pointerDidClickEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::System::Action_1<
                *mut crate::UnityEngine::EventSystems::PointerEventData,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_pointerDidClickEvent", (value))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "NoVRInputModule")]
impl quest_hook::libil2cpp::ObjectType for crate::GlobalNamespace::NoVRInputModule {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "NoVRInputModule")]
impl AsRef<crate::GlobalNamespace::IVRInputModule>
for crate::GlobalNamespace::NoVRInputModule {
    fn as_ref(&self) -> &crate::GlobalNamespace::IVRInputModule {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "NoVRInputModule")]
impl AsMut<crate::GlobalNamespace::IVRInputModule>
for crate::GlobalNamespace::NoVRInputModule {
    fn as_mut(&mut self) -> &mut crate::GlobalNamespace::IVRInputModule {
        unsafe { std::mem::transmute(self) }
    }
}