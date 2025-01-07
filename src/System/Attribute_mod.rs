#[cfg(feature = "System+Attribute")]
#[repr(C)]
#[derive(Debug)]
pub struct Attribute {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "System+Attribute")]
unsafe impl quest_hook::libil2cpp::Type for crate::System::Attribute {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "System";
    const CLASS_NAME: &'static str = "Attribute";
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
#[cfg(feature = "System+Attribute")]
impl std::ops::Deref for crate::System::Attribute {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Attribute")]
impl std::ops::DerefMut for crate::System::Attribute {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Attribute")]
impl crate::System::Attribute {
    pub fn AreFieldValuesEqual(
        thisValue: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        thatValue: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("AreFieldValuesEqual", (thisValue, thatValue))?;
        Ok(__cordl_ret.into())
    }
    pub fn Equals(
        &mut self,
        obj: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("Equals", (obj))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetCustomAttribute_Assembly2(
        element: quest_hook::libil2cpp::Gc<crate::System::Reflection::Assembly>,
        attributeType: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Attribute>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Attribute> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetCustomAttribute", (element, attributeType))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetCustomAttribute_Assembly__cordl_bool3(
        element: quest_hook::libil2cpp::Gc<crate::System::Reflection::Assembly>,
        attributeType: quest_hook::libil2cpp::Gc<crate::System::Type>,
        inherit: bool,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Attribute>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Attribute> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetCustomAttribute", (element, attributeType, inherit))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetCustomAttribute_MemberInfo0(
        element: quest_hook::libil2cpp::Gc<crate::System::Reflection::MemberInfo>,
        attributeType: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Attribute>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Attribute> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetCustomAttribute", (element, attributeType))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetCustomAttribute_MemberInfo__cordl_bool1(
        element: quest_hook::libil2cpp::Gc<crate::System::Reflection::MemberInfo>,
        attributeType: quest_hook::libil2cpp::Gc<crate::System::Type>,
        inherit: bool,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Attribute>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Attribute> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetCustomAttribute", (element, attributeType, inherit))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetCustomAttributes_Assembly10(
        element: quest_hook::libil2cpp::Gc<crate::System::Reflection::Assembly>,
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
            .invoke("GetCustomAttributes", (element))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetCustomAttributes_Assembly_Type8(
        element: quest_hook::libil2cpp::Gc<crate::System::Reflection::Assembly>,
        attributeType: quest_hook::libil2cpp::Gc<crate::System::Type>,
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
            .invoke("GetCustomAttributes", (element, attributeType))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetCustomAttributes_Assembly_Type__cordl_bool9(
        element: quest_hook::libil2cpp::Gc<crate::System::Reflection::Assembly>,
        attributeType: quest_hook::libil2cpp::Gc<crate::System::Type>,
        inherit: bool,
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
            .invoke("GetCustomAttributes", (element, attributeType, inherit))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetCustomAttributes_Assembly__cordl_bool11(
        element: quest_hook::libil2cpp::Gc<crate::System::Reflection::Assembly>,
        inherit: bool,
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
            .invoke("GetCustomAttributes", (element, inherit))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetCustomAttributes_MemberInfo2(
        element: quest_hook::libil2cpp::Gc<crate::System::Reflection::MemberInfo>,
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
            .invoke("GetCustomAttributes", (element))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetCustomAttributes_MemberInfo_Type0(
        element: quest_hook::libil2cpp::Gc<crate::System::Reflection::MemberInfo>,
        _cordl_type: quest_hook::libil2cpp::Gc<crate::System::Type>,
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
            .invoke("GetCustomAttributes", (element, _cordl_type))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetCustomAttributes_MemberInfo_Type__cordl_bool1(
        element: quest_hook::libil2cpp::Gc<crate::System::Reflection::MemberInfo>,
        _cordl_type: quest_hook::libil2cpp::Gc<crate::System::Type>,
        inherit: bool,
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
            .invoke("GetCustomAttributes", (element, _cordl_type, inherit))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetCustomAttributes_MemberInfo__cordl_bool3(
        element: quest_hook::libil2cpp::Gc<crate::System::Reflection::MemberInfo>,
        inherit: bool,
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
            .invoke("GetCustomAttributes", (element, inherit))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetCustomAttributes_Module_Type__cordl_bool7(
        element: quest_hook::libil2cpp::Gc<crate::System::Reflection::Module>,
        attributeType: quest_hook::libil2cpp::Gc<crate::System::Type>,
        inherit: bool,
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
            .invoke("GetCustomAttributes", (element, attributeType, inherit))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetCustomAttributes_Module__cordl_bool6(
        element: quest_hook::libil2cpp::Gc<crate::System::Reflection::Module>,
        inherit: bool,
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
            .invoke("GetCustomAttributes", (element, inherit))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetCustomAttributes_ParameterInfo_Type__cordl_bool4(
        element: quest_hook::libil2cpp::Gc<crate::System::Reflection::ParameterInfo>,
        attributeType: quest_hook::libil2cpp::Gc<crate::System::Type>,
        inherit: bool,
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
            .invoke("GetCustomAttributes", (element, attributeType, inherit))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetCustomAttributes_ParameterInfo__cordl_bool5(
        element: quest_hook::libil2cpp::Gc<crate::System::Reflection::ParameterInfo>,
        inherit: bool,
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
            .invoke("GetCustomAttributes", (element, inherit))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetHashCode(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("GetHashCode", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn InternalGetCustomAttributes_EventInfo1(
        element: quest_hook::libil2cpp::Gc<crate::System::Reflection::EventInfo>,
        _cordl_type: quest_hook::libil2cpp::Gc<crate::System::Type>,
        inherit: bool,
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
            .invoke("InternalGetCustomAttributes", (element, _cordl_type, inherit))?;
        Ok(__cordl_ret.into())
    }
    pub fn InternalGetCustomAttributes_PropertyInfo0(
        element: quest_hook::libil2cpp::Gc<crate::System::Reflection::PropertyInfo>,
        _cordl_type: quest_hook::libil2cpp::Gc<crate::System::Type>,
        inherit: bool,
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
            .invoke("InternalGetCustomAttributes", (element, _cordl_type, inherit))?;
        Ok(__cordl_ret.into())
    }
    pub fn InternalIsDefined_EventInfo1(
        element: quest_hook::libil2cpp::Gc<crate::System::Reflection::EventInfo>,
        attributeType: quest_hook::libil2cpp::Gc<crate::System::Type>,
        inherit: bool,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("InternalIsDefined", (element, attributeType, inherit))?;
        Ok(__cordl_ret.into())
    }
    pub fn InternalIsDefined_PropertyInfo0(
        element: quest_hook::libil2cpp::Gc<crate::System::Reflection::PropertyInfo>,
        attributeType: quest_hook::libil2cpp::Gc<crate::System::Type>,
        inherit: bool,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("InternalIsDefined", (element, attributeType, inherit))?;
        Ok(__cordl_ret.into())
    }
    pub fn InternalParamGetCustomAttributes(
        parameter: quest_hook::libil2cpp::Gc<crate::System::Reflection::ParameterInfo>,
        attributeType: quest_hook::libil2cpp::Gc<crate::System::Type>,
        inherit: bool,
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
            .invoke(
                "InternalParamGetCustomAttributes",
                (parameter, attributeType, inherit),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn IsDefaultAttribute(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("IsDefaultAttribute", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn IsDefined_MemberInfo_Type0(
        element: quest_hook::libil2cpp::Gc<crate::System::Reflection::MemberInfo>,
        attributeType: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("IsDefined", (element, attributeType))?;
        Ok(__cordl_ret.into())
    }
    pub fn IsDefined__cordl_bool1(
        element: quest_hook::libil2cpp::Gc<crate::System::Reflection::MemberInfo>,
        attributeType: quest_hook::libil2cpp::Gc<crate::System::Type>,
        inherit: bool,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("IsDefined", (element, attributeType, inherit))?;
        Ok(__cordl_ret.into())
    }
    pub fn Match(
        &mut self,
        obj: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("Match", (obj))?;
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_TypeId(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppObject,
        > = __cordl_object.invoke("get_TypeId", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Attribute")]
impl quest_hook::libil2cpp::ObjectType for crate::System::Attribute {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
