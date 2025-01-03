#[cfg(feature = "SR")]
#[repr(C)]
#[derive(Debug)]
pub struct SR {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "SR")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::SR => ""."SR"
);
#[cfg(feature = "SR")]
impl std::ops::Deref for crate::GlobalNamespace::SR {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "SR")]
impl std::ops::DerefMut for crate::GlobalNamespace::SR {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "SR")]
impl crate::GlobalNamespace::SR {
    pub fn Format_Il2CppObject1(
        resourceFormat: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        p1: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        p2: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Format", (resourceFormat, p1, p2))?;
        Ok(__cordl_ret.into())
    }
    pub fn Format_Il2CppObject_Il2CppObject2(
        resourceFormat: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        p1: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        p2: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        p3: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Format", (resourceFormat, p1, p2, p3))?;
        Ok(__cordl_ret.into())
    }
    pub fn Format_Il2CppString_Il2CppObject0(
        resourceFormat: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        p1: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Format", (resourceFormat, p1))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetResourceString(
        str: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetResourceString", (str))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetString_CultureInfo_Il2CppString_Il2CppArray1(
        culture: quest_hook::libil2cpp::Gc<crate::System::Globalization::CultureInfo>,
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        args: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<*mut quest_hook::libil2cpp::Il2CppObject>,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetString", (culture, name, args))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetString_Il2CppString2(
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class().invoke("GetString", (name))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetString_Il2CppString_Il2CppArray0(
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        args: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<*mut quest_hook::libil2cpp::Il2CppObject>,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetString", (name, args))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "SR")]
impl quest_hook::libil2cpp::ObjectType for crate::GlobalNamespace::SR {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
