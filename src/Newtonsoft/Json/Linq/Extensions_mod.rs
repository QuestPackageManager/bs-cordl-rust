#[cfg(feature = "Newtonsoft+Json+Linq+Extensions")]
#[repr(C)]
#[derive(Debug)]
pub struct Extensions {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "Newtonsoft+Json+Linq+Extensions")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Newtonsoft::Json::Linq::Extensions =>
    "Newtonsoft.Json.Linq"."Extensions"
);
#[cfg(feature = "Newtonsoft+Json+Linq+Extensions")]
impl std::ops::Deref for crate::Newtonsoft::Json::Linq::Extensions {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Newtonsoft+Json+Linq+Extensions")]
impl std::ops::DerefMut for crate::Newtonsoft::Json::Linq::Extensions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Newtonsoft+Json+Linq+Extensions")]
impl crate::Newtonsoft::Json::Linq::Extensions {
    #[cfg(feature = "Newtonsoft+Json+Linq+Extensions+__c__1_1")]
    pub type __c__1_1<T: quest_hook::libil2cpp::Type> = crate::Newtonsoft::Json::Linq::Extensions___c__1_1<
        T,
    >;
    #[cfg(feature = "Newtonsoft+Json+Linq+Extensions+__c__3_1")]
    pub type __c__3_1<T: quest_hook::libil2cpp::Type> = crate::Newtonsoft::Json::Linq::Extensions___c__3_1<
        T,
    >;
    #[cfg(feature = "Newtonsoft+Json+Linq+Extensions+__c__13_2")]
    pub type __c__13_2<T: quest_hook::libil2cpp::Type, U: quest_hook::libil2cpp::Type> = crate::Newtonsoft::Json::Linq::Extensions___c__13_2<
        T,
        U,
    >;
    #[cfg(feature = "Newtonsoft+Json+Linq+Extensions+_Convert_d__14_2")]
    pub type _Convert_d__14_2<
        T: quest_hook::libil2cpp::Type,
        U: quest_hook::libil2cpp::Type,
    > = crate::Newtonsoft::Json::Linq::Extensions__Convert_d__14_2<T, U>;
    #[cfg(feature = "Newtonsoft+Json+Linq+Extensions+__c__2_1")]
    pub type __c__2_1<T: quest_hook::libil2cpp::Type> = crate::Newtonsoft::Json::Linq::Extensions___c__2_1<
        T,
    >;
    #[cfg(feature = "Newtonsoft+Json+Linq+Extensions+_Values_d__11_2")]
    pub type _Values_d__11_2<
        T: quest_hook::libil2cpp::Type,
        U: quest_hook::libil2cpp::Type,
    > = crate::Newtonsoft::Json::Linq::Extensions__Values_d__11_2<T, U>;
    #[cfg(feature = "Newtonsoft+Json+Linq+Extensions+__c")]
    pub type __c = crate::Newtonsoft::Json::Linq::Extensions___c;
    #[cfg(feature = "Newtonsoft+Json+Linq+Extensions+__c__0_1")]
    pub type __c__0_1<T: quest_hook::libil2cpp::Type> = crate::Newtonsoft::Json::Linq::Extensions___c__0_1<
        T,
    >;
}
#[cfg(feature = "Newtonsoft+Json+Linq+Extensions")]
impl quest_hook::libil2cpp::ObjectType for crate::Newtonsoft::Json::Linq::Extensions {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
