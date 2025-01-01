#[cfg(feature = "MockLevelEndActions")]
#[repr(C)]
#[derive(Debug)]
pub struct MockLevelEndActions {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub levelFinishedEvent: *mut crate::System::Action,
    pub levelFailedEvent: *mut crate::System::Action,
}
#[cfg(feature = "MockLevelEndActions")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::MockLevelEndActions => ""
    ."MockLevelEndActions"
);
#[cfg(feature = "MockLevelEndActions")]
impl std::ops::Deref for crate::GlobalNamespace::MockLevelEndActions {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "MockLevelEndActions")]
impl std::ops::DerefMut for crate::GlobalNamespace::MockLevelEndActions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "MockLevelEndActions")]
impl crate::GlobalNamespace::MockLevelEndActions {
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
    pub fn add_levelFailedEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<crate::System::Action>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_levelFailedEvent", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn add_levelFinishedEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<crate::System::Action>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_levelFinishedEvent", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn remove_levelFailedEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<crate::System::Action>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_levelFailedEvent", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn remove_levelFinishedEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<crate::System::Action>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_levelFinishedEvent", (value))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "MockLevelEndActions")]
impl quest_hook::libil2cpp::ObjectType for crate::GlobalNamespace::MockLevelEndActions {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "MockLevelEndActions")]
impl AsRef<crate::GlobalNamespace::ILevelEndActions>
for crate::GlobalNamespace::MockLevelEndActions {
    fn as_ref(&self) -> &crate::GlobalNamespace::ILevelEndActions {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "MockLevelEndActions")]
impl AsMut<crate::GlobalNamespace::ILevelEndActions>
for crate::GlobalNamespace::MockLevelEndActions {
    fn as_mut(&mut self) -> &mut crate::GlobalNamespace::ILevelEndActions {
        unsafe { std::mem::transmute(self) }
    }
}
