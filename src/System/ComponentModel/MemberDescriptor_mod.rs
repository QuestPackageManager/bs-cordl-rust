#[cfg(feature = "System+ComponentModel+MemberDescriptor")]
#[repr(C)]
#[derive(Debug)]
pub struct MemberDescriptor {
    __cordl_parent: crate::System::Object,
    pub name: *mut crate::System::String,
    pub displayName: *mut crate::System::String,
    pub nameHash: i32,
    pub attributeCollection: *mut crate::System::ComponentModel::AttributeCollection,
    pub attributes: *mut quest_hook::libil2cpp::Il2CppArray<
        *mut crate::System::Attribute,
    >,
    pub originalAttributes: *mut quest_hook::libil2cpp::Il2CppArray<
        *mut crate::System::Attribute,
    >,
    pub attributesFiltered: bool,
    pub attributesFilled: bool,
    pub metadataVersion: i32,
    pub category: *mut crate::System::String,
    pub description: *mut crate::System::String,
    pub lockCookie: *mut crate::System::Object,
}
#[cfg(feature = "System+ComponentModel+MemberDescriptor")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::ComponentModel::MemberDescriptor =>
    "System.ComponentModel"."MemberDescriptor"
);
#[cfg(feature = "System+ComponentModel+MemberDescriptor")]
impl std::ops::Deref for crate::System::ComponentModel::MemberDescriptor {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+ComponentModel+MemberDescriptor")]
impl std::ops::DerefMut for crate::System::ComponentModel::MemberDescriptor {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+ComponentModel+MemberDescriptor")]
impl crate::System::ComponentModel::MemberDescriptor {
    pub fn GetHashCode(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("GetHashCode", ())?;
        Ok(__cordl_ret)
    }
    pub fn FillAttributes(
        &mut self,
        attributeList: *mut crate::System::Collections::IList,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("FillAttributes", (attributeList))?;
        Ok(__cordl_ret)
    }
    pub fn get_NameHashCode(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_NameHashCode", ())?;
        Ok(__cordl_ret)
    }
    pub fn FilterAttributesIfNeeded(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("FilterAttributesIfNeeded", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_AttributeArray(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut quest_hook::libil2cpp::Il2CppArray<*mut crate::System::Attribute>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppArray<
            *mut crate::System::Attribute,
        > = __cordl_object.invoke("get_AttributeArray", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_Attributes(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::ComponentModel::AttributeCollection,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::ComponentModel::AttributeCollection = __cordl_object
            .invoke("get_Attributes", ())?;
        Ok(__cordl_ret)
    }
    pub fn CheckAttributesValid(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("CheckAttributesValid", ())?;
        Ok(__cordl_ret)
    }
    pub fn Equals(
        &mut self,
        obj: *mut crate::System::Object,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("Equals", (obj))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_String0(
        &mut self,
        name: *mut crate::System::String,
        attributes: *mut quest_hook::libil2cpp::Il2CppArray<
            *mut crate::System::Attribute,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (name, attributes))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_MemberDescriptor1(
        &mut self,
        oldMemberDescriptor: *mut crate::System::ComponentModel::MemberDescriptor,
        newAttributes: *mut quest_hook::libil2cpp::Il2CppArray<
            *mut crate::System::Attribute,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (oldMemberDescriptor, newAttributes))?;
        Ok(__cordl_ret)
    }
    pub fn get_DisplayName(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("get_DisplayName", ())?;
        Ok(__cordl_ret)
    }
    pub fn set_AttributeArray(
        &mut self,
        value: *mut quest_hook::libil2cpp::Il2CppArray<*mut crate::System::Attribute>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_AttributeArray", (value))?;
        Ok(__cordl_ret)
    }
    pub fn GetInvocationTarget(
        &mut self,
        _cordl_type: *mut crate::System::Type,
        instance: *mut crate::System::Object,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Object> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Object = __cordl_object
            .invoke("GetInvocationTarget", (_cordl_type, instance))?;
        Ok(__cordl_ret)
    }
    pub fn CreateAttributeCollection(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::ComponentModel::AttributeCollection,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::ComponentModel::AttributeCollection = __cordl_object
            .invoke("CreateAttributeCollection", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_Name(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("get_Name", ())?;
        Ok(__cordl_ret)
    }
    pub fn New_String0(
        name: *mut crate::System::String,
        attributes: *mut quest_hook::libil2cpp::Il2CppArray<
            *mut crate::System::Attribute,
        >,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (name, attributes))?;
        Ok(__cordl_object)
    }
    pub fn New_MemberDescriptor1(
        oldMemberDescriptor: *mut crate::System::ComponentModel::MemberDescriptor,
        newAttributes: *mut quest_hook::libil2cpp::Il2CppArray<
            *mut crate::System::Attribute,
        >,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (oldMemberDescriptor, newAttributes))?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "System+ComponentModel+MemberDescriptor")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::ComponentModel::MemberDescriptor {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
