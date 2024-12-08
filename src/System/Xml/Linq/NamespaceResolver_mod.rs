#[cfg(feature = "System+Xml+Linq+NamespaceResolver+NamespaceDeclaration")]
#[repr(C)]
#[derive(Debug)]
pub struct NamespaceResolver_NamespaceDeclaration {
    __cordl_parent: crate::System::Object,
    pub prefix: *mut crate::System::String,
    pub ns: *mut crate::System::Xml::Linq::XNamespace,
    pub scope: i32,
    pub prev: *mut crate::System::Xml::Linq::NamespaceResolver_NamespaceDeclaration,
}
#[cfg(feature = "System+Xml+Linq+NamespaceResolver+NamespaceDeclaration")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::System::Xml::Linq::NamespaceResolver_NamespaceDeclaration => "System.Xml.Linq"
    ."NamespaceResolver/NamespaceDeclaration"
);
#[cfg(feature = "System+Xml+Linq+NamespaceResolver+NamespaceDeclaration")]
impl std::ops::Deref
for crate::System::Xml::Linq::NamespaceResolver_NamespaceDeclaration {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Xml+Linq+NamespaceResolver+NamespaceDeclaration")]
impl std::ops::DerefMut
for crate::System::Xml::Linq::NamespaceResolver_NamespaceDeclaration {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Xml+Linq+NamespaceResolver+NamespaceDeclaration")]
impl crate::System::Xml::Linq::NamespaceResolver_NamespaceDeclaration {
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
#[cfg(feature = "System+Xml+Linq+NamespaceResolver+NamespaceDeclaration")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Xml::Linq::NamespaceResolver_NamespaceDeclaration {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "System+Xml+Linq+NamespaceResolver")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct NamespaceResolver {
    pub _scope: i32,
    pub _declaration: *mut crate::System::Xml::Linq::NamespaceResolver_NamespaceDeclaration,
    pub _rover: *mut crate::System::Xml::Linq::NamespaceResolver_NamespaceDeclaration,
}
#[cfg(feature = "System+Xml+Linq+NamespaceResolver")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::System::Xml::Linq::NamespaceResolver =>
    "System.Xml.Linq"."NamespaceResolver"
);
#[cfg(feature = "System+Xml+Linq+NamespaceResolver")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::System::Xml::Linq::NamespaceResolver {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "System+Xml+Linq+NamespaceResolver")]
impl crate::System::Xml::Linq::NamespaceResolver {
    #[cfg(feature = "System+Xml+Linq+NamespaceResolver+NamespaceDeclaration")]
    pub type NamespaceDeclaration = crate::System::Xml::Linq::NamespaceResolver_NamespaceDeclaration;
    pub fn Add(
        &mut self,
        prefix: *mut crate::System::String,
        ns: *mut crate::System::Xml::Linq::XNamespace,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Add",
            (prefix, ns),
        )?;
        Ok(__cordl_ret)
    }
    pub fn AddFirst(
        &mut self,
        prefix: *mut crate::System::String,
        ns: *mut crate::System::Xml::Linq::XNamespace,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "AddFirst",
            (prefix, ns),
        )?;
        Ok(__cordl_ret)
    }
    pub fn GetPrefixOfNamespace(
        &mut self,
        ns: *mut crate::System::Xml::Linq::XNamespace,
        allowDefaultNamespace: bool,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_ret: *mut crate::System::String = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "GetPrefixOfNamespace",
            (ns, allowDefaultNamespace),
        )?;
        Ok(__cordl_ret)
    }
    pub fn PopScope(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "PopScope",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn PushScope(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "PushScope",
            (),
        )?;
        Ok(__cordl_ret)
    }
}
