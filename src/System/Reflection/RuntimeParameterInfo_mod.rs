#[cfg(feature = "System+Reflection+RuntimeParameterInfo")]
#[repr(C)]
#[derive(Debug)]
pub struct RuntimeParameterInfo {
    __cordl_parent: crate::System::Reflection::ParameterInfo,
    pub marshalAs: quest_hook::libil2cpp::Gc<
        crate::System::Runtime::InteropServices::MarshalAsAttribute,
    >,
}
#[cfg(feature = "System+Reflection+RuntimeParameterInfo")]
unsafe impl quest_hook::libil2cpp::Type
for crate::System::Reflection::RuntimeParameterInfo {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "System.Reflection";
    const CLASS_NAME: &'static str = "RuntimeParameterInfo";
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.class().is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_value_argument(_: &quest_hook::libil2cpp::Il2CppType) -> bool {
        false
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
    fn matches_value_parameter(_: &quest_hook::libil2cpp::Il2CppType) -> bool {
        false
    }
}
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
    pub fn FormatParameters(
        sb: quest_hook::libil2cpp::Gc<crate::System::Text::StringBuilder>,
        p: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<crate::System::Reflection::ParameterInfo>,
            >,
        >,
        callingConvention: crate::System::Reflection::CallingConventions,
        serialization: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("FormatParameters", (sb, p, callingConvention, serialization))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetCustomAttributes_Type__cordl_bool1(
        &mut self,
        attributeType: quest_hook::libil2cpp::Gc<crate::System::Type>,
        inherit: bool,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
            >,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
            >,
        > = __cordl_object.invoke("GetCustomAttributes", (attributeType, inherit))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetCustomAttributes__cordl_bool0(
        &mut self,
        inherit: bool,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
            >,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
            >,
        > = __cordl_object.invoke("GetCustomAttributes", (inherit))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetDefaultValueImpl(
        &mut self,
        pinfo: quest_hook::libil2cpp::Gc<crate::System::Reflection::ParameterInfo>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppObject,
        > = __cordl_object.invoke("GetDefaultValueImpl", (pinfo))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetPseudoCustomAttributes(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
            >,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
            >,
        > = __cordl_object.invoke("GetPseudoCustomAttributes", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetPseudoCustomAttributesData(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<crate::System::Reflection::CustomAttributeData>,
            >,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<crate::System::Reflection::CustomAttributeData>,
            >,
        > = __cordl_object.invoke("GetPseudoCustomAttributesData", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn IsDefined(
        &mut self,
        attributeType: quest_hook::libil2cpp::Gc<crate::System::Type>,
        inherit: bool,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("IsDefined", (attributeType, inherit))?;
        Ok(__cordl_ret.into())
    }
    pub fn New_Il2CppString_Type_i32_i32_Il2CppObject_MemberInfo_MarshalAsAttribute0(
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        _cordl_type: quest_hook::libil2cpp::Gc<crate::System::Type>,
        position: i32,
        attrs: i32,
        defaultValue: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        member: quest_hook::libil2cpp::Gc<crate::System::Reflection::MemberInfo>,
        marshalAs: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::InteropServices::MarshalAsAttribute,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(
                ".ctor",
                (name, _cordl_type, position, attrs, defaultValue, member, marshalAs),
            )?;
        Ok(__cordl_object.into())
    }
    pub fn New_ParameterInfo0(
        pinfo: quest_hook::libil2cpp::Gc<crate::System::Reflection::ParameterInfo>,
        member: quest_hook::libil2cpp::Gc<crate::System::Reflection::MemberInfo>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Reflection::ParameterInfo>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Reflection::ParameterInfo,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("New", (pinfo, member))?;
        Ok(__cordl_ret.into())
    }
    pub fn New_ParameterInfo_MemberInfo1(
        pinfo: quest_hook::libil2cpp::Gc<crate::System::Reflection::ParameterInfo>,
        member: quest_hook::libil2cpp::Gc<crate::System::Reflection::MemberInfo>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (pinfo, member))?;
        Ok(__cordl_object.into())
    }
    pub fn New_Type_MarshalAsAttribute1(
        _cordl_type: quest_hook::libil2cpp::Gc<crate::System::Type>,
        member: quest_hook::libil2cpp::Gc<crate::System::Reflection::MemberInfo>,
        marshalAs: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::InteropServices::MarshalAsAttribute,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Reflection::ParameterInfo>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Reflection::ParameterInfo,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("New", (_cordl_type, member, marshalAs))?;
        Ok(__cordl_ret.into())
    }
    pub fn New_Type_MemberInfo_MarshalAsAttribute2(
        _cordl_type: quest_hook::libil2cpp::Gc<crate::System::Type>,
        member: quest_hook::libil2cpp::Gc<crate::System::Reflection::MemberInfo>,
        marshalAs: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::InteropServices::MarshalAsAttribute,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (_cordl_type, member, marshalAs))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor_Il2CppString_Type_i32_i32_Il2CppObject_MemberInfo_MarshalAsAttribute0(
        &mut self,
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        _cordl_type: quest_hook::libil2cpp::Gc<crate::System::Type>,
        position: i32,
        attrs: i32,
        defaultValue: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        member: quest_hook::libil2cpp::Gc<crate::System::Reflection::MemberInfo>,
        marshalAs: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::InteropServices::MarshalAsAttribute,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                ".ctor",
                (name, _cordl_type, position, attrs, defaultValue, member, marshalAs),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_ParameterInfo_MemberInfo1(
        &mut self,
        pinfo: quest_hook::libil2cpp::Gc<crate::System::Reflection::ParameterInfo>,
        member: quest_hook::libil2cpp::Gc<crate::System::Reflection::MemberInfo>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (pinfo, member))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_Type_MemberInfo_MarshalAsAttribute2(
        &mut self,
        _cordl_type: quest_hook::libil2cpp::Gc<crate::System::Type>,
        member: quest_hook::libil2cpp::Gc<crate::System::Reflection::MemberInfo>,
        marshalAs: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::InteropServices::MarshalAsAttribute,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (_cordl_type, member, marshalAs))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_DefaultValue(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppObject,
        > = __cordl_object.invoke("get_DefaultValue", ())?;
        Ok(__cordl_ret.into())
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
