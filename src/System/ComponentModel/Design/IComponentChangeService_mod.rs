#[cfg(feature = "System+ComponentModel+Design+IComponentChangeService")]
#[repr(C)]
#[derive(Debug)]
pub struct IComponentChangeService {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "System+ComponentModel+Design+IComponentChangeService")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::System::ComponentModel::Design::IComponentChangeService =>
    "System.ComponentModel.Design"."IComponentChangeService"
);
#[cfg(feature = "System+ComponentModel+Design+IComponentChangeService")]
impl std::ops::Deref for crate::System::ComponentModel::Design::IComponentChangeService {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+ComponentModel+Design+IComponentChangeService")]
impl std::ops::DerefMut
for crate::System::ComponentModel::Design::IComponentChangeService {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+ComponentModel+Design+IComponentChangeService")]
impl crate::System::ComponentModel::Design::IComponentChangeService {
    pub fn OnComponentChanging(
        &mut self,
        component: *mut crate::System::Object,
        member: *mut crate::System::ComponentModel::MemberDescriptor,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnComponentChanging", (component, member))?;
        Ok(__cordl_ret)
    }
    pub fn OnComponentChanged(
        &mut self,
        component: *mut crate::System::Object,
        member: *mut crate::System::ComponentModel::MemberDescriptor,
        oldValue: *mut crate::System::Object,
        newValue: *mut crate::System::Object,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnComponentChanged", (component, member, oldValue, newValue))?;
        Ok(__cordl_ret)
    }
    pub fn from_object_mut(
        object_param: *mut quest_hook::libil2cpp::Il2CppObject,
    ) -> *mut Self {
        unsafe { (object_param as *mut Self) }
    }
}
#[cfg(feature = "System+ComponentModel+Design+IComponentChangeService")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::ComponentModel::Design::IComponentChangeService {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
