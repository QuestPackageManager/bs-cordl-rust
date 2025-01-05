#[cfg(feature = "System+ComponentModel+IRevertibleChangeTracking")]
#[repr(C)]
#[derive(Debug)]
pub struct IRevertibleChangeTracking {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "System+ComponentModel+IRevertibleChangeTracking")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::System::ComponentModel::IRevertibleChangeTracking => "System.ComponentModel"
    ."IRevertibleChangeTracking"
);
#[cfg(feature = "System+ComponentModel+IRevertibleChangeTracking")]
impl std::ops::Deref for crate::System::ComponentModel::IRevertibleChangeTracking {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+ComponentModel+IRevertibleChangeTracking")]
impl std::ops::DerefMut for crate::System::ComponentModel::IRevertibleChangeTracking {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+ComponentModel+IRevertibleChangeTracking")]
impl crate::System::ComponentModel::IRevertibleChangeTracking {
    pub fn RejectChanges(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("RejectChanges", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn from_object_mut(
        object_param: *mut quest_hook::libil2cpp::Il2CppObject,
    ) -> *mut Self {
        unsafe { (object_param as *mut Self) }
    }
}
#[cfg(feature = "System+ComponentModel+IRevertibleChangeTracking")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::ComponentModel::IRevertibleChangeTracking {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "System+ComponentModel+IRevertibleChangeTracking")]
impl AsRef<quest_hook::libil2cpp::Gc<crate::System::ComponentModel::IChangeTracking>>
for crate::System::ComponentModel::IRevertibleChangeTracking {
    fn as_ref(
        &self,
    ) -> &quest_hook::libil2cpp::Gc<crate::System::ComponentModel::IChangeTracking> {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "System+ComponentModel+IRevertibleChangeTracking")]
impl AsMut<quest_hook::libil2cpp::Gc<crate::System::ComponentModel::IChangeTracking>>
for crate::System::ComponentModel::IRevertibleChangeTracking {
    fn as_mut(
        &mut self,
    ) -> &mut quest_hook::libil2cpp::Gc<crate::System::ComponentModel::IChangeTracking> {
        unsafe { std::mem::transmute(self) }
    }
}
