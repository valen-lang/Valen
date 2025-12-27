// From Frontend/Utils/src/dev/vale/CodeHierarchy.scala

/// Result type matching Scala's Result[T, E]
pub type Result<T, E> = std::result::Result<T, E>;

/// File coordinate matching Scala's FileCoordinate
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct FileCoordinate {
    pub package_coord: PackageCoordinate,
    pub filepath: String,
}

/// Package coordinate matching Scala's PackageCoordinate
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct PackageCoordinate {
    pub module: crate::StrI,
    pub packages: Vec<crate::StrI>,
}

/// From CodeHierarchy.scala lines 218-230: IPackageResolver trait
/// Note: Uses parsing::ast::PackageCoordinate (the one used by the parser)
pub trait IPackageResolver<T> {
    fn resolve(&self, package_coord: &crate::parsing::ast::PackageCoordinate) -> Option<T>;
    
    // From CodeHierarchy.scala lines 221-229: or() method for chaining resolvers
    fn or<F>(self, fallback: F) -> OrResolver<Self, F>
    where
        Self: Sized,
        F: IPackageResolver<T>,
    {
        OrResolver {
            primary: self,
            fallback,
        }
    }
}

/// From CodeHierarchy.scala lines 221-229: Chained resolver implementation
pub struct OrResolver<P, F> {
    primary: P,
    fallback: F,
}

impl<T, P, F> IPackageResolver<T> for OrResolver<P, F>
where
    P: IPackageResolver<T>,
    F: IPackageResolver<T>,
{
    fn resolve(&self, package_coord: &crate::parsing::ast::PackageCoordinate) -> Option<T> {
        self.primary.resolve(package_coord)
            .or_else(|| self.fallback.resolve(package_coord))
    }
}

/// Implement IPackageResolver for function pointers (for lambda-style resolvers)
impl<T, F> IPackageResolver<T> for F
where
    F: Fn(&crate::parsing::ast::PackageCoordinate) -> Option<T>,
{
    fn resolve(&self, package_coord: &crate::parsing::ast::PackageCoordinate) -> Option<T> {
        self(package_coord)
    }
}

