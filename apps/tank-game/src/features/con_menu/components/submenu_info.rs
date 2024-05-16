use bevy::prelude::Component;

#[derive(Clone, PartialEq)]
pub enum SubMenuType {
    Base,
    Factory,
}

#[derive(Component)]
pub struct SubMenuInfo {
    sub_menu_type: SubMenuType,
}

impl SubMenuInfo {
    pub fn new(sub_menu_type: SubMenuType) -> Self {
        Self { sub_menu_type }
    }

    pub fn get_sub_menu_type(&self) -> SubMenuType {
        self.sub_menu_type.clone()
    }

    pub fn is_base(&self) -> bool {
        matches!(self.sub_menu_type, SubMenuType::Base)
    }

    pub fn is_factory(&self) -> bool {
        matches!(self.sub_menu_type, SubMenuType::Factory)
    }
}
