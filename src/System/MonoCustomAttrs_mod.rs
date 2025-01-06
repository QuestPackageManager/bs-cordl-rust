#[cfg(feature = "System+MonoCustomAttrs")]
#[repr(C)]
#[derive(Debug)]
pub struct MonoCustomAttrs {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "System+MonoCustomAttrs")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::MonoCustomAttrs => "System"
    ."MonoCustomAttrs"
);
#[cfg(feature = "System+MonoCustomAttrs")]
impl std::ops::Deref for crate::System::MonoCustomAttrs {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+MonoCustomAttrs")]
impl std::ops::DerefMut for crate::System::MonoCustomAttrs {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+MonoCustomAttrs")]
impl crate::System::MonoCustomAttrs {
    #[cfg(feature = "System+MonoCustomAttrs+AttributeInfo")]
    pub type AttributeInfo = crate::System::MonoCustomAttrs_AttributeInfo;
    pub fn GetBase(
        obj: quest_hook::libil2cpp::Gc<
            crate::System::Reflection::ICustomAttributeProvider,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Reflection::ICustomAttributeProvider>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Reflection::ICustomAttributeProvider,
        > = <Self as quest_hook::libil2cpp::Type>::class().invoke("GetBase", (obj))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetBaseEventDefinition(
        evt: quest_hook::libil2cpp::Gc<crate::System::Reflection::RuntimeEventInfo>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Reflection::EventInfo>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Reflection::EventInfo,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetBaseEventDefinition", (evt))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetBasePropertyDefinition(
        property: quest_hook::libil2cpp::Gc<
            crate::System::Reflection::RuntimePropertyInfo,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Reflection::PropertyInfo>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Reflection::PropertyInfo,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetBasePropertyDefinition", (property))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetCustomAttributesBase(
        obj: quest_hook::libil2cpp::Gc<
            crate::System::Reflection::ICustomAttributeProvider,
        >,
        attributeType: quest_hook::libil2cpp::Gc<crate::System::Type>,
        inheritedOnly: bool,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
            >,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
            >,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetCustomAttributesBase", (obj, attributeType, inheritedOnly))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetCustomAttributesDataBase(
        obj: quest_hook::libil2cpp::Gc<
            crate::System::Reflection::ICustomAttributeProvider,
        >,
        attributeType: quest_hook::libil2cpp::Gc<crate::System::Type>,
        inheritedOnly: bool,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IList_1<
                quest_hook::libil2cpp::Gc<crate::System::Reflection::CustomAttributeData>,
            >,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IList_1<
                quest_hook::libil2cpp::Gc<crate::System::Reflection::CustomAttributeData>,
            >,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetCustomAttributesDataBase", (obj, attributeType, inheritedOnly))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetCustomAttributesDataInternal(
        obj: quest_hook::libil2cpp::Gc<
            crate::System::Reflection::ICustomAttributeProvider,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<crate::System::Reflection::CustomAttributeData>,
            >,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<crate::System::Reflection::CustomAttributeData>,
            >,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetCustomAttributesDataInternal", (obj))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetCustomAttributesData_Type__cordl_bool1(
        obj: quest_hook::libil2cpp::Gc<
            crate::System::Reflection::ICustomAttributeProvider,
        >,
        attributeType: quest_hook::libil2cpp::Gc<crate::System::Type>,
        inherit: bool,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IList_1<
                quest_hook::libil2cpp::Gc<crate::System::Reflection::CustomAttributeData>,
            >,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IList_1<
                quest_hook::libil2cpp::Gc<crate::System::Reflection::CustomAttributeData>,
            >,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetCustomAttributesData", (obj, attributeType, inherit))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetCustomAttributesData__cordl_bool0(
        obj: quest_hook::libil2cpp::Gc<
            crate::System::Reflection::ICustomAttributeProvider,
        >,
        inherit: bool,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IList_1<
                quest_hook::libil2cpp::Gc<crate::System::Reflection::CustomAttributeData>,
            >,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IList_1<
                quest_hook::libil2cpp::Gc<crate::System::Reflection::CustomAttributeData>,
            >,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetCustomAttributesData", (obj, inherit))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetCustomAttributesInternal(
        obj: quest_hook::libil2cpp::Gc<
            crate::System::Reflection::ICustomAttributeProvider,
        >,
        attributeType: quest_hook::libil2cpp::Gc<crate::System::Type>,
        pseudoAttrs: bool,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<crate::System::Attribute>,
            >,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<crate::System::Attribute>,
            >,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetCustomAttributesInternal", (obj, attributeType, pseudoAttrs))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetCustomAttributes_Type__cordl_bool0(
        obj: quest_hook::libil2cpp::Gc<
            crate::System::Reflection::ICustomAttributeProvider,
        >,
        attributeType: quest_hook::libil2cpp::Gc<crate::System::Type>,
        inherit: bool,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
            >,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
            >,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetCustomAttributes", (obj, attributeType, inherit))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetCustomAttributes__cordl_bool1(
        obj: quest_hook::libil2cpp::Gc<
            crate::System::Reflection::ICustomAttributeProvider,
        >,
        inherit: bool,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
            >,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
            >,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetCustomAttributes", (obj, inherit))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetPseudoCustomAttributesData_ICustomAttributeProvider_Type0(
        obj: quest_hook::libil2cpp::Gc<
            crate::System::Reflection::ICustomAttributeProvider,
        >,
        attributeType: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<crate::System::Reflection::CustomAttributeData>,
            >,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<crate::System::Reflection::CustomAttributeData>,
            >,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetPseudoCustomAttributesData", (obj, attributeType))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetPseudoCustomAttributesData_Type1(
        _cordl_type: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<crate::System::Reflection::CustomAttributeData>,
            >,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<crate::System::Reflection::CustomAttributeData>,
            >,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetPseudoCustomAttributesData", (_cordl_type))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetPseudoCustomAttributes_ICustomAttributeProvider_Type0(
        obj: quest_hook::libil2cpp::Gc<
            crate::System::Reflection::ICustomAttributeProvider,
        >,
        attributeType: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
            >,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
            >,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetPseudoCustomAttributes", (obj, attributeType))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetPseudoCustomAttributes_Type1(
        _cordl_type: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
            >,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
            >,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetPseudoCustomAttributes", (_cordl_type))?;
        Ok(__cordl_ret.into())
    }
    pub fn IsDefined(
        obj: quest_hook::libil2cpp::Gc<
            crate::System::Reflection::ICustomAttributeProvider,
        >,
        attributeType: quest_hook::libil2cpp::Gc<crate::System::Type>,
        inherit: bool,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("IsDefined", (obj, attributeType, inherit))?;
        Ok(__cordl_ret.into())
    }
    pub fn IsDefinedInternal(
        obj: quest_hook::libil2cpp::Gc<
            crate::System::Reflection::ICustomAttributeProvider,
        >,
        AttributeType: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("IsDefinedInternal", (obj, AttributeType))?;
        Ok(__cordl_ret.into())
    }
    pub fn IsUserCattrProvider(
        obj: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("IsUserCattrProvider", (obj))?;
        Ok(__cordl_ret.into())
    }
    pub fn RetrieveAttributeUsage(
        attributeType: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::AttributeUsageAttribute>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::AttributeUsageAttribute,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("RetrieveAttributeUsage", (attributeType))?;
        Ok(__cordl_ret.into())
    }
    pub fn RetrieveAttributeUsageNoCache(
        attributeType: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::AttributeUsageAttribute>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::AttributeUsageAttribute,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("RetrieveAttributeUsageNoCache", (attributeType))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+MonoCustomAttrs")]
impl quest_hook::libil2cpp::ObjectType for crate::System::MonoCustomAttrs {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "System+MonoCustomAttrs+AttributeInfo")]
#[repr(C)]
#[derive(Debug)]
pub struct MonoCustomAttrs_AttributeInfo {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _usage: quest_hook::libil2cpp::Gc<crate::System::AttributeUsageAttribute>,
    pub _inheritanceLevel: i32,
}
#[cfg(feature = "System+MonoCustomAttrs+AttributeInfo")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::MonoCustomAttrs_AttributeInfo =>
    "System"."MonoCustomAttrs/AttributeInfo"
);
#[cfg(feature = "System+MonoCustomAttrs+AttributeInfo")]
impl std::ops::Deref for crate::System::MonoCustomAttrs_AttributeInfo {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+MonoCustomAttrs+AttributeInfo")]
impl std::ops::DerefMut for crate::System::MonoCustomAttrs_AttributeInfo {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+MonoCustomAttrs+AttributeInfo")]
impl crate::System::MonoCustomAttrs_AttributeInfo {
    pub fn New(
        usage: quest_hook::libil2cpp::Gc<crate::System::AttributeUsageAttribute>,
        inheritanceLevel: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (usage, inheritanceLevel))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        usage: quest_hook::libil2cpp::Gc<crate::System::AttributeUsageAttribute>,
        inheritanceLevel: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (usage, inheritanceLevel))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_InheritanceLevel(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_InheritanceLevel", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Usage(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::AttributeUsageAttribute>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::AttributeUsageAttribute,
        > = __cordl_object.invoke("get_Usage", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+MonoCustomAttrs+AttributeInfo")]
impl quest_hook::libil2cpp::ObjectType for crate::System::MonoCustomAttrs_AttributeInfo {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
