#[cfg(feature = "System+Xml+XmlNamespaceManager+NamespaceDeclaration")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct XmlNamespaceManager_NamespaceDeclaration {
    pub prefix: *mut crate::System::String,
    pub uri: *mut crate::System::String,
    pub scopeId: i32,
    pub previousNsIndex: i32,
}
#[cfg(feature = "System+Xml+XmlNamespaceManager+NamespaceDeclaration")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::System::Xml::XmlNamespaceManager_NamespaceDeclaration => "System.Xml"
    ."XmlNamespaceManager/NamespaceDeclaration"
);
#[cfg(feature = "System+Xml+XmlNamespaceManager+NamespaceDeclaration")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::System::Xml::XmlNamespaceManager_NamespaceDeclaration {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "System+Xml+XmlNamespaceManager+NamespaceDeclaration")]
impl crate::System::Xml::XmlNamespaceManager_NamespaceDeclaration {
    pub fn Set(
        &mut self,
        prefix: *mut crate::System::String,
        uri: *mut crate::System::String,
        scopeId: i32,
        previousNsIndex: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Set",
            (prefix, uri, scopeId, previousNsIndex),
        )?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "System+Xml+XmlNamespaceManager")]
#[repr(C)]
#[derive(Debug)]
pub struct XmlNamespaceManager {
    __cordl_parent: crate::System::Object,
    pub nsdecls: *mut quest_hook::libil2cpp::Il2CppArray<
        crate::System::Xml::XmlNamespaceManager_NamespaceDeclaration,
    >,
    pub lastDecl: i32,
    pub nameTable: *mut crate::System::Xml::XmlNameTable,
    pub scopeId: i32,
    pub hashTable: *mut crate::System::Collections::Generic::Dictionary_2<
        *mut crate::System::String,
        i32,
    >,
    pub useHashtable: bool,
    pub xml: *mut crate::System::String,
    pub xmlNs: *mut crate::System::String,
}
#[cfg(feature = "System+Xml+XmlNamespaceManager")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Xml::XmlNamespaceManager => "System.Xml"
    ."XmlNamespaceManager"
);
#[cfg(feature = "System+Xml+XmlNamespaceManager")]
impl std::ops::Deref for crate::System::Xml::XmlNamespaceManager {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Xml+XmlNamespaceManager")]
impl std::ops::DerefMut for crate::System::Xml::XmlNamespaceManager {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Xml+XmlNamespaceManager")]
impl crate::System::Xml::XmlNamespaceManager {
    #[cfg(feature = "System+Xml+XmlNamespaceManager+NamespaceDeclaration")]
    pub type NamespaceDeclaration = crate::System::Xml::XmlNamespaceManager_NamespaceDeclaration;
    pub fn LookupNamespace(
        &mut self,
        prefix: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("LookupNamespace", (prefix))?;
        Ok(__cordl_ret)
    }
    pub fn get_NameTable(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Xml::XmlNameTable> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Xml::XmlNameTable = __cordl_object
            .invoke("get_NameTable", ())?;
        Ok(__cordl_ret)
    }
    pub fn LookupNamespaceDecl(
        &mut self,
        prefix: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("LookupNamespaceDecl", (prefix))?;
        Ok(__cordl_ret)
    }
    pub fn LookupPrefix(
        &mut self,
        uri: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("LookupPrefix", (uri))?;
        Ok(__cordl_ret)
    }
    pub fn GetEnumerator(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Collections::IEnumerator> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Collections::IEnumerator = __cordl_object
            .invoke("GetEnumerator", ())?;
        Ok(__cordl_ret)
    }
    pub fn GetNamespacesInScope(
        &mut self,
        scope: crate::System::Xml::XmlNamespaceScope,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Collections::Generic::IDictionary_2<
            *mut crate::System::String,
            *mut crate::System::String,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Collections::Generic::IDictionary_2<
            *mut crate::System::String,
            *mut crate::System::String,
        > = __cordl_object.invoke("GetNamespacesInScope", (scope))?;
        Ok(__cordl_ret)
    }
    pub fn RemoveNamespace(
        &mut self,
        prefix: *mut crate::System::String,
        uri: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("RemoveNamespace", (prefix, uri))?;
        Ok(__cordl_ret)
    }
    pub fn PushScope(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("PushScope", ())?;
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
    pub fn _ctor_XmlNameTable1(
        &mut self,
        nameTable: *mut crate::System::Xml::XmlNameTable,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (nameTable))?;
        Ok(__cordl_ret)
    }
    pub fn PopScope(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("PopScope", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_DefaultNamespace(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("get_DefaultNamespace", ())?;
        Ok(__cordl_ret)
    }
    pub fn AddNamespace(
        &mut self,
        prefix: *mut crate::System::String,
        uri: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AddNamespace", (prefix, uri))?;
        Ok(__cordl_ret)
    }
    pub fn New_0() -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
    pub fn New_XmlNameTable1(
        nameTable: *mut crate::System::Xml::XmlNameTable,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (nameTable))?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "System+Xml+XmlNamespaceManager")]
impl quest_hook::libil2cpp::ObjectType for crate::System::Xml::XmlNamespaceManager {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
