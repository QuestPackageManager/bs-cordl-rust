#[cfg(feature = "System+Xml+Schema+ChameleonKey")]
#[repr(C)]
#[derive(Debug)]
pub struct ChameleonKey {
    __cordl_parent: crate::System::Object,
    pub targetNS: *mut crate::System::String,
    pub chameleonLocation: *mut crate::System::Uri,
    pub originalSchema: *mut crate::System::Xml::Schema::XmlSchema,
    pub hashCode: i32,
}
#[cfg(feature = "System+Xml+Schema+ChameleonKey")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Xml::Schema::ChameleonKey =>
    "System.Xml.Schema"."ChameleonKey"
);
#[cfg(feature = "System+Xml+Schema+ChameleonKey")]
impl std::ops::Deref for crate::System::Xml::Schema::ChameleonKey {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Xml+Schema+ChameleonKey")]
impl std::ops::DerefMut for crate::System::Xml::Schema::ChameleonKey {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Xml+Schema+ChameleonKey")]
impl crate::System::Xml::Schema::ChameleonKey {
    pub fn Equals(
        &mut self,
        obj: *mut crate::System::Object,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("Equals", (obj))?;
        Ok(__cordl_ret)
    }
    pub fn GetHashCode(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("GetHashCode", ())?;
        Ok(__cordl_ret)
    }
    pub fn New(
        ns: *mut crate::System::String,
        originalSchema: *mut crate::System::Xml::Schema::XmlSchema,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (ns, originalSchema))?;
        Ok(__cordl_object)
    }
    pub fn _ctor(
        &mut self,
        ns: *mut crate::System::String,
        originalSchema: *mut crate::System::Xml::Schema::XmlSchema,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (ns, originalSchema))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "System+Xml+Schema+ChameleonKey")]
impl quest_hook::libil2cpp::ObjectType for crate::System::Xml::Schema::ChameleonKey {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}