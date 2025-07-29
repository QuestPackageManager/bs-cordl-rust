#[cfg(feature = "cordl_class_System+Xml+Schema+KSStruct")]
#[repr(C)]
#[derive(Debug)]
pub struct KSStruct {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub depth: i32,
    pub ks: quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::KeySequence>,
    pub fields: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<
            quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::LocatedActiveAxis>,
        >,
    >,
}
#[cfg(feature = "cordl_class_System+Xml+Schema+KSStruct")]
unsafe impl quest_hook::libil2cpp::Type for crate::System::Xml::Schema::KSStruct {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "System.Xml.Schema";
    const CLASS_NAME: &'static str = "KSStruct";
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
#[cfg(feature = "cordl_class_System+Xml+Schema+KSStruct")]
impl std::ops::Deref for crate::System::Xml::Schema::KSStruct {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "cordl_class_System+Xml+Schema+KSStruct")]
impl std::ops::DerefMut for crate::System::Xml::Schema::KSStruct {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Xml+Schema+KSStruct")]
impl crate::System::Xml::Schema::KSStruct {
    pub fn New(
        ks: quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::KeySequence>,
        dim: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (ks, dim))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        ks: quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::KeySequence>,
        dim: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::System::Xml::Schema::KeySequence,
                            >,
                            i32,
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
            cordl_method_info.invoke_unchecked(self, (ks, dim))?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "cordl_class_System+Xml+Schema+KSStruct")]
impl quest_hook::libil2cpp::ObjectType for crate::System::Xml::Schema::KSStruct {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
