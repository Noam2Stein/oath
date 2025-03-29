use crate::*;

pub struct DumbNameContext {
    names: Vec<DumbName>,
    namespaces: Vec<Namespace>,
}

pub struct ResNameContext {
    names: Vec<ResName>,
    namespaces: Vec<Namespace>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct NameId(usize);

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct NamespaceId(usize);

impl DumbNameContext {
    pub fn new() -> Self {
        Self {
            names: Vec::new(),
            namespaces: Vec::new(),
        }
    }

    pub fn push_name(&mut self, name: DumbName) -> NameId {
        let id = NameId(self.names.len());

        self.names.push(name);

        id
    }

    pub fn push_namespace(&mut self, namespace: Namespace) -> NamespaceId {
        let id = NamespaceId(self.names.len());

        self.namespaces.push(namespace);

        id
    }

    pub fn get_name(&self, name: NameId) -> &DumbName {
        &self.names[name.0]
    }

    pub fn namespace(&self, name: NamespaceId) -> &Namespace {
        &self.namespaces[name.0]
    }
    pub fn namespace_mut(&mut self, name: NamespaceId) -> &mut Namespace {
        &mut self.namespaces[name.0]
    }

    pub fn resolve(self) -> ResNameContext {
        let mut names = Vec::with_capacity(self.names.len());
        let namespaces = self.namespaces;

        for name in self.names {
            match name {
                DumbName::Type(name, type_namespace) => match name {
                    DumbType::Struct(name) => {
                        
                    }
                },
            }
        }

        ResNameContext { names, namespaces }
    }
}
