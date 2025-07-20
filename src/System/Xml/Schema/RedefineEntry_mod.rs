#[cfg(feature = "System+Xml+Schema+RedefineEntry")]
#[repr(C)]
#[derive(Debug)]
pub struct RedefineEntry {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub redefine: quest_hook::libil2cpp::Gc<
        crate::System::Xml::Schema::XmlSchemaRedefine,
    >,
    pub schemaToUpdate: quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::XmlSchema>,
}
#[cfg(feature = "System+Xml+Schema+RedefineEntry")]
unsafe impl quest_hook::libil2cpp::Type for crate::System::Xml::Schema::RedefineEntry {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "System.Xml.Schema";
    const CLASS_NAME: &'static str = "RedefineEntry";
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
#[cfg(feature = "System+Xml+Schema+RedefineEntry")]
impl std::ops::Deref for crate::System::Xml::Schema::RedefineEntry {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Xml+Schema+RedefineEntry")]
impl std::ops::DerefMut for crate::System::Xml::Schema::RedefineEntry {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Xml+Schema+RedefineEntry")]
impl crate::System::Xml::Schema::RedefineEntry {
    pub fn New(
        external: quest_hook::libil2cpp::Gc<
            crate::System::Xml::Schema::XmlSchemaRedefine,
        >,
        schema: quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::XmlSchema>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (external, schema))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        external: quest_hook::libil2cpp::Gc<
            crate::System::Xml::Schema::XmlSchemaRedefine,
        >,
        schema: quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::XmlSchema>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::System::Xml::Schema::XmlSchemaRedefine,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::System::Xml::Schema::XmlSchema,
                            >,
                        ),
                        quest_hook::libil2cpp::Void,
                        2usize,
                    >(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), ".ctor",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (external, schema))?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Xml+Schema+RedefineEntry")]
impl quest_hook::libil2cpp::ObjectType for crate::System::Xml::Schema::RedefineEntry {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
