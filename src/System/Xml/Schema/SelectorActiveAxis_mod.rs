#[cfg(feature = "System+Xml+Schema+SelectorActiveAxis")]
#[repr(C)]
#[derive(Debug)]
pub struct SelectorActiveAxis {
    __cordl_parent: crate::System::Xml::Schema::ActiveAxis,
    pub cs: *mut crate::System::Xml::Schema::ConstraintStruct,
    pub KSs: *mut crate::System::Collections::ArrayList,
    pub KSpointer: i32,
}
#[cfg(feature = "System+Xml+Schema+SelectorActiveAxis")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Xml::Schema::SelectorActiveAxis =>
    "System.Xml.Schema"."SelectorActiveAxis"
);
#[cfg(feature = "System+Xml+Schema+SelectorActiveAxis")]
impl std::ops::Deref for crate::System::Xml::Schema::SelectorActiveAxis {
    type Target = crate::System::Xml::Schema::ActiveAxis;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Xml+Schema+SelectorActiveAxis")]
impl std::ops::DerefMut for crate::System::Xml::Schema::SelectorActiveAxis {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Xml+Schema+SelectorActiveAxis")]
impl crate::System::Xml::Schema::SelectorActiveAxis {
    pub fn EndElement(
        &mut self,
        localname: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        URN: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("EndElement", (localname, URN))?;
        Ok(__cordl_ret.into())
    }
    pub fn New(
        axisTree: quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::Asttree>,
        cs: quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::ConstraintStruct>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (axisTree, cs))?;
        Ok(__cordl_object.into())
    }
    pub fn PopKS(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::KeySequence>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Xml::Schema::KeySequence,
        > = __cordl_object.invoke("PopKS", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn PushKS(
        &mut self,
        errline: i32,
        errcol: i32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("PushKS", (errline, errcol))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        axisTree: quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::Asttree>,
        cs: quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::ConstraintStruct>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (axisTree, cs))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_lastDepth(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_lastDepth", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Xml+Schema+SelectorActiveAxis")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Xml::Schema::SelectorActiveAxis {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
