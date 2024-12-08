#[cfg(feature = "System+ComponentModel+Design+Serialization+InstanceDescriptor")]
#[repr(C)]
#[derive(Debug)]
pub struct InstanceDescriptor {
    __cordl_parent: crate::System::Object,
    pub _Arguments_k__BackingField: *mut crate::System::Collections::ICollection,
    pub _IsComplete_k__BackingField: bool,
    pub _MemberInfo_k__BackingField: *mut crate::System::Reflection::MemberInfo,
}
#[cfg(feature = "System+ComponentModel+Design+Serialization+InstanceDescriptor")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::System::ComponentModel::Design::Serialization::InstanceDescriptor =>
    "System.ComponentModel.Design.Serialization"."InstanceDescriptor"
);
#[cfg(feature = "System+ComponentModel+Design+Serialization+InstanceDescriptor")]
impl std::ops::Deref
for crate::System::ComponentModel::Design::Serialization::InstanceDescriptor {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+ComponentModel+Design+Serialization+InstanceDescriptor")]
impl std::ops::DerefMut
for crate::System::ComponentModel::Design::Serialization::InstanceDescriptor {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+ComponentModel+Design+Serialization+InstanceDescriptor")]
impl crate::System::ComponentModel::Design::Serialization::InstanceDescriptor {
    pub fn Invoke(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Object> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Object = __cordl_object
            .invoke("Invoke", ())?;
        Ok(__cordl_ret)
    }
    pub fn New_MemberInfo_ICollection0(
        member: *mut crate::System::Reflection::MemberInfo,
        arguments: *mut crate::System::Collections::ICollection,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (member, arguments))?;
        Ok(__cordl_object)
    }
    pub fn New__cordl_bool1(
        member: *mut crate::System::Reflection::MemberInfo,
        arguments: *mut crate::System::Collections::ICollection,
        isComplete: bool,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (member, arguments, isComplete))?;
        Ok(__cordl_object)
    }
    pub fn _ctor_MemberInfo_ICollection0(
        &mut self,
        member: *mut crate::System::Reflection::MemberInfo,
        arguments: *mut crate::System::Collections::ICollection,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (member, arguments))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor__cordl_bool1(
        &mut self,
        member: *mut crate::System::Reflection::MemberInfo,
        arguments: *mut crate::System::Collections::ICollection,
        isComplete: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (member, arguments, isComplete))?;
        Ok(__cordl_ret)
    }
    pub fn get_Arguments(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Collections::ICollection> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Collections::ICollection = __cordl_object
            .invoke("get_Arguments", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_MemberInfo(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Reflection::MemberInfo> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Reflection::MemberInfo = __cordl_object
            .invoke("get_MemberInfo", ())?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "System+ComponentModel+Design+Serialization+InstanceDescriptor")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::ComponentModel::Design::Serialization::InstanceDescriptor {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
