#[cfg(feature = "System+ComponentModel+MarshalByValueComponent")]
#[repr(C)]
#[derive(Debug)]
pub struct MarshalByValueComponent {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _site: quest_hook::libil2cpp::Gc<crate::System::ComponentModel::ISite>,
    pub _events: quest_hook::libil2cpp::Gc<
        crate::System::ComponentModel::EventHandlerList,
    >,
}
#[cfg(feature = "System+ComponentModel+MarshalByValueComponent")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::ComponentModel::MarshalByValueComponent
    => "System.ComponentModel"."MarshalByValueComponent"
);
#[cfg(feature = "System+ComponentModel+MarshalByValueComponent")]
impl std::ops::Deref for crate::System::ComponentModel::MarshalByValueComponent {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+ComponentModel+MarshalByValueComponent")]
impl std::ops::DerefMut for crate::System::ComponentModel::MarshalByValueComponent {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+ComponentModel+MarshalByValueComponent")]
impl crate::System::ComponentModel::MarshalByValueComponent {
    pub fn Dispose_0(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Dispose", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn Dispose__cordl_bool1(
        &mut self,
        disposing: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Dispose", (disposing))?;
        Ok(__cordl_ret.into())
    }
    pub fn Finalize(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Finalize", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetService(
        &mut self,
        service: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppObject,
        > = __cordl_object.invoke("GetService", (service))?;
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn ToString(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("ToString", ())?;
        Ok(__cordl_ret.into())
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
    pub fn get_Site(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::ComponentModel::ISite>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::ComponentModel::ISite,
        > = __cordl_object.invoke("get_Site", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+ComponentModel+MarshalByValueComponent")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::ComponentModel::MarshalByValueComponent {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "System+ComponentModel+MarshalByValueComponent")]
impl AsRef<crate::System::ComponentModel::IComponent>
for crate::System::ComponentModel::MarshalByValueComponent {
    fn as_ref(&self) -> &crate::System::ComponentModel::IComponent {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "System+ComponentModel+MarshalByValueComponent")]
impl AsMut<crate::System::ComponentModel::IComponent>
for crate::System::ComponentModel::MarshalByValueComponent {
    fn as_mut(&mut self) -> &mut crate::System::ComponentModel::IComponent {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "System+ComponentModel+MarshalByValueComponent")]
impl AsRef<crate::System::IDisposable>
for crate::System::ComponentModel::MarshalByValueComponent {
    fn as_ref(&self) -> &crate::System::IDisposable {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "System+ComponentModel+MarshalByValueComponent")]
impl AsMut<crate::System::IDisposable>
for crate::System::ComponentModel::MarshalByValueComponent {
    fn as_mut(&mut self) -> &mut crate::System::IDisposable {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "System+ComponentModel+MarshalByValueComponent")]
impl AsRef<crate::System::IServiceProvider>
for crate::System::ComponentModel::MarshalByValueComponent {
    fn as_ref(&self) -> &crate::System::IServiceProvider {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "System+ComponentModel+MarshalByValueComponent")]
impl AsMut<crate::System::IServiceProvider>
for crate::System::ComponentModel::MarshalByValueComponent {
    fn as_mut(&mut self) -> &mut crate::System::IServiceProvider {
        unsafe { std::mem::transmute(self) }
    }
}
