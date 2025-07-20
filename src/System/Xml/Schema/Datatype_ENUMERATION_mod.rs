#[cfg(feature = "System+Xml+Schema+Datatype_ENUMERATION")]
#[repr(C)]
#[derive(Debug)]
pub struct Datatype_ENUMERATION {
    __cordl_parent: crate::System::Xml::Schema::Datatype_NMTOKEN,
}
#[cfg(feature = "System+Xml+Schema+Datatype_ENUMERATION")]
unsafe impl quest_hook::libil2cpp::Type
for crate::System::Xml::Schema::Datatype_ENUMERATION {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "System.Xml.Schema";
    const CLASS_NAME: &'static str = "Datatype_ENUMERATION";
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
#[cfg(feature = "System+Xml+Schema+Datatype_ENUMERATION")]
impl std::ops::Deref for crate::System::Xml::Schema::Datatype_ENUMERATION {
    type Target = crate::System::Xml::Schema::Datatype_NMTOKEN;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Xml+Schema+Datatype_ENUMERATION")]
impl std::ops::DerefMut for crate::System::Xml::Schema::Datatype_ENUMERATION {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Xml+Schema+Datatype_ENUMERATION")]
impl crate::System::Xml::Schema::Datatype_ENUMERATION {
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
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Xml::Schema::Datatype_ENUMERATION as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), quest_hook::libil2cpp::Void, 0usize>(".ctor")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Xml::Schema::Datatype_ENUMERATION as
                    quest_hook::libil2cpp::Type > ::class(), ".ctor", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_TokenizedType(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::System::Xml::XmlTokenizedType> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Xml::Schema::Datatype_ENUMERATION as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                crate::System::Xml::XmlTokenizedType,
                0usize,
            >("get_TokenizedType")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Xml::Schema::Datatype_ENUMERATION as
                    quest_hook::libil2cpp::Type > ::class(), "get_TokenizedType", 0usize
                )
            });
        let __cordl_ret: crate::System::Xml::XmlTokenizedType = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Xml+Schema+Datatype_ENUMERATION")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Xml::Schema::Datatype_ENUMERATION {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
