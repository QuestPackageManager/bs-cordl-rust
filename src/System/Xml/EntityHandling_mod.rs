#[cfg(feature = "System+Xml+EntityHandling")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum EntityHandling {
    #[default]
    ExpandCharEntities = 2i32,
    ExpandEntities = 1i32,
}
#[cfg(feature = "System+Xml+EntityHandling")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::System::Xml::EntityHandling => "System.Xml"
    ."EntityHandling"
);
