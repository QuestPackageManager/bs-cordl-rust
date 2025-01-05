#[cfg(feature = "System+Xml+Schema+XmlSchemaParticle+EmptyParticle")]
#[repr(C)]
#[derive(Debug)]
pub struct XmlSchemaParticle_EmptyParticle {
    __cordl_parent: quest_hook::libil2cpp::Gc<
        crate::System::Xml::Schema::XmlSchemaParticle,
    >,
}
#[cfg(feature = "System+Xml+Schema+XmlSchemaParticle+EmptyParticle")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::XmlSchemaParticle_EmptyParticle
    => "System.Xml.Schema"."XmlSchemaParticle/EmptyParticle"
);
#[cfg(feature = "System+Xml+Schema+XmlSchemaParticle+EmptyParticle")]
impl std::ops::Deref for crate::GlobalNamespace::XmlSchemaParticle_EmptyParticle {
    type Target = quest_hook::libil2cpp::Gc<
        crate::System::Xml::Schema::XmlSchemaParticle,
    >;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Xml+Schema+XmlSchemaParticle+EmptyParticle")]
impl std::ops::DerefMut for crate::GlobalNamespace::XmlSchemaParticle_EmptyParticle {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Xml+Schema+XmlSchemaParticle+EmptyParticle")]
impl crate::GlobalNamespace::XmlSchemaParticle_EmptyParticle {
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
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_IsEmpty(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_IsEmpty", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Xml+Schema+XmlSchemaParticle+EmptyParticle")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::XmlSchemaParticle_EmptyParticle {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
