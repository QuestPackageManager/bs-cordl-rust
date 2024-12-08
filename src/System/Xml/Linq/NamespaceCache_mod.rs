#[cfg(feature = "System+Xml+Linq+NamespaceCache")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct NamespaceCache {
    pub _ns: *mut crate::System::Xml::Linq::XNamespace,
    pub _namespaceName: *mut crate::System::String,
}
#[cfg(feature = "System+Xml+Linq+NamespaceCache")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::System::Xml::Linq::NamespaceCache =>
    "System.Xml.Linq"."NamespaceCache"
);
#[cfg(feature = "System+Xml+Linq+NamespaceCache")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::System::Xml::Linq::NamespaceCache {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "System+Xml+Linq+NamespaceCache")]
impl crate::System::Xml::Linq::NamespaceCache {
    pub fn Get(
        &mut self,
        namespaceName: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Xml::Linq::XNamespace> {
        let __cordl_ret: *mut crate::System::Xml::Linq::XNamespace = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Get",
            (namespaceName),
        )?;
        Ok(__cordl_ret)
    }
}