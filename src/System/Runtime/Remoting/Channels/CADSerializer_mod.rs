#[cfg(feature = "System+Runtime+Remoting+Channels+CADSerializer")]
#[repr(C)]
#[derive(Debug)]
pub struct CADSerializer {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "System+Runtime+Remoting+Channels+CADSerializer")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::System::Runtime::Remoting::Channels::CADSerializer =>
    "System.Runtime.Remoting.Channels"."CADSerializer"
);
#[cfg(feature = "System+Runtime+Remoting+Channels+CADSerializer")]
impl std::ops::Deref for crate::System::Runtime::Remoting::Channels::CADSerializer {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Runtime+Remoting+Channels+CADSerializer")]
impl std::ops::DerefMut for crate::System::Runtime::Remoting::Channels::CADSerializer {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Runtime+Remoting+Channels+CADSerializer")]
impl crate::System::Runtime::Remoting::Channels::CADSerializer {}
#[cfg(feature = "System+Runtime+Remoting+Channels+CADSerializer")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Runtime::Remoting::Channels::CADSerializer {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
