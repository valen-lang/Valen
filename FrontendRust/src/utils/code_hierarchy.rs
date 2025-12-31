// From Frontend/Utils/src/dev/vale/CodeHierarchy.scala

use std::collections::HashMap;
use std::sync::Arc;

// From CodeHierarchy.scala lines 104-189
#[derive(Clone)]
pub struct FileCoordinateMap<Contents> {
    pub package_coord_to_file_coords: HashMap<Arc<PackageCoordinate>, Vec<Arc<FileCoordinate>>>,
    pub file_coord_to_contents: HashMap<Arc<FileCoordinate>, Contents>,
}

impl<Contents: Clone> FileCoordinateMap<Contents> {
    pub fn new() -> Self {
        FileCoordinateMap {
            package_coord_to_file_coords: HashMap::new(),
            file_coord_to_contents: HashMap::new(),
        }
    }
    
    // From CodeHierarchy.scala lines 112-114
    pub fn apply(&self, coord: &Arc<FileCoordinate>) -> &Contents {
        self.file_coord_to_contents.get(coord)
            .expect("FileCoordinateMap::apply - coordinate not found")
    }
    
    // From CodeHierarchy.scala lines 118-127: putPackage
    pub fn put_package(
        &mut self,
        package_coord: Arc<PackageCoordinate>,
        new_file_coord_to_contents: HashMap<Arc<FileCoordinate>, Contents>,
    ) {
        let file_coords: Vec<Arc<FileCoordinate>> = new_file_coord_to_contents.keys().cloned().collect();
        self.package_coord_to_file_coords.insert(package_coord, file_coords);
        
        for (file_coord, contents) in new_file_coord_to_contents {
            self.file_coord_to_contents.insert(file_coord, contents);
        }
    }
    
    // From CodeHierarchy.scala lines 129-135: put
    pub fn put(&mut self, file_coord: Arc<FileCoordinate>, contents: Contents) {
        assert!(!self.file_coord_to_contents.contains_key(&file_coord),
            "FileCoordinateMap::put - file coordinate already exists");
        
        self.file_coord_to_contents.insert(file_coord.clone(), contents);
        
        let package_coord = file_coord.package_coord.clone();
        let file_coords = self.package_coord_to_file_coords
            .entry(package_coord)
            .or_insert_with(Vec::new);
        file_coords.push(file_coord);
    }
    
    // From CodeHierarchy.scala lines 137-143: map
    pub fn map<T, F>(&self, func: F) -> FileCoordinateMap<T>
    where
        F: Fn(&Arc<FileCoordinate>, &Contents) -> T,
        T: Clone,
    {
        let mut result_file_coord_to_contents = HashMap::new();
        for (file_coord, contents) in &self.file_coord_to_contents {
            result_file_coord_to_contents.insert(file_coord.clone(), func(file_coord, contents));
        }
        
        FileCoordinateMap {
            package_coord_to_file_coords: self.package_coord_to_file_coords.clone(),
            file_coord_to_contents: result_file_coord_to_contents,
        }
    }
    
    // From CodeHierarchy.scala lines 145-149: flatMap
    pub fn flat_map<T, F>(&self, func: F) -> Vec<T>
    where
        F: Fn(&Arc<FileCoordinate>, &Contents) -> T,
    {
        self.file_coord_to_contents
            .iter()
            .map(|(file_coord, contents)| func(file_coord, contents))
            .collect()
    }
    
    // From CodeHierarchy.scala lines 151-153: expectOne
    pub fn expect_one(&self) -> &Contents {
        assert!(self.file_coord_to_contents.len() == 1,
            "FileCoordinateMap::expect_one - expected exactly one entry");
        self.file_coord_to_contents.values().next().unwrap()
    }
}

// From CodeHierarchy.scala lines 109, 178-188: IPackageResolver implementation
impl<Contents: Clone> IPackageResolver<HashMap<String, Contents>> for FileCoordinateMap<Contents> {
    fn resolve(&self, package_coord: &Arc<PackageCoordinate>) -> Option<HashMap<String, Contents>> {
        self.package_coord_to_file_coords
            .get(package_coord)
            .map(|file_coords| {
                file_coords
                    .iter()
                    .map(|file_coord| {
                        let contents = self.file_coord_to_contents.get(file_coord)
                            .expect("FileCoordinateMap::resolve - file coord not found in contents");
                        (file_coord.filepath.clone(), contents.clone())
                    })
                    .collect()
            })
    }
}


// TODO: move to utils/code_hierarchy.rs
/// File coordinate matching Scala's FileCoordinate
/// Interned.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct FileCoordinate {
    pub package_coord: Arc<PackageCoordinate>,
    pub filepath: String,
}

// TODO: move to utils/code_hierarchy.rs
/// Package coordinate matching Scala's PackageCoordinate
/// Interned.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct PackageCoordinate {
    pub module: Arc<crate::StrI>,
    pub packages: Vec<Arc<crate::StrI>>,
}

impl PackageCoordinate {
    // From CodeHierarchy.scala line 50: BUILTIN
    pub fn builtin(interner: &Arc<crate::Interner>, keywords: &Arc<crate::Keywords>) -> Arc<PackageCoordinate> {
        interner.intern_package_coordinate(PackageCoordinate {
            module: keywords.empty_string.clone(),
            packages: vec![],
        })
    }
}

// TODO: move to utils/code_hierarchy.rs
/// From CodeHierarchy.scala lines 218-230: IPackageResolver trait
/// Note: Uses parsing::ast::PackageCoordinate (the one used by the parser)
pub trait IPackageResolver<T> {
    fn resolve(&self, package_coord: &Arc<PackageCoordinate>) -> Option<T>;
    
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

// TODO: move to utils/code_hierarchy.rs
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
    fn resolve(&self, package_coord: &Arc<PackageCoordinate>) -> Option<T> {
        self.primary.resolve(package_coord)
            .or_else(|| self.fallback.resolve(package_coord))
    }
}

// TODO: move to utils/code_hierarchy.rs
/// Implement IPackageResolver for function pointers (for lambda-style resolvers)
impl<T, F> IPackageResolver<T> for F
where
    F: Fn(&Arc<PackageCoordinate>) -> Option<T>,
{
    fn resolve(&self, package_coord: &Arc<PackageCoordinate>) -> Option<T> {
        self(package_coord)
    }
}

