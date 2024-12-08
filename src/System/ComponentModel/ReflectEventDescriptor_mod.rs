#[cfg(feature = "System+ComponentModel+ReflectEventDescriptor")]
#[repr(C)]
#[derive(Debug)]
pub struct ReflectEventDescriptor {
    __cordl_parent: crate::System::ComponentModel::EventDescriptor,
    pub _type: *mut crate::System::Type,
    pub _componentClass: *mut crate::System::Type,
    pub _addMethod: *mut crate::System::Reflection::MethodInfo,
    pub _removeMethod: *mut crate::System::Reflection::MethodInfo,
    pub _realEvent: *mut crate::System::Reflection::EventInfo,
    pub _filledMethods: bool,
}
#[cfg(feature = "System+ComponentModel+ReflectEventDescriptor")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::ComponentModel::ReflectEventDescriptor
    => "System.ComponentModel"."ReflectEventDescriptor"
);
#[cfg(feature = "System+ComponentModel+ReflectEventDescriptor")]
impl std::ops::Deref for crate::System::ComponentModel::ReflectEventDescriptor {
    type Target = crate::System::ComponentModel::EventDescriptor;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+ComponentModel+ReflectEventDescriptor")]
impl std::ops::DerefMut for crate::System::ComponentModel::ReflectEventDescriptor {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+ComponentModel+ReflectEventDescriptor")]
impl crate::System::ComponentModel::ReflectEventDescriptor {
    pub fn FillAttributes(
        &mut self,
        attributes: *mut crate::System::Collections::IList,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("FillAttributes", (attributes))?;
        Ok(__cordl_ret)
    }
    pub fn FillEventInfoAttribute(
        &mut self,
        realEventInfo: *mut crate::System::Reflection::EventInfo,
        attributes: *mut crate::System::Collections::IList,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("FillEventInfoAttribute", (realEventInfo, attributes))?;
        Ok(__cordl_ret)
    }
    pub fn FillMethods(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("FillMethods", ())?;
        Ok(__cordl_ret)
    }
    pub fn FillSingleMethodAttribute(
        &mut self,
        realMethodInfo: *mut crate::System::Reflection::MethodInfo,
        attributes: *mut crate::System::Collections::IList,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("FillSingleMethodAttribute", (realMethodInfo, attributes))?;
        Ok(__cordl_ret)
    }
    pub fn New(
        componentClass: *mut crate::System::Type,
        eventInfo: *mut crate::System::Reflection::EventInfo,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (componentClass, eventInfo))?;
        Ok(__cordl_object)
    }
    pub fn _ctor(
        &mut self,
        componentClass: *mut crate::System::Type,
        eventInfo: *mut crate::System::Reflection::EventInfo,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (componentClass, eventInfo))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "System+ComponentModel+ReflectEventDescriptor")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::ComponentModel::ReflectEventDescriptor {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
