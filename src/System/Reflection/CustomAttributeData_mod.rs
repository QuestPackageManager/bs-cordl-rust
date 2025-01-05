#[cfg(feature = "System+Reflection+CustomAttributeData")]
#[repr(C)]
#[derive(Debug)]
pub struct CustomAttributeData {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub ctorInfo: quest_hook::libil2cpp::Gc<crate::System::Reflection::ConstructorInfo>,
    pub ctorArgs: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::IList_1<
            crate::System::Reflection::CustomAttributeTypedArgument,
        >,
    >,
    pub namedArgs: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::IList_1<
            crate::System::Reflection::CustomAttributeNamedArgument,
        >,
    >,
    pub lazyData: quest_hook::libil2cpp::Gc<
        crate::System::Reflection::CustomAttributeData_LazyCAttrData,
    >,
}
#[cfg(feature = "System+Reflection+CustomAttributeData")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Reflection::CustomAttributeData =>
    "System.Reflection"."CustomAttributeData"
);
#[cfg(feature = "System+Reflection+CustomAttributeData")]
impl std::ops::Deref for crate::System::Reflection::CustomAttributeData {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Reflection+CustomAttributeData")]
impl std::ops::DerefMut for crate::System::Reflection::CustomAttributeData {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Reflection+CustomAttributeData")]
impl crate::System::Reflection::CustomAttributeData {
    #[cfg(feature = "System+Reflection+CustomAttributeData+LazyCAttrData")]
    pub type LazyCAttrData = crate::System::Reflection::CustomAttributeData_LazyCAttrData;
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
    pub fn GetCustomAttributesInternal(
        target: quest_hook::libil2cpp::Gc<crate::System::RuntimeType>,
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
            .invoke("GetCustomAttributesInternal", (target))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetCustomAttributes_Assembly0(
        target: quest_hook::libil2cpp::Gc<crate::System::Reflection::Assembly>,
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
            .invoke("GetCustomAttributes", (target))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetCustomAttributes_MemberInfo1(
        target: quest_hook::libil2cpp::Gc<crate::System::Reflection::MemberInfo>,
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
            .invoke("GetCustomAttributes", (target))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetCustomAttributes_Module2(
        target: quest_hook::libil2cpp::Gc<crate::System::Reflection::Module>,
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
            .invoke("GetCustomAttributes", (target))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetCustomAttributes_ParameterInfo3(
        target: quest_hook::libil2cpp::Gc<crate::System::Reflection::ParameterInfo>,
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
            .invoke("GetCustomAttributes", (target))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetHashCode(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("GetHashCode", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn New_0() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn New_ConstructorInfo2(
        ctorInfo: quest_hook::libil2cpp::Gc<crate::System::Reflection::ConstructorInfo>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (ctorInfo))?;
        Ok(__cordl_object.into())
    }
    pub fn New_ConstructorInfo_Assembly_IntPtr_u32_1(
        ctorInfo: quest_hook::libil2cpp::Gc<crate::System::Reflection::ConstructorInfo>,
        assembly: quest_hook::libil2cpp::Gc<crate::System::Reflection::Assembly>,
        data: crate::System::IntPtr,
        data_length: u32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (ctorInfo, assembly, data, data_length))?;
        Ok(__cordl_object.into())
    }
    pub fn New_ConstructorInfo_IList_1_IList_1_3(
        ctorInfo: quest_hook::libil2cpp::Gc<crate::System::Reflection::ConstructorInfo>,
        ctorArgs: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IList_1<
                crate::System::Reflection::CustomAttributeTypedArgument,
            >,
        >,
        namedArgs: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IList_1<
                crate::System::Reflection::CustomAttributeNamedArgument,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (ctorInfo, ctorArgs, namedArgs))?;
        Ok(__cordl_object.into())
    }
    pub fn ResolveArguments(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ResolveArguments", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn ResolveArgumentsInternal(
        ctor: quest_hook::libil2cpp::Gc<crate::System::Reflection::ConstructorInfo>,
        assembly: quest_hook::libil2cpp::Gc<crate::System::Reflection::Assembly>,
        data: crate::System::IntPtr,
        data_length: u32,
        ctorArgs: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<
                quest_hook::libil2cpp::Il2CppArray<
                    *mut quest_hook::libil2cpp::Il2CppObject,
                >,
            >,
        >,
        namedArgs: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<
                quest_hook::libil2cpp::Il2CppArray<
                    *mut quest_hook::libil2cpp::Il2CppObject,
                >,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "ResolveArgumentsInternal",
                (ctor, assembly, data, data_length, ctorArgs, namedArgs),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn ToString(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("ToString", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn UnboxValues<T>(
        values: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<*mut quest_hook::libil2cpp::Il2CppObject>,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<T>>,
    >
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<T>,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("UnboxValues", (values))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_0(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_ConstructorInfo2(
        &mut self,
        ctorInfo: quest_hook::libil2cpp::Gc<crate::System::Reflection::ConstructorInfo>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (ctorInfo))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_ConstructorInfo_Assembly_IntPtr_u32_1(
        &mut self,
        ctorInfo: quest_hook::libil2cpp::Gc<crate::System::Reflection::ConstructorInfo>,
        assembly: quest_hook::libil2cpp::Gc<crate::System::Reflection::Assembly>,
        data: crate::System::IntPtr,
        data_length: u32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (ctorInfo, assembly, data, data_length))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_ConstructorInfo_IList_1_IList_1_3(
        &mut self,
        ctorInfo: quest_hook::libil2cpp::Gc<crate::System::Reflection::ConstructorInfo>,
        ctorArgs: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IList_1<
                crate::System::Reflection::CustomAttributeTypedArgument,
            >,
        >,
        namedArgs: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IList_1<
                crate::System::Reflection::CustomAttributeNamedArgument,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (ctorInfo, ctorArgs, namedArgs))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_AttributeType(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<crate::System::Type>> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Type> = __cordl_object
            .invoke("get_AttributeType", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Constructor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Reflection::ConstructorInfo>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Reflection::ConstructorInfo,
        > = __cordl_object.invoke("get_Constructor", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_ConstructorArguments(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IList_1<
                crate::System::Reflection::CustomAttributeTypedArgument,
            >,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IList_1<
                crate::System::Reflection::CustomAttributeTypedArgument,
            >,
        > = __cordl_object.invoke("get_ConstructorArguments", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_NamedArguments(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IList_1<
                crate::System::Reflection::CustomAttributeNamedArgument,
            >,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IList_1<
                crate::System::Reflection::CustomAttributeNamedArgument,
            >,
        > = __cordl_object.invoke("get_NamedArguments", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Reflection+CustomAttributeData")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Reflection::CustomAttributeData {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "System+Reflection+CustomAttributeData+LazyCAttrData")]
#[repr(C)]
#[derive(Debug)]
pub struct CustomAttributeData_LazyCAttrData {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub assembly: quest_hook::libil2cpp::Gc<crate::System::Reflection::Assembly>,
    pub data: crate::System::IntPtr,
    pub data_length: u32,
}
#[cfg(feature = "System+Reflection+CustomAttributeData+LazyCAttrData")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::System::Reflection::CustomAttributeData_LazyCAttrData => "System.Reflection"
    ."CustomAttributeData/LazyCAttrData"
);
#[cfg(feature = "System+Reflection+CustomAttributeData+LazyCAttrData")]
impl std::ops::Deref for crate::System::Reflection::CustomAttributeData_LazyCAttrData {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Reflection+CustomAttributeData+LazyCAttrData")]
impl std::ops::DerefMut
for crate::System::Reflection::CustomAttributeData_LazyCAttrData {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Reflection+CustomAttributeData+LazyCAttrData")]
impl crate::System::Reflection::CustomAttributeData_LazyCAttrData {
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
}
#[cfg(feature = "System+Reflection+CustomAttributeData+LazyCAttrData")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Reflection::CustomAttributeData_LazyCAttrData {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
