#[cfg(feature = "System+Runtime+InteropServices+_Assembly")]
#[repr(C)]
#[derive(Debug)]
pub struct _Assembly {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "System+Runtime+InteropServices+_Assembly")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Runtime::InteropServices::_Assembly =>
    "System.Runtime.InteropServices"."_Assembly"
);
#[cfg(feature = "System+Runtime+InteropServices+_Assembly")]
impl std::ops::Deref for crate::System::Runtime::InteropServices::_Assembly {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Runtime+InteropServices+_Assembly")]
impl std::ops::DerefMut for crate::System::Runtime::InteropServices::_Assembly {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Runtime+InteropServices+_Assembly")]
impl crate::System::Runtime::InteropServices::_Assembly {
    pub fn from_object_mut(
        object_param: *mut quest_hook::libil2cpp::Il2CppObject,
    ) -> *mut Self {
        unsafe { (object_param as *mut Self) }
    }
}
#[cfg(feature = "System+Runtime+InteropServices+_Assembly")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Runtime::InteropServices::_Assembly {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
