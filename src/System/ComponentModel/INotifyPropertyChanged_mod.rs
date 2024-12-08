#[cfg(feature = "System+ComponentModel+INotifyPropertyChanged")]
#[repr(C)]
#[derive(Debug)]
pub struct INotifyPropertyChanged {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "System+ComponentModel+INotifyPropertyChanged")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::ComponentModel::INotifyPropertyChanged
    => "System.ComponentModel"."INotifyPropertyChanged"
);
#[cfg(feature = "System+ComponentModel+INotifyPropertyChanged")]
impl std::ops::Deref for crate::System::ComponentModel::INotifyPropertyChanged {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+ComponentModel+INotifyPropertyChanged")]
impl std::ops::DerefMut for crate::System::ComponentModel::INotifyPropertyChanged {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+ComponentModel+INotifyPropertyChanged")]
impl crate::System::ComponentModel::INotifyPropertyChanged {
    pub fn from_object_mut(
        object_param: *mut quest_hook::libil2cpp::Il2CppObject,
    ) -> *mut Self {
        unsafe { (object_param as *mut Self) }
    }
}
#[cfg(feature = "System+ComponentModel+INotifyPropertyChanged")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::ComponentModel::INotifyPropertyChanged {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
