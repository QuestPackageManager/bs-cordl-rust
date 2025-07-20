#[cfg(feature = "System+Xml+Schema+ConstraintStruct")]
#[repr(C)]
#[derive(Debug)]
pub struct ConstraintStruct {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub constraint: quest_hook::libil2cpp::Gc<
        crate::System::Xml::Schema::CompiledIdentityConstraint,
    >,
    pub axisSelector: quest_hook::libil2cpp::Gc<
        crate::System::Xml::Schema::SelectorActiveAxis,
    >,
    pub axisFields: quest_hook::libil2cpp::Gc<crate::System::Collections::ArrayList>,
    pub qualifiedTable: quest_hook::libil2cpp::Gc<crate::System::Collections::Hashtable>,
    pub keyrefTable: quest_hook::libil2cpp::Gc<crate::System::Collections::Hashtable>,
    pub tableDim: i32,
}
#[cfg(feature = "System+Xml+Schema+ConstraintStruct")]
unsafe impl quest_hook::libil2cpp::Type
for crate::System::Xml::Schema::ConstraintStruct {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "System.Xml.Schema";
    const CLASS_NAME: &'static str = "ConstraintStruct";
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
#[cfg(feature = "System+Xml+Schema+ConstraintStruct")]
impl std::ops::Deref for crate::System::Xml::Schema::ConstraintStruct {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Xml+Schema+ConstraintStruct")]
impl std::ops::DerefMut for crate::System::Xml::Schema::ConstraintStruct {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Xml+Schema+ConstraintStruct")]
impl crate::System::Xml::Schema::ConstraintStruct {
    pub fn New(
        constraint: quest_hook::libil2cpp::Gc<
            crate::System::Xml::Schema::CompiledIdentityConstraint,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (constraint))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        constraint: quest_hook::libil2cpp::Gc<
            crate::System::Xml::Schema::CompiledIdentityConstraint,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<
                            crate::System::Xml::Schema::CompiledIdentityConstraint,
                        >),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), ".ctor", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (constraint))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_TableDim(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<(), i32, 0usize>("get_TableDim")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "get_TableDim", 0usize
                        )
                    })
            });
        let __cordl_ret: i32 = unsafe { method.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Xml+Schema+ConstraintStruct")]
impl quest_hook::libil2cpp::ObjectType for crate::System::Xml::Schema::ConstraintStruct {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
