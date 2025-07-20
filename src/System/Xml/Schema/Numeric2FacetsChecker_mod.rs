#[cfg(feature = "System+Xml+Schema+Numeric2FacetsChecker")]
#[repr(C)]
#[derive(Debug)]
pub struct Numeric2FacetsChecker {
    __cordl_parent: crate::System::Xml::Schema::FacetsChecker,
}
#[cfg(feature = "System+Xml+Schema+Numeric2FacetsChecker")]
unsafe impl quest_hook::libil2cpp::Type
for crate::System::Xml::Schema::Numeric2FacetsChecker {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "System.Xml.Schema";
    const CLASS_NAME: &'static str = "Numeric2FacetsChecker";
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
#[cfg(feature = "System+Xml+Schema+Numeric2FacetsChecker")]
impl std::ops::Deref for crate::System::Xml::Schema::Numeric2FacetsChecker {
    type Target = crate::System::Xml::Schema::FacetsChecker;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Xml+Schema+Numeric2FacetsChecker")]
impl std::ops::DerefMut for crate::System::Xml::Schema::Numeric2FacetsChecker {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Xml+Schema+Numeric2FacetsChecker")]
impl crate::System::Xml::Schema::Numeric2FacetsChecker {
    pub fn CheckValueFacets_Il2CppObject0(
        &mut self,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        datatype: quest_hook::libil2cpp::Gc<
            crate::System::Xml::Schema::XmlSchemaDatatype,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Exception>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppObject,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::System::Xml::Schema::XmlSchemaDatatype,
                            >,
                        ),
                        quest_hook::libil2cpp::Gc<crate::System::Exception>,
                        2usize,
                    >("CheckValueFacets")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "CheckValueFacets", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Exception> = unsafe {
            method.invoke_unchecked(self, (value, datatype))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn CheckValueFacets_f32_2(
        &mut self,
        value: f32,
        datatype: quest_hook::libil2cpp::Gc<
            crate::System::Xml::Schema::XmlSchemaDatatype,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Exception>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (
                            f32,
                            quest_hook::libil2cpp::Gc<
                                crate::System::Xml::Schema::XmlSchemaDatatype,
                            >,
                        ),
                        quest_hook::libil2cpp::Gc<crate::System::Exception>,
                        2usize,
                    >("CheckValueFacets")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "CheckValueFacets", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Exception> = unsafe {
            method.invoke_unchecked(self, (value, datatype))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn CheckValueFacets_f64_1(
        &mut self,
        value: f64,
        datatype: quest_hook::libil2cpp::Gc<
            crate::System::Xml::Schema::XmlSchemaDatatype,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Exception>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (
                            f64,
                            quest_hook::libil2cpp::Gc<
                                crate::System::Xml::Schema::XmlSchemaDatatype,
                            >,
                        ),
                        quest_hook::libil2cpp::Gc<crate::System::Exception>,
                        2usize,
                    >("CheckValueFacets")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "CheckValueFacets", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Exception> = unsafe {
            method.invoke_unchecked(self, (value, datatype))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn MatchEnumeration_Il2CppObject_XmlSchemaDatatype0(
        &mut self,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        enumeration: quest_hook::libil2cpp::Gc<crate::System::Collections::ArrayList>,
        datatype: quest_hook::libil2cpp::Gc<
            crate::System::Xml::Schema::XmlSchemaDatatype,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppObject,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::System::Collections::ArrayList,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::System::Xml::Schema::XmlSchemaDatatype,
                            >,
                        ),
                        bool,
                        3usize,
                    >("MatchEnumeration")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "MatchEnumeration", 3usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            method.invoke_unchecked(self, (value, enumeration, datatype))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn MatchEnumeration_f64_XmlValueConverter1(
        &mut self,
        value: f64,
        enumeration: quest_hook::libil2cpp::Gc<crate::System::Collections::ArrayList>,
        valueConverter: quest_hook::libil2cpp::Gc<
            crate::System::Xml::Schema::XmlValueConverter,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (
                            f64,
                            quest_hook::libil2cpp::Gc<
                                crate::System::Collections::ArrayList,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::System::Xml::Schema::XmlValueConverter,
                            >,
                        ),
                        bool,
                        3usize,
                    >("MatchEnumeration")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "MatchEnumeration", 3usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            method.invoke_unchecked(self, (value, enumeration, valueConverter))?
        };
        Ok(__cordl_ret.into())
    }
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
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<(), quest_hook::libil2cpp::Void, 0usize>(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), ".ctor", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Xml+Schema+Numeric2FacetsChecker")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Xml::Schema::Numeric2FacetsChecker {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
