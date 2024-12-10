#[cfg(feature = "Newtonsoft+Json+Linq+JTokenEqualityComparer")]
#[repr(C)]
#[derive(Debug)]
pub struct JTokenEqualityComparer {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "Newtonsoft+Json+Linq+JTokenEqualityComparer")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Newtonsoft::Json::Linq::JTokenEqualityComparer
    => "Newtonsoft.Json.Linq"."JTokenEqualityComparer"
);
#[cfg(feature = "Newtonsoft+Json+Linq+JTokenEqualityComparer")]
impl std::ops::Deref for crate::Newtonsoft::Json::Linq::JTokenEqualityComparer {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Newtonsoft+Json+Linq+JTokenEqualityComparer")]
impl std::ops::DerefMut for crate::Newtonsoft::Json::Linq::JTokenEqualityComparer {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Newtonsoft+Json+Linq+JTokenEqualityComparer")]
impl crate::Newtonsoft::Json::Linq::JTokenEqualityComparer {
    pub fn Equals(
        &mut self,
        x: quest_hook::libil2cpp::Gc<crate::Newtonsoft::Json::Linq::JToken>,
        y: quest_hook::libil2cpp::Gc<crate::Newtonsoft::Json::Linq::JToken>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("Equals", (x, y))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetHashCode(
        &mut self,
        obj: quest_hook::libil2cpp::Gc<crate::Newtonsoft::Json::Linq::JToken>,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("GetHashCode", (obj))?;
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
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Newtonsoft+Json+Linq+JTokenEqualityComparer")]
impl quest_hook::libil2cpp::ObjectType
for crate::Newtonsoft::Json::Linq::JTokenEqualityComparer {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "Newtonsoft+Json+Linq+JTokenEqualityComparer")]
impl AsRef<
    crate::System::Collections::Generic::IEqualityComparer_1<
        *mut crate::Newtonsoft::Json::Linq::JToken,
    >,
> for crate::Newtonsoft::Json::Linq::JTokenEqualityComparer {
    fn as_ref(
        &self,
    ) -> &crate::System::Collections::Generic::IEqualityComparer_1<
        *mut crate::Newtonsoft::Json::Linq::JToken,
    > {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "Newtonsoft+Json+Linq+JTokenEqualityComparer")]
impl AsMut<
    crate::System::Collections::Generic::IEqualityComparer_1<
        *mut crate::Newtonsoft::Json::Linq::JToken,
    >,
> for crate::Newtonsoft::Json::Linq::JTokenEqualityComparer {
    fn as_mut(
        &mut self,
    ) -> &mut crate::System::Collections::Generic::IEqualityComparer_1<
        *mut crate::Newtonsoft::Json::Linq::JToken,
    > {
        unsafe { std::mem::transmute(self) }
    }
}
