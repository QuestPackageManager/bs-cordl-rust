#[cfg(feature = "UnityEngine+ProBuilder+Poly2Tri+DTSweep")]
#[repr(C)]
#[derive(Debug)]
pub struct DTSweep {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "UnityEngine+ProBuilder+Poly2Tri+DTSweep")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::ProBuilder::Poly2Tri::DTSweep =>
    "UnityEngine.ProBuilder.Poly2Tri"."DTSweep"
);
#[cfg(feature = "UnityEngine+ProBuilder+Poly2Tri+DTSweep")]
impl std::ops::Deref for crate::UnityEngine::ProBuilder::Poly2Tri::DTSweep {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+ProBuilder+Poly2Tri+DTSweep")]
impl std::ops::DerefMut for crate::UnityEngine::ProBuilder::Poly2Tri::DTSweep {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+ProBuilder+Poly2Tri+DTSweep")]
impl crate::UnityEngine::ProBuilder::Poly2Tri::DTSweep {
    pub const PI_3div4: f64 = 2.356194490192345f64;
    pub const PI_div2: f64 = 1.5707963267948966f64;
    #[cfg(feature = "UnityEngine+ProBuilder+Poly2Tri+DTSweep+__c__DisplayClass21_0")]
    pub type __c__DisplayClass21_0 = crate::UnityEngine::ProBuilder::Poly2Tri::DTSweep___c__DisplayClass21_0;
}
#[cfg(feature = "UnityEngine+ProBuilder+Poly2Tri+DTSweep")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::ProBuilder::Poly2Tri::DTSweep {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
