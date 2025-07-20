#[cfg(feature = "System+Xml+Schema+Asttree")]
#[repr(C)]
#[derive(Debug)]
pub struct Asttree {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _fAxisArray: quest_hook::libil2cpp::Gc<crate::System::Collections::ArrayList>,
    pub _xpathexpr: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub _isField: bool,
    pub _nsmgr: quest_hook::libil2cpp::Gc<crate::System::Xml::XmlNamespaceManager>,
}
#[cfg(feature = "System+Xml+Schema+Asttree")]
unsafe impl quest_hook::libil2cpp::Type for crate::System::Xml::Schema::Asttree {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "System.Xml.Schema";
    const CLASS_NAME: &'static str = "Asttree";
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
#[cfg(feature = "System+Xml+Schema+Asttree")]
impl std::ops::Deref for crate::System::Xml::Schema::Asttree {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Xml+Schema+Asttree")]
impl std::ops::DerefMut for crate::System::Xml::Schema::Asttree {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Xml+Schema+Asttree")]
impl crate::System::Xml::Schema::Asttree {
    pub fn CompileXPath(
        &mut self,
        xPath: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        isField: bool,
        nsmgr: quest_hook::libil2cpp::Gc<crate::System::Xml::XmlNamespaceManager>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                            bool,
                            quest_hook::libil2cpp::Gc<
                                crate::System::Xml::XmlNamespaceManager,
                            >,
                        ),
                        quest_hook::libil2cpp::Void,
                        3usize,
                    >("CompileXPath")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "CompileXPath", 3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (xPath, isField, nsmgr))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn IsAttribute(
        ast: quest_hook::libil2cpp::Gc<crate::MS::Internal::Xml::XPath::Axis>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_static_method::<
                        (quest_hook::libil2cpp::Gc<
                            crate::MS::Internal::Xml::XPath::Axis,
                        >),
                        bool,
                        1usize,
                    >("IsAttribute")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "IsAttribute", 1usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked((), (ast))? };
        Ok(__cordl_ret.into())
    }
    pub fn IsDescendantOrSelf(
        ast: quest_hook::libil2cpp::Gc<crate::MS::Internal::Xml::XPath::Axis>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_static_method::<
                        (quest_hook::libil2cpp::Gc<
                            crate::MS::Internal::Xml::XPath::Axis,
                        >),
                        bool,
                        1usize,
                    >("IsDescendantOrSelf")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "IsDescendantOrSelf", 1usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked((), (ast))? };
        Ok(__cordl_ret.into())
    }
    pub fn IsNameTest(
        ast: quest_hook::libil2cpp::Gc<crate::MS::Internal::Xml::XPath::Axis>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_static_method::<
                        (quest_hook::libil2cpp::Gc<
                            crate::MS::Internal::Xml::XPath::Axis,
                        >),
                        bool,
                        1usize,
                    >("IsNameTest")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "IsNameTest", 1usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked((), (ast))? };
        Ok(__cordl_ret.into())
    }
    pub fn IsSelf(
        ast: quest_hook::libil2cpp::Gc<crate::MS::Internal::Xml::XPath::Axis>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_static_method::<
                        (quest_hook::libil2cpp::Gc<
                            crate::MS::Internal::Xml::XPath::Axis,
                        >),
                        bool,
                        1usize,
                    >("IsSelf")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "IsSelf", 1usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked((), (ast))? };
        Ok(__cordl_ret.into())
    }
    pub fn New(
        xPath: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        isField: bool,
        nsmgr: quest_hook::libil2cpp::Gc<crate::System::Xml::XmlNamespaceManager>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (xPath, isField, nsmgr))?;
        Ok(__cordl_object.into())
    }
    pub fn SetURN(
        &mut self,
        axis: quest_hook::libil2cpp::Gc<crate::MS::Internal::Xml::XPath::Axis>,
        nsmgr: quest_hook::libil2cpp::Gc<crate::System::Xml::XmlNamespaceManager>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::MS::Internal::Xml::XPath::Axis,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::System::Xml::XmlNamespaceManager,
                            >,
                        ),
                        quest_hook::libil2cpp::Void,
                        2usize,
                    >("SetURN")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "SetURN", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (axis, nsmgr))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        xPath: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        isField: bool,
        nsmgr: quest_hook::libil2cpp::Gc<crate::System::Xml::XmlNamespaceManager>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                            bool,
                            quest_hook::libil2cpp::Gc<
                                crate::System::Xml::XmlNamespaceManager,
                            >,
                        ),
                        quest_hook::libil2cpp::Void,
                        3usize,
                    >(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), ".ctor", 3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (xPath, isField, nsmgr))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_SubtreeArray(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Collections::ArrayList>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (),
                        quest_hook::libil2cpp::Gc<crate::System::Collections::ArrayList>,
                        0usize,
                    >("get_SubtreeArray")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "get_SubtreeArray", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::ArrayList,
        > = unsafe { method.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Xml+Schema+Asttree")]
impl quest_hook::libil2cpp::ObjectType for crate::System::Xml::Schema::Asttree {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
