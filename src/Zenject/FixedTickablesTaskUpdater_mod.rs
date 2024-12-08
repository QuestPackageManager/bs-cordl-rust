#[cfg(feature = "Zenject+FixedTickablesTaskUpdater")]
#[repr(C)]
#[derive(Debug)]
pub struct FixedTickablesTaskUpdater {
    __cordl_parent: crate::Zenject::TaskUpdater_1<*mut crate::Zenject::IFixedTickable>,
}
#[cfg(feature = "Zenject+FixedTickablesTaskUpdater")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Zenject::FixedTickablesTaskUpdater => "Zenject"
    ."FixedTickablesTaskUpdater"
);
#[cfg(feature = "Zenject+FixedTickablesTaskUpdater")]
impl std::ops::Deref for crate::Zenject::FixedTickablesTaskUpdater {
    type Target = crate::Zenject::TaskUpdater_1<*mut crate::Zenject::IFixedTickable>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Zenject+FixedTickablesTaskUpdater")]
impl std::ops::DerefMut for crate::Zenject::FixedTickablesTaskUpdater {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Zenject+FixedTickablesTaskUpdater")]
impl crate::Zenject::FixedTickablesTaskUpdater {
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
    pub fn UpdateItem(
        &mut self,
        task: *mut crate::Zenject::IFixedTickable,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UpdateItem", (task))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "Zenject+FixedTickablesTaskUpdater")]
impl quest_hook::libil2cpp::ObjectType for crate::Zenject::FixedTickablesTaskUpdater {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
