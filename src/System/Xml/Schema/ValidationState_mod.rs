#[cfg(feature = "System+Xml+Schema+ValidationState")]
#[repr(C)]
#[derive(Debug)]
pub struct ValidationState {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub IsNill: bool,
    pub IsDefault: bool,
    pub NeedValidateChildren: bool,
    pub CheckRequiredAttribute: bool,
    pub ValidationSkipped: bool,
    pub ProcessContents: crate::System::Xml::Schema::XmlSchemaContentProcessing,
    pub Validity: crate::System::Xml::Schema::XmlSchemaValidity,
    pub ElementDecl: quest_hook::libil2cpp::Gc<
        crate::System::Xml::Schema::SchemaElementDecl,
    >,
    pub ElementDeclBeforeXsi: quest_hook::libil2cpp::Gc<
        crate::System::Xml::Schema::SchemaElementDecl,
    >,
    pub LocalName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub Namespace: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub Constr: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<
            quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::ConstraintStruct>,
        >,
    >,
    pub CurrentState: crate::System::Xml::Schema::StateUnion,
    pub HasMatched: bool,
    pub CurPos: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<
            quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::BitSet>,
        >,
    >,
    pub AllElementsSet: quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::BitSet>,
    pub RunningPositions: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::List_1<
            crate::System::Xml::Schema::RangePositionInfo,
        >,
    >,
    pub TooComplex: bool,
}
#[cfg(feature = "System+Xml+Schema+ValidationState")]
unsafe impl quest_hook::libil2cpp::Type for crate::System::Xml::Schema::ValidationState {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "System.Xml.Schema";
    const CLASS_NAME: &'static str = "ValidationState";
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
#[cfg(feature = "System+Xml+Schema+ValidationState")]
impl std::ops::Deref for crate::System::Xml::Schema::ValidationState {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Xml+Schema+ValidationState")]
impl std::ops::DerefMut for crate::System::Xml::Schema::ValidationState {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Xml+Schema+ValidationState")]
impl crate::System::Xml::Schema::ValidationState {
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
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), quest_hook::libil2cpp::Void, 0usize>(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), ".ctor",
                            0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Xml+Schema+ValidationState")]
impl quest_hook::libil2cpp::ObjectType for crate::System::Xml::Schema::ValidationState {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
