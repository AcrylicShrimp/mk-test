use super::component_type::ComponentType;
use super::container_factory::ContainerFactory;

pub struct ComponentTypeWithFactory {
    component_type: ComponentType,
    factory: Box<dyn ContainerFactory>,
}

impl ComponentTypeWithFactory {
    pub fn new(
        component_type: ComponentType,
        factory: Box<dyn ContainerFactory>,
    ) -> ComponentTypeWithFactory {
        ComponentTypeWithFactory {
            component_type,
            factory,
        }
    }

    pub fn component_type(&self) -> &ComponentType {
        &self.component_type
    }

    pub fn factory(&self) -> &dyn ContainerFactory {
        &*self.factory
    }
}
