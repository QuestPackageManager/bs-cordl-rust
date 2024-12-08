#[cfg(feature = "System+Reflection+CustomAttributeData")]
#[repr(C)]
#[derive(Debug)]
pub struct CustomAttributeData {
    __cordl_parent: crate::System::Object,
    pub ctorInfo: *mut crate::System::Reflection::ConstructorInfo,
    pub ctorArgs: *mut crate::System::Collections::Generic::IList_1<
        crate::System::Reflection::CustomAttributeTypedArgument,
    >,
    pub namedArgs: *mut crate::System::Collections::Generic::IList_1<
        crate::System::Reflection::CustomAttributeNamedArgument,
    >,
    pub lazyData: *mut crate::System::Reflection::CustomAttributeData_LazyCAttrData,
}
#[cfg(feature = "System+Reflection+CustomAttributeData")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Reflection::CustomAttributeData =>
    "System.Reflection"."CustomAttributeData"
);
#[cfg(feature = "System+Reflection+CustomAttributeData")]
impl std::ops::Deref for crate::System::Reflection::CustomAttributeData {
    type Target = crate::System::Object;
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
        obj: *mut crate::System::Object,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("Equals", (obj))?;
        Ok(__cordl_ret)
    }
    pub fn GetHashCode(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("GetHashCode", ())?;
        Ok(__cordl_ret)
    }
    pub fn New_0() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
    pub fn New_ConstructorInfo2(
        ctorInfo: *mut crate::System::Reflection::ConstructorInfo,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (ctorInfo))?;
        Ok(__cordl_object)
    }
    pub fn New_ConstructorInfo_Assembly_IntPtr_u32_1(
        ctorInfo: *mut crate::System::Reflection::ConstructorInfo,
        assembly: *mut crate::System::Reflection::Assembly,
        data: crate::System::IntPtr,
        data_length: u32,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (ctorInfo, assembly, data, data_length))?;
        Ok(__cordl_object)
    }
    pub fn New_ConstructorInfo_IList_1_IList_1_3(
        ctorInfo: *mut crate::System::Reflection::ConstructorInfo,
        ctorArgs: *mut crate::System::Collections::Generic::IList_1<
            crate::System::Reflection::CustomAttributeTypedArgument,
        >,
        namedArgs: *mut crate::System::Collections::Generic::IList_1<
            crate::System::Reflection::CustomAttributeNamedArgument,
        >,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (ctorInfo, ctorArgs, namedArgs))?;
        Ok(__cordl_object)
    }
    pub fn ResolveArguments(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ResolveArguments", ())?;
        Ok(__cordl_ret)
    }
    pub fn ToString(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("ToString", ())?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_0(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_ConstructorInfo2(
        &mut self,
        ctorInfo: *mut crate::System::Reflection::ConstructorInfo,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (ctorInfo))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_ConstructorInfo_Assembly_IntPtr_u32_1(
        &mut self,
        ctorInfo: *mut crate::System::Reflection::ConstructorInfo,
        assembly: *mut crate::System::Reflection::Assembly,
        data: crate::System::IntPtr,
        data_length: u32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (ctorInfo, assembly, data, data_length))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_ConstructorInfo_IList_1_IList_1_3(
        &mut self,
        ctorInfo: *mut crate::System::Reflection::ConstructorInfo,
        ctorArgs: *mut crate::System::Collections::Generic::IList_1<
            crate::System::Reflection::CustomAttributeTypedArgument,
        >,
        namedArgs: *mut crate::System::Collections::Generic::IList_1<
            crate::System::Reflection::CustomAttributeNamedArgument,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (ctorInfo, ctorArgs, namedArgs))?;
        Ok(__cordl_ret)
    }
    pub fn get_AttributeType(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Type> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Type = __cordl_object
            .invoke("get_AttributeType", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_Constructor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Reflection::ConstructorInfo> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Reflection::ConstructorInfo = __cordl_object
            .invoke("get_Constructor", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_ConstructorArguments(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Collections::Generic::IList_1<
            crate::System::Reflection::CustomAttributeTypedArgument,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Collections::Generic::IList_1<
            crate::System::Reflection::CustomAttributeTypedArgument,
        > = __cordl_object.invoke("get_ConstructorArguments", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_NamedArguments(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Collections::Generic::IList_1<
            crate::System::Reflection::CustomAttributeNamedArgument,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Collections::Generic::IList_1<
            crate::System::Reflection::CustomAttributeNamedArgument,
        > = __cordl_object.invoke("get_NamedArguments", ())?;
        Ok(__cordl_ret)
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
    __cordl_parent: crate::System::Object,
    pub assembly: *mut crate::System::Reflection::Assembly,
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
    type Target = crate::System::Object;
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
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret)
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
