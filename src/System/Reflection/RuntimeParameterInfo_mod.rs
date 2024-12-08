#[cfg(feature = "System+Reflection+RuntimeParameterInfo")]
#[repr(C)]
#[derive(Debug)]
pub struct RuntimeParameterInfo {
    __cordl_parent: crate::System::Reflection::ParameterInfo,
    pub marshalAs: *mut crate::System::Runtime::InteropServices::MarshalAsAttribute,
}
#[cfg(feature = "System+Reflection+RuntimeParameterInfo")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Reflection::RuntimeParameterInfo =>
    "System.Reflection"."RuntimeParameterInfo"
);
#[cfg(feature = "System+Reflection+RuntimeParameterInfo")]
impl std::ops::Deref for crate::System::Reflection::RuntimeParameterInfo {
    type Target = crate::System::Reflection::ParameterInfo;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Reflection+RuntimeParameterInfo")]
impl std::ops::DerefMut for crate::System::Reflection::RuntimeParameterInfo {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Reflection+RuntimeParameterInfo")]
impl crate::System::Reflection::RuntimeParameterInfo {
    pub fn GetCustomAttributes_Type__cordl_bool1(
        &mut self,
        attributeType: *mut crate::System::Type,
        inherit: bool,
    ) -> quest_hook::libil2cpp::Result<
        *mut quest_hook::libil2cpp::Il2CppArray<*mut crate::System::Object>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppArray<
            *mut crate::System::Object,
        > = __cordl_object.invoke("GetCustomAttributes", (attributeType, inherit))?;
        Ok(__cordl_ret)
    }
    pub fn GetCustomAttributes__cordl_bool0(
        &mut self,
        inherit: bool,
    ) -> quest_hook::libil2cpp::Result<
        *mut quest_hook::libil2cpp::Il2CppArray<*mut crate::System::Object>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppArray<
            *mut crate::System::Object,
        > = __cordl_object.invoke("GetCustomAttributes", (inherit))?;
        Ok(__cordl_ret)
    }
    pub fn GetDefaultValueImpl(
        &mut self,
        pinfo: *mut crate::System::Reflection::ParameterInfo,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Object> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Object = __cordl_object
            .invoke("GetDefaultValueImpl", (pinfo))?;
        Ok(__cordl_ret)
    }
    pub fn GetPseudoCustomAttributes(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut quest_hook::libil2cpp::Il2CppArray<*mut crate::System::Object>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppArray<
            *mut crate::System::Object,
        > = __cordl_object.invoke("GetPseudoCustomAttributes", ())?;
        Ok(__cordl_ret)
    }
    pub fn GetPseudoCustomAttributesData(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut quest_hook::libil2cpp::Il2CppArray<
            *mut crate::System::Reflection::CustomAttributeData,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppArray<
            *mut crate::System::Reflection::CustomAttributeData,
        > = __cordl_object.invoke("GetPseudoCustomAttributesData", ())?;
        Ok(__cordl_ret)
    }
    pub fn IsDefined(
        &mut self,
        attributeType: *mut crate::System::Type,
        inherit: bool,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("IsDefined", (attributeType, inherit))?;
        Ok(__cordl_ret)
    }
    pub fn New_ParameterInfo_MemberInfo1(
        pinfo: *mut crate::System::Reflection::ParameterInfo,
        member: *mut crate::System::Reflection::MemberInfo,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (pinfo, member))?;
        Ok(__cordl_object)
    }
    pub fn New_String_Type_i32_i32_Object_MemberInfo_MarshalAsAttribute0(
        name: *mut crate::System::String,
        _cordl_type: *mut crate::System::Type,
        position: i32,
        attrs: i32,
        defaultValue: *mut crate::System::Object,
        member: *mut crate::System::Reflection::MemberInfo,
        marshalAs: *mut crate::System::Runtime::InteropServices::MarshalAsAttribute,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(
                ".ctor",
                (name, _cordl_type, position, attrs, defaultValue, member, marshalAs),
            )?;
        Ok(__cordl_object)
    }
    pub fn New_Type_MemberInfo_MarshalAsAttribute2(
        _cordl_type: *mut crate::System::Type,
        member: *mut crate::System::Reflection::MemberInfo,
        marshalAs: *mut crate::System::Runtime::InteropServices::MarshalAsAttribute,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (_cordl_type, member, marshalAs))?;
        Ok(__cordl_object)
    }
    pub fn _ctor_ParameterInfo_MemberInfo1(
        &mut self,
        pinfo: *mut crate::System::Reflection::ParameterInfo,
        member: *mut crate::System::Reflection::MemberInfo,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (pinfo, member))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_String_Type_i32_i32_Object_MemberInfo_MarshalAsAttribute0(
        &mut self,
        name: *mut crate::System::String,
        _cordl_type: *mut crate::System::Type,
        position: i32,
        attrs: i32,
        defaultValue: *mut crate::System::Object,
        member: *mut crate::System::Reflection::MemberInfo,
        marshalAs: *mut crate::System::Runtime::InteropServices::MarshalAsAttribute,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                ".ctor",
                (name, _cordl_type, position, attrs, defaultValue, member, marshalAs),
            )?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_Type_MemberInfo_MarshalAsAttribute2(
        &mut self,
        _cordl_type: *mut crate::System::Type,
        member: *mut crate::System::Reflection::MemberInfo,
        marshalAs: *mut crate::System::Runtime::InteropServices::MarshalAsAttribute,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (_cordl_type, member, marshalAs))?;
        Ok(__cordl_ret)
    }
    pub fn get_DefaultValue(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Object> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Object = __cordl_object
            .invoke("get_DefaultValue", ())?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "System+Reflection+RuntimeParameterInfo")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Reflection::RuntimeParameterInfo {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
