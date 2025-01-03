#[cfg(feature = "OculusMetrics")]
#[repr(C)]
#[derive(Debug)]
pub struct OculusMetrics {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _metrics: *mut quest_hook::libil2cpp::Il2CppArray<
        *mut quest_hook::libil2cpp::Il2CppString,
    >,
    pub _sample: *mut quest_hook::libil2cpp::Il2CppArray<f32>,
    pub _samples: *mut crate::System::Collections::Generic::List_1<f32>,
}
#[cfg(feature = "OculusMetrics")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::OculusMetrics => ""
    ."OculusMetrics"
);
#[cfg(feature = "OculusMetrics")]
impl std::ops::Deref for crate::GlobalNamespace::OculusMetrics {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OculusMetrics")]
impl std::ops::DerefMut for crate::GlobalNamespace::OculusMetrics {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OculusMetrics")]
impl crate::GlobalNamespace::OculusMetrics {
    pub fn CaptureFrame(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("CaptureFrame", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn Dispose(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Dispose", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GenerateListReport(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("GenerateListReport", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GenerateTableReport(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("GenerateTableReport", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn New(
        initialFrameCapacity: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (initialFrameCapacity))?;
        Ok(__cordl_object.into())
    }
    pub fn RecordMetrics(
        initialFrameCapacity: i32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::OculusMetrics>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::OculusMetrics,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("RecordMetrics", (initialFrameCapacity))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        initialFrameCapacity: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (initialFrameCapacity))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "OculusMetrics")]
impl quest_hook::libil2cpp::ObjectType for crate::GlobalNamespace::OculusMetrics {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "OculusMetrics")]
impl AsRef<crate::System::IDisposable> for crate::GlobalNamespace::OculusMetrics {
    fn as_ref(&self) -> &crate::System::IDisposable {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "OculusMetrics")]
impl AsMut<crate::System::IDisposable> for crate::GlobalNamespace::OculusMetrics {
    fn as_mut(&mut self) -> &mut crate::System::IDisposable {
        unsafe { std::mem::transmute(self) }
    }
}
