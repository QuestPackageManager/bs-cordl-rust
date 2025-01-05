#[cfg(feature = "System+Runtime+Remoting+SoapServices")]
#[repr(C)]
#[derive(Debug)]
pub struct SoapServices {
    __cordl_parent: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
}
#[cfg(feature = "System+Runtime+Remoting+SoapServices")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Runtime::Remoting::SoapServices =>
    "System.Runtime.Remoting"."SoapServices"
);
#[cfg(feature = "System+Runtime+Remoting+SoapServices")]
impl std::ops::Deref for crate::System::Runtime::Remoting::SoapServices {
    type Target = quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Runtime+Remoting+SoapServices")]
impl std::ops::DerefMut for crate::System::Runtime::Remoting::SoapServices {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Runtime+Remoting+SoapServices")]
impl crate::System::Runtime::Remoting::SoapServices {
    #[cfg(feature = "System+Runtime+Remoting+SoapServices+TypeInfo")]
    pub type TypeInfo = crate::System::Runtime::Remoting::SoapServices_TypeInfo;
    pub fn CodeXmlNamespaceForClrTypeNamespace(
        typeNamespace: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        assemblyName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "CodeXmlNamespaceForClrTypeNamespace",
                (typeNamespace, assemblyName),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn EncodeNs(
        ns: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class().invoke("EncodeNs", (ns))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetAssemblyName(
        mb: quest_hook::libil2cpp::Gc<crate::System::Reflection::MethodBase>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetAssemblyName", (mb))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetNameKey(
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        namspace: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetNameKey", (name, namspace))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetXmlElementForInteropType(
        _cordl_type: quest_hook::libil2cpp::Gc<crate::System::Type>,
        xmlElement: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        >,
        xmlNamespace: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "GetXmlElementForInteropType",
                (_cordl_type, xmlElement, xmlNamespace),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn GetXmlNamespaceForMethodCall(
        mb: quest_hook::libil2cpp::Gc<crate::System::Reflection::MethodBase>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetXmlNamespaceForMethodCall", (mb))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetXmlNamespaceForMethodResponse(
        mb: quest_hook::libil2cpp::Gc<crate::System::Reflection::MethodBase>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetXmlNamespaceForMethodResponse", (mb))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetXmlTypeForInteropType(
        _cordl_type: quest_hook::libil2cpp::Gc<crate::System::Type>,
        xmlType: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        >,
        xmlTypeNamespace: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "GetXmlTypeForInteropType",
                (_cordl_type, xmlType, xmlTypeNamespace),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn PreLoad_Gc0(
        assembly: quest_hook::libil2cpp::Gc<crate::System::Reflection::Assembly>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("PreLoad", (assembly))?;
        Ok(__cordl_ret.into())
    }
    pub fn PreLoad_Gc1(
        _cordl_type: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("PreLoad", (_cordl_type))?;
        Ok(__cordl_ret.into())
    }
    pub fn RegisterInteropXmlElement(
        xmlElement: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        xmlNamespace: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        _cordl_type: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "RegisterInteropXmlElement",
                (xmlElement, xmlNamespace, _cordl_type),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn RegisterInteropXmlType(
        xmlType: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        xmlTypeNamespace: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        _cordl_type: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("RegisterInteropXmlType", (xmlType, xmlTypeNamespace, _cordl_type))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_XmlNsForClrTypeWithAssembly() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_XmlNsForClrTypeWithAssembly", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_XmlNsForClrTypeWithNs() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_XmlNsForClrTypeWithNs", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_XmlNsForClrTypeWithNsAndAssembly() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_XmlNsForClrTypeWithNsAndAssembly", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Runtime+Remoting+SoapServices")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Runtime::Remoting::SoapServices {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "System+Runtime+Remoting+SoapServices+TypeInfo")]
#[repr(C)]
#[derive(Debug)]
pub struct SoapServices_TypeInfo {
    __cordl_parent: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    pub Attributes: quest_hook::libil2cpp::Gc<crate::System::Collections::Hashtable>,
    pub Elements: quest_hook::libil2cpp::Gc<crate::System::Collections::Hashtable>,
}
#[cfg(feature = "System+Runtime+Remoting+SoapServices+TypeInfo")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Runtime::Remoting::SoapServices_TypeInfo
    => "System.Runtime.Remoting"."SoapServices/TypeInfo"
);
#[cfg(feature = "System+Runtime+Remoting+SoapServices+TypeInfo")]
impl std::ops::Deref for crate::System::Runtime::Remoting::SoapServices_TypeInfo {
    type Target = quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Runtime+Remoting+SoapServices+TypeInfo")]
impl std::ops::DerefMut for crate::System::Runtime::Remoting::SoapServices_TypeInfo {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Runtime+Remoting+SoapServices+TypeInfo")]
impl crate::System::Runtime::Remoting::SoapServices_TypeInfo {
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
#[cfg(feature = "System+Runtime+Remoting+SoapServices+TypeInfo")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Runtime::Remoting::SoapServices_TypeInfo {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
