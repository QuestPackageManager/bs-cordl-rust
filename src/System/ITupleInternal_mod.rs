#[cfg(feature = "System+ITupleInternal")]
#[repr(C)]
#[derive(Debug)]
pub struct ITupleInternal {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "System+ITupleInternal")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::ITupleInternal => "System"
    ."ITupleInternal"
);
#[cfg(feature = "System+ITupleInternal")]
impl std::ops::Deref for crate::System::ITupleInternal {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+ITupleInternal")]
impl std::ops::DerefMut for crate::System::ITupleInternal {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+ITupleInternal")]
impl crate::System::ITupleInternal {
    pub fn ToString(
        &mut self,
        sb: quest_hook::libil2cpp::Gc<crate::System::Text::StringBuilder>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("ToString", (sb))?;
        Ok(__cordl_ret.into())
    }
    pub fn from_object_mut(
        object_param: *mut quest_hook::libil2cpp::Il2CppObject,
    ) -> *mut Self {
        unsafe { (object_param as *mut Self) }
    }
}
#[cfg(feature = "System+ITupleInternal")]
impl quest_hook::libil2cpp::ObjectType for crate::System::ITupleInternal {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "System+ITupleInternal")]
impl AsRef<crate::System::Runtime::CompilerServices::ITuple>
for crate::System::ITupleInternal {
    fn as_ref(&self) -> &crate::System::Runtime::CompilerServices::ITuple {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "System+ITupleInternal")]
impl AsMut<crate::System::Runtime::CompilerServices::ITuple>
for crate::System::ITupleInternal {
    fn as_mut(&mut self) -> &mut crate::System::Runtime::CompilerServices::ITuple {
        unsafe { std::mem::transmute(self) }
    }
}
