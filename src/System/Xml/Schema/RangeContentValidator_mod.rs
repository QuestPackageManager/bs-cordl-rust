#[cfg(feature = "cordl_class_System+Xml+Schema+RangeContentValidator")]
#[repr(C)]
#[derive(Debug)]
pub struct RangeContentValidator {
    __cordl_parent: crate::System::Xml::Schema::ContentValidator,
    pub firstpos: quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::BitSet>,
    pub followpos: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<
            quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::BitSet>,
        >,
    >,
    pub positionsWithRangeTerminals: quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::BitSet>,
    pub symbols: quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::SymbolsDictionary>,
    pub positions: quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::Positions>,
    pub minMaxNodesCount: i32,
    pub endMarkerPos: i32,
}
#[cfg(feature = "cordl_class_System+Xml+Schema+RangeContentValidator")]
unsafe impl quest_hook::libil2cpp::Type for crate::System::Xml::Schema::RangeContentValidator {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "System.Xml.Schema";
    const CLASS_NAME: &'static str = "RangeContentValidator";
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
#[cfg(feature = "System+Xml+Schema+RangeContentValidator")]
impl std::ops::Deref for crate::System::Xml::Schema::RangeContentValidator {
    type Target = crate::System::Xml::Schema::ContentValidator;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Xml+Schema+RangeContentValidator")]
impl std::ops::DerefMut for crate::System::Xml::Schema::RangeContentValidator {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Xml+Schema+RangeContentValidator")]
impl crate::System::Xml::Schema::RangeContentValidator {
    pub fn CompleteValidation(
        &mut self,
        context: quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::ValidationState>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<
                            crate::System::Xml::Schema::ValidationState,
                        >),
                        bool,
                        1usize,
                    >("CompleteValidation")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "CompleteValidation", 1usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe { cordl_method_info.invoke_unchecked(self, (context))? };
        Ok(__cordl_ret.into())
    }
    pub fn ExpectedElements(
        &mut self,
        context: quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::ValidationState>,
        isRequiredOnly: bool,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Collections::ArrayList>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::ValidationState>,
                        bool,
                    ), quest_hook::libil2cpp::Gc<crate::System::Collections::ArrayList>, 2usize>(
                        "ExpectedElements",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "ExpectedElements",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Collections::ArrayList> =
            unsafe { cordl_method_info.invoke_unchecked(self, (context, isRequiredOnly))? };
        Ok(__cordl_ret.into())
    }
    pub fn ExpectedParticles(
        &mut self,
        context: quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::ValidationState>,
        isRequiredOnly: bool,
        schemaSet: quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::XmlSchemaSet>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Collections::ArrayList>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::ValidationState>,
                        bool,
                        quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::XmlSchemaSet>,
                    ), quest_hook::libil2cpp::Gc<crate::System::Collections::ArrayList>, 3usize>(
                        "ExpectedParticles",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "ExpectedParticles",
                            3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Collections::ArrayList> = unsafe {
            cordl_method_info.invoke_unchecked(self, (context, isRequiredOnly, schemaSet))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn InitValidation(
        &mut self,
        context: quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::ValidationState>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<
                            crate::System::Xml::Schema::ValidationState,
                        >),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("InitValidation")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "InitValidation", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (context))? };
        Ok(__cordl_ret.into())
    }
    pub fn New(
        firstpos: quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::BitSet>,
        followpos: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::BitSet>,
            >,
        >,
        symbols: quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::SymbolsDictionary>,
        positions: quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::Positions>,
        endMarkerPos: i32,
        contentType: crate::System::Xml::Schema::XmlSchemaContentType,
        isEmptiable: bool,
        positionsWithRangeTerminals: quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::BitSet>,
        minmaxNodesCount: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self =
            <Self as quest_hook::libil2cpp::Type>::class().instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object).invoke_void(
            ".ctor",
            (
                firstpos,
                followpos,
                symbols,
                positions,
                endMarkerPos,
                contentType,
                isEmptiable,
                positionsWithRangeTerminals,
                minmaxNodesCount,
            ),
        )?;
        Ok(__cordl_object.into())
    }
    pub fn ValidateElement(
        &mut self,
        name: quest_hook::libil2cpp::Gc<crate::System::Xml::XmlQualifiedName>,
        context: quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::ValidationState>,
        errorCode: quest_hook::libil2cpp::ByRefMut<i32>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>>
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<crate::System::Xml::XmlQualifiedName>,
                        quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::ValidationState>,
                        quest_hook::libil2cpp::ByRefMut<i32>,
                    ), quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>, 3usize>(
                        "ValidateElement",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "ValidateElement",
                            3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject> =
            unsafe { cordl_method_info.invoke_unchecked(self, (name, context, errorCode))? };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        firstpos: quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::BitSet>,
        followpos: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::BitSet>,
            >,
        >,
        symbols: quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::SymbolsDictionary>,
        positions: quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::Positions>,
        endMarkerPos: i32,
        contentType: crate::System::Xml::Schema::XmlSchemaContentType,
        isEmptiable: bool,
        positionsWithRangeTerminals: quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::BitSet>,
        minmaxNodesCount: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::BitSet>,
                        quest_hook::libil2cpp::Gc<
                            quest_hook::libil2cpp::Il2CppArray<
                                quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::BitSet>,
                            >,
                        >,
                        quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::SymbolsDictionary>,
                        quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::Positions>,
                        i32,
                        crate::System::Xml::Schema::XmlSchemaContentType,
                        bool,
                        quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::BitSet>,
                        i32,
                    ), quest_hook::libil2cpp::Void, 9usize>(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            ".ctor",
                            9usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(
                self,
                (
                    firstpos,
                    followpos,
                    symbols,
                    positions,
                    endMarkerPos,
                    contentType,
                    isEmptiable,
                    positionsWithRangeTerminals,
                    minmaxNodesCount,
                ),
            )?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "cordl_class_System+Xml+Schema+RangeContentValidator")]
impl quest_hook::libil2cpp::ObjectType for crate::System::Xml::Schema::RangeContentValidator {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
