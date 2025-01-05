#[cfg(feature = "System+Reflection+CustomAttributeExtensions")]
#[repr(C)]
#[derive(Debug)]
pub struct CustomAttributeExtensions {
    __cordl_parent: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
}
#[cfg(feature = "System+Reflection+CustomAttributeExtensions")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Reflection::CustomAttributeExtensions =>
    "System.Reflection"."CustomAttributeExtensions"
);
#[cfg(feature = "System+Reflection+CustomAttributeExtensions")]
impl std::ops::Deref for crate::System::Reflection::CustomAttributeExtensions {
    type Target = quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Reflection+CustomAttributeExtensions")]
impl std::ops::DerefMut for crate::System::Reflection::CustomAttributeExtensions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Reflection+CustomAttributeExtensions")]
impl crate::System::Reflection::CustomAttributeExtensions {
    pub fn GetCustomAttribute_Gc0(
        element: quest_hook::libil2cpp::Gc<crate::System::Reflection::Assembly>,
        attributeType: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Attribute>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Attribute> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetCustomAttribute", (element, attributeType))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetCustomAttribute_Gc1(
        element: quest_hook::libil2cpp::Gc<crate::System::Reflection::MemberInfo>,
        attributeType: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Attribute>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Attribute> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetCustomAttribute", (element, attributeType))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetCustomAttribute_Gc2<T>(
        element: quest_hook::libil2cpp::Gc<crate::System::Reflection::Assembly>,
    ) -> quest_hook::libil2cpp::Result<T>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: T = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetCustomAttribute", (element))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetCustomAttribute_Gc3<T>(
        element: quest_hook::libil2cpp::Gc<crate::System::Reflection::MemberInfo>,
    ) -> quest_hook::libil2cpp::Result<T>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: T = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetCustomAttribute", (element))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetCustomAttribute_Gc__cordl_bool4(
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
    pub fn GetCustomAttribute__cordl_bool5<T>(
        element: quest_hook::libil2cpp::Gc<crate::System::Reflection::MemberInfo>,
        inherit: bool,
    ) -> quest_hook::libil2cpp::Result<T>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: T = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetCustomAttribute", (element, inherit))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetCustomAttributes_Gc0(
        element: quest_hook::libil2cpp::Gc<crate::System::Reflection::Assembly>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Gc<crate::System::Attribute>>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Gc<crate::System::Attribute>,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetCustomAttributes", (element))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetCustomAttributes_Gc1(
        element: quest_hook::libil2cpp::Gc<crate::System::Reflection::MemberInfo>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Gc<crate::System::Attribute>>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Gc<crate::System::Attribute>,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetCustomAttributes", (element))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetCustomAttributes_Gc2(
        element: quest_hook::libil2cpp::Gc<crate::System::Reflection::MemberInfo>,
        attributeType: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Gc<crate::System::Attribute>>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Gc<crate::System::Attribute>,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetCustomAttributes", (element, attributeType))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetCustomAttributes_Gc3<T>(
        element: quest_hook::libil2cpp::Gc<crate::System::Reflection::MemberInfo>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<T>>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Gc<T> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetCustomAttributes", (element))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetCustomAttributes_Gc__cordl_bool4(
        element: quest_hook::libil2cpp::Gc<crate::System::Reflection::MemberInfo>,
        attributeType: quest_hook::libil2cpp::Gc<crate::System::Type>,
        inherit: bool,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Gc<crate::System::Attribute>>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Gc<crate::System::Attribute>,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetCustomAttributes", (element, attributeType, inherit))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetCustomAttributes__cordl_bool5<T>(
        element: quest_hook::libil2cpp::Gc<crate::System::Reflection::MemberInfo>,
        inherit: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<T>>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Gc<T> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetCustomAttributes", (element, inherit))?;
        Ok(__cordl_ret.into())
    }
    pub fn IsDefined(
        element: quest_hook::libil2cpp::Gc<crate::System::Reflection::MemberInfo>,
        attributeType: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("IsDefined", (element, attributeType))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Reflection+CustomAttributeExtensions")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Reflection::CustomAttributeExtensions {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
