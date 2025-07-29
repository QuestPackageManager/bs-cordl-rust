#[cfg(feature = "cordl_class_Org+BouncyCastle+Cms+SimpleAttributeTableGenerator")]
#[repr(C)]
#[derive(Debug)]
pub struct SimpleAttributeTableGenerator {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub attributes: quest_hook::libil2cpp::Gc<
        crate::Org::BouncyCastle::Asn1::Cms::AttributeTable,
    >,
}
#[cfg(feature = "cordl_class_Org+BouncyCastle+Cms+SimpleAttributeTableGenerator")]
unsafe impl quest_hook::libil2cpp::Type
for crate::Org::BouncyCastle::Cms::SimpleAttributeTableGenerator {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "Org.BouncyCastle.Cms";
    const CLASS_NAME: &'static str = "SimpleAttributeTableGenerator";
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
#[cfg(feature = "Org+BouncyCastle+Cms+SimpleAttributeTableGenerator")]
impl std::ops::Deref for crate::Org::BouncyCastle::Cms::SimpleAttributeTableGenerator {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Cms+SimpleAttributeTableGenerator")]
impl std::ops::DerefMut
for crate::Org::BouncyCastle::Cms::SimpleAttributeTableGenerator {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Cms+SimpleAttributeTableGenerator")]
impl crate::Org::BouncyCastle::Cms::SimpleAttributeTableGenerator {
    pub fn GetAttributes(
        &mut self,
        parameters: quest_hook::libil2cpp::Gc<crate::System::Collections::IDictionary>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Asn1::Cms::AttributeTable>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<
                            crate::System::Collections::IDictionary,
                        >),
                        quest_hook::libil2cpp::Gc<
                            crate::Org::BouncyCastle::Asn1::Cms::AttributeTable,
                        >,
                        1usize,
                    >("GetAttributes")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "GetAttributes", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::Cms::AttributeTable,
        > = unsafe { cordl_method_info.invoke_unchecked(self, (parameters))? };
        Ok(__cordl_ret.into())
    }
    pub fn New(
        attributes: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::Cms::AttributeTable,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (attributes))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        attributes: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::Cms::AttributeTable,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<
                            crate::Org::BouncyCastle::Asn1::Cms::AttributeTable,
                        >),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), ".ctor",
                            1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (attributes))?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "cordl_class_Org+BouncyCastle+Cms+SimpleAttributeTableGenerator")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Cms::SimpleAttributeTableGenerator {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "Org+BouncyCastle+Cms+SimpleAttributeTableGenerator")]
impl AsRef<crate::Org::BouncyCastle::Cms::CmsAttributeTableGenerator>
for crate::Org::BouncyCastle::Cms::SimpleAttributeTableGenerator {
    fn as_ref(&self) -> &crate::Org::BouncyCastle::Cms::CmsAttributeTableGenerator {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "Org+BouncyCastle+Cms+SimpleAttributeTableGenerator")]
impl AsMut<crate::Org::BouncyCastle::Cms::CmsAttributeTableGenerator>
for crate::Org::BouncyCastle::Cms::SimpleAttributeTableGenerator {
    fn as_mut(
        &mut self,
    ) -> &mut crate::Org::BouncyCastle::Cms::CmsAttributeTableGenerator {
        unsafe { std::mem::transmute(self) }
    }
}
