#[cfg(feature = "cordl_class_System+Xml+Schema+Datatype_dayTimeDuration")]
#[repr(C)]
#[derive(Debug)]
pub struct Datatype_dayTimeDuration {
    __cordl_parent: crate::System::Xml::Schema::Datatype_duration,
}
#[cfg(feature = "cordl_class_System+Xml+Schema+Datatype_dayTimeDuration")]
unsafe impl quest_hook::libil2cpp::Type for crate::System::Xml::Schema::Datatype_dayTimeDuration {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "System.Xml.Schema";
    const CLASS_NAME: &'static str = "Datatype_dayTimeDuration";
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.class()
            .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
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
#[cfg(feature = "System+Xml+Schema+Datatype_dayTimeDuration")]
impl std::ops::Deref for crate::System::Xml::Schema::Datatype_dayTimeDuration {
    type Target = crate::System::Xml::Schema::Datatype_duration;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Xml+Schema+Datatype_dayTimeDuration")]
impl std::ops::DerefMut for crate::System::Xml::Schema::Datatype_dayTimeDuration {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Xml+Schema+Datatype_dayTimeDuration")]
impl crate::System::Xml::Schema::Datatype_dayTimeDuration {
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self =
            <Self as quest_hook::libil2cpp::Type>::class().instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn TryParseValue(
        &mut self,
        s: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        nameTable: quest_hook::libil2cpp::Gc<crate::System::Xml::XmlNameTable>,
        nsmgr: quest_hook::libil2cpp::Gc<crate::System::Xml::IXmlNamespaceResolver>,
        typedValue: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<crate::System::Exception>> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        quest_hook::libil2cpp::Gc<crate::System::Xml::XmlNameTable>,
                        quest_hook::libil2cpp::Gc<crate::System::Xml::IXmlNamespaceResolver>,
                        quest_hook::libil2cpp::ByRefMut<
                            quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                        >,
                    ), quest_hook::libil2cpp::Gc<crate::System::Exception>, 4usize>(
                        "TryParseValue"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "TryParseValue",
                            4usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Exception> =
            unsafe { cordl_method_info.invoke_unchecked(self, (s, nameTable, nsmgr, typedValue))? };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(&mut self) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), quest_hook::libil2cpp::Void, 0usize>(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            ".ctor",
                            0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_TypeCode(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::System::Xml::Schema::XmlTypeCode> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), crate::System::Xml::Schema::XmlTypeCode, 0usize>(
                        "get_TypeCode",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "get_TypeCode",
                            0usize
                        )
                    })
            });
        let __cordl_ret: crate::System::Xml::Schema::XmlTypeCode =
            unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "cordl_class_System+Xml+Schema+Datatype_dayTimeDuration")]
impl quest_hook::libil2cpp::ObjectType for crate::System::Xml::Schema::Datatype_dayTimeDuration {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
