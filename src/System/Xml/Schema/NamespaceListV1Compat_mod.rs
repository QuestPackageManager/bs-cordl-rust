#[cfg(feature = "System+Xml+Schema+NamespaceListV1Compat")]
#[repr(C)]
#[derive(Debug)]
pub struct NamespaceListV1Compat {
    __cordl_parent: crate::System::Xml::Schema::NamespaceList,
}
#[cfg(feature = "System+Xml+Schema+NamespaceListV1Compat")]
unsafe impl quest_hook::libil2cpp::Type
for crate::System::Xml::Schema::NamespaceListV1Compat {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "System.Xml.Schema";
    const CLASS_NAME: &'static str = "NamespaceListV1Compat";
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
#[cfg(feature = "System+Xml+Schema+NamespaceListV1Compat")]
impl std::ops::Deref for crate::System::Xml::Schema::NamespaceListV1Compat {
    type Target = crate::System::Xml::Schema::NamespaceList;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Xml+Schema+NamespaceListV1Compat")]
impl std::ops::DerefMut for crate::System::Xml::Schema::NamespaceListV1Compat {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Xml+Schema+NamespaceListV1Compat")]
impl crate::System::Xml::Schema::NamespaceListV1Compat {
    pub fn Allows(
        &mut self,
        ns: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>),
                bool,
                1usize,
            >("Allows")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "Allows", 1usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked(self, (ns)) };
        Ok(__cordl_ret.into())
    }
    pub fn New(
        namespaces: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        targetNamespace: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (namespaces, targetNamespace))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        namespaces: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        targetNamespace: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                ),
                quest_hook::libil2cpp::Void,
                2usize,
            >(".ctor")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, ".ctor", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (namespaces, targetNamespace))
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Xml+Schema+NamespaceListV1Compat")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Xml::Schema::NamespaceListV1Compat {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
