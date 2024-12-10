#[cfg(feature = "Zenject+LateTickablesTaskUpdater")]
#[repr(C)]
#[derive(Debug)]
pub struct LateTickablesTaskUpdater {
    __cordl_parent: crate::Zenject::TaskUpdater_1<*mut crate::Zenject::ILateTickable>,
}
#[cfg(feature = "Zenject+LateTickablesTaskUpdater")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Zenject::LateTickablesTaskUpdater => "Zenject"
    ."LateTickablesTaskUpdater"
);
#[cfg(feature = "Zenject+LateTickablesTaskUpdater")]
impl std::ops::Deref for crate::Zenject::LateTickablesTaskUpdater {
    type Target = crate::Zenject::TaskUpdater_1<*mut crate::Zenject::ILateTickable>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Zenject+LateTickablesTaskUpdater")]
impl std::ops::DerefMut for crate::Zenject::LateTickablesTaskUpdater {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Zenject+LateTickablesTaskUpdater")]
impl crate::Zenject::LateTickablesTaskUpdater {
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn UpdateItem(
        &mut self,
        task: quest_hook::libil2cpp::Gc<crate::Zenject::ILateTickable>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UpdateItem", (task))?;
        Ok(__cordl_ret.into())
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
#[cfg(feature = "Zenject+LateTickablesTaskUpdater")]
impl quest_hook::libil2cpp::ObjectType for crate::Zenject::LateTickablesTaskUpdater {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
