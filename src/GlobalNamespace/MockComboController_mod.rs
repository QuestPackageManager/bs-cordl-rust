#[cfg(feature = "MockComboController")]
#[repr(C)]
#[derive(Debug)]
pub struct MockComboController {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub comboDidChangeEvent: *mut crate::System::Action_1<i32>,
    pub comboBreakingEventHappenedEvent: *mut crate::System::Action,
}
#[cfg(feature = "MockComboController")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::MockComboController => ""
    ."MockComboController"
);
#[cfg(feature = "MockComboController")]
impl std::ops::Deref for crate::GlobalNamespace::MockComboController {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "MockComboController")]
impl std::ops::DerefMut for crate::GlobalNamespace::MockComboController {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "MockComboController")]
impl crate::GlobalNamespace::MockComboController {
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
    pub fn add_comboBreakingEventHappenedEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<crate::System::Action>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_comboBreakingEventHappenedEvent", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn add_comboDidChangeEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<crate::System::Action_1<i32>>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_comboDidChangeEvent", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn remove_comboBreakingEventHappenedEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<crate::System::Action>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_comboBreakingEventHappenedEvent", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn remove_comboDidChangeEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<crate::System::Action_1<i32>>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_comboDidChangeEvent", (value))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "MockComboController")]
impl quest_hook::libil2cpp::ObjectType for crate::GlobalNamespace::MockComboController {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "MockComboController")]
impl AsRef<crate::GlobalNamespace::IComboController>
for crate::GlobalNamespace::MockComboController {
    fn as_ref(&self) -> &crate::GlobalNamespace::IComboController {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "MockComboController")]
impl AsMut<crate::GlobalNamespace::IComboController>
for crate::GlobalNamespace::MockComboController {
    fn as_mut(&mut self) -> &mut crate::GlobalNamespace::IComboController {
        unsafe { std::mem::transmute(self) }
    }
}
