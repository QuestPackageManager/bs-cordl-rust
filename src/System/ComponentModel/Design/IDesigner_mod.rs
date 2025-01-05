#[cfg(feature = "System+ComponentModel+Design+IDesigner")]
#[repr(C)]
#[derive(Debug)]
pub struct IDesigner {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "System+ComponentModel+Design+IDesigner")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::ComponentModel::Design::IDesigner =>
    "System.ComponentModel.Design"."IDesigner"
);
#[cfg(feature = "System+ComponentModel+Design+IDesigner")]
impl std::ops::Deref for crate::System::ComponentModel::Design::IDesigner {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+ComponentModel+Design+IDesigner")]
impl std::ops::DerefMut for crate::System::ComponentModel::Design::IDesigner {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+ComponentModel+Design+IDesigner")]
impl crate::System::ComponentModel::Design::IDesigner {
    pub fn from_object_mut(
        object_param: *mut quest_hook::libil2cpp::Il2CppObject,
    ) -> *mut Self {
        unsafe { (object_param as *mut Self) }
    }
}
#[cfg(feature = "System+ComponentModel+Design+IDesigner")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::ComponentModel::Design::IDesigner {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "System+ComponentModel+Design+IDesigner")]
impl AsRef<quest_hook::libil2cpp::Gc<crate::System::IDisposable>>
for crate::System::ComponentModel::Design::IDesigner {
    fn as_ref(&self) -> &quest_hook::libil2cpp::Gc<crate::System::IDisposable> {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "System+ComponentModel+Design+IDesigner")]
impl AsMut<quest_hook::libil2cpp::Gc<crate::System::IDisposable>>
for crate::System::ComponentModel::Design::IDesigner {
    fn as_mut(&mut self) -> &mut quest_hook::libil2cpp::Gc<crate::System::IDisposable> {
        unsafe { std::mem::transmute(self) }
    }
}
